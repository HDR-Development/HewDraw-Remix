use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let log_mask = if !fighter.is_flag(*FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOOMERANG) {
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64
    }
    else {
        0
    };
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
        log_mask,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    return false.into();
}

unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    let (motion, motion_air) = if fighter.is_flag(*FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOOMERANG) {
        (hash40("special_s1_blank"), hash40("special_air_s1_blank"))
    }
    else {
        (hash40("special_s1"), hash40("special_air_s1"))
    };
    fighter.set_int64(motion, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION);
    fighter.set_int64(motion_air, *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION_AIR);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return true.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && (fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool()) {
        return false.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return true.into();
        }
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return true.into();
        }
    }
    if link_situation_helper(fighter).get_bool() {
        special_s_set_kinetics(fighter);
    }

    // tap/hold input for boomerang distance
    if fighter.check_hold_input(0, 8, Buttons::SpecialAll) {
        fighter.on_flag(*FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FLICK);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_HAJIKI_NUM);
    }

    return false.into();
}

unsafe extern "C" fn special_s_set_kinetics(fighter: &mut L2CFighterCommon) {
    let motion_id = if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        *FIGHTER_LINK_STATUS_BOOMERANG_WORK_INT_MOTION_AIR
    };
    let motion = Hash40::new_raw(fighter.get_int64(motion_id));
    if fighter.is_flag(*FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FIRST) {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_LINK_STATUS_BOOMERANG_FLAG_FIRST);
    }
}

unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

unsafe extern "C" fn special_s_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_flag(*FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOOMERANG) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    return false.into();
}

// FIGHTER_LINK_STATUS_KIND_SPECIAL_S2

unsafe extern "C" fn special_s2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        0,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    false.into()
}

unsafe extern "C" fn special_s2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s2_set_kinetics(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_main_loop as *const () as _))
}

unsafe extern "C" fn special_s2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return true.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return true.into();
        }
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return true.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        let mtrans = fighter.get_int(*FIGHTER_LINK_STATUS_BOOMERANG_CATCH_INT_MTRANS);
        if fighter.global_table[PREV_SITUATION_KIND] != mtrans
        && fighter.global_table[SITUATION_KIND] == mtrans {
            special_s2_set_kinetics(fighter);
        }
    }
    return false.into();
}

unsafe extern "C" fn special_s2_set_kinetics(fighter: &mut L2CFighterCommon) {
    let (motion, other_situation) = if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        (Hash40::new("special_s2"), *SITUATION_KIND_AIR)
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        (Hash40::new("special_air_s2"), *SITUATION_KIND_GROUND)
    };
    if fighter.is_flag(*FIGHTER_LINK_STATUS_BOOMERANG_CATCH_FLAG_FIRST) {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_LINK_STATUS_BOOMERANG_CATCH_FLAG_FIRST);
    }
    fighter.set_int(other_situation, *FIGHTER_LINK_STATUS_BOOMERANG_CATCH_INT_MTRANS);
}

unsafe extern "C" fn special_s2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exit);

    agent.status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2, special_s2_pre);
    agent.status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2, special_s2_main);
    agent.status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2, special_s2_end);
}
