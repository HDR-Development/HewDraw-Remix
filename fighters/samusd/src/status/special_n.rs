use super::*;

unsafe extern "C" fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    return 0.into();
}

unsafe extern "C" fn special_n_c_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
    fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_c"), L2CValue::Hash40s("special_air_n_c"), false.into());
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    
    fighter.main_shift(special_n_c_main_loop)
}

unsafe extern "C" fn special_n_c_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_check_charge_cancel_jump_mini_attack();
    fighter.sub_air_check_dive();
    let cancel_type = fighter.get_int(*FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
    if cancel_type == *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_GUARD {
        if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            fighter.set_int(*FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
        }
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if cancel_type == *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP {
                FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
            }
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_c"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_c"), -1.0, 1.0, 0.0, false, false);
        }
        if !StatusModule::is_changing(fighter.module_accessor) {
            fighter.set_int(*FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
        }
    }
    let mut shift_cancel_status = false;
    if cancel_type != *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE {
        if MotionModule::is_end(fighter.module_accessor)
        || CancelModule::is_enable_cancel(fighter.module_accessor) {
            shift_cancel_status = true;
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            shift_cancel_status = true;
        }
    }
    if shift_cancel_status {
        fighter.fastshift(L2CValue::Ptr(special_n_cancel_helper as *const () as _));
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
          return 1.into();
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_n_cancel_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cancel_type = fighter.get_int(*FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
    match cancel_type {
        0x1 => {},  // FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE
        0x2 => {},  // FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE_F
        0x3 => {},  // FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE_B
        0x4 => {    // FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_GUARD
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), false.into());
            return 1.into();
        },
        0x5 => {    // FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), false.into());
            return 1.into();
        },
        0x6 => {},  // FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR
        _ => {
            fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
            return 1.into();
        }
    };

    return 0.into();
}

unsafe extern "C" fn special_n_h_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
    fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_h"), L2CValue::Hash40s("special_air_n_h"), false.into());
    fighter.enable_transition_term_many(&[
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR
    ]);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);

    fighter.main_shift(special_n_h_main_loop)
}

unsafe extern "C" fn special_n_h_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_h"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_h"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if fighter.is_pad_flag(PadFlag::SpecialTrigger)
    || fighter.is_pad_flag(PadFlag::AttackTrigger) {
        fighter.change_motion_by_situation("special_n_f", "special_air_n_f", 0.0, 1.0, false, 0.0, false, false);
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F.into(), true.into());
        return 1.into();
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.sub_check_jump_in_charging().get_bool() {
            fighter.set_int(*FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C.into(), true.into());
            return 1.into();
        }
        if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
            fighter.set_int(*FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C.into(), true.into());
            return 1.into();
        }
    }
    else {
        if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
            fighter.set_int(*FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C.into(), true.into());
            return 1.into();
        }
        if fighter.get_num_used_jumps() < fighter.get_jump_count_max()
        && fighter.sub_check_jump_in_charging().get_bool() {
            fighter.set_int(*FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_JUMP_AERIAL, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_JUMP_CANCEL.into(), true.into());
            return 1.into();
        }
    }
    if !fighter.global_table[IS_STOPPING].get_bool() {
        fighter.inc_int(*FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        let count = fighter.get_int(*FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        let cshot_charge_frame = fighter.get_param_float("param_special_n", "cshot_charge_frame");
        let mut charged = false;
        if cshot_charge_frame <= count as f32 {
            fighter.set_int(count, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E.into(), false.into());
            charged = true;
        }
        else {
            charged = false;
        }
        if charged {
            return 1.into();
        }
        else {
            let count = fighter.get_int(*FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
            let cshot_charge_frame = fighter.get_param_float("param_special_n", "cshot_charge_frame");
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x26b38955ef), count as f32 / cshot_charge_frame);
            return 0.into();
        }
    }
    else {
        let count = fighter.get_int(*FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        let cshot_charge_frame = fighter.get_param_float("param_special_n", "cshot_charge_frame");
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x26b38955ef), count as f32 / cshot_charge_frame);
        return 0.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_n_f_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    if fighter.is_flag(*FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT) {
        let count = fighter.get_int(*FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);    // l90
        let cshot_spd_min = fighter.get_param_float("param_special_n", "cshot_shot_spd_min");   // l80
        let cshot_spd_max = fighter.get_param_float("param_special_n", "cshot_shot_spd_max");  // lc0
        let cshot_charge_frame = fighter.get_param_float("param_special_n", "cshot_charge_frame");  // la0
        let mut speed_calc = cshot_spd_min + (cshot_spd_max - cshot_spd_min) * (count as f32 / cshot_charge_frame) - 0.01;    // l60
        speed_calc *= fighter.lr();
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent); // l70
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_calc, speed_y);
        fighter.off_flag(*FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_exec);
    agent.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E, special_n_exec);

    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C, special_n_c_main);
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, special_n_h_main);

    agent.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, special_n_f_exec);
}