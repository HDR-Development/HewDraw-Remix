// status imports
use super::*;
use globals::*;

// This file contains code for wavelanding

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            attack_combo_none_uniq_chk_button,
            attack_combo_uniq_chk_button,
            check_100_count_button
        );
    }
}

#[inline(always)]
pub unsafe fn only_jabs(fighter: &mut L2CFighterCommon) -> bool {
    return !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)
    && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH)
    && fighter.global_table[CMD_CAT1].get_i32() & (
        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 |
        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 |
        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 |
        *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH
    ) == 0;
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_attack_combo_none_uniq_chk_button)]
unsafe fn attack_combo_none_uniq_chk_button(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue) {
    if param_1.get_bool() == false {
        if ControlModule::check_button_on(fighter.module_accessor, param_2.get_i32())
        && only_jabs(fighter) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
            }
        }
        fighter.attack_uniq_chk_command(param_3.clone());
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            let countdown = WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0);
            if countdown & 1 == 0 {
                return;
            }
        }
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_attack_combo_uniq_chk_button)]
unsafe fn attack_combo_uniq_chk_button(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue) {
    if param_1.get_bool() == false {
        fighter.attack_uniq_chk_command(param_3.clone());
        if fighter.global_table[CMD_CAT1].get_i32() & param_3.get_i32() != 0
        && only_jabs(fighter) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
            }
        }
        let button = param_2.get_i32();
        if !ControlModule::check_button_on(fighter.module_accessor, button) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
        }
        else {
            if !AttackModule::is_infliction_status(fighter.module_accessor, 0x7f) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RELEASE_BUTTON) {
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
                        ComboModule::reset(fighter.module_accessor);
                    }
                }
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
                && only_jabs(fighter) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
                    if ControlModule::check_button_on_trriger(fighter.module_accessor, button) {
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER);
                    }
                }
            }
            else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
            && only_jabs(fighter) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_COMBO) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                let combo_count = ComboModule::count(fighter.module_accessor) as i32;
                let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
                if combo_count != attack_combo_max {
                    return;
                }
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART);
                ComboModule::reset(fighter.module_accessor);
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO) {
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER) {
                        return;
                    }
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
                }
            }
        }
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            let something = WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0);
            if something & 1 == 0 {
                return;
            }
        }
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_check_100_count_button)]
unsafe fn check_100_count_button(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let button = param_1.get_i32();
    // if fighter.global_table[IN_HITLAG].get_bool() {
    if only_jabs(fighter) {
        if !ControlModule::check_button_on_trriger(fighter.module_accessor, button) {
            if !ControlModule::check_button_on_release(fighter.module_accessor, button) {
                return;
            }
        }
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_COUNT);
    }
    // else {
    //     if !ControlModule::check_button_on_trriger(fighter.module_accessor, button) {
    //         if !ControlModule::check_button_on_release(fighter.module_accessor, button) {
    //             return;
    //         }
    //     }
    //     WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_COUNT);
    // }
}
