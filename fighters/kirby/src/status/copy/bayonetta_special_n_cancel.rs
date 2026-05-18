use super::*;

// FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N

unsafe extern "C" fn special_n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        return smashline::original_status(Init, fighter, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N)(fighter);
    }
    air_stall(fighter);
    0.into()
}

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, 0);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_N_FOOT);
        fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT);
        fighter.set_int64(hash40("special_n_start_f") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_G);
        fighter.set_int64(hash40("special_air_n_start_f") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_A);
    } else {
        fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_N_FOOT);
        fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT);
        fighter.set_int64(hash40("special_n_start_h") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_G);
        fighter.set_int64(hash40("special_air_n_start_h") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_A);
    }
    motion_handling(fighter, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    motion_handling(fighter, true);
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_CHARGE.into(), false.into())
    }
    0.into()
}

unsafe extern "C" fn special_n_charge_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_n_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT) {
        fighter.set_int64(hash40("special_n_charge_f") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_G);
        fighter.set_int64(hash40("special_air_n_charge_f") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_A);
    } else {
        fighter.set_int64(hash40("special_n_charge_h") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_G);
        fighter.set_int64(hash40("special_air_n_charge_h") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_A);
    }
    fighter.set_float(1.0, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLOAT_MOTION_RATE);
    motion_handling(fighter, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_charge_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // set motion
    motion_handling(fighter, true);
    // cancel status on input
    if !StopModule::is_stop(fighter.module_accessor) && cancel_check(fighter).get_bool() {
        StatusModule::change_status_force(fighter.module_accessor, statuses::bayonetta::SPECIAL_N_CANCEL, false);
    }
    if fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_STEP) == 0 {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT) {
                fighter.set_int64(hash40("special_n_loop_f") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_G);
                fighter.set_int64(hash40("special_air_n_loop_f") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_A);
                fighter.sub_change_motion_by_situation_kirby_copy(Hash40::new("special_n_loop_f").into(), Hash40::new("special_air_n_loop_f").into(), false.into());
            } else {
                fighter.set_int64(hash40("special_n_loop_h") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_G);
                fighter.set_int64(hash40("special_air_n_loop_h") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_A);
                fighter.sub_change_motion_by_situation_kirby_copy(Hash40::new("special_n_loop_h").into(), Hash40::new("special_air_n_loop_h").into(), false.into());
            }
            app::FighterUtil::flash_eye_info(fighter.module_accessor);
            PLAY_SE(fighter, Hash40::new("se_bayonetta_special_n05"));
            fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_CHARGE_MAX);
            fighter.set_int(1, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_STEP);
        }
    }
    0.into()
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
        fighter.set_int64(hash40("special_n_end_f") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_G);
        fighter.set_int64(hash40("special_air_n_end_f") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_A);
    } else {
        fighter.set_int64(hash40("special_n_end_h") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_G);
        fighter.set_int64(hash40("special_air_n_end_h") as i64, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_A);
    }
    let cancel_frame = fighter.get_param_int("param_special_n", "cancel_frame");
    fighter.set_int(cancel_frame, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME);
    fighter.set_float(1.0, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLOAT_MOTION_RATE);
    motion_handling(fighter, false);
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        air_stall(fighter);
    } //cut speed f0 of cancel
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("bayonetta_bulletclimax_circle"), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("bayonetta_chargebullet_start"), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_cancel_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //frame counter
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME);
    let cancel_frame = fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME);
    //set anims and kinetic stuff
    let drift = if cancel_frame > 0 {false} else {true};
    motion_handling(fighter, drift);
    //cancel frame check
    if fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_CANCEL_FRAME) == 0 {
        let status = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE);
        if status != 0 {
            StatusModule::change_status_force(fighter.module_accessor, status, false);
            return 1.into();
        } else {
            CancelModule::enable_cancel(fighter.module_accessor);
            fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            if !fighter.is_situation(*SITUATION_KIND_GROUND) {
                drift_limits(fighter);
            }
        } // drift when actionable
    }
    // act out of it if no cancel status buffered
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
    }
    return 0.into();
}

unsafe extern "C" fn special_n_cancel_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn air_stall(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let y_speed = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let air_start_speed_mul_x = fighter.get_param_float("param_special_n", "air_start_speed_mul_x");
    let air_start_speed_mul_y = fighter.get_param_float("param_special_n", "air_start_speed_mul_y");
    let startup_min_y = -0.5;
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, x_speed * air_start_speed_mul_x, 0.0);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, (y_speed * air_start_speed_mul_y).max(startup_min_y));
    0.into()
}


unsafe extern "C" fn motion_handling(fighter: &mut L2CFighterCommon, drift: bool) -> L2CValue {
    let mot_gr = fighter.get_int64(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_G);
    let mot_air = fighter.get_int64(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_A);
    let air = if drift {*FIGHTER_KINETIC_TYPE_MOTION_FALL} else {*FIGHTER_KINETIC_TYPE_AIR_STOP};
    fighter.sub_air_check_dive();
    if StatusModule::is_changing(fighter.module_accessor) {
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, air);
        fighter.sub_change_motion_by_situation_kirby_copy(Hash40::new_raw(mot_gr).into(), Hash40::new_raw(mot_air).into(), false.into());
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
           drift_limits(fighter);
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, air);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(fighter.module_accessor, Hash40::new_raw(mot_gr), -1.0, 1.0, 0.0, false, false);
        } else {
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(fighter.module_accessor, Hash40::new_raw(mot_air), -1.0, 1.0, 0.0, false, false);
            drift_limits(fighter);
        }
    }
    return 0.into();
}

unsafe extern "C" fn drift_limits(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_accel_y = fighter.get_param_float("param_special_n", "air_start_accel_y");
    let air_stable_y = fighter.get_param_float("param_special_n", "air_start_max_speed_y");
    let air_accel_x_mul= 0.04;
    let max_air_speed_x = 0.4;
    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, max_air_speed_x, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_stable_y);
    0.into()
}

unsafe extern "C" fn cancel_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_button_off(Buttons::Special) {fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_FIRE.into(), false.into()); }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump) && fighter.sub_check_button_frick().get_bool())) {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, *FIGHTER_STATUS_KIND_JUMP_SQUAT);
            StatusModule::change_status_force(fighter.module_accessor, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, false);
        }
        if fighter.sub_check_command_guard().get_bool() {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, *FIGHTER_STATUS_KIND_WAIT);
            StatusModule::change_status_force(fighter.module_accessor, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, false);
        }
    } else {
        fighter.check_jump_cancel(false, false, false);
        if fighter.is_cat_flag(Cat1::AirEscape) {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, *FIGHTER_STATUS_KIND_FALL);
            StatusModule::change_status_force(fighter.module_accessor, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, false);
        }
    }
    return 0.into();
}

unsafe extern "C" fn special_n_fire_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let remaining_repeats = fighter.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_ADD_FIRE_COUNT);
    VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE, remaining_repeats);
    // re-uses flag from cancel, resets on start status. can't enter firing and cancel on the same nspecial usage
    if ![*FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_CHARGE, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_FIRE, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_END].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_N_EFFECT_OFF);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
        agent.status(Init, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N, special_n_init);
        agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N, special_n_main);

        agent.status(Init, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_CHARGE, special_n_charge_init);
        agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_CHARGE, special_n_charge_main);

        agent.status(Pre, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, special_n_cancel_pre);
        agent.status(Main, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, special_n_cancel_main);
        agent.status(End, statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL, special_n_cancel_end);

        agent.status(End, *FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_FIRE, special_n_fire_end);
}