use super::*;

pub unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    let motion = if VarModule::is_flag(fighter.object(), vars::duckhunt::instance::SPECIAL_HI2_ENABLE) { Hash40::new("special_hi_2") } else { Hash40::new("special_hi") };
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    special_hi_set_physics(fighter, false);

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);

    //ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_RETICLE, false, -1);
    let reticle_motion = if VarModule::is_flag(fighter.object(), vars::duckhunt::instance::SPECIAL_HI2_ENABLE) { Hash40::new("special_hi2") } else { Hash40::new("special_hi") };
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_RETICLE, motion, true, -1.0);

    VarModule::on_flag(fighter.object(), vars::duckhunt::instance::SPECIAL_HI2_ENABLE);
    fighter.main_shift(special_hi_main_loop)
}

unsafe extern "C" fn special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        if fighter.is_motion(Hash40::new("special_hi")) {
            // shot 1
            let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT.into() } else { FIGHTER_STATUS_KIND_FALL.into() };
            fighter.change_status(status, false.into());
            return 1.into();
        }
        else {
            // shot 2
            let speed_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi2_end_speed_x_mul");
            let accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi2_end_accel_x_mul");
            WorkModule::set_float(fighter.module_accessor, speed_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
            WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MUL_FALL_X_ACCEL);
            let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT.into() } else { FIGHTER_STATUS_KIND_FALL_SPECIAL.into() };
            fighter.change_status(status, false.into());
            return 1.into();
        }
    }
    // enable shot physics
    if VarModule::is_flag(fighter.object(), vars::duckhunt::status::SPECIAL_HI_JUMP) {
        VarModule::off_flag(fighter.object(), vars::duckhunt::status::SPECIAL_HI_JUMP);
        special_hi_set_physics(fighter, true);
    }
    // shot 3
    if VarModule::is_flag(fighter.object(), vars::duckhunt::status::SPECIAL_HI2_ENABLE_SHOT)
    && fighter.is_cat_flag(Cat1::SpecialHi) {
        fighter.change_status(FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_set_physics(fighter: &mut L2CFighterCommon, jump: bool) -> L2CValue {
    if !jump {
        let start_speed_mul_x = fighter.get_param_float("param_special_hi", "start_speed_mul_x");
        let calc_speed_x = start_speed_mul_x * fighter.stick_x();
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, calc_speed_x, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

        let speed_y = if fighter.is_motion(Hash40::new("special_hi")) {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi1_start_speed_y")
        } else {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi2_start_speed_y")
        };
        let accel_y = if fighter.is_motion(Hash40::new("special_hi")) {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi1_start_accel_y")
        } else {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi2_start_accel_y")
        };
        let stable_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);

        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, stable_speed_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        
        let speed_y = if fighter.is_motion(Hash40::new("special_hi")) {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi1_jump_speed_y")
        } else {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi2_jump_speed_y")
        };
        let accel_y = if fighter.is_motion(Hash40::new("special_hi")) {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi1_jump_accel_y")
        } else {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi2_jump_accel_y")
        };
        let stable_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);

        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, stable_speed_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_3"), 0.0, 1.0, false, 0.0, false, false);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_RETICLE, Hash40::new("special_hi3"), true, -1.0);
    let speed_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi3_start_speed_y");

    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    fighter.main_shift(special_hi_end_main_loop)
}

unsafe extern "C" fn special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT.into() } else { FIGHTER_STATUS_KIND_FALL_SPECIAL.into() };
        fighter.change_status(status, false.into());
        return 1.into();
    }
    if fighter.status_frame() == ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.hi3_stall_end_frame") {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        
        let speed_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi3_end_speed_x_mul");
        let accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi3_end_accel_x_mul");
        WorkModule::set_float(fighter.module_accessor, speed_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
        WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MUL_FALL_X_ACCEL);
    }
    let control_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.hi3_control_frame");
    if fighter.status_frame() == control_frame {
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    if fighter.status_frame() >= control_frame {
        fighter.sub_air_check_dive();
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_init);
    agent.status(Main, *FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END, special_hi_end_main);
}