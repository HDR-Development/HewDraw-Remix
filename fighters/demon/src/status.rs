use super::*;
use globals::*;

utils::import_noreturn!(common::shoto_status::{
    fgc_end_dashback,
    ryu_idkwhatthisis2
});

extern "Rust" {
    // from common::shoto_status
    fn ryu_kara_cancel(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn ryu_attack_main_uniq_chk(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn ryu_attack_main_uniq_chk4(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue;
    fn ryu_final_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue;
    fn ryu_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue;
    fn fgc_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

// FIGHTER_DEMON_STATUS_KIND_DASH //

pub unsafe extern "C" fn status_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Dash_Sub();
    fighter.sub_shift_status_main(L2CValue::Ptr(status_dash_main as *const () as _))
}

unsafe extern "C" fn status_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.status_Dash_Main_common(L2CValue::I32(0)).get_bool() {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP {
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
                    let attack_stand_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_attack_step"), hash40("dash_to_attack_stand1_frame"));
                    if fighter.global_table[CURRENT_FRAME].get_i32() <= attack_stand_frame {
                        fighter.change_status(
                            L2CValue::I32(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1), // We don't want to change to ESCAPE_AIR_SLIDE in case they do a nair dodge
                            L2CValue::Bool(true)
                        );
                        return L2CValue::I32(0);
                    }
                }
            }
        }
    }
    return L2CValue::I32(0);
}

// FIGHTER_STATUS_KIND_TURN_DASH //

pub unsafe extern "C" fn pre_turndash(fighter: &mut L2CFighterCommon) -> L2CValue {
    app::FighterSpecializer_Demon::update_opponent_lr_1on1(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN_DASH);
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr != 0.0 {
        if PostureModule::lr(fighter.module_accessor) == lr {
            if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_STATUS_KIND_TURN {
                StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_DASH_BACK);
                return L2CValue::I32(1);
            }
        }
    }
    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN);
    return 1.into()
}

// FIGHTER_DEMON_STATUS_KIND_DASH_BACK //

pub unsafe extern "C" fn main_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub unsafe extern "C" fn end_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::shoto_status::fgc_end_dashback(fighter);
    smashline::original_status(End, fighter, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK)(fighter)
}

// FIGHTER_STATUS_KIND_ATTACK //
// Here to force Kazuya to only use neutral attack to continue the combo.

unsafe extern "C" fn demon_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_NEXT_STATUS);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT_CLIFF_STOP);
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_main_loop as *const () as _))
}

extern "Rust" {
    fn only_jabs(fighter: &mut L2CFighterCommon) -> bool;
}

unsafe extern "C" fn demon_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let change_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_NEXT_STATUS);
    if change_to == *FIGHTER_STATUS_KIND_NONE {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_12") {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && only_jabs(fighter) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH) {
                    fighter.change_status(FIGHTER_DEMON_STATUS_KIND_FLASH_PUNCH.into(), true.into());
                    return 0.into();
                }
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && only_jabs(fighter) {
                if StatusModule::is_changing(fighter.module_accessor) == false {
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_ENABLE_COMBO) {
                        WorkModule::set_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_NEXT_STATUS);
                    }
                }
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
        if change_to == *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO.into(), true.into());
            return 0.into();
        }
    }
    fighter.status_Attack_Main()
}

// FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO //
// Here to force Kazuya to only use neutral attack to continue the combo.

unsafe extern "C" fn demon_attackcombo_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    demon_attackcombo_main_mot_helper(fighter, 2.into());
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attackcombo_main_loop as *const () as _))
}

unsafe extern "C" fn demon_attackcombo_main_mot_helper(fighter: &mut L2CFighterCommon, count: L2CValue) {
    let val = count.get_i32();
    WorkModule::set_int(fighter.module_accessor, val, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_COMBO);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_NEXT_STATUS);
    let mot : u64;
    match val {
        2 => mot = hash40("attack_13"),
        3 => mot = hash40("attack_14"),
        4 => mot = hash40("attack_15"),
        5 => mot = hash40("attack_16"),
        6 => mot = hash40("attack_17"),
        7 => mot = hash40("attack_18"),
        8 => mot = hash40("attack_19"),
        _ => mot = hash40("attack_110"),
    }
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
}

unsafe extern "C" fn demon_attackcombo_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    let mut next_status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_NEXT_STATUS);
    if next_status != *FIGHTER_STATUS_KIND_NONE {
        if next_status == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2 {
            next_status = demon_attackcombo_main_loop_helper_first(fighter, next_status);
        }
    }
    else {
        next_status = demon_attackcombo_main_loop_helper_first(fighter, next_status);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
        if next_status == *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO {
            let combo_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_COMBO);
            demon_attackcombo_main_mot_helper(fighter, (combo_count + 1).into());
            notify_event_msc_cmd!(fighter, 0x2b94de0d96u64, FIGHTER_LOG_ACTION_CATEGORY_ATTACK, FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_16);
            return 0.into();
        }
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        return 0.into();
    }
    if next_status != *FIGHTER_STATUS_KIND_NONE {
        if next_status == *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
        fighter.change_status(next_status.into(), true.into());
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn demon_attackcombo_main_loop_helper_first(fighter: &mut L2CFighterCommon, next_status: i32) -> i32 {
    let combo_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_COMBO);
    let mut status = next_status;
    if combo_step != 4 {
        if combo_step != 6 {
            if combo_step != 7 {
                status = demon_attackcombo_main_loop_helper_second(fighter, status);
            }
        }
    }
    else {
        status = demon_attackcombo_main_loop_helper_second(fighter, status);
    }
    if status == *FIGHTER_STATUS_KIND_NONE {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && only_jabs(fighter) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                if !StatusModule::is_changing(fighter.module_accessor) {
                    status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO;
                }
            }
        }
    }
    WorkModule::set_int(fighter.module_accessor, status, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_NEXT_STATUS);
    status
}

unsafe extern "C" fn demon_attackcombo_main_loop_helper_second(fighter: &mut L2CFighterCommon, next_status: i32) -> i32 {
    let mut status = next_status;
    if next_status != *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2 {
        if fighter.global_table[CMD_CAT4].get_i32() & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND == 0 {
            if fighter.global_table[CMD_CAT4].get_i32() & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT == 0 {
                if fighter.global_table[CMD_CAT4].get_i32() & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_323CATCH == 0 {
                    if fighter.global_table[CMD_CAT4].get_i32() & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A == 0 {
                        if fighter.global_table[CMD_CAT4].get_i32() & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6AB == 0 {
                            status = *FIGHTER_STATUS_KIND_NONE;
                        }
                        else {
                            status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1;
                        }
                    }
                    else {
                        status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2;
                    }
                }
                else {
                    status = *FIGHTER_DEMON_STATUS_KIND_CATCH_COMMAND;
                }
            }
            else {
                status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F;
            }
        }
        else {
            status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S;
        }
    }
    else {
        if fighter.global_table[CMD_CAT4].get_i32() & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG != 0 {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM) {
                status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L;
            }
            else {
                status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE;
            }
        }
        if fighter.global_table[CMD_CAT4].get_i32() & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623BLONG != 0 {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM) {
                status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE;
            }
        }
    }
    status
}

// FIGHTER_STATUS_KIND_WAIT //

pub unsafe extern "C" fn wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Wait()
}

// vanilla script

pub unsafe extern "C" fn wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_wait_common();
    fighter.sub_wait_motion_mtrans();
    fighter.sub_shift_status_main(L2CValue::Ptr(fgc_wait_main_loop as *const () as _))
}

pub unsafe extern "C" fn fgc_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_Wait_Main().get_bool() {
        return 0.into();
    }
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr != 0.0 && PostureModule::lr(fighter.module_accessor) != lr {
        let stick_x_corrected = fighter.global_table[STICK_X].get_f32() * (PostureModule::lr(fighter.module_accessor) * -1.0);
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let walk_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x"));
        let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));

        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK) {
            if walk_stick_x <= stick_x_corrected {
                if squat_stick_y < stick_y {
                    fighter.change_status(FIGHTER_RYU_STATUS_KIND_WALK_BACK.into(), true.into());
                    return 0.into();
                }
            }
        }
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_TURN_AUTO.into(), false.into());
        return 0.into();
    }
    0.into()
}

// FIGHTER_STATUS_KIND_LANDING //

pub unsafe extern "C" fn landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_landing_main(fighter)
}

// FIGHTER_STATUS_KIND_ATTACK_AIR //
// For fixing momentum transfer

pub unsafe extern "C" fn attackair_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter)
}

pub fn install() {
    smashline::Agent::new("demon")
        .status(Main, *FIGHTER_STATUS_KIND_DASH, status_dash)
        .status(Pre, *FIGHTER_STATUS_KIND_TURN_DASH, pre_turndash)
        .status(Main, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK, main_dashback)
        .status(End, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK, end_dashback)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK, demon_attack_main)
        .status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO, demon_attackcombo_main)
        .status(Pre, *FIGHTER_STATUS_KIND_WAIT, wait_pre)
        .status(Main, *FIGHTER_STATUS_KIND_WAIT, wait_main)
        .status(Main, *FIGHTER_STATUS_KIND_LANDING, landing_main)
        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, attackair_pre)
        .install();
}