use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        main_attack,
        escape_f_end,
        escape_b_end,
        batwithin_end
    );
}

// FIGHTER_STATUS_KIND_ATTACK //

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn main_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    let combo_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_type"), 0);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_check_attack_mtrans as *const () as _));
    if combo_type != *FIGHTER_COMBO_TYPE_NONE {
        if combo_type != *FIGHTER_COMBO_TYPE_HIT {
            if combo_type == *FIGHTER_COMBO_TYPE_SUCCEED {
                if !StopModule::is_stop(fighter.module_accessor) {
                    fighter.attack_combo_uniq_chk(false.into());
                }
                fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_attack_combo_uniq_chk as *const () as _));
            }
        }
        else {
            if !StopModule::is_stop(fighter.module_accessor) {
                bayonetta_attack_main_hit_uniq_chk(fighter, false.into());
            }
            fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(bayonetta_attack_main_hit_uniq_chk as *const () as _));
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART);
        }
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.attack_combo_none_uniq_chk(false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_attack_combo_none_uniq_chk as *const () as _));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_attack_main_loop as *const () as _))
}

extern "Rust" {
    fn only_jabs(fighter: &mut L2CFighterCommon) -> bool;
}

unsafe extern "C" fn bayonetta_attack_main_hit_uniq_chk(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() == false {
        fighter.attack_uniq_chk();
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && only_jabs(fighter) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_COMBO) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                let combo_count = ComboModule::count(fighter.module_accessor) as i32;
                let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
                if combo_count != attack_combo_max {
                    return 0.into();
                }
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART);
                ComboModule::reset(fighter.module_accessor);
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO) {
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER) {
                        return 0.into();
                    }
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
                }
            }
        }
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            let count_down_int = WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0);
            if count_down_int & 1 == 0 {
                return 0.into();
            }
        }
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    0.into()
}

unsafe extern "C" fn bayonetta_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_13") {
        fighter.check_100_count();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    let attack100_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack100_type"), 0);
    if attack100_type != *FIGHTER_ATTACK100_TYPE_NONE {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100) {
            let count_100 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_COUNT);
            let attack_100_enable_cnt = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_100_enable_cnt"), 0);
            if attack_100_enable_cnt <= count_100
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_100.into(), false.into());
                return 1.into();
            }
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if 0 < jump_attack_frame {
        if fighter.sub_check_button_jump().get_bool() {
            let mot = MotionModule::motion_kind(fighter.module_accessor);
            let data = fighter.get_mini_jump_attack_data_cancel_function(mot.into()).get_int();
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(data), -1);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if jump_attack_frame == 1 {
        if fighter.global_table[IS_STOPPING].get_bool() == false {
            let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            if 0 < attack_kind {
                FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            }
        }
    }
    let combo_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_type"), 0);
    if combo_type != *FIGHTER_COMBO_TYPE_NONE {
        if combo_type == *FIGHTER_COMBO_TYPE_HIT {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                return 0.into();
            }
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
            return 0.into();
        }
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        return 0.into();
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

// FIGHTER_STATUS_KIND_ESCAPE_F //

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_ESCAPE_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn escape_f_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN {
        fighter.sub_status_end_EscaleFB();
    }
    0.into()
}

// FIGHTER_STATUS_KIND_ESCAPE_B //

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_ESCAPE_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn escape_b_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN {
        fighter.sub_status_end_EscaleFB();
    }
    0.into()
}

// FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN //

#[status_script(agent = "bayonetta", status = FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn batwithin_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let batwithin_status_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_BATWITHIN_INT_STATUS_KIND);
    if [*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&batwithin_status_kind) {
        fighter.sub_status_end_EscaleFB();
    }
    if batwithin_status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        fighter.status_end_EscapeAir();
    }
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
    ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 255);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_BAT, ArticleOperationTarget(0));
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}