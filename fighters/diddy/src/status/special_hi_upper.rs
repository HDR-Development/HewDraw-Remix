use super::*;

pub unsafe extern "C" fn special_hi_upper_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut start_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_jet_start_spd_y"));
    let lr = PostureModule::lr(fighter.module_accessor);
    let special_hi_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_HI_WORK_INT_SITUATION);
    if special_hi_situation == *SITUATION_KIND_AIR {
        let mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_jet_air_start_spd_y_mul"));
        start_spd_y *= mul;
    }

    let charge_value = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_VALUE) as f32;
    let start_spd_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_jet_start_spd_y_mul"));
    start_spd_y += charge_value * start_spd_y_mul;

    let start_spd_y_mul_powerup = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_jet_start_spd_y_mul_powerup"));
    start_spd_y *= start_spd_y_mul_powerup;

    let start_angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_HI_WORK_FLOAT_UPPER_START_ANGLE);
    let radians = start_angle.to_radians();

    let spd_x = start_spd_y * radians.cos() * lr;
    let spd_y = start_spd_y * radians.sin();

    WorkModule::set_float(fighter.module_accessor, start_angle, *FIGHTER_DIDDY_STATUS_SPECIAL_HI_WORK_FLOAT_UPPER_ANGLE);

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
        0.0,
        spd_y,
        0.0,
        0.0,
        0.0
    );

    sv_kinetic_energy!(
        enable,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY
    );

    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -air_accel_y
    );

    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_speed_y_stable
    );

    let air_brake_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_brake_y"), 0);
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_brake_y
    );

    let air_speed_y_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("air_speed_y_limit"));
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_speed_y_limit
    );

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_AIR,
        spd_x,
        0.0,
        0.0,
        0.0,
        0.0
    );

    sv_kinetic_energy!(
        enable,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );

    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        air_speed_y_stable
    );

    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        air_speed_y_limit
    );

    if let Some(target) = smashline::api::get_target_function("lua2cpp_diddy.nrs", 0xba40) {
        let special_hi_upper_momentum: fn(&mut L2CValue, &mut L2CFighterCommon) = std::mem::transmute(target);
        let angle_x = &mut L2CValue::F32(0.0);
        special_hi_upper_momentum(angle_x, fighter);
        WorkModule::set_float(fighter.module_accessor, angle_x.get_f32(), *FIGHTER_DIDDY_STATUS_SPECIAL_HI_WORK_FLOAT_ANGLE_X);
    }

    fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_hi_jump"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    fighter.main_shift(special_hi_upper_main_loop)
}

unsafe extern "C" fn special_hi_upper_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        return 0.into();
    }

    // <HDR>
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 0.into();
    }
    // </HDR>

    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let explosion_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_ground_hit_explosion_speed"));
    let speed_length = sv_math::vec2_length(sum_speed_x, sum_speed_y);
    if explosion_speed < speed_length {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_HI_FLAG_FALL_START)
        && sum_speed_y < 0.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_HI_FLAG_FALL_START);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_HI_FLAG_ROLL_COMP_START) {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_FALL_ROLL, false);
        }
    }
    // else {
        // usually has stuff about bonking but we're just going to completely ignore that
    // }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER, special_hi_upper_main);
}