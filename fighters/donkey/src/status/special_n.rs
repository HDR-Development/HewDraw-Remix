use super::*;

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
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
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_n_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.on_flag(*FIGHTER_DONKEY_STATUS_SPECIAL_N_FLAG_MOT_CHANGE);
    let add_motion_rate = fighter.get_param_float("param_special_n", "add_motion_rate");
    let count = fighter.get_int(*FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
    fighter.set_float(count as f32 * add_motion_rate + 1.0, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_RATE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_special_n_loop(fighter, false.into());
    }

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_n_loop as *const () as _));
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.main_shift(special_n_loop_main_loop)
}

unsafe extern "C" fn sub_special_n_loop(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        fighter.inc_int(*FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        let add_motion_rate = fighter.get_param_float("param_special_n", "add_motion_rate");
        let rate = fighter.get_float(*FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_RATE);
        MotionModule::set_rate(fighter.module_accessor, rate + add_motion_rate);
        fighter.set_float(rate + add_motion_rate, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_RATE);
    }

    return 0.into();
}

unsafe extern "C" fn special_n_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            if fighter.is_flag(*FIGHTER_DONKEY_STATUS_SPECIAL_N_FLAG_MOT_CHANGE) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_loop"), -1.0, 1.0, 0.0, false, false);
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            if fighter.is_flag(*FIGHTER_DONKEY_STATUS_SPECIAL_N_FLAG_MOT_CHANGE) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_loop"), -1.0, 1.0, 0.0, false, false);
            }
        }
        fighter.off_flag(*FIGHTER_DONKEY_STATUS_SPECIAL_N_FLAG_MOT_CHANGE);
        let rate = fighter.get_float(*FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_RATE);
        MotionModule::set_rate(fighter.module_accessor, rate);
    }
    if fighter.is_pad_flag(PadFlag::SpecialTrigger)
    || fighter.is_pad_flag(PadFlag::AttackTrigger) {
        fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_ATTACK.into(), true.into());
        return 1.into();
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
            fighter.set_int(*FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
            return 1.into();
        }
    }
    let count = fighter.get_int(*FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
    let max_charge_frame = fighter.get_param_int("param_special_n", "max_charge_frame");
    if count < max_charge_frame {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if fighter.sub_check_jump_in_charging().get_bool() {
                fighter.set_int(*FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                return 1.into();
            }
        }
        else {
            if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
                fighter.set_int(*FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                return 1.into();
            }
            if fighter.sub_check_jump_in_charging().get_bool() {
                fighter.set_int(*FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_JUMP, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_JUMP_CANCEL.into(), true.into());
                return 1.into();
            }
        }
    }
    else {
        app::FighterUtil::flash_eye_info(fighter.module_accessor);
        app::FighterUtil::set_face_motion_by_priority(fighter.module_accessor, FighterFacial(*FIGHTER_FACIAL_SPECIAL), Hash40::new("special_n_max_face"));
        fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_n_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.on_flag(*FIGHTER_DONKEY_STATUS_SPECIAL_N_FLAG_MOT_CHANGE);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.main_shift(special_n_cancel_main_loop)
}

unsafe extern "C" fn special_n_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_check_charge_cancel_jump_mini_attack();
    // stub cancel types we don't want
    let cancel = fighter.get_int(*FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
    let cancel_type = if [
        *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR,
        *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE,
        *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE_F,
        *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE_B,
    ].contains(&cancel) { *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_NONE } else { cancel };
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if cancel_type == *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP {
                FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
            }
        }
    }
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            if fighter.is_flag(*FIGHTER_DONKEY_STATUS_SPECIAL_N_FLAG_MOT_CHANGE) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_cancel"), -1.0, 1.0, 0.0, false, false);
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            if fighter.is_flag(*FIGHTER_DONKEY_STATUS_SPECIAL_N_FLAG_MOT_CHANGE) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_cancel"), -1.0, 1.0, 0.0, false, false);
            }
        }
        fighter.off_flag(*FIGHTER_DONKEY_STATUS_SPECIAL_N_FLAG_MOT_CHANGE);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if cancel_type == *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), false.into());
            return 1.into();
        }
        if cancel_type == *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
            ControlModule::clear_command(fighter.module_accessor, true);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        if cancel_type == *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_JUMP {
            let aerial_type = fighter.get_param_int("aerial_type", "0");
            if aerial_type == *FIGHTER_JUMP_AERIAL_TYPE_NORMAL {
                fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FLY.into(), false.into());
            }
            return 1.into();
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre);
    agent.status(Main, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_LOOP, special_n_loop_main);
    agent.status(Main, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_CANCEL, special_n_cancel_main);
}