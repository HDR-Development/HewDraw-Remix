use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int64(fighter.battle_object, vars::robot::status::SPECIAL_S_MOTION_KIND, hash40("special_s"));
    VarModule::set_int64(fighter.battle_object, vars::robot::status::SPECIAL_S_MOTION_KIND_AIR, hash40("special_air_s"));

    special_s_motion_helper(fighter, false);

    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if VarModule::is_flag(fighter.battle_object, vars::robot::status::SPECIAL_S_HOP) {
        VarModule::off_flag(fighter.battle_object, vars::robot::status::SPECIAL_S_HOP);
        let lr = PostureModule::lr(fighter.module_accessor);
        let speed_x = 1.1;
        let speed_y = 0.6;
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                speed_x * lr,
                0.0
            );
        }
        else if !VarModule::is_flag(fighter.battle_object, vars::robot::instance::SPECIAL_S_AIR_USED) {
            VarModule::on_flag(fighter.battle_object, vars::robot::instance::SPECIAL_S_AIR_USED);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                speed_x * lr,
                0.0
            );
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                speed_y
            );
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

    if StatusModule::is_situation_changed(fighter.module_accessor)
    || force_change {
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
    }

    0.into()
}

unsafe extern "C" fn special_s_motion_helper(fighter: &mut L2CFighterCommon, inherit: bool) {
    let (kinetic, correct, motion_const) = if fighter.is_situation(*SITUATION_KIND_GROUND) {
        (*FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK, vars::robot::status::SPECIAL_S_MOTION_KIND)
    }
    else {
        (*FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR, vars::robot::status::SPECIAL_S_MOTION_KIND_AIR)
    };
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
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

unsafe extern "C" fn stub_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);

    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
    agent.status(ExecStop, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, stub_status);
}
