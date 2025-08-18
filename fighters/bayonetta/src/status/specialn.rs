use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_N

unsafe extern "C" fn special_n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_N_FOOT);
    } else {fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_N_FOOT); }
    VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, 0);
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
}

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_start_f").into(), Hash40::new("special_air_n_start_f").into(), false.into());
    } else {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_start_h").into(), Hash40::new("special_air_n_start_h").into(), false.into());
    }
    fighter.set_float(1.0, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLOAT_MOTION_RATE);
    motion_handling(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {motion_handling(fighter); }
    if MotionModule::is_end(fighter.module_accessor) {fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE.into(), false.into()); }
    return 0.into();
}

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE

unsafe extern "C" fn special_n_charge_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn special_n_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_charge_f").into(), Hash40::new("special_air_n_charge_f").into(), false.into());
    } else {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_charge_h").into(), Hash40::new("special_air_n_charge_h").into(), false.into());
    }
    fighter.set_float(1.0, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLOAT_MOTION_RATE);
    motion_handling(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_charge_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {motion_handling(fighter); }
    if !StopModule::is_stop(fighter.module_accessor) && cancel_check(fighter).get_bool() {StatusModule::change_status_force(fighter.module_accessor, statuses::bayonetta::SPECIAL_N_CANCEL, false); }
    if fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_STEP) == 0 {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT) {
                fighter.sub_change_motion_by_situation(Hash40::new("special_n_loop_f").into(), Hash40::new("special_air_n_loop_f").into(), true.into());
            } else {
                fighter.sub_change_motion_by_situation(Hash40::new("special_n_loop_h").into(), Hash40::new("special_air_n_loop_h").into(), true.into());
            }
            app::FighterUtil::flash_eye_info(fighter.module_accessor);
            PLAY_SE(fighter, Hash40::new("se_bayonetta_special_n05"));
            fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_CHARGE_MAX);
            fighter.set_int(1, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_STEP);
        }
    }
    return 0.into();
}

// statuses::bayonetta::SPECIAL_N_CANCEL

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
    let cancel_frame_param = fighter.get_param_int("param_special_n", "cancel_frame");
    let special_lag = fighter.get_float(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME) as i32; //remaining special lag
    //cancel frame = iasa
    fighter.set_int(cancel_frame_param, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME);
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_end_f").into(), Hash40::new("special_air_n_end_f").into(), false.into());
    } else {//cancel lag is the greatest of the param or special lag total 
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_end_h").into(), Hash40::new("special_air_n_end_h").into(), false.into());
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if special_lag > cancel_frame_param {fighter.set_int(special_lag, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME); }
        } //if grounded with stored special lag convert to cancel lag
    }
    fighter.set_float(1.0, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLOAT_MOTION_RATE);
    cancel_motion(fighter);
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("bayonetta_bulletclimax_circle"), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("bayonetta_chargebullet_start"), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_cancel_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //set anims and kinetic stuff
    if StatusModule::is_situation_changed(fighter.module_accessor) {cancel_motion(fighter); }
    //frame counter
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME);
    let cancel_frame_param = fighter.get_param_int("param_special_n", "cancel_frame");
    let cancel_frame = fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME);
    let special_lag = fighter.get_float(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
    //pause if special lag
    if fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME) > cancel_frame_param && MotionModule::rate(fighter.module_accessor) >= 0.0 {
        let motion_frame = MotionModule::frame(fighter.module_accessor); 
        let cancel_frame_param_min = cancel_frame_param as f32;
        MotionModule::set_rate(fighter.module_accessor, (14.0 - motion_frame)/cancel_frame_param_min.max(special_lag - cancel_frame_param as f32));
    } //slow down
    //unpause motion if lag frame is over    
    if cancel_frame == cancel_frame_param && !StatusModule::is_changing(fighter.module_accessor) {
        MotionModule::set_rate(fighter.module_accessor, (58.0 - 14.0)/(18.0 + special_lag * 0.2));
        var_reset(fighter);//clears lag and resources after lag has been experienced
    }
    //cancel frame check
    if cancel_frame == 0 {
        let status = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE);
        if status != 0 {
            StatusModule::change_status_force(fighter.module_accessor, status, false); 
            return 1.into();
        } else {
            CancelModule::enable_cancel(fighter.module_accessor); 
            if !fighter.is_situation(*SITUATION_KIND_GROUND) {drift_limits(fighter);}
        } //drift when cancel
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
    0.into()
}

unsafe extern "C" fn special_n_cancel_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {var_reset(fighter); }
    0.into()
}

unsafe extern "C" fn motion_handling(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        //air -> gr
        if fighter.is_motion(Hash40::new("special_air_n_charge_h")) {MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_charge_h"), -1.0, 1.0, 0.0, false, false); }
        else if fighter.is_motion(Hash40::new("special_air_n_start_h")) {MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start_h"), -1.0, 1.0, 0.0, false, false); }
        else if fighter.is_motion(Hash40::new("special_air_n_loop_h")) {MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_loop_h"), -1.0, 1.0, 0.0, false, false); }
        else if fighter.is_motion(Hash40::new("special_air_n_end_h")) {MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_end_h"), -1.0, 1.0, 0.0, false, false); }
    } else {
        drift_limits(fighter);
    }
    return 0.into();
}

unsafe extern "C" fn drift_limits(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_y = fighter.get_param_float("param_special_n", "air_start_speed_mul_y");
    let air_accel_y = fighter.get_param_float("param_special_n", "air_start_accel_y");
    let air_stable_y = fighter.get_param_float("param_special_n", "air_start_max_speed_y");
    let air_accel_x_mul: f32 = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.air_accel_x_mul");
    let max_air_speed_x: f32 = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.max_air_speed_x");
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, max_air_speed_x, 0.0);
    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_stable_y);
    0.into()
}

unsafe extern "C" fn cancel_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_button_off(Buttons::Special) {fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_FIRE.into(), false.into()); }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump) && fighter.sub_check_button_frick().get_bool())) {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, *FIGHTER_STATUS_KIND_JUMP_SQUAT);
        }
        else if fighter.sub_check_command_guard().get_bool() {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, 0);
        } else {
            return false.into()
        }
        return true.into()
    } else {
        fighter.check_jump_cancel(false, false);
        if fighter.is_cat_flag(Cat1::AirEscape) {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, 0);
            return true.into()
        }
    }
    return false.into();
}

unsafe extern "C" fn var_reset(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_float(0.0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
    fighter.set_int(0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT);
    fighter.set_int(0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_REUSE_FRAME);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    fighter.set_int(0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AFTER_ACTION);
    return 0.into();
}

unsafe extern "C" fn cancel_motion(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    } else if StatusModule::is_changing(fighter.module_accessor) {
        let start_y = fighter.get_param_float("param_special_n", "air_start_speed_mul_y");
        let air_accel_y = fighter.get_param_float("param_special_n", "air_start_accel_y");
        let air_stable_y = fighter.get_param_float("param_special_n", "air_start_max_speed_y");
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::KineticEnergy;
        let speed_y = lua_bind::KineticEnergy::get_speed_y(gravity_energy);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y * start_y);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_stable_y);
    }
    0.into()
}

unsafe extern "C" fn special_n_fire_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let remaining_repeats = fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_ADD_FIRE_COUNT);
    VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, remaining_repeats);
    if !(&[*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_END].contains(&fighter.global_table[STATUS_KIND].get_i32())) {
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_N_EFFECT_OFF);
    }
    0.into()
}

//unsafe extern "C" fn special_n_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT) {
//        fighter.sub_change_motion_by_situation(Hash40::new("special_n_end_f").into(), Hash40::new("special_air_n_end_f").into(), false.into());
//    } else {
//        fighter.sub_change_motion_by_situation(Hash40::new("special_n_end_h").into(), Hash40::new("special_air_n_end_h").into(), false.into());
//    }
//    //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x31b6af34a0), false);
//    fighter.set_float(1.0, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLOAT_MOTION_RATE);
//    let special_landing_frame_mul = fighter.get_param_float("param_special_n", "special_landing_frame_mul");
//    let special_lag_base = fighter.get_float(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME); //remaining special landing lag
//    let special_lag = (special_landing_frame_mul * special_lag_base) as i32;
//    let base_endlag= if fighter.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.base_endlag") as i32} else {25}; //32 faf van, 25 here and 40 max
//    fighter.set_int(base_endlag.max(special_lag),*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME);
//    calc_motion_rates(fighter);
//    motion_handling(fighter);
//    fighter.clear_lua_stack();
//    lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("bayonetta_bulletclimax_circle"), true, true);
//    sv_module_access::effect(fighter.lua_state_agent);
//    fighter.clear_lua_stack();
//    lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("bayonetta_chargebullet_start"), true, true);
//    sv_module_access::effect(fighter.lua_state_agent);
//    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_end_main_loop as *const () as _))
//}
//
//unsafe extern "C" fn special_n_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//    if StatusModule::is_situation_changed(fighter.module_accessor) {motion_handling(fighter); }
//    if CancelModule::is_enable_cancel(fighter.module_accessor) {
//        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
//        || fighter.sub_air_check_fall_common().get_bool() {
//            return 1.into();
//        }
//    }
//    if MotionModule::is_end(fighter.module_accessor) {
//        if fighter.is_situation(*SITUATION_KIND_GROUND) {fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into()); }
//        else {fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into()); }
//    }
//    return 0.into();
//}
//
//unsafe extern "C" fn special_n_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
//    fighter.clear_lua_stack();
//    lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("bayonetta_bulletclimax_circle"), true, true);
//    sv_module_access::effect(fighter.lua_state_agent);
//    var_reset(fighter);
//    0.into()
//}
//
//unsafe extern "C" fn calc_motion_rates(fighter: &mut L2CFighterCommon) {
//    //ugh
//    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
//    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true);
//    let mut motion_rate = 1.0;
//    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_END) {
//        //base endlag
//        let base_end_lag = fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME) as f32; //use frame count set earlier
//        //check rounds of fire
//        let max_repeat = fighter.get_param_int("param_special_n", "add_fire_max");
//        let remaining_repeats = fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_ADD_FIRE_COUNT);
//        let used_rounds = (max_repeat - remaining_repeats) as f32;
//        let lag_per_round = if fighter.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.lag_per_round")} else {5.0};
//        motion_rate = (base_end_lag + lag_per_round*used_rounds)/cancel_frame;
//        println!("cancel_frame: {}", cancel_frame);
//        println!("used_rounds: {}", used_rounds);
//        println!("motion_rate: {}", motion_rate);
//    } else {
//        let base_end_lag= if fighter.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_n.base_endlag")} else {25.0}; //32 faf van, 25 here and 40 max
//        motion_rate = base_end_lag/cancel_frame;
//    }
//    VarModule::set_float(fighter.battle_object, vars::bayonetta::status::SPECIAL_N_MOTION_RATE_BACKUP, motion_rate);
//    fighter.set_float(motion_rate, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLOAT_MOTION_RATE);
//}

pub fn install(agent: &mut Agent) {
        agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_init);
        agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
        
        agent.status(Init, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE, special_n_charge_init);
        agent.status(Main, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE, special_n_charge_main);

        agent.status(Pre, statuses::bayonetta::SPECIAL_N_CANCEL, special_n_cancel_pre);
        agent.status(Main, statuses::bayonetta::SPECIAL_N_CANCEL, special_n_cancel_main);
        agent.status(End, statuses::bayonetta::SPECIAL_N_CANCEL, special_n_cancel_end);

        agent.status(End, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_FIRE, special_n_fire_end);

        //agent.status(Main, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_END, special_n_end_main);
        //agent.status(End, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_END, special_n_end_end);
}
