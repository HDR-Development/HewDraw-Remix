use super::*;

// FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT

pub unsafe extern "C" fn special_n_shoot_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::lucario::instance::IS_POWERED_UP);
    smashline::original_status(Pre, fighter, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT)(fighter)
}

unsafe extern "C" fn special_n_shoot_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_save_charge_status(fighter);
    0.into()
}

// FIGHTER_STATUS_KIND_SPECIAL_N

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_start") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_start") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    special_n_set_kinetic(fighter);
    let max_charge_frame = fighter.get_param_float("param_special_n", "max_charge_frame");
    let curr_charge_frame = fighter.get_int(*FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME) as f32;
    if curr_charge_frame >= max_charge_frame {
        fighter.on_flag(*FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_FLAG_CHARGE_MAX);
    } else {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_HOLD);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_SHOOT);
    }
    special_n_joint_translate(fighter);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if !StopModule::is_stop(fighter.module_accessor)
        && fighter.is_button_trigger(Buttons::Special) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_SHOOT);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_HOLD);
        }
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_n_set_kinetic(fighter);
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        // ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, false, -1);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_SHOOT) {
            fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
            return 0.into();
        }
        ControlModule::clear_command(fighter.module_accessor, true);
        if fighter.is_flag(*FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_FLAG_CHARGE_MAX) {
            fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX.into(), false.into());
            ArticleModule::change_status(
                fighter.module_accessor,
                *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL,
                *WEAPON_LUCARIO_AURABALL_STATUS_KIND_CHARGE,
                ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)
            );
            return 0.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_HOLD) {
            fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD.into(), false.into());
            return 0.into();
        }
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_save_charge_status(fighter);
    0.into()
}

pub unsafe extern "C" fn special_n_joint_translate(fighter: &mut L2CFighterCommon) {
    let havel = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let haver = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(
        fighter.module_accessor,
        Hash40::new("havel"),
        havel,
        true
    );
    ModelModule::joint_global_position(
        fighter.module_accessor,
        Hash40::new("haver"),
        haver,
        true
    );
    let pos = Vector3f{x: havel.x + haver.x, y: havel.y + haver.y, z: havel.z + haver.z};
    let new_pos = Vector3f{x: pos.x * 0.5, y: pos.y * 0.5, z: pos.z * 0.5};
    ModelModule::set_joint_translate(
        fighter.module_accessor,
        Hash40::new("throw"),
        &new_pos,
        true,
        false
    );
}

// FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD

unsafe extern "C" fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    ArticleModule::change_status(
        fighter.module_accessor,
        *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL,
        *WEAPON_LUCARIO_AURABALL_STATUS_KIND_CHARGE,
        ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)
    );
    special_n_set_kinetic(fighter);

    // skip forward in the motion to the current charge
    let max_charge_frame = fighter.get_param_float("param_special_n", "max_charge_frame");
    let curr_charge_frame = fighter.get_int(*FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME) as f32;
    let motion_end_frame = MotionModule::end_frame(fighter.module_accessor);
    let frame = motion_end_frame * curr_charge_frame / max_charge_frame;
    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, frame, true, false, false);

    // motion rate the motion to match charge rate
    let rate = motion_end_frame / max_charge_frame;
    MotionModule::set_rate(fighter.module_accessor, rate);

    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_hold_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n_set_kinetic(fighter);
        return 0.into();
    }
    
    if fighter.is_button_trigger(Buttons::Special) {
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        return 0.into();
    }

    // if fighter.check_jump_cancel(false) {
    //     return 0.into();
    // }

    if special_n_check_cancel(fighter).get_bool() {
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX.into(), false.into());
        return 0.into();
    }

    0.into()
}

unsafe extern "C" fn special_n_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_save_charge_status(fighter);
    0.into()
}

// FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX

unsafe extern "C" fn special_n_max_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_max") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_max") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    special_n_set_kinetic(fighter);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_max_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_max_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n_set_kinetic(fighter);
        return 0.into();
    }
    
    if fighter.is_button_trigger(Buttons::Special) {
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        return 0.into();
    }

    // if fighter.check_jump_cancel(false) {
    //     return 0.into();
    // }

    if special_n_check_cancel(fighter).get_bool() {
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
        return 0.into();
    }

    0.into()
}

unsafe extern "C" fn special_n_max_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_save_charge_status(fighter);
    0.into()
}

unsafe extern "C" fn special_n_check_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if fighter.is_button_on(Buttons::Guard) {
            fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            return true.into();
        }
        if fighter.sub_check_jump_in_charging_for_cancel_status((*FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS).into()).get_bool() {
            return true.into();
        }
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        // vanilla code for transitioning directly into spotdodge/guard is removed
        // this is to prevent accidental dodges/rolls during ASC
        // if fighter.is_cat_flag(Cat2::StickEscape) {
        //     if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
        //         fighter.set_int(*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
        //     } else {
        //         fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
        //     }
        //     return true.into();
        // }
        // if fighter.is_cat_flag(Cat2::StickEscapeF) {
        //     if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F) {
        //         fighter.set_int(*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
        //     } else {
        //         fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
        //     }
        //     return true.into();
        // }
        // if fighter.is_cat_flag(Cat2::StickEscapeB) {
        //     if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B) {
        //         fighter.set_int(*FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
        //     } else {
        //         fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
        //     }
        //     return true.into();
        // }
        if fighter.sub_check_command_guard().get_bool() {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
                fighter.set_int(*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            } else {
                fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            return true.into();
        }
        if fighter.sub_check_jump_in_charging_for_cancel_status((*FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS).into()).get_bool() {
            return true.into();
        }
    }
    return false.into();
}

unsafe extern "C" fn special_n_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
            MotionModule::change_motion_inherit_frame_keep_rate(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                -1.0,
                1.0,
                0.0
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
        }
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
            MotionModule::change_motion_inherit_frame_keep_rate(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                -1.0,
                1.0,
                0.0
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
        }
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
}

pub unsafe extern "C" fn special_n_save_charge_status(fighter: &mut L2CFighterCommon) {
    let curr_status = StatusModule::status_kind(fighter.module_accessor);
    let next_status = fighter.global_table[STATUS_KIND].get_i32();
    let is_kirby = fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_KIRBY;
    // define statuses for kirby or lucario
    let special_n = *FIGHTER_STATUS_KIND_SPECIAL_N;
    let special_n_hold =    if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD}     else {*FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD};
    let special_n_max =     if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_MAX}      else {*FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX};
    let special_n_shoot =   if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_SHOOT}    else {*FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT};
    let special_n_cancel =  if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL}   else {*FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL};

    // handle charge storage
    // store charge if in cancel status or if moving between valid statuses
    if curr_status == special_n_cancel
    || [special_n_hold, special_n_max, special_n_shoot, special_n_cancel].contains(&next_status) {
        let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL);
        if !article.is_null() {
            let article_object_id = app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_module_accessor = app::sv_battle_object::module_accessor(article_object_id);
            let charge_frame = WorkModule::get_int(article_module_accessor, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
            app::FighterSpecializer_Lucario::save_aura_ball_status(fighter.module_accessor, true, charge_frame);
        }
    } else {
        app::FighterSpecializer_Lucario::save_aura_ball_status(fighter.module_accessor, false, 0);
    }

    // handle article removal
    let is_exist = ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL);
    // remove the article if it exists and won't be needed by the next status
    if is_exist && ![special_n_hold, special_n_max, special_n_shoot].contains(&next_status) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(0));
    }

    // handle max charge effects
    // request the max charge effect if we're canceling from max charge status
    // else, always remove the effect (for simplicity)
    if curr_status == special_n_max && next_status == special_n_cancel {
        EffectModule::req_common(fighter.module_accessor, Hash40::new("charge_max"), 0.0);
    } else {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
    }

}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_pre);
    agent.status(End, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_end);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_end);
    agent.status(Main, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD, special_n_hold_main);
    agent.status(End, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD, special_n_hold_end);
    agent.status(Main, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX, special_n_max_main);
    agent.status(End, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX, special_n_max_end);
}