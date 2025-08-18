use super::*;

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_special_n(fighter, false.into());
    }

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_n as *const () as _));
    fighter.main_shift(special_n_main_loop)
}

unsafe fn sub_special_n(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() { return 0.into(); }
    if fighter.is_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_SHOOT) {
        if fighter.get_int(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_WORK_INT_LOOP_COUNT) <= 0
        || fighter.is_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_COUNT_CHECK) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM);
            fighter.off_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_COUNT_CHECK);
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_FOOD, false, -1);
        fighter.inc_int(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_WORK_INT_LOOP_COUNT);
        fighter.off_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_SHOOT);
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.on_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_BUTTON_RELEASE);
    }
    if fighter.is_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_COUNT_ENABLE) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.inc_int(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_WORK_INT_TRIGGER_COUNT);
        }
    }
    let mut set_loop_frame = false;
    let mut loop_start_frame = -1;
    let loop_count_max = fighter.get_param_int("param_special_n", "loop_count_max");
    if fighter.get_int(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_WORK_INT_LOOP_COUNT) < loop_count_max {
        if fighter.is_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_LOOP_CHECK) {
            if fighter.is_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_BUTTON_RELEASE) {
                set_loop_frame = true;
                fighter.off_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_LOOP_CHECK);
            }
        }
        if fighter.is_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_RAPID_CHECK) {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                set_loop_frame = true;
                fighter.off_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_RAPID_CHECK);
            }
        }
    }
    if fighter.is_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_COUNT_CHECK) {
        let loop_trigger_count = fighter.get_param_int("param_special_n", "loop_trigger_count");
        if fighter.get_int(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_WORK_INT_TRIGGER_COUNT) <= loop_trigger_count {
            set_loop_frame = true;
            loop_start_frame = 1;
        }
        fighter.off_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_COUNT_ENABLE);
        fighter.set_int(0, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_WORK_INT_LOOP_COUNT);
    }
    if set_loop_frame {
        if loop_start_frame < 0 {
            loop_start_frame = fighter.get_param_int("param_special_n", "loop_start_frame");
        }
        let frame1 = (loop_start_frame - 1) as f32;
        let frame2 = (loop_start_frame - 2) as f32;
        MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, frame1, true, false, false);
        MotionAnimcmdModule::exec_motion_lines_initialize(fighter.module_accessor, frame2, true);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.set_int(*SITUATION_KIND_AIR, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_WORK_INT_MTRANS);
        }
        else {
            fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_WORK_INT_MTRANS);
        }
        fighter.set_int(0, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_WORK_INT_TRIGGER_COUNT);
    }

    return 0.into();
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || !fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            if fighter.is_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_MOT_CHANGE) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_MOT_CHANGE);
            }
            fighter.set_int(*SITUATION_KIND_AIR, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_WORK_INT_MTRANS);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            if fighter.is_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_MOT_CHANGE) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_MOT_CHANGE);
            }
            fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_WORK_INT_MTRANS);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
}