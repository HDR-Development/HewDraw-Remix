use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    return false.into();
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int64(fighter.battle_object, vars::robot::status::SPECIAL_S_MOTION_KIND, hash40("special_s"));
    VarModule::set_int64(fighter.battle_object, vars::robot::status::SPECIAL_S_MOTION_KIND_AIR, hash40("special_air_s"));

    special_s_motion_helper(fighter, false);
    special_s_kinetic_helper(fighter);

    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into();
        }
    }

    let mut force_change = false;
    if VarModule::is_flag(fighter.battle_object, vars::robot::status::SPECIAL_S_BRANCH_DECIDE) {
        VarModule::off_flag(fighter.battle_object, vars::robot::status::SPECIAL_S_BRANCH_DECIDE);
        let stick_y =  ControlModule::get_stick_y(fighter.module_accessor);
        let (ground, air) = if stick_y > 0.3 {
            ("special_s_hi", "special_air_s_hi")
        }
        else if stick_y < -0.3 {
            ("special_s_lw", "special_air_s_lw")
        }
        else {
            ("special_s", "special_air_s")
        };

        VarModule::set_int64(fighter.battle_object, vars::robot::status::SPECIAL_S_MOTION_KIND, hash40(ground));
        VarModule::set_int64(fighter.battle_object, vars::robot::status::SPECIAL_S_MOTION_KIND_AIR, hash40(air));

        force_change = true;
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_s_motion_helper(fighter, true);
        special_s_kinetic_helper(fighter);
    }
    else if force_change {
        special_s_motion_helper(fighter, true);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return true.into();
    }

    return false.into();
}

unsafe extern "C" fn special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if !VarModule::is_flag(fighter.battle_object, vars::robot::status::SPECIAL_S_FALL) {
            let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let ground_accel_x_add = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.ground_accel_x_add");
            let ground_accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.ground_accel_x_mul");
            let left_stick_x = fighter.left_stick_x();
            if left_stick_x.abs() > 0.0 {
                let drift = left_stick_x * ground_accel_x_mul + left_stick_x.signum() * ground_accel_x_add;
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x + drift);
            }
        }
    } else {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if VarModule::is_flag(fighter.battle_object, vars::robot::status::SPECIAL_S_FALL) {
            let air_speed_x_stable = fighter.get_param_float("air_speed_x_stable", "");
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable, 0.0);

            let air_accel_y = fighter.get_param_float("air_accel_y", "");
            let air_speed_y_stable = fighter.get_param_float("air_speed_y_stable", "");
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
            return false.into();
        }
    }
    return false.into();
}

unsafe extern "C" fn special_s_motion_helper(fighter: &mut L2CFighterCommon, inherit: bool) {
    let (correct, motion_const) = if fighter.is_situation(*SITUATION_KIND_GROUND) {
        (*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK, vars::robot::status::SPECIAL_S_MOTION_KIND)
    }
    else {
        (*GROUND_CORRECT_KIND_AIR, vars::robot::status::SPECIAL_S_MOTION_KIND_AIR)
    };
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    let motion = VarModule::get_int64(fighter.battle_object, motion_const);
    if inherit {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new_raw(motion),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(motion),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
}

unsafe extern "C" fn special_s_kinetic_helper(fighter: &mut L2CFighterCommon) {
    let start_mul_spd_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.start_mul_spd_x");
    KineticModule::mul_speed(fighter.module_accessor, &Vector3f { x: start_mul_spd_x, y: 1.0, z: 1.0 }, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        // FIGHTER_KINETIC_ENERGY_ID_STOP
        let ground_speed_x_limit = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.ground_speed_x_limit");
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ground_speed_x_limit, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x);
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        // FIGHTER_KINETIC_ENERGY_ID_CONTROL
        let air_speed_x_limit = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.air_speed_x_limit");
        let air_speed_x_stable = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.air_speed_x_stable");
        let air_accel_x_add = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.air_accel_x_add");
        let air_accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.air_accel_x_mul");
        let air_brake_x = fighter.get_param_float("air_brake_x", "");
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_limit, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

        // FIGHTER_KINETIC_ENERGY_ID_STOP
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -air_brake_x, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

        // FIGHTER_KINETIC_ENERGY_ID_GRAVITY
        if VarModule::is_flag(fighter.battle_object, vars::robot::instance::SPECIAL_S_AIR_USED) {
            let air_accel_y = fighter.get_param_float("air_accel_y", "");
            let air_speed_y_stable = fighter.get_param_float("air_speed_y_stable", "");
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
            VarModule::off_flag(fighter.battle_object, vars::robot::status::SPECIAL_S_HOP);
        } else {
            let start_mul_spd_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.start_mul_spd_y");
            let attack_acl_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.attack_acl_y");
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y * start_mul_spd_y, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -attack_acl_y);
            VarModule::on_flag(fighter.battle_object, vars::robot::status::SPECIAL_S_HOP);
        }
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        VarModule::on_flag(fighter.battle_object, vars::robot::instance::SPECIAL_S_AIR_USED);
    }
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
}

unsafe extern "C" fn stub_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exec);

    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
    agent.status(ExecStop, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
}
