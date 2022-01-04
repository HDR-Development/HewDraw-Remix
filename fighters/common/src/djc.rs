use super::*;

use globals::*;

/// Regular attack air status script except uses the animation's movement by default.
/// To be used by fighters who have double jump cancel
pub unsafe fn attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(L2CValue::Bool(false));
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_status_loop as *const () as _))
}

/// Performs the leniency check 
pub unsafe extern "C" fn attack_air_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_JUMP_AERIAL {
        let djc_leniency_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "djc_leniency_frame");
        if fighter.global_table[CURRENT_FRAME].get_i32() <= djc_leniency_frame && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
    }
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOPPING].get_bool() {
            fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
        }
        0.into()
    }
    else {
        1.into()
    }
}