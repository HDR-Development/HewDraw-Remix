use super::*;

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH

unsafe extern "C" fn special_s2_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_s2_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    let s2_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("s2_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, s2_dash_frame, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    if !StopModule::is_stop(fighter.module_accessor) {
        special_s2_dash_substatus(fighter, false.into());
    }
    VarModule::set_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_GROUND_START, fighter.is_situation(*SITUATION_KIND_GROUND));
    special_s2_dash_change_motion(fighter);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s2_dash_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_dash_main_loop as *const () as _))
}

unsafe extern "C" fn special_s2_dash_change_motion(fighter: &mut L2CFighterCommon) {
    let mot = if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        Hash40::new("special_air_s2_dash")
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        Hash40::new("special_s2_dash")
    };
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
        MotionModule::change_motion(fighter.module_accessor, mot, 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    }
    else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, mot, -1.0, 1.0, 0.0, false, false);
    }
}

unsafe extern "C" fn special_s2_dash_substatus(fighter: &mut L2CFighterCommon, unk: L2CValue) -> L2CValue {
    if unk.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    }
    return 0.into();
}

unsafe extern "C" fn special_s2_main_loop_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH {
        let cont = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && GroundModule::is_status_cliff(fighter.module_accessor) {
            fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END.into(), true.into());
            true
        }
        else {
            false
        };
        if cont {
            return 0.into();
        }
    }
    1.into()
}

unsafe extern "C" fn special_s2_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool()
    || fighter.sub_ground_check_stop_wall().get_bool() {
        return 0.into();
    }
    let dash_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    if dash_count <= 0 {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END.into(), false.into());
        return 0.into();
    }
    fighter.check_wall_jump_cancel();
    if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_GROUND_START) {
        VarModule::set_float(fighter.battle_object, vars::common::instance::JUMP_SPEED_MAX_MUL, 1.346);  // 1.75 max jump speed out of Quick Draw (copied from Ike)
        fighter.check_jump_cancel(true, false);
    }
    // attack cancel
    if fighter.is_cat_flag(Cat1::SpecialAny) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK, true);
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_s2_dash_change_motion(fighter);
    }
    special_s2_main_loop_helper(fighter);
    return 0.into();
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK

unsafe extern "C" fn special_s2_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    let s2_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("s2_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, s2_dash_frame, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    special_s2_attack_mot_change(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_attack_main_loop as *const () as _))
}

unsafe extern "C" fn special_s2_attack_mot_change(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_attack"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s2_attack"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_attack"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_attack"), -1.0, 1.0, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn special_s2_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        special_s2_attack_main_helper(fighter);
    }
    else {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    return 0.into()
}

unsafe extern "C" fn special_s2_attack_main_helper(fighter: &mut L2CFighterCommon) {
    if !MotionModule::is_end(fighter.module_accessor) {
        if !StatusModule::is_changing(fighter.module_accessor) {
            if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                    special_s2_attack_mot_change(fighter);
                }
            }
            else {
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    special_s2_attack_mot_change(fighter);
                }
            }
        }
        special_s2_main_loop_helper(fighter);
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
                if StatusModule::is_changing(fighter.module_accessor) {
                    if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                            special_s2_attack_mot_change(fighter);
                        }
                    }
                    else {
                        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                            special_s2_attack_mot_change(fighter);
                        }
                    }
                }
                special_s2_main_loop_helper(fighter);
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), true.into());
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }
    }
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END

unsafe extern "C" fn special_s2_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_EDGE_CANCEL);
    smashline::original_status(Pre, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END)(fighter)
}

unsafe extern "C" fn special_s2_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    special_s2_end_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_end_main_loop as *const () as _))
}

unsafe extern "C" fn special_s2_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into()
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if !MotionModule::is_end(fighter.module_accessor) {
            if StatusModule::is_changing(fighter.module_accessor) {
                if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND {
                    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
                        // OG [ special_s2_end_helper(fighter); ]
                        // custom [
                        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                        fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES));
                        fighter.change_status(
                            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                            L2CValue::Bool(true)
                        );
                        // ]
                        sub_special_s2_end(fighter);
                        return 0.into()
                    }
                }
                else {
                    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                        VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_EDGE_CANCEL);
                        special_s2_end_helper(fighter);
                        sub_special_s2_end(fighter);
                        return 0.into()
                    }
                }
            }
            else {
                // custom [
                if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_AIR {
                    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                        VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_EDGE_CANCEL);
                    }
                }
                if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND {
                    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
                        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                        fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES));
                        fighter.change_status(
                            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                            L2CValue::Bool(true)
                        );
                        sub_special_s2_end(fighter);
                        return 0.into()
                    }
                }
                // ]
                special_s2_end_helper(fighter);
            }
            sub_special_s2_end(fighter);
        }
        else {
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_FALL_SPECIAL),
                    L2CValue::Bool(true)
                );
            }
            else {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_WAIT),
                    L2CValue::Bool(true)
                );
            }
        }
    }
    else {
        if !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into()
            }
        }
    }
    return 0.into()
}

unsafe extern "C" fn special_s2_end_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_end"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s2_end"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        // OG [ GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP)); ]
        // custom [
        if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_S2_EDGE_CANCEL) {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
        fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_NONE));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_end"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_end"), -1.0, 1.0, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn sub_special_s2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND_INTERRUPT] == FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            if GroundModule::is_status_cliff(fighter.module_accessor) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END),
                    L2CValue::Bool(true)
                );
                return 1.into()
            }
        }
    }
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH, special_s2_dash_pre);
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH, special_s2_dash_main);

    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK, special_s2_attack_main);
    
    agent.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END, special_s2_end_pre);
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END, special_s2_end_main);
}