use super::*;
use globals::*;

::utils::import!(
    common::djc::{
        sub_attack_air_inherit_jump_aerial_motion_uniq_process_init,
        sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec,
        attack_air_main_status,
        attack_air_main_status_loop
    }
);

// ::utils::import!{common::djc::sub_attack_air_inherit_jump_aerial_motion_uniq_process_init}
// ::utils::import!{common::djc::sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec}
// ::utils::import!{common::djc::attack_air_main_status}
// ::utils::import!{common::djc::attack_air_main_status_loop}

// TAGS: DJC, Double Jump Cancel, Peach
// Reimplemented to be similar to other djc characters because peach doesn't make the same function calls as in vanilla

unsafe extern "C" fn peach_attack_air_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT
    && fighter.global_table[PREV_STATUS_KIND] != FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START {
        let _ = common::djc::sub_attack_air_inherit_jump_aerial_motion_uniq_process_init(fighter);
    }
    0.into()
}

// TAGS: DJC, Double Jump Cancel, Peach

unsafe extern "C" fn peach_attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::djc::sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec(fighter)
}

// TAGS: DJC, Double Jump Cancel, Peach
// Reimplements the setup main script for peach's aerials to transition into double jump cancel code (if applicable)

unsafe extern "C" fn peach_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT
    && fighter.global_table[PREV_STATUS_KIND] != FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START {
        fighter.sub_attack_air_common(false.into());
        MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
        return fighter.main_shift(peach_attack_air_no_float_main_loop);
    }

    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);

    let fighter_log_attack_kind = match motion_kind {
        ::utils::hash40!("attack_air_n") => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N,
        ::utils::hash40!("attack_air_f") => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F,
        ::utils::hash40!("attack_air_b") => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B,
        ::utils::hash40!("attack_air_lw") => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW,
        ::utils::hash40!("attack_air_hi") => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI,
        _ => {
            fighter.sub_attack_air_common(false.into());
            MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
            return fighter.main_shift(common::djc::attack_air_main_status);
        }
    };
    smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, fighter_log_attack_kind);
    let _ = fighter.status_AttackAir_Main_common();
    WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    fighter.main_shift(peach_attack_air_main_loop)
}

unsafe extern "C" fn peach_attack_air_no_float_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Moved above is_enable_cancel for readability concerns
    let can_shoot_item = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_AIR);
    let can_attack_air = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    let is_trigger_opt = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0; // are we pressing the attack button in any capacity?
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    // We are pressing the A button and can either shoot an item or can do an aerial
    && (is_trigger_opt && ((can_shoot_item && fighter.sub_is_item_shoot_air().get_bool()) || can_attack_air))
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    && fighter.global_table[STICK_Y].get_f32() <= WorkModule::get_param_float(fighter.module_accessor, smash::hash40("common"), smash::hash40("squat_stick_y"))
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_JUMP_FROM_WATER)
    {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START.into(), true.into());
        return 1.into();
    }
    if fighter.status_AttackAir_Main_common().get_bool() {
        return 0.into();
    }
    fighter.sub_air_check_superleaf_fall_slowly();
    if !fighter.global_table[IS_STOPPING].get_bool() {
        fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
    }
    0.into()
}

// Default reimplementation of the main loop for an aerial
// No special functionality
unsafe extern "C" fn peach_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);

    let _ = fighter.status_AttackAir_Main_common();
    WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    0.into()
}
pub fn install() {
    smashline::Agent::new("peach")
        .status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, peach_attack_air_init)
        .status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, peach_attack_air_exec)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, peach_attack_air_main)
        .install();
}
