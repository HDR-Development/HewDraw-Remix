use super::*;

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
        special_n_physics(fighter, true);
    }
    
    fighter.main_shift(special_n_main_loop)
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT.into() } else { FIGHTER_STATUS_KIND_FALL.into() };
        fighter.change_status(status, false.into());
        return 0.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            if fighter.status_frame() < ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_n.dive_end_frame") {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 22.0, 1.0, false, 0.0, false, false);
            }
            else {  // temporary until air anim is created (will probably just make autocancel)
                WorkModule::set_float(fighter.module_accessor, 20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
            special_n_physics(fighter, true);
        }
    }
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if fighter.status_frame() == ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_n.dive_start_frame") {
            special_n_physics(fighter, false);
        }
        if fighter.status_frame() == ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_n.dive_end_frame") {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
    
    return 0.into();
}

unsafe extern "C" fn special_n_physics(fighter: &mut L2CFighterCommon, init: bool) {
    if init {
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let limit_speed_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_limit_speed_y");
        let start_speed_y = speed_y.clamp(0.0, limit_speed_y);
        let accel_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_accel_y");
        let stable_speed_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_stable_speed_y");
        let facing = PostureModule::lr(fighter.module_accessor);
        let brake_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_brake_speed_x");
        let limit_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_limit_speed_x");
        let start_speed_x = speed_x.abs().clamp(0.6, limit_speed_x) * facing;
        let stable_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_stable_speed_x");

        sv_kinetic_energy!(reset_energy, fighter, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, start_speed_y);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, accel_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, limit_speed_y);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, stable_speed_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

        sv_kinetic_energy!(reset_energy, fighter, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, start_speed_x, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, brake_speed_x, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, limit_speed_x * facing, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, stable_speed_x, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let control_x_stable_speed = air_speed_x_stable * ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.start_control_x_mul");
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, control_x_stable_speed, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    else {
        let dive_speed_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.dive_speed_y");
        let dive_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.dive_speed_x");

        sv_kinetic_energy!(reset_energy, fighter, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, dive_speed_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, dive_speed_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

        let facing = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(reset_energy, fighter, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, dive_speed_x * facing, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, dive_speed_x * facing, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
}

unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_end);
}