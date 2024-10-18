use super::*;

// FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N

unsafe extern "C" fn special_n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_N_FOOT);
    } else {fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_N_FOOT); }
    VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, 0);
    smashline::original_status(Init, fighter, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N)(fighter)
}

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT) {
        fighter.sub_change_motion_by_situation(Hash40::new("bayonetta_special_n_start_f").into(), Hash40::new("bayonetta_special_air_n_start_f").into(), false.into());
    } else {
        fighter.sub_change_motion_by_situation(Hash40::new("bayonetta_special_n_start_h").into(), Hash40::new("bayonetta_special_air_n_start_h").into(), false.into());
    }
    fighter.set_float(1.0, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLOAT_MOTION_RATE);
    motion_handling(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {motion_handling(fighter); }
    if MotionModule::is_end(fighter.module_accessor) {fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_CHARGE.into(), false.into()); }
    return 0.into();
}

unsafe extern "C" fn special_n_charge_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn special_n_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT) {
        fighter.sub_change_motion_by_situation(Hash40::new("bayonetta_special_n_charge_f").into(), Hash40::new("bayonetta_special_air_n_charge_f").into(), false.into());
    } else {
        fighter.sub_change_motion_by_situation(Hash40::new("bayonetta_special_n_charge_h").into(), Hash40::new("bayonetta_special_air_n_charge_h").into(), false.into());
    }
    fighter.set_float(1.0, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLOAT_MOTION_RATE);
    motion_handling(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_charge_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {motion_handling(fighter); }
    if !StopModule::is_stop(fighter.module_accessor) {cancel_check(fighter); }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CHARGE_FRAME);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    if fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CHARGE_FRAME) <= fighter.get_param_int("param_special_n", "charge_frame_max") {
        if fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_STEP) == 0 {
            if MotionModule::is_end(fighter.module_accessor) {
                if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT) {
                    fighter.sub_change_motion_by_situation(Hash40::new("bayonetta_special_n_loop_f").into(), Hash40::new("bayonetta_special_air_n_loop_f").into(), true.into());
                } else {
                    fighter.sub_change_motion_by_situation(Hash40::new("bayonetta_special_n_loop_h").into(), Hash40::new("bayonetta_special_air_n_loop_h").into(), true.into());
                }
                app::FighterUtil::flash_eye_info(fighter.module_accessor);
                PLAY_SE(fighter, Hash40::new("se_bayonetta_special_n05"));
                fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_CHARGE_MAX);
                fighter.set_int(1, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_STEP);
            }
        }
    } else {// charge limit
        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_FIRE.into(), false.into());
    }
    return 0.into();
}

unsafe extern "C" fn special_n_cancel_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_BAYONETTA_STATUS_WORK_KEEP_FLAG_SPECIAL_N_FLAG,
        *FIGHTER_BAYONETTA_STATUS_WORK_KEEP_FLAG_SPECIAL_N_INT,
        *FIGHTER_BAYONETTA_STATUS_WORK_KEEP_FLAG_SPECIAL_N_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_n_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT) {
        fighter.sub_change_motion_by_situation(Hash40::new("bayonetta_special_n_end_f").into(), Hash40::new("bayonetta_special_air_n_end_f").into(), false.into());
    } else {
        fighter.sub_change_motion_by_situation(Hash40::new("bayonetta_special_n_end_h").into(), Hash40::new("bayonetta_special_air_n_end_h").into(), false.into());
    }
    let cancel_frame = fighter.get_param_int("param_special_n", "cancel_frame");
    fighter.set_int(cancel_frame, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME);
    //fighter.set_float(1.0, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLOAT_MOTION_RATE);
    motion_handling(fighter);
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("bayonetta_bulletclimax_circle"), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("bayonetta_chargebullet_start"), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_cancel_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME);
    if fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME) == 0 {
        let status = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE);
        if status != 0 {
            StatusModule::change_status_force(fighter.module_accessor, status, false);
            return 1.into();
        } else {CancelModule::enable_cancel(fighter.module_accessor); }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into()); }
        else {fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into()); }
    }
    return 0.into();
}

unsafe extern "C" fn special_n_cancel_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn motion_handling(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        //motion
        if fighter.is_motion(Hash40::new("bayonetta_special_air_n_charge_h")) {MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("bayonetta_special_n_charge_h"), -1.0, 1.0, 0.0, true, true); }
        else if fighter.is_motion(Hash40::new("bayonetta_special_air_n_start_h")) {MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("bayonetta_special_n_start_h"), -1.0, 1.0, 0.0, true, true); }
        else if fighter.is_motion(Hash40::new("bayonetta_special_air_n_loop_h")) {MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("bayonetta_special_n_loop_h"), -1.0, 1.0, 0.0, true, true); }
        else if fighter.is_motion(Hash40::new("bayonetta_special_air_n_end_h")) {MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("bayonetta_special_n_end_h"), -1.0, 1.0, 0.0, true, true); }
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let air_accel_y = fighter.get_param_float("param_special_n", "air_start_accel_y");
        let air_stable_y = fighter.get_param_float("param_special_n", "air_start_max_speed_y");
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_stable_y);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.04);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.009);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
    }
    return 0.into();
}

unsafe extern "C" fn cancel_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_button_off(Buttons::Special) {fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_FIRE.into(), false.into()); }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.is_cat_flag(Cat2::StickEscape) {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, *FIGHTER_STATUS_KIND_ESCAPE);
            StatusModule::change_status_force(fighter.module_accessor, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, false);
        }
        else if fighter.is_cat_flag(Cat2::StickEscapeF) {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, *FIGHTER_STATUS_KIND_ESCAPE_F);
            StatusModule::change_status_force(fighter.module_accessor, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, false);
        }
        else if fighter.is_cat_flag(Cat2::StickEscapeB) {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, *FIGHTER_STATUS_KIND_ESCAPE_B);
            StatusModule::change_status_force(fighter.module_accessor, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, false);
        }
        else if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump) && fighter.sub_check_button_frick().get_bool())) {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, *FIGHTER_STATUS_KIND_JUMP_SQUAT);
            StatusModule::change_status_force(fighter.module_accessor, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, false);
        }
        if fighter.sub_check_command_guard().get_bool() {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, *FIGHTER_STATUS_KIND_WAIT);
            StatusModule::change_status_force(fighter.module_accessor, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, false);
        }
    } else {
        fighter.check_jump_cancel(false, false);
        if fighter.is_cat_flag(Cat1::AirEscape) {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, *FIGHTER_STATUS_KIND_FALL);
            StatusModule::change_status_force(fighter.module_accessor, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, false);
        }
    }
    return 0.into();
}

pub fn install(agent: &mut Agent) {
        agent.status(Init, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N, special_n_init);
        agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N, special_n_main);
        agent.status(Init, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_CHARGE, special_n_charge_init);
        agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_CHARGE, special_n_charge_main);
        agent.status(Pre, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, special_n_cancel_pre);
        agent.status(Main, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, special_n_cancel_main);
        agent.status(End, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, special_n_cancel_end);
}