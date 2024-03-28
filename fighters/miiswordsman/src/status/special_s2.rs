use super::*;

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH

unsafe extern "C" fn special_s2_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    let s2_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("s2_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, s2_dash_frame, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    special_s2_dash_mot_change(fighter);
    // if !StopModule::is_stop(fighter.module_accessor) {
    //     WorkModule::dec_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    // }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s2_dash_dec_int as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_dash_main as *const () as _))
}

unsafe extern "C" fn special_s2_dash_mot_change(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_dash"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s2_dash"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_dash"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s2_dash"), -1.0, 1.0, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn special_s2_dash_dec_int(fighter: &mut L2CFighterCommon, unk: L2CValue) -> L2CValue {
    if unk.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    }
    return 0.into()
}

unsafe extern "C" fn special_s2_dash_unk(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut val = 0;
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            let is_status_cliff = GroundModule::is_status_cliff(fighter.module_accessor);
            if is_status_cliff {
                fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END.into(), true.into());
                val = 1;
            }
        }
    }
    return val.into()
}

unsafe extern "C" fn special_s2_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut sub_check = fighter.sub_transition_group_check_air_cliff().get_bool();
    if sub_check {
        return 0.into();
    }
    sub_check = fighter.sub_ground_check_stop_wall().get_bool();
    if sub_check {
        return 0.into();
    }
    // custom [
    // Jump and Attack cancels
    let pad_flag = ControlModule::get_pad_flag(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND && MotionModule::frame(fighter.module_accessor) > 7.0 {
        if fighter.check_jump_cancel(true, false) {
            return 1.into()
        }
    }
    if compare_mask(pad_flag, *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) || compare_mask(pad_flag, *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK.into(),true.into());
        return 1.into()
    }
    // Wall Jump
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        if !VarModule::is_flag(fighter.battle_object, vars::common::instance::SPECIAL_WALL_JUMP) {
            let touch_right = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
            let touch_left = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
            if touch_left || touch_right {
                if compare_mask(ControlModule::get_command_flag_cat(fighter.module_accessor, 0), *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH | *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) {
                    VarModule::on_flag(fighter.battle_object, vars::common::instance::SPECIAL_WALL_JUMP);
                    fighter.change_status(FIGHTER_STATUS_KIND_WALL_JUMP.into(),true.into());
                    return 1.into()
                }
            }
        }
    }
    // ]
    let slash_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    if slash_frame <= 0 {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END.into(), true.into());
    }
    if StatusModule::is_changing(fighter.module_accessor) {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                special_s2_dash_mot_change(fighter);
            }
        }
        else {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                special_s2_dash_mot_change(fighter);
            }
        }
    }
    else {
        special_s2_dash_mot_change(fighter);
    }
    special_s2_dash_unk(fighter);
    return 0.into()
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK

unsafe extern "C" fn special_s2_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    let s2_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("s2_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, s2_dash_frame, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_WORK_INT_DASH_COUNT);
    special_s2_attack_mot_change(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_attack_main as *const () as _))
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

unsafe extern "C" fn special_s2_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        special_s2_dash_unk(fighter);
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
                special_s2_dash_unk(fighter);
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

unsafe extern "C" fn pre_special_s2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::GALE_STAB_EDGE_CANCEL);
    smashline::original_status(Pre, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END)(fighter)
}

unsafe extern "C" fn special_s2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_CONTINUE_MOT);
    special_s2_end_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_end_Main as *const () as _))
}

unsafe extern "C" fn special_s2_end_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
                        VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::GALE_STAB_EDGE_CANCEL);
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
                        VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::GALE_STAB_EDGE_CANCEL);
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
        if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::status::GALE_STAB_EDGE_CANCEL) {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
        // ]
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
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH, special_s2_dash);

    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK, special_s2_attack);
    
    agent.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END, pre_special_s2_end);
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END, special_s2_end);
}