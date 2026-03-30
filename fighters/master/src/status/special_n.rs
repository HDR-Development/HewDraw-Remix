use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_N

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_motion_by_situation("special_n_start", "special_air_n_start", 0.0, 1.0, false, 0.0, false, false);
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
    // skipped checking flag and setting the flag to the returned bool
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
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, Hash40::new("special_n_start"), false, -1.0);
    }
    fighter.main_shift(special_n_main_loop)
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && (fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_n"));
    // spawn charge gfx
    if fighter.is_flag(*FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_RELEASE_BUTTON) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, Hash40::new("haved_2"), true, MotionModule::frame(fighter.module_accessor));
        } // changed to make effect timing semi-automatic when the main move gets motion rated (arrow acmd has van (0.9) divided by what the current hdr one is)
        fighter.off_flag(*FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_RELEASE_BUTTON);
    }
    // fire during window
    if fighter.is_flag(*FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_CAN_SHOOT)
    && fighter.is_button_off(Buttons::Special) {
        fighter.change_status(FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_SHOOT.into(), true.into());
        return 1.into();
    }
    // full charge
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_MAX_SHOOT.into(), true.into());
        return 1.into();
    }
    // cancel/turn
    let hold_frame = fighter.get_int(*FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_HOLD_COUNT);
    let cancel_start_frame = fighter.get_param_int("param_special_n", "cancel_start_frame");
    let cancel_end_frame = fighter.get_param_int("param_special_n", "cancel_end_frame");
    if cancel_start_frame <= hold_frame
    && cancel_end_frame >= hold_frame {
        match cancel_checks(fighter) {
            1 => fighter.change_status(FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_TURN.into(), false.into()),
            2 => fighter.change_status(FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into()),
            3 => fighter.change_status(FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_JUMP_CANCEL.into(), true.into()),
            _ => (),
        }
    }
    // landing handle
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_motion_inherit_frame_by_situation("special_n_start", "special_air_n_start", -1.0, 1.0, 0.0, false, false);
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            SoundModule::play_landing_se(fighter.module_accessor, Hash40::new("se_master_landing_01"));
        }
    }
    0.into()
}

unsafe extern "C" fn cancel_checks(fighter: &mut L2CFighterCommon) -> i32 {
    // turn
    if fighter.is_flag(*FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_TURN) 
    && fighter.is_stick_backward() {
        let motion_frame = fighter.motion_frame();
        fighter.set_float(motion_frame, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_FLOAT_INHERIT_MOTION_FRAME);
        return 1;
    }

    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.sub_check_jump_in_charging().get_bool() {
            fighter.set_int(*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            return 2;
        }
        if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
            fighter.set_int(0, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            return 2;
        }
    }
    else {
        if fighter.sub_check_command_guard().get_bool() || fighter.is_pad_flag(PadFlag::GuardTrigger) {
            fighter.set_int(0, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            return 2;
        }
        if fighter.sub_check_jump_in_charging().get_bool() { // jc
            fighter.set_int(*FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_AIR_JUMP_AERIAL, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            return 3;
        }
    }
    0.into()
}

unsafe extern "C" fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.inc_int(*FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_HOLD_COUNT);
    0.into()
}

unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // kill arrow interrupt
    if ![*FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_TURN,
         *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_HOLD,
         *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_CANCEL,
         *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_JUMP_CANCEL,
         *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_SHOOT,
         *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_MAX_SHOOT].contains(&fighter.global_table[STATUS_KIND].get_i32())
    {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW2, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    // kill charge visuals on cancel
    if [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_JUMP_CANCEL,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_MAX_SHOOT].contains(&fighter.global_table[STATUS_KIND].get_i32())
    {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1) {
            let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1);
            let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_boma = sv_battle_object::module_accessor(article_id);
            EffectModule::kill_kind(article_boma, Hash40::new("master_bow_hold1"), true, true);
            EffectModule::kill_kind(article_boma, Hash40::new("master_bow_hold2"), true, true);
        }
    }
    0.into()
}

// FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_HOLD

unsafe extern "C" fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.get_float(*FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_FLOAT_INHERIT_MOTION_FRAME);
    fighter.change_motion_by_situation("special_n_start", "special_air_n_start", frame, 1.0, false, 0.0, true, false);
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
    // skipped checking flag and setting the flag to the returned bool
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
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, Hash40::new("special_n_start"), false, -1.0);
    }
    ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, frame);
    fighter.main_shift(special_n_main_loop)
}

// FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_TURN

unsafe extern "C" fn special_n_turn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.on_flag(*FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_AFTER_TURN);
    fighter.change_motion_by_situation("special_n_turn", "special_air_n_turn", 0.0, 1.0, false, 0.0, false, false);
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, Hash40::new("special_n_turn"), false, -1.0);
    }
    PostureModule::reverse_lr(fighter.module_accessor);
    fighter.main_shift(special_n_turn_main_loop)
}

unsafe extern "C" fn special_n_turn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    let turn_frame = fighter.motion_frame();
    let frame = fighter.get_float(*FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_FLOAT_INHERIT_MOTION_FRAME);
    // fix effect desync
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1) {
        if turn_frame + frame >= 44.0 && ArticleModule::motion_kind(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) != hash40("haved_2") {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, Hash40::new("haved_2"), true, turn_frame + frame);
            fighter.on_flag(*FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_CAN_SHOOT);
        }
        if turn_frame + frame >= 74.0 {
            fighter.off_flag(*FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_CAN_SHOOT);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.set_float(turn_frame + frame, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_FLOAT_INHERIT_MOTION_FRAME);
        fighter.change_status(FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_HOLD.into(), false.into())
    }
    fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_n"));
    // landing handle
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_motion_inherit_frame_by_situation("special_n_turn", "special_air_n_turn", -1.0, 1.0, 0.0, false, false);
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            SoundModule::play_landing_se(fighter.module_accessor, Hash40::new("se_master_landing_01"));
        }
    }
    0.into()
}

// FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_CANCEL

unsafe extern "C" fn special_n_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_motion_by_situation("special_n_cancel", "special_air_n_cancel", 0.0, 1.0, false, 0.0, false, false);
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
    fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, Hash40::new("special_n_cancel"), false, -1.0);
    }
    fighter.main_shift(special_n_cancel_main_loop)
}

unsafe extern "C" fn special_n_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_motion_inherit_frame_by_situation("special_n_cancel", "special_air_n_cancel", 0.0, 1.0, 0.0, false, false);
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    // cancel
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        let cancel_type = fighter.get_int(*FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
        // currently just jump squat
        if cancel_type != 0 {
            fighter.change_status(cancel_type.into(), false.into());
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
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_end);

    agent.status(Main, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_HOLD, special_n_hold_main);
    agent.status(End, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_HOLD, special_n_end);

    agent.status(Main, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_TURN, special_n_turn_main);
    agent.status(Exec, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_TURN, special_n_exec);
    agent.status(End, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_TURN, special_n_end);

    agent.status(Main, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_CANCEL, special_n_cancel_main);
}