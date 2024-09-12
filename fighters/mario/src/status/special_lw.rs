use super::*;

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        VarModule::on_flag(fighter.battle_object, vars::mario::status::SPECIAL_LW_GROUND_START);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_light"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        VarModule::off_flag(fighter.battle_object, vars::mario::status::SPECIAL_LW_GROUND_START);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_light"), 0.0, 1.0, false, 0.0, false, false);
    }
    special_lw_set_kinetic(fighter, false);

    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_light"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            special_lw_set_kinetic(fighter, true);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_light"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if fighter.status_frame() == 30 {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            let air_speed_x_stable = fighter.get_param_float("air_speed_x_stable", "");
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * 0.2, 0.0);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.2);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.2);
        }
        else {
            let air_accel_y = fighter.get_param_float("air_accel_y", "");
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y * 0.5);
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT.into() } else { FIGHTER_STATUS_KIND_FALL.into() };
        fighter.change_status(status, false.into());
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_lw_set_kinetic(fighter: &mut L2CFighterCommon, edge_slide: bool) {
    if !VarModule::is_flag(fighter.battle_object, vars::mario::instance::SPECIAL_LW_DISABLE_STALL) {
        let air_accel_y = fighter.get_param_float("air_accel_y", "");
        let air_speed_y_stable = fighter.get_param_float("air_speed_y_stable", "");
        let start_speed_y = if edge_slide {
            if fighter.status_frame() < 25 {
                ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.edge_early_start_speed_y")
            } else {
                ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.edge_late_start_speed_y")
            }
        } else {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.air_start_speed_y")
        };
        let air_accel_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.air_accel_y_mul");
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, start_speed_y);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y * air_accel_y_mul);
    }
    
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let air_speed_x_stable = fighter.get_param_float("air_speed_x_stable", "");
    let air_accel_x_add = fighter.get_param_float("air_accel_x_add", "");
    let air_accel_x_mul = fighter.get_param_float("air_accel_x_mul", "");
    let speed_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.speed_x_mul");
    let accel_x_mul = if fighter.is_situation(*SITUATION_KIND_GROUND) {
        ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.ground_accel_x_mul")
    } else {
        ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.air_accel_x_mul")
    };
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x * speed_x_mul, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * speed_x_mul, 0.0);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * speed_x_mul, 0.0);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add * accel_x_mul);
    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul * accel_x_mul);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
}

unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end);
}