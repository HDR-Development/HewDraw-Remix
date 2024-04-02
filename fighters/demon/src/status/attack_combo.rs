use super::*;

extern "Rust" {
    fn only_jabs(fighter: &mut L2CFighterCommon) -> bool;
}

// FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO 

// Here to force Kazuya to only use neutral attack to continue the combo.

unsafe extern "C" fn attack_combo_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    demon_attackcombo_main_mot_helper(fighter, 2.into());
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_combo_main_loop as *const () as _))
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

unsafe extern "C" fn attack_combo_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO, attack_combo_main);
}