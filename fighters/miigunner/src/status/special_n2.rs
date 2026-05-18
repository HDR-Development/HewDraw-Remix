use super::*;

pub unsafe extern "C" fn special_n2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE);
    fighter.set_int(*FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_START, *FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_INT_STEP_PREV);
    special_n2_change_motion(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_special_n2(fighter, false.into());
    }

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_n2 as *const () as _));
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02) + -1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02) + -1);

    fighter.main_shift(special_n2_main_loop)
}

unsafe fn special_n2_change_motion(fighter: &mut L2CFighterCommon) {
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) { Hash40::new("special_n2_start") } else { Hash40::new("special_air_n2_start") };
    if fighter.is_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, motion, -1.0, 1.0, 0.0);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE);
    }
}

unsafe extern "C" fn sub_special_n2(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        if fighter.is_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_LOOP_ACCEPT)
        && fighter.is_pad_flag(PadFlag::SpecialTrigger) {
            fighter.on_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_LOOP);
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_n2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
    fighter.check_land_cancel(Some(6.0));
    let mut step = fighter.get_int(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_INT_STEP);
    let mut change_motion = false;
    if !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if step == *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_START {
                if MotionModule::is_end(fighter.module_accessor) {
                    fighter.off_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE);
                    change_motion = true;
                }
                else if fighter.is_prev_situation(*SITUATION_KIND_AIR) {
                    change_motion = true;
                }
            }
            else if step == *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_SHOT {
                if MotionModule::is_end(fighter.module_accessor) {
                    fighter.off_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE);
                    change_motion = true;
                }
                else if fighter.is_prev_situation(*SITUATION_KIND_AIR) {
                    change_motion = true;
                }
            }
            else if step == *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_END {
                if some_func_ass_func(fighter) {
                    return 0.into();
                }
                if fighter.is_prev_situation(*SITUATION_KIND_AIR) {
                    change_motion = true;
                }
            }
        }
        else {
            if step == *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_START {
                if MotionModule::is_end(fighter.module_accessor) {
                    fighter.off_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE);
                    change_motion = true;
                }
                else if fighter.is_prev_situation(*SITUATION_KIND_AIR) {
                    change_motion = true;
                }
            }
            else if step == *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_SHOT {
                if MotionModule::is_end(fighter.module_accessor) {
                    fighter.off_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE);
                    change_motion = true;
                }
                else if fighter.is_prev_situation(*SITUATION_KIND_AIR) {
                    change_motion = true;
                }
            }
            else if step == *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_END {
                if some_func_ass_func(fighter) {
                    return 0.into();
                }
                if fighter.is_prev_situation(*SITUATION_KIND_GROUND) {
                    change_motion = true;
                }
            }
        }
    }
    if change_motion {
        if step != *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_SHOT {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.inc_int(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_INT_STEP);
            }
        }
        else {
            if MotionModule::is_end(fighter.module_accessor) {
                if !fighter.is_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_LOOP) {
                    fighter.set_int(*FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_END, *FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_INT_STEP);
                }
                else {
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, FIGHTER_LOG_ATTACK_KIND_SPECIAL_N, true);
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02) + -1);
                }
                fighter.off_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_LOOP);
                ControlModule::clear_command(fighter.module_accessor, true);
            }
        }
        step = fighter.get_int(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_INT_STEP);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
            if step == *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_START {
                func_two_electric_boogaloo(fighter);
            }
            else if step == *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_SHOT {
                if fighter.is_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE) {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_n2_loop"), -1.0, 1.0, 0.0);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n2_loop"), 0.0, 1.0, false, 0.0, false, false);
                    fighter.on_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE);
                }
            }
            else if step == *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_END {
                if fighter.is_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE) {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_n2_end"), -1.0, 1.0, 0.0);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n2_end"), 0.0, 1.0, false, 0.0, false, false);
                    fighter.on_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE);
                }
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
            if step == *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_START {
                func_three_please_kill_me(fighter);
            }
            else if step == *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_SHOT {
                if fighter.is_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE) {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_n2_loop"), -1.0, 1.0, 0.0);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n2_loop"), 0.0, 1.0, false, 0.0, false, false);
                    fighter.on_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE);
                }
            }
            else if step == *FIGHTER_MIIGUNNER_RAPID_SHOT_STEP_END {
                if fighter.is_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE) {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_n2_end"), -1.0, 1.0, 0.0);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n2_end"), 0.0, 1.0, false, 0.0, false, false);
                    fighter.on_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE);
                }
            }
        }
    }

    return 0.into();
}

unsafe fn some_func_ass_func(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return true;
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return true;
        }
    }

    return false;
}

unsafe fn func_two_electric_boogaloo(fighter: &mut L2CFighterCommon) {
    if fighter.is_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_n2_start"), -1.0, 1.0, 0.0);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n2_start"), 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE);
    }
}

unsafe fn func_three_please_kill_me(fighter: &mut L2CFighterCommon) {
    if fighter.is_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_n2_start"), -1.0, 1.0, 0.0);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n2_start"), 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_CONTINUE);
    }
}