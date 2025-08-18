use super::*;

unsafe extern "C" fn squat_disable_terms(fighter: &mut L2CFighterCommon) {
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);

    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
    
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);

    if VarModule::get_int(fighter.battle_object, vars::samus::instance::SPECIAL_LW_BOMB_LOCKOUT) > 0 {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_RV);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    }
}

pub unsafe extern "C" fn squat_check_bomb_input(fighter: &mut L2CFighterCommon) -> bool {
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
        ControlModule::clear_command(fighter.module_accessor, false);
        let bomb_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"),hash40("bomb_max_req"));
        if (ArticleModule::get_active_num(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB) as i32) < bomb_max 
        && VarModule::get_int(fighter.battle_object, vars::samus::instance::SPECIAL_LW_BOMB_LOCKOUT) <= 0 {
            VarModule::on_flag(fighter.battle_object, vars::samus::instance::SPECIAL_LW_INPUT_FROM_CRAWL);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
            return true;
        }
    }
    if VarModule::get_int(fighter.battle_object, vars::samus::instance::SPECIAL_LW_BOMB_LOCKOUT) <= 0 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_RV);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    }

    return false;
}

pub unsafe extern "C" fn squat_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return squat_main(fighter,true);
}

pub unsafe extern "C" fn squat_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return squat_main(fighter,false);
}

unsafe extern "C" fn squat_main(fighter: &mut L2CFighterCommon, forward: bool) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::samus::instance::SPECIAL_LW_CRAWL);
    if forward {
        //fighter.status_SquatF();
        status_squatf_reimpl(fighter);
        return fighter.main_shift(squat_f_main_loop)
    }
    else {
        //fighter.status_SquatB();
        status_squatb_reimpl(fighter);
        return fighter.main_shift(squat_b_main_loop)
    }
}

unsafe extern "C" fn status_squatf_reimpl(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_squat_common();
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_RV);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_B);
    // let from_normal_squat = fighter.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT])
    //     && !fighter.is_motion_one_of(&[Hash40::new("squat_f"), Hash40::new("squat_b"), Hash40::new("squat_n")]);
    // if from_normal_squat {
    //     MotionModule::change_motion(fighter.module_accessor, Hash40::new("squat_entry"), 0.0, 1.0, false, 0.0, false, false);
    // }
    // else {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("squat_f"), 0.0, 1.0, false, 0.0, false, false);
    // }

    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_SquatF_Main as *const () as _))
}

unsafe extern "C" fn status_squatb_reimpl(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_squat_common();
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_RV);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_F);
    // let from_normal_squat = fighter.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT])
    //     && !fighter.is_motion_one_of(&[Hash40::new("squat_f"), Hash40::new("squat_b"), Hash40::new("squat_n")]);
    // if from_normal_squat {
    //     MotionModule::change_motion(fighter.module_accessor, Hash40::new("squat_entry"), 0.0, 1.0, false, 0.0, false, false);
    // }
    // else {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("squat_b"), 0.0, 1.0, false, 0.0, false, false);
    // }

    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_SquatB_Main as *const () as _))
}

unsafe extern "C" fn squat_f_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatF_Main();
    squat_disable_terms(fighter);
    // if motion squat_entry_f and is end, change motion to squatf
    // if fighter.is_motion(Hash40::new("squat_entry")) && MotionModule::is_end(fighter.module_accessor) {
    //     MotionModule::change_motion(fighter.module_accessor, Hash40::new("squat_f"), 0.0, 1.0, false, 0.0, false, false);
    // }
    if squat_check_bomb_input(fighter) {
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn squat_b_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatB_Main();
    return squat_main_loop(fighter);
}

unsafe extern "C" fn squat_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    squat_disable_terms(fighter);
    if squat_check_bomb_input(fighter) {
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn squat_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_prev_status_one_of(&[
        *FIGHTER_STATUS_KIND_SQUAT_F,
        *FIGHTER_STATUS_KIND_SQUAT_B,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW,
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G
    ]) {
        VarModule::off_flag(fighter.battle_object, vars::samus::instance::SPECIAL_LW_CRAWL);
        return fighter.status_SquatWait();
    }
    VarModule::on_flag(fighter.battle_object, vars::samus::instance::SPECIAL_LW_CRAWL);

    fighter.status_SquatWait_common(0xc0.into());

    MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("squat_n"), 6.0,0.0, 0.0);
    VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);

    fighter.main_shift(squat_wait_main_loop)
}

unsafe extern "C" fn squat_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatWait_Main();
    squat_main_loop(fighter);

    return 0.into();
}

unsafe extern "C" fn squat_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let next = StatusModule::status_kind_next(fighter.module_accessor);
    if ![
        *FIGHTER_STATUS_KIND_SQUAT_F,
        *FIGHTER_STATUS_KIND_SQUAT_B,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G
    ].contains(&next) {
        VarModule::off_flag(fighter.battle_object, vars::samus::instance::SPECIAL_LW_CRAWL);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SQUAT_WAIT, squat_wait_main);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SQUAT_WAIT, squat_exit);

    agent.status(Main, *FIGHTER_STATUS_KIND_SQUAT_F, squat_f_main);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SQUAT_F, squat_exit);

    agent.status(Main, *FIGHTER_STATUS_KIND_SQUAT_B, squat_b_main);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SQUAT_B, squat_exit);
}