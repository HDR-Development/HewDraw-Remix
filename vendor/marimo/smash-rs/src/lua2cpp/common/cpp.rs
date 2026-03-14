use crate::*;
use super::class::*;

extern "C" {
    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32main_func__fighter_status_attackEv"]
    pub(super) fn main_func__fighter_status_attack(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32main_func__fighter_status_damageEv"]
    pub(super) fn main_func__fighter_status_damage(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31main_func__fighter_status_guardEv"]
    pub(super) fn main_func__fighter_status_guard(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30main_func__fighter_status_itemEv"]
    pub(super) fn main_func__fighter_status_item(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32main_func__fighter_status_ladderEv"]
    pub(super) fn main_func__fighter_status_ladder(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35main_func__fighter_status_swallowedEv"]
    pub(super) fn main_func__fighter_status_swallowed(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30main_func__fighter_status_swimEv"]
    pub(super) fn main_func__fighter_status_swim(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48main_func__fighter_status_bayonetta_final_targetEv"]
    pub(super) fn main_func__fighter_status_bayonetta_final_target(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41local_func__fighter_status_final_damage_1Ev"]
    pub(super) fn local_func__fighter_status_final_damage_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49local_func__fighter_status_rockman_final_target_1Ev"]
    pub(super) fn local_func__fighter_status_rockman_final_target_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51local_func__fighter_status_bayonetta_final_target_1Ev"]
    pub(super) fn local_func__fighter_status_bayonetta_final_target_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51local_func__fighter_status_bayonetta_final_target_2Ev"]
    pub(super) fn local_func__fighter_status_bayonetta_final_target_2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33local_func__fighter_status_item_1Ev"]
    pub(super) fn local_func__fighter_status_item_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33local_func__fighter_status_item_2Ev"]
    pub(super) fn local_func__fighter_status_item_2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusGuard__landing_effect_controlEv"]
    pub(super) fn FighterStatusGuard__landing_effect_control(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusGuard__check_hit_stop_delay_flickEN3lib8L2CValueE"]
    pub(super) fn FighterStatusGuard__check_hit_stop_delay_flick(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34local_func__fighter_status_guard_2Ev"]
    pub(super) fn local_func__fighter_status_guard_2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusGuard__check_hit_stop_delayEN3lib8L2CValueE"]
    pub(super) fn FighterStatusGuard__check_hit_stop_delay(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34local_func__fighter_status_guard_1Ev"]
    pub(super) fn local_func__fighter_status_guard_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusGuard__is_continue_just_shield_countEv"]
    pub(super) fn FighterStatusGuard__is_continue_just_shield_count(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41FighterStatusGuard__set_just_shield_scaleEv"]
    pub(super) fn FighterStatusGuard__set_just_shield_scale(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37FighterStatusGuard__calc_shield_scaleEN3lib8L2CValueE"]
    pub(super) fn FighterStatusGuard__calc_shield_scale(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusGuard__set_guard_blend_motionEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn FighterStatusGuard__set_guard_blend_motion(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusGuard__set_guard_blend_motion_angleEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusGuard__set_guard_blend_motion_angle(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39FighterStatusDamage__effect_exit_statusEv"]
    pub(super) fn FighterStatusDamage__effect_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45FighterStatusDamage__req_fly_roll_smoke_firstEv"]
    pub(super) fn FighterStatusDamage__req_fly_roll_smoke_first(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_update_damage_fly_effectEN3lib8L2CValueES2_S2_S2_S2_S2_S2_"]
    pub(super) fn sub_update_damage_fly_effect(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39FighterStatusDamage__check_smoke_effectEv"]
    pub(super) fn FighterStatusDamage__check_smoke_effect(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusDamage__is_enable_damage_fly_effectEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn FighterStatusDamage__is_enable_damage_fly_effect(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusDamage__get_dolly_stadium_wall_break_speedEv"]
    pub(super) fn FighterStatusDamage__get_dolly_stadium_wall_break_speed(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon60FighterStatusDamage__check_dolly_stadium_wall_break_to_deathEv"]
    pub(super) fn FighterStatusDamage__check_dolly_stadium_wall_break_to_death(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_damage_2Ev"]
    pub(super) fn local_func__fighter_status_damage_2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_damage_1Ev"]
    pub(super) fn local_func__fighter_status_damage_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusDamage__set_delay_motion_interpolationEN3lib8L2CValueE"]
    pub(super) fn FighterStatusDamage__set_delay_motion_interpolation(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusDamage__get_damage_stop_motion_intp_frameEN3lib8L2CValueE"]
    pub(super) fn FighterStatusDamage__get_damage_stop_motion_intp_frame(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusDamage__is_elec_or_paralyze_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusDamage__is_elec_or_paralyze_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusDamage__requestVectorAdjustEffectEN3lib8L2CValueES2_S2_S2_S2_S2_"]
    pub(super) fn FighterStatusDamage__requestVectorAdjustEffect(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusDamage__correctDamageVectorEffectEN3lib8L2CValueE"]
    pub(super) fn FighterStatusDamage__correctDamageVectorEffect(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusDamage__correctDamageVectorCommonEN3lib8L2CValueE"]
    pub(super) fn FighterStatusDamage__correctDamageVectorCommon(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusDamage__correctDamageVectorEv"]
    pub(super) fn FighterStatusDamage__correctDamageVector(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_attack_1Ev"]
    pub(super) fn local_func__fighter_status_attack_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sys_line_system_initEv"]
    pub(super) fn sys_line_system_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_begin_added_linesEN3lib8L2CValueE"]
    pub(super) fn sub_begin_added_lines(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sys_line_status_end_controlEv"]
    pub(super) fn sys_line_status_end_control(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_end_added_linesEv"]
    pub(super) fn sub_end_added_lines(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon5RESETEv"]
    pub(super) fn RESET(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20global_fighter_resetEv"]
    pub(super) fn global_fighter_reset(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_fighter_pre_end_statusEv"]
    pub(super) fn sub_fighter_pre_end_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sys_line_system_control_fighterEv"]
    pub(super) fn sys_line_system_control_fighter(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15call_calc_paramEv"]
    pub(super) fn call_calc_param(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25call_notify_event_gimmickEN3lib8L2CValueE"]
    pub(super) fn call_notify_event_gimmick(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15call_leave_stopEN3lib8L2CValueES2_"]
    pub(super) fn call_leave_stop(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17call_on_change_lrEN3lib8L2CValueES2_"]
    pub(super) fn call_on_change_lr(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17call_check_attackEN3lib8L2CValueES2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_"]
    pub(super) fn call_check_attack(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack, arg8: lib::L2CValueHack, arg9: lib::L2CValueHack, arg10: lib::L2CValueHack, arg11: lib::L2CValueHack, arg12: lib::L2CValueHack, arg13: lib::L2CValueHack, arg14: lib::L2CValueHack, arg15: lib::L2CValueHack, arg16: lib::L2CValueHack, arg17: lib::L2CValueHack, arg18: lib::L2CValueHack, arg19: lib::L2CValueHack, arg20: lib::L2CValueHack, arg21: lib::L2CValueHack, arg22: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40local_func__fighter_common_line_system_2EN3lib8L2CValueES2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_"]
    pub(super) fn local_func__fighter_common_line_system_2(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack, arg8: lib::L2CValueHack, arg9: lib::L2CValueHack, arg10: lib::L2CValueHack, arg11: lib::L2CValueHack, arg12: lib::L2CValueHack, arg13: lib::L2CValueHack, arg14: lib::L2CValueHack, arg15: lib::L2CValueHack, arg16: lib::L2CValueHack, arg17: lib::L2CValueHack, arg18: lib::L2CValueHack, arg19: lib::L2CValueHack, arg20: lib::L2CValueHack, arg21: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40local_func__fighter_common_line_system_1EN3lib8L2CValueES2_S2_"]
    pub(super) fn local_func__fighter_common_line_system_1(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17call_check_damageEN3lib8L2CValueES2_S2_S2_S2_S2_S2_S2_"]
    pub(super) fn call_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack, arg8: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38main_func__fighter_status_final_damageEv"]
    pub(super) fn main_func__fighter_status_final_damage(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46main_func__fighter_status_rockman_final_targetEv"]
    pub(super) fn main_func__fighter_status_rockman_final_target(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_PackunFinalCaptureEv"]
    pub(super) fn status_pre_PackunFinalCapture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_PackunFinalCaptureEv"]
    pub(super) fn status_PackunFinalCapture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_shift_status_mainEN3lib8L2CValueE"]
    pub(super) fn sub_shift_status_main(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_PackunFinalCapture_MainEv"]
    pub(super) fn status_PackunFinalCapture_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_PackunFinalCaptureEv"]
    pub(super) fn status_end_PackunFinalCapture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44FighterStatusPackunFinalCapture_check_damageEv"]
    pub(super) fn FighterStatusPackunFinalCapture_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_get_motion_kind_CaptureJackWireEv"]
    pub(super) fn sub_get_motion_kind_CaptureJackWire(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_CaptureJackWireEv"]
    pub(super) fn status_pre_CaptureJackWire(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_CapturePulledEv"]
    pub(super) fn status_pre_CapturePulled(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_CapturePulled_commonEN3lib8L2CValueES2_"]
    pub(super) fn status_pre_CapturePulled_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_CaptureJackWireEv"]
    pub(super) fn status_CaptureJackWire(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19CapturePulledCommonEv"]
    pub(super) fn CapturePulledCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_CaptureJackWire_MainEv"]
    pub(super) fn status_CaptureJackWire_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_check_capture_cutEN3lib8L2CValueES2_"]
    pub(super) fn sub_check_capture_cut(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39FighterStatusCaptrue_set_correct_groundEv"]
    pub(super) fn FighterStatusCaptrue_set_correct_ground(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_CaptureJackWire_uniq_process_initEv"]
    pub(super) fn sub_CaptureJackWire_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_capture_pulled_uniq_process_init_commonEN3lib8L2CValueE"]
    pub(super) fn sub_capture_pulled_uniq_process_init_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36local_func__fighter_status_capture_1Ev"]
    pub(super) fn local_func__fighter_status_capture_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_capture_pulled_uniq_process_init_coreEN3lib8L2CValueES2_"]
    pub(super) fn sub_capture_pulled_uniq_process_init_core(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13getMotionKindEN3lib8L2CValueES2_"]
    pub(super) fn getMotionKind(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_capture_pulled_uniq_process_initEv"]
    pub(super) fn sub_capture_pulled_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_CaptureJackWire_uniq_process_execEv"]
    pub(super) fn sub_CaptureJackWire_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_capture_uniq_process_exec_commonEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn sub_capture_uniq_process_exec_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28CaptureCommonJumpButtonCountEv"]
    pub(super) fn CaptureCommonJumpButtonCount(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_CaptureJackWireEv"]
    pub(super) fn status_end_CaptureJackWire(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_CapturePulledEv"]
    pub(super) fn status_end_CapturePulled(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_JackFinalTargetStartEv"]
    pub(super) fn status_pre_JackFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_FinalTargetStartMainEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_pre_FinalTargetStartMain(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_JackFinalTargetStartEv"]
    pub(super) fn status_JackFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_JackFinalTargetStart_MainEv"]
    pub(super) fn status_JackFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_JackFinalTargetStartEv"]
    pub(super) fn status_end_JackFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_JackFinalTargetDamageEv"]
    pub(super) fn status_pre_JackFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_JackFinalTargetDamage_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_JackFinalTargetDamage_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_JackFinalTargetDamageEv"]
    pub(super) fn status_JackFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_JackFinalTargetDamage_MainEv"]
    pub(super) fn status_JackFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_JackFinalTargetDamageEv"]
    pub(super) fn status_end_JackFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusJackFinalTargetDamage_check_damageEv"]
    pub(super) fn FighterStatusJackFinalTargetDamage_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_JackFinalTargetEndEv"]
    pub(super) fn status_pre_JackFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_JackFinalTargetEndEv"]
    pub(super) fn status_JackFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_JackFinalTargetEnd_MainEv"]
    pub(super) fn status_JackFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_JackFinalTargetEndEv"]
    pub(super) fn status_end_JackFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_BraveFinalTargetStartEv"]
    pub(super) fn status_BraveFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_BraveFinalTargetStart_MainEv"]
    pub(super) fn status_BraveFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_BraveFinalTargetStartEv"]
    pub(super) fn status_end_BraveFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_BraveFinalTargetDamageEv"]
    pub(super) fn status_pre_BraveFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_BraveFinalTargetDamageEv"]
    pub(super) fn status_BraveFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_BraveFinalTargetDamage_MainEv"]
    pub(super) fn status_BraveFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_BraveFinalTargetDamageEv"]
    pub(super) fn status_end_BraveFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_BraveFinalTargetEndEv"]
    pub(super) fn status_pre_BraveFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_BuddyFinalTargetStartEv"]
    pub(super) fn status_BuddyFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_BuddyFinalTargetStart_MainEv"]
    pub(super) fn status_BuddyFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_BuddyFinalTargetStartEv"]
    pub(super) fn status_end_BuddyFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_BuddyFinalTargetStart_init_statusEv"]
    pub(super) fn sub_BuddyFinalTargetStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_BuddyFinalTargetDamageEv"]
    pub(super) fn status_pre_BuddyFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_BuddyFinalTargetDamageEv"]
    pub(super) fn status_BuddyFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_BuddyFinalTargetDamage_MainEv"]
    pub(super) fn status_BuddyFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusBuddyFinalTargetDamage_exec_fix_cameraEv"]
    pub(super) fn FighterStatusBuddyFinalTargetDamage_exec_fix_camera(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_BuddyFinalTargetDamageEv"]
    pub(super) fn status_end_BuddyFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusBuddyFinalTargetDamage_check_damageEv"]
    pub(super) fn FighterStatusBuddyFinalTargetDamage_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_BuddyFinalTargetEndEv"]
    pub(super) fn status_pre_BuddyFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_BuddyFinalTargetEndEv"]
    pub(super) fn status_BuddyFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_BuddyFinalTargetEnd_MainEv"]
    pub(super) fn status_BuddyFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_BuddyFinalTargetEndEv"]
    pub(super) fn status_end_BuddyFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_pre_DollySuperSpecial2CaptureEv"]
    pub(super) fn status_pre_DollySuperSpecial2Capture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_DollySuperSpecial2CaptureEv"]
    pub(super) fn status_DollySuperSpecial2Capture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37status_DollySuperSpecial2Capture_MainEv"]
    pub(super) fn status_DollySuperSpecial2Capture_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47sub_DollySuperSpecial2Capture_uniq_process_initEv"]
    pub(super) fn sub_DollySuperSpecial2Capture_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47sub_DollySuperSpecial2Capture_uniq_process_execEv"]
    pub(super) fn sub_DollySuperSpecial2Capture_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_end_DollySuperSpecial2CaptureEv"]
    pub(super) fn status_end_DollySuperSpecial2Capture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47sub_DollySuperSpecial2Capture_uniq_process_exitEv"]
    pub(super) fn sub_DollySuperSpecial2Capture_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_DollySuperSpecial2Capture_check_damageEN3lib8L2CValueE"]
    pub(super) fn sub_DollySuperSpecial2Capture_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_DollyFinalTargetWaitEv"]
    pub(super) fn status_pre_DollyFinalTargetWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_DollyFinalTargetWaitEv"]
    pub(super) fn status_DollyFinalTargetWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_DollyFinalTargetWait_MainEv"]
    pub(super) fn status_DollyFinalTargetWait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_DollyFinalTargetWaitEv"]
    pub(super) fn status_end_DollyFinalTargetWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusDollyFinalTargetWait_check_damageEv"]
    pub(super) fn FighterStatusDollyFinalTargetWait_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_DollyFinalTargetStartEv"]
    pub(super) fn status_pre_DollyFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_DollyFinalTargetStartEv"]
    pub(super) fn status_DollyFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_DollyFinalTargetStart_MainEv"]
    pub(super) fn status_DollyFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_DollyFinalTargetStartEv"]
    pub(super) fn status_end_DollyFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_DollyFinalTargetDamageEv"]
    pub(super) fn status_pre_DollyFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_DollyFinalTargetDamageEv"]
    pub(super) fn status_DollyFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_DollyFinalTargetDamage_MainEv"]
    pub(super) fn status_DollyFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_DollyFinalTargetDamageEv"]
    pub(super) fn status_end_DollyFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusDollyFinalTargetDamage_check_damageEv"]
    pub(super) fn FighterStatusDollyFinalTargetDamage_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_DollyFinalTargetEndEv"]
    pub(super) fn status_pre_DollyFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_DollyFinalTargetEndEv"]
    pub(super) fn status_DollyFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_DollyFinalTargetEnd_MainEv"]
    pub(super) fn status_DollyFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_DollyFinalTargetEndEv"]
    pub(super) fn status_end_DollyFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_CaptureMasterSwordEv"]
    pub(super) fn status_pre_CaptureMasterSword(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CaptureMasterSwordEv"]
    pub(super) fn status_CaptureMasterSword(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_CaptureMasterSword_MainEv"]
    pub(super) fn status_CaptureMasterSword_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_CaptureMasterSwordEv"]
    pub(super) fn status_end_CaptureMasterSword(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessCaptureMasterSword_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureMasterSword_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_MasterFinalTargetStartEv"]
    pub(super) fn status_MasterFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_MasterFinalTargetStart_MainEv"]
    pub(super) fn status_MasterFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_MasterFinalTargetStartEv"]
    pub(super) fn status_end_MasterFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_MasterFinalTargetStart_init_statusEv"]
    pub(super) fn sub_MasterFinalTargetStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_MasterFinalTargetDamageEv"]
    pub(super) fn status_pre_MasterFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_MasterFinalTargetDamageEv"]
    pub(super) fn status_MasterFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_MasterFinalTargetDamage_MainEv"]
    pub(super) fn status_MasterFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_MasterFinalTargetDamageEv"]
    pub(super) fn status_end_MasterFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusMasterFinalTargetDamage_check_damageEv"]
    pub(super) fn FighterStatusMasterFinalTargetDamage_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_MasterFinalTargetEndEv"]
    pub(super) fn status_pre_MasterFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_MasterFinalTargetEndEv"]
    pub(super) fn status_MasterFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_MasterFinalTargetEnd_MainEv"]
    pub(super) fn status_MasterFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_MasterFinalTargetEndEv"]
    pub(super) fn status_end_MasterFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48local_func__fighter_status_tantan_final_target_1Ev"]
    pub(super) fn local_func__fighter_status_tantan_final_target_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50local_func__fighter_status_tantan_final_target_1_1Ev"]
    pub(super) fn local_func__fighter_status_tantan_final_target_1_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50local_func__fighter_status_tantan_final_target_2_1Ev"]
    pub(super) fn local_func__fighter_status_tantan_final_target_2_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50local_func__fighter_status_tantan_final_target_3_1Ev"]
    pub(super) fn local_func__fighter_status_tantan_final_target_3_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_TantanFinalTargetStartEv"]
    pub(super) fn status_pre_TantanFinalTargetStart(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_FinalTargetStartFoxEv"]
    pub(super) fn status_pre_FinalTargetStartFox(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_TantanFinalTargetStartEv"]
    pub(super) fn status_TantanFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_TantanFinalTargetStart_MainEv"]
    pub(super) fn status_TantanFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_TantanFinalTargetStartEv"]
    pub(super) fn status_end_TantanFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_TantanFinalTargetStart_init_statusEv"]
    pub(super) fn sub_TantanFinalTargetStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_TantanFinalTargetDamageEv"]
    pub(super) fn status_pre_TantanFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_TantanFinalTargetDamageEv"]
    pub(super) fn status_TantanFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_TantanFinalTargetDamage_MainEv"]
    pub(super) fn status_TantanFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50local_func__fighter_status_tantan_final_target_4_1Ev"]
    pub(super) fn local_func__fighter_status_tantan_final_target_4_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_TantanFinalTargetDamageEv"]
    pub(super) fn status_end_TantanFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusTantanFinalTargetDamage_check_damageEv"]
    pub(super) fn FighterStatusTantanFinalTargetDamage_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_TantanFinalTargetEndEv"]
    pub(super) fn status_pre_TantanFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_TantanFinalTargetEndEv"]
    pub(super) fn status_TantanFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_TantanFinalTargetEnd_MainEv"]
    pub(super) fn status_TantanFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_TantanFinalTargetEndEv"]
    pub(super) fn status_end_TantanFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_CapturePulledPickelEv"]
    pub(super) fn status_pre_CapturePulledPickel(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_CapturePulledPickelEv"]
    pub(super) fn status_CapturePulledPickel(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24CapturePulledCommon_MainEv"]
    pub(super) fn CapturePulledCommon_Main(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22flip_have_item_captureEv"]
    pub(super) fn flip_have_item_capture(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CapturePulled_MainEv"]
    pub(super) fn status_CapturePulled_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33is_capture_pulled_special_fighterEv"]
    pub(super) fn is_capture_pulled_special_fighter(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_CapturePulledPickelEv"]
    pub(super) fn status_end_CapturePulledPickel(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_CatchedPickelTrolleyEv"]
    pub(super) fn status_pre_CatchedPickelTrolley(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_CatchedPickelTrolleyEv"]
    pub(super) fn status_CatchedPickelTrolley(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_CatchedPickelTrolley_MainEv"]
    pub(super) fn status_CatchedPickelTrolley_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_CatchedPickelTrolleyEv"]
    pub(super) fn status_end_CatchedPickelTrolley(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessCatchedPickelTrolley_set_scaleEv"]
    pub(super) fn FighterStatusUniqProcessCatchedPickelTrolley_set_scale(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessCatchedPickelTrolley_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessCatchedPickelTrolley_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessCatchedPickelTrolley_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessCatchedPickelTrolley_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessCatchedPickelTrolley_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessCatchedPickelTrolley_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessCatchedPickelTrolley_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessCatchedPickelTrolley_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_PickelFinalTargetWaitEv"]
    pub(super) fn status_pre_PickelFinalTargetWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_PickelFinalTargetWait_mtransEN3lib8L2CValueE"]
    pub(super) fn sub_PickelFinalTargetWait_mtrans(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_PickelFinalTargetWait_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_PickelFinalTargetWait_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_PickelFinalTargetWaitEv"]
    pub(super) fn status_PickelFinalTargetWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_PickelFinalTargetWait_MainEv"]
    pub(super) fn status_PickelFinalTargetWait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_PickelFinalTargetWaitEv"]
    pub(super) fn status_end_PickelFinalTargetWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusPickelFinalTargetWait_check_damageEv"]
    pub(super) fn FighterStatusPickelFinalTargetWait_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_req_PickelFinalTargetStart_smoke_effectEv"]
    pub(super) fn sub_req_PickelFinalTargetStart_smoke_effect(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_PickelFinalTargetStartEv"]
    pub(super) fn status_PickelFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_PickelFinalTargetStart_MainEv"]
    pub(super) fn status_PickelFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_PickelFinalTargetStartEv"]
    pub(super) fn status_end_PickelFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_PickelFinalTargetDamageEv"]
    pub(super) fn status_pre_PickelFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_PickelFinalTargetDamageEv"]
    pub(super) fn status_PickelFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_set_PickelFinalTargetDamage_rot_angle_yEv"]
    pub(super) fn sub_set_PickelFinalTargetDamage_rot_angle_y(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_PickelFinalTargetDamage_MainEv"]
    pub(super) fn status_PickelFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessPickelFinalTargetDamage_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessPickelFinalTargetDamage_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon65FighterStatusUniqProcessPickelFinalTargetDamage_exec_fix_pos_slowEv"]
    pub(super) fn FighterStatusUniqProcessPickelFinalTargetDamage_exec_fix_pos_slow(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_PickelFinalTargetDamageEv"]
    pub(super) fn status_end_PickelFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_ElementFinalTargetStartEv"]
    pub(super) fn status_pre_ElementFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_ElementFinalTargetStartEv"]
    pub(super) fn status_ElementFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_ElementFinalTargetStart_MainEv"]
    pub(super) fn status_ElementFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_ElementFinalTargetStartEv"]
    pub(super) fn status_end_ElementFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_pre_ElementFinalTargetDamageEv"]
    pub(super) fn status_pre_ElementFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_ElementFinalTargetDamageEv"]
    pub(super) fn status_ElementFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_ElementFinalTargetDamage_MainEv"]
    pub(super) fn status_ElementFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_end_ElementFinalTargetDamageEv"]
    pub(super) fn status_end_ElementFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_ElementFinalTargetEndEv"]
    pub(super) fn status_pre_ElementFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_ElementFinalTargetEndEv"]
    pub(super) fn status_ElementFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_ElementFinalTargetEnd_MainEv"]
    pub(super) fn status_ElementFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_DemonDivedEv"]
    pub(super) fn status_pre_DemonDived(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_DemonDivedEv"]
    pub(super) fn status_DemonDived(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_DemonDived_MainEv"]
    pub(super) fn status_DemonDived_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_DemonDivedEv"]
    pub(super) fn status_end_DemonDived(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_DemonDived_initEv"]
    pub(super) fn sub_uniq_process_DemonDived_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_DemonDived_execEv"]
    pub(super) fn sub_uniq_process_DemonDived_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_DemonDived_exitEv"]
    pub(super) fn sub_uniq_process_DemonDived_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_uniq_process_DemonDived_exec_fix_cameraEv"]
    pub(super) fn sub_uniq_process_DemonDived_exec_fix_camera(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessDemonDived_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessDemonDived_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_DemonFinalTargetStartEv"]
    pub(super) fn status_pre_DemonFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_DemonFinalTargetStartEv"]
    pub(super) fn status_DemonFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_DemonFinalTargetStart_MainEv"]
    pub(super) fn status_DemonFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_DemonFinalTargetStartEv"]
    pub(super) fn status_end_DemonFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_DemonFinalDamageEv"]
    pub(super) fn status_pre_DemonFinalDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_DemonFinalTargetDamageEv"]
    pub(super) fn status_DemonFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_FinalDamageCommonEv"]
    pub(super) fn status_FinalDamageCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_DemonFinalTargetDamage_MainEv"]
    pub(super) fn status_DemonFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_DemonFinalTargetDamageEv"]
    pub(super) fn status_end_DemonFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessDemonFinalDamage_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessDemonFinalDamage_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessFinalDamage_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamage_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58FighterStatusUniqProcessFinalDamage_get_damage_motion_randEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamage_get_damage_motion_rand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessDemonFinalDamage_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessDemonFinalDamage_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessFinalDamage_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamage_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_DemonFinalTargetEndEv"]
    pub(super) fn status_pre_DemonFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_DemonFinalTargetEndEv"]
    pub(super) fn status_DemonFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_DemonFinalTargetEnd_MainEv"]
    pub(super) fn status_DemonFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_EdgeFinalTargetStartEv"]
    pub(super) fn status_pre_EdgeFinalTargetStart(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_EdgeFinalTargetStartEv"]
    pub(super) fn status_EdgeFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_EdgeFinalTargetStart_MainEv"]
    pub(super) fn status_EdgeFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_EdgeFinalTargetStartEv"]
    pub(super) fn status_end_EdgeFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_EdgeFinalTargetStart_init_statusEv"]
    pub(super) fn sub_EdgeFinalTargetStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_EdgeFinalTargetDamageEv"]
    pub(super) fn status_pre_EdgeFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_EdgeFinalTargetDamageEv"]
    pub(super) fn status_EdgeFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_EdgeFinalTargetDamage_MainEv"]
    pub(super) fn status_EdgeFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_EdgeFinalTargetDamageEv"]
    pub(super) fn status_end_EdgeFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusEdgeFinalTargetDamage_check_damageEv"]
    pub(super) fn FighterStatusEdgeFinalTargetDamage_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_EdgeFinalTargetEndEv"]
    pub(super) fn status_pre_EdgeFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_EdgeFinalTargetEndEv"]
    pub(super) fn status_EdgeFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_EdgeFinalTargetEnd_MainEv"]
    pub(super) fn status_EdgeFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_EdgeFinalTargetEndEv"]
    pub(super) fn status_end_EdgeFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50local_func__fighter_status_capture_trail_keyhole_1Ev"]
    pub(super) fn local_func__fighter_status_capture_trail_keyhole_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52local_func__fighter_status_capture_trail_keyhole_1_1Ev"]
    pub(super) fn local_func__fighter_status_capture_trail_keyhole_1_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52local_func__fighter_status_capture_trail_keyhole_2_1Ev"]
    pub(super) fn local_func__fighter_status_capture_trail_keyhole_2_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52local_func__fighter_status_capture_trail_keyhole_3_1Ev"]
    pub(super) fn local_func__fighter_status_capture_trail_keyhole_3_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_CaptureTrailKeyholeEv"]
    pub(super) fn status_pre_CaptureTrailKeyhole(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_CaptureTrailKeyholeEv"]
    pub(super) fn status_CaptureTrailKeyhole(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_CaptureTrailKeyhole_MainEv"]
    pub(super) fn status_CaptureTrailKeyhole_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_CaptureTrailKeyholeEv"]
    pub(super) fn status_end_CaptureTrailKeyhole(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_capture_trail_ease_in_quadEN3lib8L2CValueE"]
    pub(super) fn sub_capture_trail_ease_in_quad(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_capture_trail_ease_out_quadEN3lib8L2CValueE"]
    pub(super) fn sub_capture_trail_ease_out_quad(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_capture_trail_ease_in_cubicEN3lib8L2CValueE"]
    pub(super) fn sub_capture_trail_ease_in_cubic(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_capture_trail_ease_out_cubicEN3lib8L2CValueE"]
    pub(super) fn sub_capture_trail_ease_out_cubic(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_capture_trail_ease_in_quintEN3lib8L2CValueE"]
    pub(super) fn sub_capture_trail_ease_in_quint(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_capture_trail_ease_out_quintEN3lib8L2CValueE"]
    pub(super) fn sub_capture_trail_ease_out_quint(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_capture_trail_ease_in_circEN3lib8L2CValueE"]
    pub(super) fn sub_capture_trail_ease_in_circ(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_capture_trail_ease_out_circEN3lib8L2CValueE"]
    pub(super) fn sub_capture_trail_ease_out_circ(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_capture_trail_keyhole_statusEN3lib8L2CValueE"]
    pub(super) fn sub_capture_trail_keyhole_status(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessCaptureTrailKeyhole_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureTrailKeyhole_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessCaptureTrailKeyhole_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureTrailKeyhole_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessCaptureTrailKeyhole_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureTrailKeyhole_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52local_func__fighter_status_capture_trail_keyhole_4_1Ev"]
    pub(super) fn local_func__fighter_status_capture_trail_keyhole_4_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_TrailReboundEv"]
    pub(super) fn status_pre_TrailRebound(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_TrailReboundEv"]
    pub(super) fn status_TrailRebound(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_TrailRebound_MainEv"]
    pub(super) fn status_TrailRebound_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_wait_ground_check_commonEN3lib8L2CValueE"]
    pub(super) fn sub_wait_ground_check_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_air_check_fall_commonEv"]
    pub(super) fn sub_air_check_fall_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_trail_change_motion_by_situationEN3lib8L2CValueES2_"]
    pub(super) fn sub_trail_change_motion_by_situation(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_transition_group_check_air_landingEv"]
    pub(super) fn sub_transition_group_check_air_landing(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_transition_group_check_air_cliffEv"]
    pub(super) fn sub_transition_group_check_air_cliff(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_rocketbelt_hover_checkEv"]
    pub(super) fn sub_rocketbelt_hover_check(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_transition_group_check_air_specialEv"]
    pub(super) fn sub_transition_group_check_air_special(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_transition_group_check_air_item_throwEv"]
    pub(super) fn sub_transition_group_check_air_item_throw(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_transition_group_check_air_lassoEv"]
    pub(super) fn sub_transition_group_check_air_lasso(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_transition_group_check_air_escapeEv"]
    pub(super) fn sub_transition_group_check_air_escape(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_transition_group_check_air_attackEv"]
    pub(super) fn sub_transition_group_check_air_attack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_transition_group_check_air_tread_jumpEv"]
    pub(super) fn sub_transition_group_check_air_tread_jump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_transition_group_check_air_wall_jumpEv"]
    pub(super) fn sub_transition_group_check_air_wall_jump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_transition_group_check_air_jump_aerialEv"]
    pub(super) fn sub_transition_group_check_air_jump_aerial(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15sub_is_fly_nextEN3lib8L2CValueE"]
    pub(super) fn sub_is_fly_next(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_fighter_general_term_is_can_attach_wallEv"]
    pub(super) fn sub_fighter_general_term_is_can_attach_wall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_fighter_general_term_is_can_wall_jumpEv"]
    pub(super) fn sub_fighter_general_term_is_can_wall_jump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39FL_sub_fighter_float_next_tread_speed_yEv"]
    pub(super) fn FL_sub_fighter_float_next_tread_speed_y(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_transition_group_check_air_jump_attackEv"]
    pub(super) fn sub_transition_group_check_air_jump_attack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_is_item_shoot_airEv"]
    pub(super) fn sub_is_item_shoot_air(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_is_fly_next_jump_triggerEN3lib8L2CValueE"]
    pub(super) fn sub_is_fly_next_jump_trigger(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25can_entry_cliff_air_lassoEv"]
    pub(super) fn can_entry_cliff_air_lasso(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_AIRChkDropItemImmEv"]
    pub(super) fn sub_AIRChkDropItemImm(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_AIRChkGetItemImmEv"]
    pub(super) fn sub_AIRChkGetItemImm(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_GetLightItemImmEN3lib8L2CValueE"]
    pub(super) fn sub_GetLightItemImm(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_transition_group_check_special_commandEv"]
    pub(super) fn sub_transition_group_check_special_command(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50sub_transition_group_check_ground_jump_mini_attackEv"]
    pub(super) fn sub_transition_group_check_ground_jump_mini_attack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_transition_group_check_ground_itemEv"]
    pub(super) fn sub_transition_group_check_ground_item(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_transition_group_check_ground_catchEv"]
    pub(super) fn sub_transition_group_check_ground_catch(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_transition_group_check_ground_escapeEv"]
    pub(super) fn sub_transition_group_check_ground_escape(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_transition_group_check_ground_guardEv"]
    pub(super) fn sub_transition_group_check_ground_guard(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_transition_group_check_ground_specialEv"]
    pub(super) fn sub_transition_group_check_ground_special(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_transition_group_check_ground_attackEv"]
    pub(super) fn sub_transition_group_check_ground_attack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_transition_group_check_ground_jumpEv"]
    pub(super) fn sub_transition_group_check_ground_jump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_transition_group_check_groundEN3lib8L2CValueE"]
    pub(super) fn sub_transition_group_check_ground(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_check_command_squatEv"]
    pub(super) fn sub_check_command_squat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_check_command_walkEv"]
    pub(super) fn sub_check_command_walk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_check_button_jumpEv"]
    pub(super) fn sub_check_button_jump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_check_button_frickEv"]
    pub(super) fn sub_check_button_frick(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_transition_specialflag_hoistEv"]
    pub(super) fn sub_transition_specialflag_hoist(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_check_command_guardEv"]
    pub(super) fn sub_check_command_guard(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30change_status_jump_mini_attackEN3lib8L2CValueE"]
    pub(super) fn change_status_jump_mini_attack(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37change_status_jump_mini_attack_commonEN3lib8L2CValueES2_"]
    pub(super) fn change_status_jump_mini_attack_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_TrailReboundEv"]
    pub(super) fn status_end_TrailRebound(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessTrailRebound_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessTrailRebound_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessTrailRebound_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessTrailRebound_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessTrailRebound_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessTrailRebound_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_TrailFinalTargetDamageEv"]
    pub(super) fn status_pre_TrailFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_TrailFinalTargetDamageEv"]
    pub(super) fn status_TrailFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_TrailFinalTargetDamage_MainEv"]
    pub(super) fn status_TrailFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusTrailFinalTargetDamage_exec_fix_cameraEv"]
    pub(super) fn FighterStatusTrailFinalTargetDamage_exec_fix_camera(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_TrailFinalTargetDamageEv"]
    pub(super) fn status_end_TrailFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusTrailFinalTargetDamage_check_damageEv"]
    pub(super) fn FighterStatusTrailFinalTargetDamage_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_TrailFinalTargetEndEv"]
    pub(super) fn status_pre_TrailFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_TrailFinalTargetEndEv"]
    pub(super) fn status_TrailFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_TrailFinalTargetEnd_MainEv"]
    pub(super) fn status_TrailFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_TrailFinalTargetEndEv"]
    pub(super) fn status_end_TrailFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_wait_ground_check_common_preEv"]
    pub(super) fn sub_wait_ground_check_common_pre(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_air_check_fall_common_preEv"]
    pub(super) fn sub_air_check_fall_common_pre(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_check_damage_knock_outEv"]
    pub(super) fn sub_check_damage_knock_out(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_transition_group_disguiseEN3lib8L2CValueE"]
    pub(super) fn sub_transition_group_disguise(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_transition_term_id_cont_disguiseEN3lib8L2CValueE"]
    pub(super) fn sub_transition_term_id_cont_disguise(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_check_jump_in_chargingEv"]
    pub(super) fn sub_check_jump_in_charging(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44sub_check_jump_in_charging_for_cancel_statusEN3lib8L2CValueE"]
    pub(super) fn sub_check_jump_in_charging_for_cancel_status(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_check_fly_in_charging_for_cancel_statusEN3lib8L2CValueE"]
    pub(super) fn sub_check_fly_in_charging_for_cancel_status(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_check_charge_cancel_jump_mini_attackEv"]
    pub(super) fn sub_check_charge_cancel_jump_mini_attack(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_transition_group_check_air_cliff_forceEv"]
    pub(super) fn sub_transition_group_check_air_cliff_force(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_status_pre_SpecialNCommonEv"]
    pub(super) fn sub_status_pre_SpecialNCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_status_pre_FinalCommonEv"]
    pub(super) fn sub_status_pre_FinalCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20delete_clatter_inputEv"]
    pub(super) fn delete_clatter_input(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_fighter_change_statusEN3lib8L2CValueES2_"]
    pub(super) fn sub_fighter_change_status(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_fighter_check_abnormal_fail_in_attackEN3lib8L2CValueE"]
    pub(super) fn sub_fighter_check_abnormal_fail_in_attack(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_fighter_change_status_shiftEv"]
    pub(super) fn sub_fighter_change_status_shift(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_fighter_remake_control_commandEv"]
    pub(super) fn sub_fighter_remake_control_command(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_fighter_line_status_systemEv"]
    pub(super) fn sub_fighter_line_status_system(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_fighter_general_term_is_cliff_check_posEv"]
    pub(super) fn sub_fighter_general_term_is_cliff_check_pos(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_fighter_cliff_checkEN3lib8L2CValueE"]
    pub(super) fn sub_fighter_cliff_check(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_utility_mediate_motionEv"]
    pub(super) fn status_utility_mediate_motion(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_fighter_final_end_to_fall_commonEv"]
    pub(super) fn sub_fighter_final_end_to_fall_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_fighter_do_control_passableEv"]
    pub(super) fn sub_fighter_do_control_passable(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_get_adjust_rate_from_cancel_frameEN3lib8L2CValueES2_"]
    pub(super) fn sub_get_adjust_rate_from_cancel_frame(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_get_adjust_rate_from_cancel_frame_invEN3lib8L2CValueES2_"]
    pub(super) fn sub_get_adjust_rate_from_cancel_frame_inv(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_set_meteor_hit_id_to_workEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn sub_set_meteor_hit_id_to_work(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_off_passive_opponentEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_off_passive_opponent(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_set_reaction_frame_by_current_motionEv"]
    pub(super) fn sub_set_reaction_frame_by_current_motion(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_set_reaction_frame_by_motionEN3lib8L2CValueE"]
    pub(super) fn sub_set_reaction_frame_by_motion(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44sub_set_special_start_common_kinetic_settingEN3lib8L2CValueE"]
    pub(super) fn sub_set_special_start_common_kinetic_setting(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52sub_set_special_start_inherit_common_kinetic_settingEN3lib8L2CValueE"]
    pub(super) fn sub_set_special_start_inherit_common_kinetic_setting(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51sub_exec_special_start_common_kinetic_setting_innerEN3lib8L2CValueES2_"]
    pub(super) fn sub_exec_special_start_common_kinetic_setting_inner(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58sub_exec_special_start_common_kinetic_setting_gravity_funcEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_exec_special_start_common_kinetic_setting_gravity_func(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_exec_special_start_common_kinetic_settingEN3lib8L2CValueE"]
    pub(super) fn sub_exec_special_start_common_kinetic_setting(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_change_motion_by_situation_innnerEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn sub_change_motion_by_situation_innner(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_change_motion_by_situationEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_change_motion_by_situation(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_change_motion_by_situation_kirby_copyEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_change_motion_by_situation_kirby_copy(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_set_ground_correct_by_situationEN3lib8L2CValueE"]
    pub(super) fn sub_set_ground_correct_by_situation(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_change_kinetic_type_by_situationEN3lib8L2CValueES2_"]
    pub(super) fn sub_change_kinetic_type_by_situation(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_remove_exist_article_at_status_endEN3lib8L2CValueES2_"]
    pub(super) fn sub_remove_exist_article_at_status_end(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14sub_is_body_upEv"]
    pub(super) fn sub_is_body_up(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessAirLassoCommon_notify_event_linkEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessAirLassoCommon_notify_event_link(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon66FighterStatusUniqProcessAirLassoCommon_change_motion_sync_shoulderEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessAirLassoCommon_change_motion_sync_shoulder(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58FighterStatusUniqProcessAirLassoCommon_exec_map_correctionEv"]
    pub(super) fn FighterStatusUniqProcessAirLassoCommon_exec_map_correction(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_AirLassoEv"]
    pub(super) fn status_pre_AirLasso(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_air_lasso_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_air_lasso_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_AirLassoEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn status_AirLasso(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_air_lasso_mainEv"]
    pub(super) fn status_air_lasso_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45FighterStatusUniqProcessAirLasso_check_attackEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessAirLasso_check_attack(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21remove_exist_article2Ev"]
    pub(super) fn remove_exist_article2(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_AirLassoEv"]
    pub(super) fn status_end_AirLasso(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44FighterStatusUniqProcessAirLasso_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessAirLasso_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44FighterStatusUniqProcessAirLasso_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessAirLasso_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_AirLassoReachEv"]
    pub(super) fn status_pre_AirLassoReach(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_AirLassoReachEN3lib8L2CValueE"]
    pub(super) fn status_AirLassoReach(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_air_lasso_reach_mainEv"]
    pub(super) fn status_air_lasso_reach_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessAirLassoReach_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessAirLassoReach_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessAirLassoReach_exec_fix_pos_commonEv"]
    pub(super) fn FighterStatusUniqProcessAirLassoReach_exec_fix_pos_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_AirLassoReachEv"]
    pub(super) fn status_end_AirLassoReach(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessAirLassoReach_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessAirLassoReach_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessAirLassoReach_exec_fix_pos_slowEv"]
    pub(super) fn FighterStatusUniqProcessAirLassoReach_exec_fix_pos_slow(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_AirLassoHangEv"]
    pub(super) fn status_pre_AirLassoHang(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_air_lasso_hang_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_air_lasso_hang_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_AirLassoHangEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_AirLassoHang(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_air_lasso_hang_mainEv"]
    pub(super) fn status_air_lasso_hang_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46sub_transition_group_check_air_cliff_air_lassoEv"]
    pub(super) fn sub_transition_group_check_air_cliff_air_lasso(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_check_over_cliffEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn sub_check_over_cliff(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_air_lasso_landing_limit_speedEv"]
    pub(super) fn sub_air_lasso_landing_limit_speed(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_AirLassoHangEv"]
    pub(super) fn status_end_AirLassoHang(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_AirLassoHangCommonEN3lib8L2CValueES2_"]
    pub(super) fn status_end_AirLassoHangCommon(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessAirLassoHang_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessAirLassoHang_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessAirLassoHang_exec_map_correctionEv"]
    pub(super) fn FighterStatusUniqProcessAirLassoHang_exec_map_correction(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessAirLassoHang_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessAirLassoHang_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_AirLassoRewindEv"]
    pub(super) fn status_pre_AirLassoRewind(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_air_lasso_rewind_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_air_lasso_rewind_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_AirLassoRewindEN3lib8L2CValueE"]
    pub(super) fn status_AirLassoRewind(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_air_lasso_rewind_mainEv"]
    pub(super) fn status_air_lasso_rewind_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_AirLassoRewindEv"]
    pub(super) fn status_end_AirLassoRewind(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessAirLassoRewind_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessAirLassoRewind_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessAirLassoRewind_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessAirLassoRewind_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_AirLassoFailureEv"]
    pub(super) fn status_pre_AirLassoFailure(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_air_lasso_failure_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_air_lasso_failure_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_AirLassoFailureEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_AirLassoFailure(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_AirLassoFailureCommonEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn status_AirLassoFailureCommon(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_air_lasso_failure_mainEv"]
    pub(super) fn status_air_lasso_failure_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_AirLassoFailureEv"]
    pub(super) fn status_end_AirLassoFailure(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessAirLassoFailure_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessAirLassoFailure_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_AirLassoLandingEv"]
    pub(super) fn status_pre_AirLassoLanding(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_pre_landing_kinetic_typeEv"]
    pub(super) fn sub_pre_landing_kinetic_type(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_AirLassoLandingEN3lib8L2CValueE"]
    pub(super) fn status_AirLassoLanding(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_LandingSubEv"]
    pub(super) fn status_LandingSub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_LandingStiffnessEv"]
    pub(super) fn status_LandingStiffness(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_air_lasso_landing_mainEv"]
    pub(super) fn status_air_lasso_landing_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_Landing_MainSubEv"]
    pub(super) fn status_Landing_MainSub(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_landing_cancel_check_damage_faceEv"]
    pub(super) fn sub_landing_cancel_check_damage_face(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_landing_cancel_damage_faceEv"]
    pub(super) fn sub_landing_cancel_damage_face(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_landing_uniq_check_stransEv"]
    pub(super) fn sub_landing_uniq_check_strans(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_landing_ground_check_commonEv"]
    pub(super) fn sub_landing_ground_check_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23FL_get_LandingStiffnessEv"]
    pub(super) fn FL_get_LandingStiffness(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_landing_ground_check_common_preEv"]
    pub(super) fn sub_landing_ground_check_common_pre(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_landing_uniq_checkEv"]
    pub(super) fn sub_landing_uniq_check(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_Landing_MainEv"]
    pub(super) fn status_Landing_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_AirLassoLandingEv"]
    pub(super) fn status_end_AirLassoLanding(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_attach_wallEv"]
    pub(super) fn status_pre_attach_wall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_attach_wallEv"]
    pub(super) fn status_attach_wall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_attach_wall_uniqEv"]
    pub(super) fn sub_attach_wall_uniq(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_attach_wall_mainEv"]
    pub(super) fn status_attach_wall_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_uniq_process_Passive_exec_fix_posEv"]
    pub(super) fn sub_uniq_process_Passive_exec_fix_pos(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_attach_wallEv"]
    pub(super) fn status_end_attach_wall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_attach_wall_waitEv"]
    pub(super) fn status_pre_attach_wall_wait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_attach_wall_waitEv"]
    pub(super) fn status_attach_wall_wait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_attach_wall_wait_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_attach_wall_wait_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_attach_wall_wait_mainEv"]
    pub(super) fn status_attach_wall_wait_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_attach_wall_waitEv"]
    pub(super) fn status_end_attach_wall_wait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_DetachWallEv"]
    pub(super) fn status_pre_DetachWall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_DetachWallEv"]
    pub(super) fn status_DetachWall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_DetachWall_MainEv"]
    pub(super) fn status_DetachWall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_DetachWallEv"]
    pub(super) fn status_end_DetachWall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_DetachWallFallEv"]
    pub(super) fn status_pre_DetachWallFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_DetachWallFallEv"]
    pub(super) fn status_DetachWallFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_DetachWallFall_MainEv"]
    pub(super) fn status_DetachWallFall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_DetachWallFallEv"]
    pub(super) fn status_end_DetachWallFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_DetachWallJumpEv"]
    pub(super) fn status_pre_DetachWallJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_DetachWallJumpEv"]
    pub(super) fn status_DetachWallJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_JumpAerialSubEN3lib8L2CValueES2_"]
    pub(super) fn status_JumpAerialSub(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_JumpAerial_MainEv"]
    pub(super) fn status_JumpAerial_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_air_check_stop_ceilEv"]
    pub(super) fn sub_air_check_stop_ceil(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_air_check_superleaf_fall_slowlyEv"]
    pub(super) fn sub_air_check_superleaf_fall_slowly(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_is_fall_slowlyEv"]
    pub(super) fn sub_is_fall_slowly(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_jump_aerial_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_jump_aerial_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_fall_common_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_fall_common_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_glide_stick_check_uniqEv"]
    pub(super) fn sub_glide_stick_check_uniq(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_air_check_diveEv"]
    pub(super) fn sub_air_check_dive(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11sub_is_diveEv"]
    pub(super) fn sub_is_dive(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16check_mach_stampEv"]
    pub(super) fn check_mach_stamp(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_DetachWallJump_MainEv"]
    pub(super) fn status_DetachWallJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_DetachWallJumpEv"]
    pub(super) fn status_end_DetachWallJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34get_mini_jump_attack_data_log_infoEN3lib8L2CValueE"]
    pub(super) fn get_mini_jump_attack_data_log_info(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41get_mini_jump_attack_data_cancel_functionEN3lib8L2CValueE"]
    pub(super) fn get_mini_jump_attack_data_cancel_function(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13is_smash_holdEN3lib8L2CValueE"]
    pub(super) fn is_smash_hold(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_pre_AttackEv"]
    pub(super) fn status_pre_Attack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_status_AttackCommonEv"]
    pub(super) fn sub_status_AttackCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_status_AttackComboCommonEv"]
    pub(super) fn sub_status_AttackComboCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_status_AttackComboCommon_buttonEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_status_AttackComboCommon_button(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21attack_combo_uniq_chkEN3lib8L2CValueE"]
    pub(super) fn attack_combo_uniq_chk(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28attack_combo_uniq_chk_buttonEN3lib8L2CValueES2_S2_"]
    pub(super) fn attack_combo_uniq_chk_button(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23attack_uniq_chk_commandEN3lib8L2CValueE"]
    pub(super) fn attack_uniq_chk_command(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26attack_combo_none_uniq_chkEN3lib8L2CValueE"]
    pub(super) fn attack_combo_none_uniq_chk(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33attack_combo_none_uniq_chk_buttonEN3lib8L2CValueES2_S2_"]
    pub(super) fn attack_combo_none_uniq_chk_button(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13status_AttackEv"]
    pub(super) fn status_Attack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19check_attack_mtransEv"]
    pub(super) fn check_attack_mtrans(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_Attack_MainEv"]
    pub(super) fn status_Attack_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_Attack_Main_buttonEN3lib8L2CValueES2_"]
    pub(super) fn status_Attack_Main_button(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22check_100_count_buttonEN3lib8L2CValueE"]
    pub(super) fn check_100_count_button(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25attack_mtrans_pre_processEv"]
    pub(super) fn attack_mtrans_pre_process(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26attack_mtrans_post_processEv"]
    pub(super) fn attack_mtrans_post_process(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15check_100_countEv"]
    pub(super) fn check_100_count(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15attack_uniq_chkEv"]
    pub(super) fn attack_uniq_chk(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13attack_mtransEv"]
    pub(super) fn attack_mtrans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_end_AttackEv"]
    pub(super) fn status_end_Attack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43FighterStatusUniqProcessAttack_check_attackEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessAttack_check_attack(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_attack_2Ev"]
    pub(super) fn local_func__fighter_status_attack_2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_attack_3Ev"]
    pub(super) fn local_func__fighter_status_attack_3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_Attack100Ev"]
    pub(super) fn status_pre_Attack100(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_status_Attack100_commonEv"]
    pub(super) fn sub_status_Attack100_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Attack100Ev"]
    pub(super) fn status_Attack100(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_Attack100_Main_uniq_funcEN3lib8L2CValueE"]
    pub(super) fn status_Attack100_Main_uniq_func(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25attack_100_start_uniq_chkEN3lib8L2CValueE"]
    pub(super) fn attack_100_start_uniq_chk(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_attack_100_uniq_checkEN3lib8L2CValueE"]
    pub(super) fn sub_attack_100_uniq_check(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_attack_100_uniq_check_buttonEN3lib8L2CValueES2_"]
    pub(super) fn sub_attack_100_uniq_check_button(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_Attack100_MainEv"]
    pub(super) fn status_Attack100_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_Attack100Ev"]
    pub(super) fn status_end_Attack100(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessAttack100_check_attackEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessAttack100_check_attack(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_AttackDashEv"]
    pub(super) fn status_pre_AttackDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_attack_dash_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_attack_dash_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_AttackDashEv"]
    pub(super) fn status_AttackDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_AttackDash_MainEv"]
    pub(super) fn status_AttackDash_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_AttackDashEv"]
    pub(super) fn status_end_AttackDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_AttackS3_commonEN3lib8L2CValueE"]
    pub(super) fn status_pre_AttackS3_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_AttackS3Ev"]
    pub(super) fn status_pre_AttackS3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_check_attack_lw_status_func_commonEv"]
    pub(super) fn sub_check_attack_lw_status_func_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_attack3_uniq_check_is_buttonEv"]
    pub(super) fn sub_attack3_uniq_check_is_button(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_attack3_uniq_checkEN3lib8L2CValueE"]
    pub(super) fn sub_attack3_uniq_check(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_attack3_uniq_check_paramEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_attack3_uniq_check_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_AttackS3CommonEv"]
    pub(super) fn status_AttackS3Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_AttackS3Ev"]
    pub(super) fn status_AttackS3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_AttackS3_Main_paramEN3lib8L2CValueE"]
    pub(super) fn status_AttackS3_Main_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22attack_s3_mtrans_paramEN3lib8L2CValueE"]
    pub(super) fn attack_s3_mtrans_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16attack_s3_mtransEv"]
    pub(super) fn attack_s3_mtrans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_AttackS3_MainEv"]
    pub(super) fn status_AttackS3_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_AttackS3Ev"]
    pub(super) fn status_end_AttackS3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_AttackHi3Ev"]
    pub(super) fn status_pre_AttackHi3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_AttackHi3_CommonEN3lib8L2CValueES2_"]
    pub(super) fn status_AttackHi3_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_AttackHi3_Common_paramEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_AttackHi3_Common_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_AttackHi3Ev"]
    pub(super) fn status_AttackHi3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_AttackHi3_MainEv"]
    pub(super) fn status_AttackHi3_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_AttackHi3_Main_minjumpEv"]
    pub(super) fn status_AttackHi3_Main_minjump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_AttackHi3Ev"]
    pub(super) fn status_end_AttackHi3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_AttackLw3_commonEN3lib8L2CValueE"]
    pub(super) fn status_pre_AttackLw3_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_AttackLw3Ev"]
    pub(super) fn status_pre_AttackLw3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_AttackLw3_commonEv"]
    pub(super) fn status_AttackLw3_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_AttackLw3_common_paramEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_AttackLw3_common_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_AttackLw3Ev"]
    pub(super) fn status_AttackLw3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_AttackLw3_MainEv"]
    pub(super) fn status_AttackLw3_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_AttackLw3Ev"]
    pub(super) fn status_end_AttackLw3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessAttackLw3_check_attackEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessAttackLw3_check_attack(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_attack_4Ev"]
    pub(super) fn local_func__fighter_status_attack_4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_attack_5Ev"]
    pub(super) fn local_func__fighter_status_attack_5(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_AttackS4StartEv"]
    pub(super) fn status_pre_AttackS4Start(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_smash_hold_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_smash_hold_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_attack_s4_turn_revEv"]
    pub(super) fn sub_attack_s4_turn_rev(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_end_attack_s4_turn_revEv"]
    pub(super) fn sub_end_attack_s4_turn_rev(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_AttackS4Start_CommonEv"]
    pub(super) fn status_AttackS4Start_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_AttackS4StartEv"]
    pub(super) fn status_AttackS4Start(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_AttackS4Start_MainEv"]
    pub(super) fn status_AttackS4Start_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_AttackXX4StartEv"]
    pub(super) fn status_end_AttackXX4Start(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_AttackS4StartEv"]
    pub(super) fn status_end_AttackS4Start(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_AttackS4HoldEv"]
    pub(super) fn status_pre_AttackS4Hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_AttackS4Hold_CommonEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_pre_AttackS4Hold_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_AttackS4HoldEv"]
    pub(super) fn status_AttackS4Hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_AttackS4Hold_mainEv"]
    pub(super) fn status_AttackS4Hold_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_AttackS4HoldEv"]
    pub(super) fn status_end_AttackS4Hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_AttackS4Ev"]
    pub(super) fn status_pre_AttackS4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_AttackS4_CommonEN3lib8L2CValueE"]
    pub(super) fn status_pre_AttackS4_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_AttackS4Ev"]
    pub(super) fn status_AttackS4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12sub_AttackS4EN3lib8L2CValueE"]
    pub(super) fn sub_AttackS4(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_AttackS4_MainEv"]
    pub(super) fn status_AttackS4_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16attack_s4_mtransEv"]
    pub(super) fn attack_s4_mtrans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_AttackS4Ev"]
    pub(super) fn status_end_AttackS4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_attack_s4_common_uniq_process_initEv"]
    pub(super) fn sub_attack_s4_common_uniq_process_init(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_attack_xx4_uniq_process_initEv"]
    pub(super) fn sub_attack_xx4_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_set_attack_s4_power_mulEv"]
    pub(super) fn sub_set_attack_s4_power_mul(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_attack_xx4_uniq_process_execEv"]
    pub(super) fn sub_attack_xx4_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_attack_xx4_common_uniq_process_exitEv"]
    pub(super) fn sub_attack_xx4_common_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_attack_s4_uniq_process_exitEv"]
    pub(super) fn sub_attack_s4_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23item_swing_motion4_holdEv"]
    pub(super) fn item_swing_motion4_hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21get_swing_motion_dataEN3lib8L2CValueES2_"]
    pub(super) fn get_swing_motion_data(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39local_func__fighter_status_item_swing_1Ev"]
    pub(super) fn local_func__fighter_status_item_swing_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39local_func__fighter_status_item_swing_2Ev"]
    pub(super) fn local_func__fighter_status_item_swing_2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39local_func__fighter_status_item_swing_3Ev"]
    pub(super) fn local_func__fighter_status_item_swing_3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39local_func__fighter_status_item_swing_4Ev"]
    pub(super) fn local_func__fighter_status_item_swing_4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39local_func__fighter_status_item_swing_5Ev"]
    pub(super) fn local_func__fighter_status_item_swing_5(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39local_func__fighter_status_item_swing_6Ev"]
    pub(super) fn local_func__fighter_status_item_swing_6(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39local_func__fighter_status_item_swing_7Ev"]
    pub(super) fn local_func__fighter_status_item_swing_7(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39local_func__fighter_status_item_swing_8Ev"]
    pub(super) fn local_func__fighter_status_item_swing_8(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39local_func__fighter_status_item_swing_9Ev"]
    pub(super) fn local_func__fighter_status_item_swing_9(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40local_func__fighter_status_item_swing_10Ev"]
    pub(super) fn local_func__fighter_status_item_swing_10(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_AttackHi4StartEv"]
    pub(super) fn status_pre_AttackHi4Start(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_AttackHi4Start_commonEN3lib8L2CValueE"]
    pub(super) fn status_pre_AttackHi4Start_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_status_AttackHi4Start_commonEN3lib8L2CValueE"]
    pub(super) fn sub_status_AttackHi4Start_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_AttackHi4Start_commonEN3lib8L2CValueE"]
    pub(super) fn status_AttackHi4Start_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_AttackHi4Start_MainEv"]
    pub(super) fn status_AttackHi4Start_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_AttackHi4StartEv"]
    pub(super) fn status_AttackHi4Start(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_AttackHi4StartEv"]
    pub(super) fn status_end_AttackHi4Start(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_AttackHi4HoldEv"]
    pub(super) fn status_pre_AttackHi4Hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_AttackHi4Hold_CommonEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_pre_AttackHi4Hold_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_AttackHi4Hold_commonEN3lib8L2CValueE"]
    pub(super) fn status_AttackHi4Hold_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_AttackHi4Hold_mainEv"]
    pub(super) fn status_AttackHi4Hold_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_AttackHi4HoldEv"]
    pub(super) fn status_AttackHi4Hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_AttackHi4HoldEv"]
    pub(super) fn status_end_AttackHi4Hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_AttackHi4Ev"]
    pub(super) fn status_pre_AttackHi4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_AttackHi4_commonEN3lib8L2CValueE"]
    pub(super) fn status_AttackHi4_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_AttackHi4Ev"]
    pub(super) fn status_AttackHi4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_AttackHi4_MainEv"]
    pub(super) fn status_AttackHi4_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_AttackHi4Ev"]
    pub(super) fn status_end_AttackHi4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_AttackLw4StartEv"]
    pub(super) fn status_pre_AttackLw4Start(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_AttackLw4Start_commonEv"]
    pub(super) fn status_AttackLw4Start_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_AttackLw4StartEv"]
    pub(super) fn status_AttackLw4Start(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_AttackLw4Start_MainEv"]
    pub(super) fn status_AttackLw4Start_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_AttackLw4StartEv"]
    pub(super) fn status_end_AttackLw4Start(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_AttackLw4HoldEv"]
    pub(super) fn status_pre_AttackLw4Hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_AttackLw4Hold_CommonEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_pre_AttackLw4Hold_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_AttackLw4Hold_commonEv"]
    pub(super) fn status_AttackLw4Hold_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_AttackLw4HoldEv"]
    pub(super) fn status_AttackLw4Hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_AttackLw4Hold_mainEv"]
    pub(super) fn status_AttackLw4Hold_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_AttackLw4HoldEv"]
    pub(super) fn status_end_AttackLw4Hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_AttackLw4Ev"]
    pub(super) fn status_pre_AttackLw4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_AttackLw4_commonEv"]
    pub(super) fn status_AttackLw4_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24attack_lw4_mtrans_commonEN3lib8L2CValueE"]
    pub(super) fn attack_lw4_mtrans_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17attack_lw4_mtransEv"]
    pub(super) fn attack_lw4_mtrans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_AttackLw4Ev"]
    pub(super) fn status_AttackLw4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_AttackLw4_Main_paramEN3lib8L2CValueE"]
    pub(super) fn status_AttackLw4_Main_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_AttackLw4_MainEv"]
    pub(super) fn status_AttackLw4_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_AttackLw4Ev"]
    pub(super) fn status_end_AttackLw4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_AttackAirEv"]
    pub(super) fn status_pre_AttackAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_attack_air_kindEv"]
    pub(super) fn sub_attack_air_kind(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_attack_air_kind_set_log_infoEv"]
    pub(super) fn sub_attack_air_kind_set_log_info(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_attack_air_commonEN3lib8L2CValueE"]
    pub(super) fn sub_attack_air_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15attack_air_uniqEN3lib8L2CValueE"]
    pub(super) fn attack_air_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14sub_attack_airEv"]
    pub(super) fn sub_attack_air(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_AttackAirEv"]
    pub(super) fn status_AttackAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_AttackAir_MainEv"]
    pub(super) fn status_AttackAir_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_AttackAir_Main_commonEv"]
    pub(super) fn status_AttackAir_Main_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24attack_air_common_stransEv"]
    pub(super) fn attack_air_common_strans(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_attack_air_uniq_process_exec_fix_posEv"]
    pub(super) fn sub_attack_air_uniq_process_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_AttackAirEv"]
    pub(super) fn status_end_AttackAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_attack_air_uniq_process_initEv"]
    pub(super) fn sub_attack_air_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_attack_air_uniq_process_execEv"]
    pub(super) fn sub_attack_air_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_attack_air_uniq_process_exitEv"]
    pub(super) fn sub_attack_air_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon59sub_attack_air_inherit_jump_aerial_motion_uniq_process_initEv"]
    pub(super) fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon59sub_attack_air_inherit_jump_aerial_motion_uniq_process_execEv"]
    pub(super) fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon67sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_posEv"]
    pub(super) fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon59sub_attack_air_inherit_jump_aerial_motion_uniq_process_exitEv"]
    pub(super) fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_ReboundStopEv"]
    pub(super) fn status_pre_ReboundStop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_ReboundStop_commonEv"]
    pub(super) fn status_ReboundStop_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_ReboundStopEv"]
    pub(super) fn status_ReboundStop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_ReboundStop_MainEv"]
    pub(super) fn status_ReboundStop_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_ReboundStopEv"]
    pub(super) fn status_end_ReboundStop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_ReboundEv"]
    pub(super) fn status_pre_Rebound(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_ReboundEv"]
    pub(super) fn status_Rebound(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_Rebound_MainEv"]
    pub(super) fn status_Rebound_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_ReboundEv"]
    pub(super) fn status_end_Rebound(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_ReboundJumpEv"]
    pub(super) fn status_pre_ReboundJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_Jump_Common_paramEN3lib8L2CValueE"]
    pub(super) fn status_pre_Jump_Common_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_Jump_CommonEv"]
    pub(super) fn status_pre_Jump_Common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_ReboundJumpEv"]
    pub(super) fn status_ReboundJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17rebound_jump_uniqEN3lib8L2CValueE"]
    pub(super) fn rebound_jump_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_ReboundJump_MainEv"]
    pub(super) fn status_ReboundJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_ReboundJumpEv"]
    pub(super) fn status_end_ReboundJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_BittenWarioEv"]
    pub(super) fn status_pre_BittenWario(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_BittenWarioEv"]
    pub(super) fn status_BittenWario(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_status_BittenWario_uniqEv"]
    pub(super) fn sub_status_BittenWario_uniq(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_BittenWario_MainEv"]
    pub(super) fn status_BittenWario_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24check_common_BittenWarioEv"]
    pub(super) fn check_common_BittenWario(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessBittenWario_exec_fix_pos_counterEv"]
    pub(super) fn FighterStatusUniqProcessBittenWario_exec_fix_pos_counter(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessBittenWario_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessBittenWario_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessBittenWario_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessBittenWario_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessBittenWario_init_status_subEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessBittenWario_init_status_sub(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessBittenWario_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessBittenWario_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessBittenWario_exec_fix_pos_slowEv"]
    pub(super) fn FighterStatusUniqProcessBittenWario_exec_fix_pos_slow(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessBittenWario_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessBittenWario_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41local_func__fighter_status_bitten_wario_1Ev"]
    pub(super) fn local_func__fighter_status_bitten_wario_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusCapture_set_invalid_captureEv"]
    pub(super) fn FighterStatusCapture_set_invalid_capture(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessBittenWario_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessBittenWario_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_BittenWarioEndEv"]
    pub(super) fn status_pre_BittenWarioEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_BittenWarioEndEv"]
    pub(super) fn status_BittenWarioEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_BittenWarioEnd_MainEv"]
    pub(super) fn status_BittenWarioEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon59FighterStatusUniqProcessBittenWarioEnd_exec_fix_pos_counterEv"]
    pub(super) fn FighterStatusUniqProcessBittenWarioEnd_exec_fix_pos_counter(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessBittenWarioEnd_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessBittenWarioEnd_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessBittenWarioEnd_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessBittenWarioEnd_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessBittenWarioEnd_init_status_subEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessBittenWarioEnd_init_status_sub(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessBittenWarioEnd_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessBittenWarioEnd_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45local_func__fighter_status_bitten_wario_end_1Ev"]
    pub(super) fn local_func__fighter_status_bitten_wario_end_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessBittenWarioEnd_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessBittenWarioEnd_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_BittenWarioStartEv"]
    pub(super) fn status_pre_BittenWarioStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_BittenWarioStartEv"]
    pub(super) fn status_BittenWarioStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_BittenWarioStart_MainEv"]
    pub(super) fn status_BittenWarioStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon61FighterStatusUniqProcessBittenWarioStart_exec_fix_pos_counterEv"]
    pub(super) fn FighterStatusUniqProcessBittenWarioStart_exec_fix_pos_counter(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessBittenWarioStart_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessBittenWarioStart_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessBittenWarioStart_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessBittenWarioStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessBittenWarioStart_init_status_subEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessBittenWarioStart_init_status_sub(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessBittenWarioStart_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessBittenWarioStart_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47local_func__fighter_status_bitten_wario_start_1Ev"]
    pub(super) fn local_func__fighter_status_bitten_wario_start_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessBittenWarioStart_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessBittenWarioStart_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_BossEntryEv"]
    pub(super) fn status_pre_BossEntry(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_BossEntryEv"]
    pub(super) fn status_BossEntry(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_BossEntry_MainEv"]
    pub(super) fn status_BossEntry_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_BossEntryEv"]
    pub(super) fn status_end_BossEntry(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30is_turn_motion_bossdeadsubfuncEv"]
    pub(super) fn is_turn_motion_bossdeadsubfunc(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31is_turned_frame_bossdeadsubfuncEv"]
    pub(super) fn is_turned_frame_bossdeadsubfunc(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26reverse_lr_bossdeadsubfuncEv"]
    pub(super) fn reverse_lr_bossdeadsubfunc(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_BossDeadEv"]
    pub(super) fn status_pre_BossDead(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_BossDeadEv"]
    pub(super) fn status_BossDead(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_BossDead_MainEv"]
    pub(super) fn status_BossDead_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_BossDeadEv"]
    pub(super) fn status_end_BossDead(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15sub_exit_commonEv"]
    pub(super) fn sub_exit_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_update_bury_shakeEv"]
    pub(super) fn sub_update_bury_shake(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_update_bury_effectEN3lib8L2CValueE"]
    pub(super) fn sub_update_bury_effect(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12BuryRecoveryEv"]
    pub(super) fn BuryRecovery(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17check_damage_buryEN3lib8L2CValueE"]
    pub(super) fn check_damage_bury(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_BuryEv"]
    pub(super) fn status_pre_Bury(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13sub_bury_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_bury_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_BuryEv"]
    pub(super) fn status_Bury(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Bury_MainEv"]
    pub(super) fn status_Bury_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_BuryEv"]
    pub(super) fn status_end_Bury(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29uniq_process_Bury_init_statusEv"]
    pub(super) fn uniq_process_Bury_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33local_func__fighter_status_bury_1Ev"]
    pub(super) fn local_func__fighter_status_bury_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusUniqProcessDamage_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessDamage_init_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessDamage_check_hit_stop_delayEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessDamage_check_hit_stop_delay(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36local_func__fighter_status_damage_16Ev"]
    pub(super) fn local_func__fighter_status_damage_16(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45FighterStatusUniqProcessDamage_hit_stop_delayEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn FighterStatusUniqProcessDamage_hit_stop_delay(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36local_func__fighter_status_damage_14Ev"]
    pub(super) fn local_func__fighter_status_damage_14(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36local_func__fighter_status_damage_15Ev"]
    pub(super) fn local_func__fighter_status_damage_15(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29uniq_process_Bury_exec_statusEv"]
    pub(super) fn uniq_process_Bury_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38uniq_process_Bury_exec_fix_pos_counterEv"]
    pub(super) fn uniq_process_Bury_exec_fix_pos_counter(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30uniq_process_Bury_exec_fix_posEv"]
    pub(super) fn uniq_process_Bury_exec_fix_pos(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27uniq_process_Bury_exec_stopEv"]
    pub(super) fn uniq_process_Bury_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29uniq_process_Bury_exit_statusEv"]
    pub(super) fn uniq_process_Bury_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_damage_uniq_process_exitEv"]
    pub(super) fn sub_damage_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_BuryWaitEv"]
    pub(super) fn status_pre_BuryWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_BuryWaitEv"]
    pub(super) fn status_BuryWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_BuryWait_MainEv"]
    pub(super) fn status_BuryWait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26uniq_process_BuryWait_mainEv"]
    pub(super) fn uniq_process_BuryWait_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42uniq_process_BuryWait_exec_fix_pos_counterEv"]
    pub(super) fn uniq_process_BuryWait_exec_fix_pos_counter(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_BuryWaitEv"]
    pub(super) fn status_end_BuryWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33uniq_process_BuryWait_init_statusEv"]
    pub(super) fn uniq_process_BuryWait_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34uniq_process_BuryWait_exec_fix_posEv"]
    pub(super) fn uniq_process_BuryWait_exec_fix_pos(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33uniq_process_BuryWait_exec_statusEv"]
    pub(super) fn uniq_process_BuryWait_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31uniq_process_BuryWait_exec_stopEv"]
    pub(super) fn uniq_process_BuryWait_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33uniq_process_BuryWait_exit_statusEv"]
    pub(super) fn uniq_process_BuryWait_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16sub_BuryJumpUniqEN3lib8L2CValueE"]
    pub(super) fn sub_BuryJumpUniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_BuryJumpEv"]
    pub(super) fn status_pre_BuryJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_BuryJumpEv"]
    pub(super) fn status_BuryJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_BuryJump_MainEv"]
    pub(super) fn status_BuryJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26uniq_process_BuryJump_mainEv"]
    pub(super) fn uniq_process_BuryJump_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_BuryJumpEv"]
    pub(super) fn status_end_BuryJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33uniq_process_BuryJump_init_statusEv"]
    pub(super) fn uniq_process_BuryJump_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42uniq_process_BuryJump_exec_fix_pos_counterEv"]
    pub(super) fn uniq_process_BuryJump_exec_fix_pos_counter(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34uniq_process_BuryJump_exec_fix_posEv"]
    pub(super) fn uniq_process_BuryJump_exec_fix_pos(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33uniq_process_BuryJump_exec_statusEv"]
    pub(super) fn uniq_process_BuryJump_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33uniq_process_BuryJump_exit_statusEv"]
    pub(super) fn uniq_process_BuryJump_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44FighterStatusUniqProcessCapture_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCapture_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43FighterStatusUniqProcessCapture_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessCapture_exit_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41FighterStatusUniqProcessCapture_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessCapture_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_CapturePulledEv"]
    pub(super) fn status_CapturePulled(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_capture_pulled_uniq_process_execEv"]
    pub(super) fn sub_capture_pulled_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_capture_pulled_uniq_process_exec_commonEN3lib8L2CValueE"]
    pub(super) fn sub_capture_pulled_uniq_process_exec_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32process_fix_camera_CapturePulledEv"]
    pub(super) fn process_fix_camera_CapturePulled(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_capture_pulled_uniq_process_exitEv"]
    pub(super) fn sub_capture_pulled_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_capture_pulled_uniq_process_exit_coreEN3lib8L2CValueE"]
    pub(super) fn sub_capture_pulled_uniq_process_exit_core(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_CaptureWaitEv"]
    pub(super) fn status_pre_CaptureWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_capture_wait_uniq_process_initEv"]
    pub(super) fn sub_capture_wait_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_capture_wait_uniq_process_init_coreEN3lib8L2CValueE"]
    pub(super) fn sub_capture_wait_uniq_process_init_core(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_CaptureWaitEv"]
    pub(super) fn status_CaptureWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CaptureWait_LoopEv"]
    pub(super) fn status_CaptureWait_Loop(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CaptureWait_MainEv"]
    pub(super) fn status_CaptureWait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17CaptureWaitCommonEv"]
    pub(super) fn CaptureWaitCommon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_capture_wait_uniq_process_fix_posEv"]
    pub(super) fn sub_capture_wait_uniq_process_fix_pos(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_capture_wait_uniq_process_execEv"]
    pub(super) fn sub_capture_wait_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30process_fix_camera_CaptureWaitEv"]
    pub(super) fn process_fix_camera_CaptureWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_CaptureWaitEv"]
    pub(super) fn status_end_CaptureWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_CaptureDamageEv"]
    pub(super) fn status_pre_CaptureDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_capture_damage_uniq_process_initEv"]
    pub(super) fn sub_capture_damage_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_CaptureDamageEv"]
    pub(super) fn status_CaptureDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24CaptureDamageCommon_LoopEv"]
    pub(super) fn CaptureDamageCommon_Loop(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CaptureDamage_MainEv"]
    pub(super) fn status_CaptureDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19CaptureDamageCommonEv"]
    pub(super) fn CaptureDamageCommon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_capture_damage_uniq_process_execEv"]
    pub(super) fn sub_capture_damage_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32process_fix_camera_CaptureDamageEv"]
    pub(super) fn process_fix_camera_CaptureDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_CaptureDamageEv"]
    pub(super) fn status_end_CaptureDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_CaptureCutEv"]
    pub(super) fn status_pre_CaptureCut(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_capture_cut_uniq_process_initEv"]
    pub(super) fn sub_capture_cut_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_CaptureCutEv"]
    pub(super) fn status_CaptureCut(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_CaptureCut_MainEv"]
    pub(super) fn status_CaptureCut_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_CaptureCutEv"]
    pub(super) fn status_end_CaptureCut(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_CaptureJumpEv"]
    pub(super) fn status_pre_CaptureJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_CaptureJumpEv"]
    pub(super) fn status_CaptureJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CaptureJump_MainEv"]
    pub(super) fn status_CaptureJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_CaptureJumpEv"]
    pub(super) fn status_end_CaptureJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_pre_ThrownEv"]
    pub(super) fn status_pre_Thrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_thrown_uniq_process_initEv"]
    pub(super) fn sub_thrown_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15sub_Thrown_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_Thrown_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_Thrown_commonEv"]
    pub(super) fn status_Thrown_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13status_ThrownEv"]
    pub(super) fn status_Thrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_Thrown_MainEv"]
    pub(super) fn status_Thrown_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusUniqProcessThrown_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessThrown_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusUniqProcessThrown_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessThrown_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_thrown_uniq_process_exitEv"]
    pub(super) fn sub_thrown_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusUniqProcessThrown_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessThrown_exit_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_end_ThrownEv"]
    pub(super) fn status_end_Thrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43FighterStatusUniqProcessThrown_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessThrown_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_CaptureBlackholeEv"]
    pub(super) fn status_pre_CaptureBlackhole(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CaptureBlackholeEv"]
    pub(super) fn status_CaptureBlackhole(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_CaptureBlackhole_MainEv"]
    pub(super) fn status_CaptureBlackhole_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_capture_blackhole_scale_animationEv"]
    pub(super) fn sub_capture_blackhole_scale_animation(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessCaptureBlackhole_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessCaptureBlackhole_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_CaptureBlackholeEv"]
    pub(super) fn status_end_CaptureBlackhole(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessCaptureBlackhole_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureBlackhole_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46local_func__fighter_status_capture_blackhole_1Ev"]
    pub(super) fn local_func__fighter_status_capture_blackhole_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessCaptureBlackhole_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureBlackhole_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46local_func__fighter_status_capture_blackhole_2Ev"]
    pub(super) fn local_func__fighter_status_capture_blackhole_2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46local_func__fighter_status_capture_blackhole_3Ev"]
    pub(super) fn local_func__fighter_status_capture_blackhole_3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46local_func__fighter_status_capture_blackhole_4Ev"]
    pub(super) fn local_func__fighter_status_capture_blackhole_4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46local_func__fighter_status_capture_blackhole_5Ev"]
    pub(super) fn local_func__fighter_status_capture_blackhole_5(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessCaptureBlackhole_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureBlackhole_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46local_func__fighter_status_capture_blackhole_6Ev"]
    pub(super) fn local_func__fighter_status_capture_blackhole_6(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessCaptureBlackhole_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureBlackhole_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_CaptureMastercoreEv"]
    pub(super) fn status_pre_CaptureMastercore(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_CaptureMastercoreEv"]
    pub(super) fn status_CaptureMastercore(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_CaptureMastercore_MainEv"]
    pub(super) fn status_CaptureMastercore_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_CaptureMastercoreEv"]
    pub(super) fn status_end_CaptureMastercore(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessCaptureMastercore_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureMastercore_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_CaptureMasterhandEv"]
    pub(super) fn status_pre_CaptureMasterhand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_CaptureMasterhandEv"]
    pub(super) fn status_CaptureMasterhand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_CaptureMasterhand_MainEv"]
    pub(super) fn status_CaptureMasterhand_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessCaptureMasterhand_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessCaptureMasterhand_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_CaptureMasterhandEv"]
    pub(super) fn status_end_CaptureMasterhand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessCaptureMasterhand_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureMasterhand_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessCaptureMasterhand_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureMasterhand_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessCaptureMasterhand_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureMasterhand_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_CaptureNabbitEv"]
    pub(super) fn status_pre_CaptureNabbit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_CaptureNabbitEv"]
    pub(super) fn status_CaptureNabbit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CaptureNabbit_MainEv"]
    pub(super) fn status_CaptureNabbit_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessCaptureNabbit_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessCaptureNabbit_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_CaptureNabbitEv"]
    pub(super) fn status_end_CaptureNabbit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessCaptureNabbit_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureNabbit_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessCaptureNabbit_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureNabbit_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessCaptureNabbit_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureNabbit_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_CapturePulledFishingrodEv"]
    pub(super) fn status_pre_CapturePulledFishingrod(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_CapturePulledFishingrodEv"]
    pub(super) fn status_CapturePulledFishingrod(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_CapturePulledFishingrod_MainEv"]
    pub(super) fn status_CapturePulledFishingrod_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_CapturePulledFishingrodEv"]
    pub(super) fn status_end_CapturePulledFishingrod(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_capture_octopus_initEv"]
    pub(super) fn sub_capture_octopus_init(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_capture_octopus_execEv"]
    pub(super) fn sub_capture_octopus_exec(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_capture_octopus_exitEv"]
    pub(super) fn sub_capture_octopus_exit(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44sub_capture_pulled_octopus_uniq_process_initEv"]
    pub(super) fn sub_capture_pulled_octopus_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44local_func__fighter_status_capture_octopus_1EN3lib8L2CValueE"]
    pub(super) fn local_func__fighter_status_capture_octopus_1(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44sub_capture_pulled_octopus_uniq_process_execEv"]
    pub(super) fn sub_capture_pulled_octopus_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44sub_capture_pulled_octopus_uniq_process_exitEv"]
    pub(super) fn sub_capture_pulled_octopus_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_CapturePulledOctopusEv"]
    pub(super) fn status_pre_CapturePulledOctopus(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_CapturePulledOctopusEv"]
    pub(super) fn status_CapturePulledOctopus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_CapturePulledOctopus_MainEv"]
    pub(super) fn status_CapturePulledOctopus_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_CapturePulledOctopusEv"]
    pub(super) fn status_end_CapturePulledOctopus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_capture_wait_octopus_uniq_process_initEv"]
    pub(super) fn sub_capture_wait_octopus_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_capture_wait_octopus_uniq_process_execEv"]
    pub(super) fn sub_capture_wait_octopus_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_capture_wait_octopus_uniq_process_exitEv"]
    pub(super) fn sub_capture_wait_octopus_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_CaptureWaitOctopusEv"]
    pub(super) fn status_pre_CaptureWaitOctopus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CaptureWaitOctopusEv"]
    pub(super) fn status_CaptureWaitOctopus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_CaptureWaitOctopus_MainEv"]
    pub(super) fn status_CaptureWaitOctopus_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_CaptureWaitOctopusEv"]
    pub(super) fn status_end_CaptureWaitOctopus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_CaptureYoshiEv"]
    pub(super) fn status_pre_CaptureYoshi(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_CaptureYoshiEv"]
    pub(super) fn status_CaptureYoshi(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_CaptureYoshi_mtransEv"]
    pub(super) fn status_CaptureYoshi_mtrans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_CaptureYoshi_MainEv"]
    pub(super) fn status_CaptureYoshi_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_CaptureYoshiEv"]
    pub(super) fn status_end_CaptureYoshi(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessCaptureYoshi_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureYoshi_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessCaptureYoshi_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureYoshi_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42local_func__fighter_status_capture_yoshi_1Ev"]
    pub(super) fn local_func__fighter_status_capture_yoshi_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42local_func__fighter_status_capture_yoshi_2Ev"]
    pub(super) fn local_func__fighter_status_capture_yoshi_2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_capture_yoshi_execEv"]
    pub(super) fn sub_uniq_process_capture_yoshi_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessCaptureYoshi_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureYoshi_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_YoshiEggEv"]
    pub(super) fn status_pre_YoshiEgg(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_YoshiEggEv"]
    pub(super) fn status_YoshiEgg(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_YoshiEgg_MainEv"]
    pub(super) fn status_YoshiEgg_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessYoshiEgg_exec_fix_pos_counterEv"]
    pub(super) fn FighterStatusUniqProcessYoshiEgg_exec_fix_pos_counter(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45FighterStatusUniqProcessYoshiEgg_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessYoshiEgg_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_YoshiEggEv"]
    pub(super) fn status_end_YoshiEgg(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44FighterStatusUniqProcessYoshiEgg_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessYoshiEgg_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44FighterStatusUniqProcessYoshiEgg_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessYoshiEgg_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusUniqProcessYoshiEgg_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessYoshiEgg_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44FighterStatusUniqProcessYoshiEgg_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessYoshiEgg_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45FighterStatusUniqProcessYoshiEgg_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessYoshiEgg_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_CapturePulledYoshiEv"]
    pub(super) fn status_pre_CapturePulledYoshi(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CapturePulledYoshiEv"]
    pub(super) fn status_CapturePulledYoshi(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_CapturePulledYoshi_MainEv"]
    pub(super) fn status_CapturePulledYoshi_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_CapturePulledYoshiEv"]
    pub(super) fn status_end_CapturePulledYoshi(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessCapturePulledYoshi_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessCapturePulledYoshi_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessCapturePulledYoshi_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessCapturePulledYoshi_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42local_func__fighter_status_capture_yoshi_3Ev"]
    pub(super) fn local_func__fighter_status_capture_yoshi_3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_CaptureWaitYoshiEv"]
    pub(super) fn status_pre_CaptureWaitYoshi(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CaptureWaitYoshiEv"]
    pub(super) fn status_CaptureWaitYoshi(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_CaptureWaitYoshi_MainEv"]
    pub(super) fn status_CaptureWaitYoshi_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessCaptureWaitYoshi_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessCaptureWaitYoshi_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_CaptureWaitYoshiEv"]
    pub(super) fn status_end_CaptureWaitYoshi(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessCaptureWaitYoshi_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureWaitYoshi_init_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessCaptureWaitYoshi_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureWaitYoshi_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42local_func__fighter_status_capture_yoshi_4Ev"]
    pub(super) fn local_func__fighter_status_capture_yoshi_4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessCaptureWaitYoshi_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureWaitYoshi_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_CaptureDamageYoshiEv"]
    pub(super) fn status_pre_CaptureDamageYoshi(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CaptureDamageYoshiEv"]
    pub(super) fn status_CaptureDamageYoshi(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_CaptureDamageYoshi_MainEv"]
    pub(super) fn status_CaptureDamageYoshi_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessCaptureDamageYoshi_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessCaptureDamageYoshi_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_CaptureDamageYoshiEv"]
    pub(super) fn status_end_CaptureDamageYoshi(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessCaptureDamageYoshi_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureDamageYoshi_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessCaptureDamageYoshi_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureDamageYoshi_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42local_func__fighter_status_capture_yoshi_5Ev"]
    pub(super) fn local_func__fighter_status_capture_yoshi_5(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessCaptureDamageYoshi_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureDamageYoshi_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_pre_CatchEv"]
    pub(super) fn status_pre_Catch(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_Catch_commonEN3lib8L2CValueE"]
    pub(super) fn status_pre_Catch_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16sub_status_CatchEv"]
    pub(super) fn sub_status_Catch(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12status_CatchEv"]
    pub(super) fn status_Catch(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_Catch_MainEv"]
    pub(super) fn status_Catch_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_end_CatchEv"]
    pub(super) fn status_end_Catch(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_CatchPullEv"]
    pub(super) fn status_pre_CatchPull(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_CatchPull_commonEN3lib8L2CValueE"]
    pub(super) fn status_pre_CatchPull_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_catch_pull_uniq_process_initEv"]
    pub(super) fn sub_catch_pull_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CatchPull_commonEN3lib8L2CValueE"]
    pub(super) fn status_CatchPull_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_CatchPullEv"]
    pub(super) fn status_CatchPull(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_CatchPull_MainEv"]
    pub(super) fn status_CatchPull_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon9CatchContEv"]
    pub(super) fn CatchCont(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12IsThrowStickEv"]
    pub(super) fn IsThrowStick(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34local_func__fighter_status_catch_1Ev"]
    pub(super) fn local_func__fighter_status_catch_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_is_not_throw_status_kindEN3lib8L2CValueE"]
    pub(super) fn sub_is_not_throw_status_kind(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_catch_pull_uniq_process_exitEv"]
    pub(super) fn sub_catch_pull_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_is_throw_status_kindEN3lib8L2CValueE"]
    pub(super) fn sub_is_throw_status_kind(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_CatchPullEv"]
    pub(super) fn status_end_CatchPull(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessCatchPull_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCatchPull_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_CatchDashEv"]
    pub(super) fn status_pre_CatchDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_CatchDash_commonEN3lib8L2CValueE"]
    pub(super) fn status_pre_CatchDash_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_status_CatchDashEv"]
    pub(super) fn sub_status_CatchDash(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13CatchDashUniqEN3lib8L2CValueE"]
    pub(super) fn CatchDashUniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_CatchDashEv"]
    pub(super) fn status_CatchDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_CatchDash_MainEv"]
    pub(super) fn status_CatchDash_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_CatchDashEv"]
    pub(super) fn status_end_CatchDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_CatchDashPullEv"]
    pub(super) fn status_pre_CatchDashPull(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_CatchDashPull_commonEN3lib8L2CValueE"]
    pub(super) fn status_CatchDashPull_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_CatchDashPullEv"]
    pub(super) fn status_CatchDashPull(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CatchDashPull_MainEv"]
    pub(super) fn status_CatchDashPull_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_CatchDashPullEv"]
    pub(super) fn status_end_CatchDashPull(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_CatchTurnEv"]
    pub(super) fn status_pre_CatchTurn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_CatchTurn_commonEN3lib8L2CValueE"]
    pub(super) fn status_pre_CatchTurn_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_status_CatchTurnEv"]
    pub(super) fn sub_status_CatchTurn(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_CatchTurnEv"]
    pub(super) fn status_CatchTurn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_CatchTurn_MainEv"]
    pub(super) fn status_CatchTurn_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_CatchTurnEv"]
    pub(super) fn status_end_CatchTurn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_CatchWaitEv"]
    pub(super) fn status_pre_CatchWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_catch_wait_uniq_process_initEv"]
    pub(super) fn sub_catch_wait_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CatchWait_commonEN3lib8L2CValueE"]
    pub(super) fn status_CatchWait_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_CatchWait_MainEv"]
    pub(super) fn status_CatchWait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_CatchWaitEv"]
    pub(super) fn status_CatchWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_catch_wait_uniq_process_exitEv"]
    pub(super) fn sub_catch_wait_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_CatchWaitEv"]
    pub(super) fn status_end_CatchWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_CatchAttackEv"]
    pub(super) fn status_pre_CatchAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CatchAttack_commonEN3lib8L2CValueE"]
    pub(super) fn status_CatchAttack_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CatchAttack_MainEv"]
    pub(super) fn status_CatchAttack_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_CatchAttackEv"]
    pub(super) fn status_CatchAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_CatchAttackEv"]
    pub(super) fn status_end_CatchAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_CatchCutEv"]
    pub(super) fn status_pre_CatchCut(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_catch_cut_uniq_process_initEv"]
    pub(super) fn sub_catch_cut_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_CatchCutEv"]
    pub(super) fn status_CatchCut(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_CatchCut_MainEv"]
    pub(super) fn status_CatchCut_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_CatchCutEv"]
    pub(super) fn status_end_CatchCut(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_CatchJumpEv"]
    pub(super) fn status_pre_CatchJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_CatchJumpEv"]
    pub(super) fn status_CatchJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_CatchJump_MainEv"]
    pub(super) fn status_CatchJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_CatchJumpEv"]
    pub(super) fn status_end_CatchJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_pre_ThrowEv"]
    pub(super) fn status_pre_Throw(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Throw_SubEv"]
    pub(super) fn status_Throw_Sub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14ThrowLogCommonEv"]
    pub(super) fn ThrowLogCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon9ThrowUniqEv"]
    pub(super) fn ThrowUniq(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12status_ThrowEv"]
    pub(super) fn status_Throw(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_Throw_MainEv"]
    pub(super) fn status_Throw_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_end_ThrowEv"]
    pub(super) fn status_end_Throw(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_throw_uniq_process_initEv"]
    pub(super) fn sub_throw_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_throw_uniq_process_exitEv"]
    pub(super) fn sub_throw_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusUniqProcessThrow_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessThrow_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_CatchedGanonEv"]
    pub(super) fn status_pre_CatchedGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_CatchedGanonEv"]
    pub(super) fn status_CatchedGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_CatchedGanon_MainEv"]
    pub(super) fn status_CatchedGanon_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_CatchedGanonEv"]
    pub(super) fn status_end_CatchedGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_uniq_process_CatchedGanon_initEv"]
    pub(super) fn sub_uniq_process_CatchedGanon_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_uniq_process_CatchedGanon_exitEv"]
    pub(super) fn sub_uniq_process_CatchedGanon_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessCatchedGanon_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCatchedGanon_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_CatchedAirGanonEv"]
    pub(super) fn status_pre_CatchedAirGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_CatchedAirGanonEv"]
    pub(super) fn status_CatchedAirGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_CatchedAirGanon_MainEv"]
    pub(super) fn status_CatchedAirGanon_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_CatchedAirGanonEv"]
    pub(super) fn status_end_CatchedAirGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_uniq_process_CatchedAirGanon_initEv"]
    pub(super) fn sub_uniq_process_CatchedAirGanon_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_uniq_process_CatchedAirGanon_exitEv"]
    pub(super) fn sub_uniq_process_CatchedAirGanon_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessCatchedAirGanon_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCatchedAirGanon_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_CatchedAirFallGanonEv"]
    pub(super) fn status_pre_CatchedAirFallGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_CatchedAirFallGanonEv"]
    pub(super) fn status_CatchedAirFallGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_CatchedAirFallGanon_MainEv"]
    pub(super) fn status_CatchedAirFallGanon_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_CatchedAirFallGanonEv"]
    pub(super) fn status_end_CatchedAirFallGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_uniq_process_CatchedAirFallGanon_initEv"]
    pub(super) fn sub_uniq_process_CatchedAirFallGanon_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_uniq_process_CatchedAirFallGanon_exitEv"]
    pub(super) fn sub_uniq_process_CatchedAirFallGanon_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessCatchedAirFallGanon_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCatchedAirFallGanon_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_CatchedAirEndGanonEv"]
    pub(super) fn status_pre_CatchedAirEndGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CatchedAirEndGanonEv"]
    pub(super) fn status_CatchedAirEndGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_CatchedAirEndGanon_MainEv"]
    pub(super) fn status_CatchedAirEndGanon_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_CatchedAirEndGanonEv"]
    pub(super) fn status_end_CatchedAirEndGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_uniq_process_CatchedAirEndGanon_initEv"]
    pub(super) fn sub_uniq_process_CatchedAirEndGanon_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_uniq_process_CatchedAirEndGanon_exitEv"]
    pub(super) fn sub_uniq_process_CatchedAirEndGanon_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessCatchedAirEndGanon_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCatchedAirEndGanon_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_CatchedCutGanonEv"]
    pub(super) fn status_pre_CatchedCutGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_CatchedCutGanonEv"]
    pub(super) fn status_CatchedCutGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_CatchedCutGanon_MainEv"]
    pub(super) fn status_CatchedCutGanon_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_CatchedCutGanonEv"]
    pub(super) fn status_end_CatchedCutGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_CatchedRefletEv"]
    pub(super) fn status_pre_CatchedReflet(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_CatchedRefletEv"]
    pub(super) fn status_CatchedReflet(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CatchedReflet_MainEv"]
    pub(super) fn status_CatchedReflet_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32process_fix_camera_CatchedRefletEv"]
    pub(super) fn process_fix_camera_CatchedReflet(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_CatchedRefletEv"]
    pub(super) fn status_end_CatchedReflet(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessCatchedReflet_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCatchedReflet_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_CatchedRidleyEv"]
    pub(super) fn status_pre_CatchedRidley(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_CatchedRidleyEv"]
    pub(super) fn status_CatchedRidley(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CatchedRidley_MainEv"]
    pub(super) fn status_CatchedRidley_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_CatchedRidleyEv"]
    pub(super) fn status_end_CatchedRidley(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_CatchedRidley_initEv"]
    pub(super) fn sub_uniq_process_CatchedRidley_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_CatchedRidley_execEv"]
    pub(super) fn sub_uniq_process_CatchedRidley_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46sub_uniq_process_CatchedRidley_exec_fix_cameraEv"]
    pub(super) fn sub_uniq_process_CatchedRidley_exec_fix_camera(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_CatchedRidley_exitEv"]
    pub(super) fn sub_uniq_process_CatchedRidley_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessCatchedRidley_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCatchedRidley_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_DraggedRidleyEv"]
    pub(super) fn status_pre_DraggedRidley(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_DraggedRidleyEv"]
    pub(super) fn status_DraggedRidley(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_DraggedRidley_MainEv"]
    pub(super) fn status_DraggedRidley_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_DraggedRidleyEv"]
    pub(super) fn status_end_DraggedRidley(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_DraggedRidley_initEv"]
    pub(super) fn sub_uniq_process_DraggedRidley_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_DraggedRidley_execEv"]
    pub(super) fn sub_uniq_process_DraggedRidley_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_DraggedRidley_exitEv"]
    pub(super) fn sub_uniq_process_DraggedRidley_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19set_cliff_xlu_frameEN3lib8L2CValueE"]
    pub(super) fn set_cliff_xlu_frame(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_CliffCatchMoveEv"]
    pub(super) fn status_pre_CliffCatchMove(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_CliffCatchMoveEv"]
    pub(super) fn status_CliffCatchMove(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_CliffCatchMove_MainEv"]
    pub(super) fn status_CliffCatchMove_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_CliffCatchMoveEv"]
    pub(super) fn status_end_CliffCatchMove(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_cliff_catch_move_uniq_process_initEv"]
    pub(super) fn sub_cliff_catch_move_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_cliff_catch_move_uniq_process_init_commonEN3lib8L2CValueES2_"]
    pub(super) fn sub_cliff_catch_move_uniq_process_init_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_cliff_catch_move_uniq_process_exec_statusEv"]
    pub(super) fn sub_cliff_catch_move_uniq_process_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_cliff_catch_move_uniq_process_exitEv"]
    pub(super) fn sub_cliff_catch_move_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_is_leave_cliffEN3lib8L2CValueE"]
    pub(super) fn sub_is_leave_cliff(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_CliffCatchEv"]
    pub(super) fn status_pre_CliffCatch(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_status_CliffCatchCommonEv"]
    pub(super) fn sub_status_CliffCatchCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_CliffCatchEv"]
    pub(super) fn status_CliffCatch(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_status_CliffCatch_MainCommonEv"]
    pub(super) fn sub_status_CliffCatch_MainCommon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_cliff_uniq_process_mainEv"]
    pub(super) fn sub_cliff_uniq_process_main(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_cliff_uniq_process_exec_fix_posEv"]
    pub(super) fn sub_cliff_uniq_process_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_CliffCatchSubEv"]
    pub(super) fn status_CliffCatchSub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_CliffCatch_MainEv"]
    pub(super) fn status_CliffCatch_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_CliffCatchEv"]
    pub(super) fn status_end_CliffCatch(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_cliff_uniq_process_initEv"]
    pub(super) fn sub_cliff_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28get_cliff_wait_hit_xlu_frameEv"]
    pub(super) fn get_cliff_wait_hit_xlu_frame(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_cliff_common_inputEv"]
    pub(super) fn sub_cliff_common_input(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_cliff_uniq_process_exec_statusEv"]
    pub(super) fn sub_cliff_uniq_process_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_cliff_uniq_process_exit_CommonEN3lib8L2CValueE"]
    pub(super) fn sub_cliff_uniq_process_exit_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_cliff_uniq_process_exitEv"]
    pub(super) fn sub_cliff_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_cliff_wait_uniq_process_initEv"]
    pub(super) fn sub_cliff_wait_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_cliff_wait_uniq_process_init_commonEN3lib8L2CValueE"]
    pub(super) fn sub_cliff_wait_uniq_process_init_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_CliffWaitEv"]
    pub(super) fn status_pre_CliffWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23enable_cliff_wait_stansEv"]
    pub(super) fn enable_cliff_wait_stans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_cliff_wait_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_cliff_wait_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_CliffWait_SubEv"]
    pub(super) fn status_CliffWait_Sub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_CliffWaitEv"]
    pub(super) fn status_CliffWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_CliffWait_MainEv"]
    pub(super) fn status_CliffWait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_CliffWaitEv"]
    pub(super) fn status_end_CliffWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_CliffAttackEv"]
    pub(super) fn status_pre_CliffAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_CliffAttackEv"]
    pub(super) fn status_CliffAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CliffAttack_MainEv"]
    pub(super) fn status_CliffAttack_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_CliffAttackEv"]
    pub(super) fn status_end_CliffAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_CliffClimbEv"]
    pub(super) fn status_pre_CliffClimb(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_CliffClimbEv"]
    pub(super) fn status_CliffClimb(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_CliffClimb_MainEv"]
    pub(super) fn status_CliffClimb_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_CliffClimbEv"]
    pub(super) fn status_end_CliffClimb(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_CliffEscapeEv"]
    pub(super) fn status_pre_CliffEscape(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_CliffEscapeEv"]
    pub(super) fn status_CliffEscape(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CliffEscape_MainEv"]
    pub(super) fn status_CliffEscape_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_CliffEscapeEv"]
    pub(super) fn status_end_CliffEscape(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_CliffJump1Ev"]
    pub(super) fn status_pre_CliffJump1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_CliffJump1Ev"]
    pub(super) fn status_CliffJump1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_CliffJump1_MainEv"]
    pub(super) fn status_CliffJump1_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_CliffJump1Ev"]
    pub(super) fn status_end_CliffJump1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_CliffJump2Ev"]
    pub(super) fn status_pre_CliffJump2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_CliffJump2Ev"]
    pub(super) fn status_CliffJump2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_cliff_jump2_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_cliff_jump2_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_CliffJump2_MainEv"]
    pub(super) fn status_CliffJump2_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_air_check_stop_ceil_preEv"]
    pub(super) fn sub_air_check_stop_ceil_pre(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_CliffJump2Ev"]
    pub(super) fn status_end_CliffJump2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_CliffJump3Ev"]
    pub(super) fn status_pre_CliffJump3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_CliffJump3Ev"]
    pub(super) fn status_CliffJump3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_CliffJump3_MainEv"]
    pub(super) fn status_CliffJump3_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_CliffJump3Ev"]
    pub(super) fn status_end_CliffJump3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_CliffRobbedEv"]
    pub(super) fn status_pre_CliffRobbed(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_CliffRobbedEv"]
    pub(super) fn status_CliffRobbed(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_cliff_robbed_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_cliff_robbed_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CliffRobbed_MainEv"]
    pub(super) fn status_CliffRobbed_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_CliffRobbedEv"]
    pub(super) fn status_end_CliffRobbed(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_MissFootEv"]
    pub(super) fn status_pre_MissFoot(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_MissFootEv"]
    pub(super) fn status_MissFoot(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_MissFoot_MainEv"]
    pub(super) fn status_MissFoot_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17sub_AirChkPassiveEv"]
    pub(super) fn sub_AirChkPassive(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17is_enable_passiveEv"]
    pub(super) fn is_enable_passive(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_check_passive_buttonEN3lib8L2CValueE"]
    pub(super) fn sub_check_passive_button(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_MissFootPassiveEv"]
    pub(super) fn sub_MissFootPassive(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_MissFootEv"]
    pub(super) fn status_end_MissFoot(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_ClungCaptainEv"]
    pub(super) fn status_pre_ClungCaptain(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_ClungCaptainEv"]
    pub(super) fn status_ClungCaptain(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_ClungCaptain_MainEv"]
    pub(super) fn status_ClungCaptain_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_ClungCaptainEv"]
    pub(super) fn status_end_ClungCaptain(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_uniq_process_ClungCaptain_initEv"]
    pub(super) fn sub_uniq_process_ClungCaptain_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_uniq_process_ClungCaptain_exitEv"]
    pub(super) fn sub_uniq_process_ClungCaptain_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessClungCaptain_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessClungCaptain_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_ClungDiddyEv"]
    pub(super) fn status_pre_ClungDiddy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_ClungDiddyEv"]
    pub(super) fn status_ClungDiddy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_clung_diddy_mtransEv"]
    pub(super) fn sub_clung_diddy_mtrans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_ClungDiddy_MainEv"]
    pub(super) fn status_ClungDiddy_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessClungDiddy_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessClungDiddy_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_ClungDiddyEv"]
    pub(super) fn status_end_ClungDiddy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessClungDiddy_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessClungDiddy_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessClungDiddy_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessClungDiddy_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessClungDiddy_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessClungDiddy_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessClungDiddy_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessClungDiddy_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_ClungDamageDiddyEv"]
    pub(super) fn status_pre_ClungDamageDiddy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_ClungDamageDiddyEv"]
    pub(super) fn status_ClungDamageDiddy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_clung_damage_diddy_mtransEv"]
    pub(super) fn sub_clung_damage_diddy_mtrans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_ClungDamageDiddy_MainEv"]
    pub(super) fn status_ClungDamageDiddy_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessClungDamageDiddy_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessClungDamageDiddy_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_ClungDamageDiddyEv"]
    pub(super) fn status_end_ClungDamageDiddy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessClungDamageDiddy_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessClungDamageDiddy_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessClungDamageDiddy_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessClungDamageDiddy_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessClungDamageDiddy_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessClungDamageDiddy_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_ClungThrownDiddyEv"]
    pub(super) fn status_pre_ClungThrownDiddy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23clung_thrown_diddy_uniqEN3lib8L2CValueE"]
    pub(super) fn clung_thrown_diddy_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_ClungThrownDiddyEv"]
    pub(super) fn status_ClungThrownDiddy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_ClungThrownJumpDiddy_MainEv"]
    pub(super) fn status_ClungThrownJumpDiddy_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessClungThrownDiddy_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessClungThrownDiddy_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_ClungThrownDiddyEv"]
    pub(super) fn status_end_ClungThrownDiddy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessClungThrownDiddy_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessClungThrownDiddy_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessClungThrownDiddy_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessClungThrownDiddy_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_ClungThrownBlankDiddyEv"]
    pub(super) fn status_pre_ClungThrownBlankDiddy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_ClungThrownBlankDiddyEv"]
    pub(super) fn status_ClungThrownBlankDiddy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_ClungThrownBlankDiddy_MainEv"]
    pub(super) fn status_ClungThrownBlankDiddy_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_ClungThrownBlankDiddyEv"]
    pub(super) fn status_end_ClungThrownBlankDiddy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_ClungGanonEv"]
    pub(super) fn status_pre_ClungGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_ClungGanonEv"]
    pub(super) fn status_ClungGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_ClungGanon_MainEv"]
    pub(super) fn status_ClungGanon_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_ClungGanon_mainEv"]
    pub(super) fn sub_uniq_process_ClungGanon_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_ClungGanonEv"]
    pub(super) fn status_end_ClungGanon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_ClungGanon_initEv"]
    pub(super) fn sub_uniq_process_ClungGanon_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_uniq_process_ClungGanon_exec_statusEv"]
    pub(super) fn sub_uniq_process_ClungGanon_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_uniq_process_ClungGanon_exec_stopEv"]
    pub(super) fn sub_uniq_process_ClungGanon_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_ClungGanon_exitEv"]
    pub(super) fn sub_uniq_process_ClungGanon_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessClungGanon_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessClungGanon_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53sub_FighterStatusDamage_get_coll_stop_slidable_lengthEv"]
    pub(super) fn sub_FighterStatusDamage_get_coll_stop_slidable_length(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52sub_FighterStatusDamage_get_damage_fly_angle_composeEv"]
    pub(super) fn sub_FighterStatusDamage_get_damage_fly_angle_compose(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51sub_FighterStatusDamage_correctDamageVectorExecStopEv"]
    pub(super) fn sub_FighterStatusDamage_correctDamageVectorExecStop(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusDamage_init_damage_speed_upEN3lib8L2CValueES2_S2_"]
    pub(super) fn FighterStatusDamage_init_damage_speed_up(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44FighterStatusDamage__create_damage_log_tableEv"]
    pub(super) fn FighterStatusDamage__create_damage_log_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_pre_DamageEv"]
    pub(super) fn status_pre_Damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13status_DamageEv"]
    pub(super) fn status_Damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22damage_ground_chk_uniqEv"]
    pub(super) fn damage_ground_chk_uniq(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_Damage_MainEv"]
    pub(super) fn status_Damage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_end_DamageEv"]
    pub(super) fn status_end_Damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_DamageAirEv"]
    pub(super) fn status_pre_DamageAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_DamageAirEv"]
    pub(super) fn status_DamageAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19damage_air_chk_uniqEN3lib8L2CValueE"]
    pub(super) fn damage_air_chk_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_DamageAir_MainEv"]
    pub(super) fn status_DamageAir_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_DamageAirEv"]
    pub(super) fn status_end_DamageAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25check_ryu_final_damage_03EN3lib8L2CValueE"]
    pub(super) fn check_ryu_final_damage_03(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_DamageFlyEv"]
    pub(super) fn status_pre_DamageFly(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_DamageFlyEv"]
    pub(super) fn status_DamageFly(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_DamageFly_CommonEv"]
    pub(super) fn status_DamageFly_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_DamageFly_MainEv"]
    pub(super) fn status_DamageFly_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_DamageFinishCamera_execEv"]
    pub(super) fn status_DamageFinishCamera_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_DamageFlyCommonEv"]
    pub(super) fn sub_DamageFlyCommon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_AirChkDamageReflectWallEv"]
    pub(super) fn sub_AirChkDamageReflectWall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_AirChkDamageReflectCeilEv"]
    pub(super) fn sub_AirChkDamageReflectCeil(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_AirChkDamageReflectFloorEv"]
    pub(super) fn sub_AirChkDamageReflectFloor(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_AirChkPassiveWallJumpEv"]
    pub(super) fn sub_AirChkPassiveWallJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_AirChkPassiveWallEv"]
    pub(super) fn sub_AirChkPassiveWall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_AirChkPassiveCeilEv"]
    pub(super) fn sub_AirChkPassiveCeil(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_DamageFlyChkUniqEv"]
    pub(super) fn sub_DamageFlyChkUniq(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14sub_AirChkDownEv"]
    pub(super) fn sub_AirChkDown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_AirChkPassive_for_damageEv"]
    pub(super) fn sub_AirChkPassive_for_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_check_passive_button_for_damageEN3lib8L2CValueE"]
    pub(super) fn sub_check_passive_button_for_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_DamageFinishCamera_initEv"]
    pub(super) fn status_DamageFinishCamera_init(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38status_DamageFinishCamera_StartEndSlowEv"]
    pub(super) fn status_DamageFinishCamera_StartEndSlow(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33ftStatusUniqProcessDamageFly_initEN3lib8L2CValueE"]
    pub(super) fn ftStatusUniqProcessDamageFly_init(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46sub_ftStatusUniqProcessDamageFly_getMotionKindEv"]
    pub(super) fn sub_ftStatusUniqProcessDamageFly_getMotionKind(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45virtual_ftStatusUniqProcessDamage_init_commonEN3lib8L2CValueE"]
    pub(super) fn virtual_ftStatusUniqProcessDamage_init_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_ftStatusUniqProcessDamage_getMotionKindEv"]
    pub(super) fn sub_ftStatusUniqProcessDamage_getMotionKind(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22set_damage_motion_rateEN3lib8L2CValueES2_S2_"]
    pub(super) fn set_damage_motion_rate(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23calc_damage_motion_rateEN3lib8L2CValueES2_S2_"]
    pub(super) fn calc_damage_motion_rate(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27reaction_frame_mul_speed_upEv"]
    pub(super) fn reaction_frame_mul_speed_up(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46sub_ftStatusUniqProcessDamage_getMotionKindSubEN3lib8L2CValueES2_"]
    pub(super) fn sub_ftStatusUniqProcessDamage_getMotionKindSub(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40ftStatusUniqProcessDamageFly_init_commonEN3lib8L2CValueE"]
    pub(super) fn ftStatusUniqProcessDamageFly_init_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37ftStatusUniqProcessDamage_init_commonEv"]
    pub(super) fn ftStatusUniqProcessDamage_init_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49sub_ftStatusUniqProcessDamageFly_getMotionKindSubEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessDamageFly_getMotionKindSub(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_DamageFly_setup_stransEv"]
    pub(super) fn sub_DamageFly_setup_strans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_DamageFlyCommon_check_deadEv"]
    pub(super) fn sub_DamageFlyCommon_check_dead(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_DamageFlyCommon_initEv"]
    pub(super) fn sub_DamageFlyCommon_init(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19damage_fly_chk_uniqEN3lib8L2CValueE"]
    pub(super) fn damage_fly_chk_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_DamageFlyEv"]
    pub(super) fn status_end_DamageFly(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_end_damage_fly_roll_to_deadEv"]
    pub(super) fn sub_end_damage_fly_roll_to_dead(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_DamageFlyRollEv"]
    pub(super) fn status_pre_DamageFlyRoll(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_DamageFlyRollEv"]
    pub(super) fn status_DamageFlyRoll(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_DamageFlyRoll_CommonEv"]
    pub(super) fn status_DamageFlyRoll_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_DamageFlyRoll_MainEv"]
    pub(super) fn status_DamageFlyRoll_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_DamageFlyRollEv"]
    pub(super) fn status_end_DamageFlyRoll(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33ftStatusUniqProcessDamageAir_initEN3lib8L2CValueE"]
    pub(super) fn ftStatusUniqProcessDamageAir_init(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_damage_uniq_process_getMotionKindAirEv"]
    pub(super) fn sub_damage_uniq_process_getMotionKindAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_damage_uniq_process_getMotionKindAirSubEN3lib8L2CValueE"]
    pub(super) fn sub_damage_uniq_process_getMotionKindAirSub(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40ftStatusUniqProcessDamageFly_exec_commonEv"]
    pub(super) fn ftStatusUniqProcessDamageFly_exec_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37ftStatusUniqProcessDamage_exec_commonEv"]
    pub(super) fn ftStatusUniqProcessDamage_exec_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50sub_ftStatusUniqProcessDamageFly_execFixPosCounterEv"]
    pub(super) fn sub_ftStatusUniqProcessDamageFly_execFixPosCounter(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_PacmanFinal_end_eyeEN3lib8L2CValueE"]
    pub(super) fn sub_PacmanFinal_end_eye(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33ftStatusUniqProcessDamageFly_execEv"]
    pub(super) fn ftStatusUniqProcessDamageFly_exec(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45virtual_ftStatusUniqProcessDamage_exec_commonEv"]
    pub(super) fn virtual_ftStatusUniqProcessDamage_exec_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_ftStatusUniqProcessDamageFly_exitStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessDamageFly_exitStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47sub_ftStatusUniqProcessDamageFlyRoll_exitStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessDamageFlyRoll_exitStatus(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37ftStatusUniqProcessDamageFlyRoll_initEN3lib8L2CValueE"]
    pub(super) fn ftStatusUniqProcessDamageFlyRoll_init(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37ftStatusUniqProcessDamageFlyRoll_execEv"]
    pub(super) fn ftStatusUniqProcessDamageFlyRoll_exec(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39ftStatusUniqProcessDamageFlyMeteor_execEv"]
    pub(super) fn ftStatusUniqProcessDamageFlyMeteor_exec(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_ftStatusUniqProcessDamageFlyRoll_execStopEv"]
    pub(super) fn sub_ftStatusUniqProcessDamageFlyRoll_execStop(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_DamageFallEv"]
    pub(super) fn status_pre_DamageFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_DamageFall_commonEv"]
    pub(super) fn sub_DamageFall_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_damage_fall_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_damage_fall_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_DamageFallEv"]
    pub(super) fn status_DamageFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_DamageFall_MainEv"]
    pub(super) fn status_DamageFall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28check_damage_fall_transitionEv"]
    pub(super) fn check_damage_fall_transition(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_damage_fall_uniq_process_exec_fix_posEv"]
    pub(super) fn sub_damage_fall_uniq_process_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26set_damage_fall_transitionEN3lib8L2CValueE"]
    pub(super) fn set_damage_fall_transition(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_ItemShoot_GenesisCommon_Main_NewEv"]
    pub(super) fn sub_ItemShoot_GenesisCommon_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_damage_fall_uniq_process_mainEv"]
    pub(super) fn sub_damage_fall_uniq_process_main(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_DamageFallEv"]
    pub(super) fn status_end_DamageFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_damage_fall_uniq_process_initEv"]
    pub(super) fn sub_damage_fall_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_damage_fall_uniq_process_exec_statusEv"]
    pub(super) fn sub_damage_fall_uniq_process_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_pre_IceEv"]
    pub(super) fn status_pre_Ice(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon10status_IceEv"]
    pub(super) fn status_Ice(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14sub_ice_mtransEv"]
    pub(super) fn sub_ice_mtrans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_Ice_MainEv"]
    pub(super) fn status_Ice_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusUniqProcessIce_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessIce_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36local_func__fighter_status_damage_10Ev"]
    pub(super) fn local_func__fighter_status_damage_10(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36local_func__fighter_status_damage_11Ev"]
    pub(super) fn local_func__fighter_status_damage_11(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36local_func__fighter_status_damage_12Ev"]
    pub(super) fn local_func__fighter_status_damage_12(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11sub_IceUniqEN3lib8L2CValueE"]
    pub(super) fn sub_IceUniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_end_IceEv"]
    pub(super) fn status_end_Ice(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39FighterStatusUniqProcessIce_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessIce_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_damage_4Ev"]
    pub(super) fn local_func__fighter_status_damage_4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_damage_5Ev"]
    pub(super) fn local_func__fighter_status_damage_5(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_damage_6Ev"]
    pub(super) fn local_func__fighter_status_damage_6(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_damage_7Ev"]
    pub(super) fn local_func__fighter_status_damage_7(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_damage_8Ev"]
    pub(super) fn local_func__fighter_status_damage_8(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_damage_9Ev"]
    pub(super) fn local_func__fighter_status_damage_9(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35local_func__fighter_status_damage_3Ev"]
    pub(super) fn local_func__fighter_status_damage_3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39FighterStatusUniqProcessIce_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessIce_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37FighterStatusUniqProcessIce_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessIce_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39FighterStatusUniqProcessIce_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessIce_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusUniqProcessIce_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessIce_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_IceJumpEv"]
    pub(super) fn status_pre_IceJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_IceJumpEv"]
    pub(super) fn status_IceJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15sub_IceJumpUniqEN3lib8L2CValueE"]
    pub(super) fn sub_IceJumpUniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_IceJump_MainEv"]
    pub(super) fn status_IceJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_IceJumpEv"]
    pub(super) fn status_end_IceJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_BindEv"]
    pub(super) fn status_pre_Bind(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_BindEv"]
    pub(super) fn status_Bind(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Bind_MainEv"]
    pub(super) fn status_Bind_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_BindEv"]
    pub(super) fn status_end_Bind(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_PitFallEv"]
    pub(super) fn status_pre_PitFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_PitFallEv"]
    pub(super) fn status_PitFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_PitFall_MainEv"]
    pub(super) fn status_PitFall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_PitFallEv"]
    pub(super) fn status_end_PitFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_ftStatusUniqProcessPitFall_initStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessPitFall_initStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_ftStatusUniqProcessPitFall_exitStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessPitFall_exitStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_DamageSongStartEv"]
    pub(super) fn status_pre_DamageSongStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_DamageSongStartEv"]
    pub(super) fn status_DamageSongStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_DamageSongStart_MainEv"]
    pub(super) fn status_DamageSongStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_DamageSongStartEv"]
    pub(super) fn status_end_DamageSongStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_DamageSongExitEv"]
    pub(super) fn sub_DamageSongExit(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_DamageSongEv"]
    pub(super) fn status_pre_DamageSong(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_DamageSongEv"]
    pub(super) fn status_DamageSong(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_DamageSong_MainEv"]
    pub(super) fn status_DamageSong_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_DamageSongEv"]
    pub(super) fn status_end_DamageSong(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_DamageSongFallEv"]
    pub(super) fn status_pre_DamageSongFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_DamageSongFallEv"]
    pub(super) fn status_DamageSongFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_DamageSongFall_MainEv"]
    pub(super) fn status_DamageSongFall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_DamageSongFallEv"]
    pub(super) fn status_end_DamageSongFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_DamageSongEndEv"]
    pub(super) fn status_pre_DamageSongEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_DamageSongEndEv"]
    pub(super) fn status_DamageSongEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_DamageSongEnd_MainEv"]
    pub(super) fn status_DamageSongEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_DamageSongEndEv"]
    pub(super) fn status_end_DamageSongEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_DamageSleepStartEv"]
    pub(super) fn status_pre_DamageSleepStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_DamageSleepStartEv"]
    pub(super) fn status_DamageSleepStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_DamageSleepStart_MainEv"]
    pub(super) fn status_DamageSleepStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_DamageSleepStartEv"]
    pub(super) fn status_end_DamageSleepStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_DamageSleepEv"]
    pub(super) fn status_pre_DamageSleep(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_DamageSleepEv"]
    pub(super) fn status_DamageSleep(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_DamageSleep_MainEv"]
    pub(super) fn status_DamageSleep_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_DamageSleepEv"]
    pub(super) fn status_end_DamageSleep(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_DamageSleepFallEv"]
    pub(super) fn status_pre_DamageSleepFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_DamageSleepFallEv"]
    pub(super) fn status_DamageSleepFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_DamageSleepFall_MainEv"]
    pub(super) fn status_DamageSleepFall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_DamageSleepFallEv"]
    pub(super) fn status_end_DamageSleepFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_DamageSleepEndEv"]
    pub(super) fn status_pre_DamageSleepEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_DamageSleepEndEv"]
    pub(super) fn status_DamageSleepEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_DamageSleepEnd_MainEv"]
    pub(super) fn status_DamageSleepEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_DamageSleepEndEv"]
    pub(super) fn status_end_DamageSleepEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_damage_uniq_process_init_with_cppEv"]
    pub(super) fn sub_damage_uniq_process_init_with_cpp(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_damage_uniq_process_initEv"]
    pub(super) fn sub_damage_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36local_func__fighter_status_damage_13Ev"]
    pub(super) fn local_func__fighter_status_damage_13(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_PacmanFinal_start_eyeEN3lib8L2CValueE"]
    pub(super) fn sub_PacmanFinal_start_eye(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38virtual_ftStatusUniqProcessDamage_initEN3lib8L2CValueE"]
    pub(super) fn virtual_ftStatusUniqProcessDamage_init(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30ftStatusUniqProcessDamage_initEN3lib8L2CValueE"]
    pub(super) fn ftStatusUniqProcessDamage_init(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_damage_uniq_process_mainEv"]
    pub(super) fn sub_damage_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38virtual_ftStatusUniqProcessDamage_execEv"]
    pub(super) fn virtual_ftStatusUniqProcessDamage_exec(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_damage_uniq_process_post_mainEv"]
    pub(super) fn sub_damage_uniq_process_post_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25exec_damage_elec_hit_stopEv"]
    pub(super) fn exec_damage_elec_hit_stop(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessDamage_check_hit_stop_delay_flickEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessDamage_check_hit_stop_delay_flick(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_damage_uniq_process_mainStopEv"]
    pub(super) fn sub_damage_uniq_process_mainStop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30ftStatusUniqProcessDamage_execEv"]
    pub(super) fn ftStatusUniqProcessDamage_exec(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_DamageFlyReflectLREv"]
    pub(super) fn status_pre_DamageFlyReflectLR(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_DamageFlyReflectLREv"]
    pub(super) fn status_DamageFlyReflectLR(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_DamageFlyReflectLR_MainEv"]
    pub(super) fn status_DamageFlyReflectLR_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_AirChkDamageDownEv"]
    pub(super) fn sub_AirChkDamageDown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_DamageFlyReflectLREv"]
    pub(super) fn status_end_DamageFlyReflectLR(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23end_damage_reflect_stopEv"]
    pub(super) fn end_damage_reflect_stop(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_DamageFlyReflect_effectEN3lib8L2CValueE"]
    pub(super) fn sub_DamageFlyReflect_effect(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44sub_ftStatusUniqProcessDamageFly_initReflectEv"]
    pub(super) fn sub_ftStatusUniqProcessDamageFly_initReflect(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25start_damage_reflect_stopEv"]
    pub(super) fn start_damage_reflect_stop(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_damage_fly_reflect_lr_uniq_process_initEv"]
    pub(super) fn sub_damage_fly_reflect_lr_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_damage_fly_refect_uniq_process_mainEv"]
    pub(super) fn sub_damage_fly_refect_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_DamageFlyReflectUEv"]
    pub(super) fn status_pre_DamageFlyReflectU(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_DamageFlyReflectUEv"]
    pub(super) fn status_DamageFlyReflectU(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_DamageFlyReflectU_MainEv"]
    pub(super) fn status_DamageFlyReflectU_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_DamageFlyReflectUEv"]
    pub(super) fn status_end_DamageFlyReflectU(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_damage_fly_reflect_u_uniq_process_initEv"]
    pub(super) fn sub_damage_fly_reflect_u_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_DamageFlyReflectDEv"]
    pub(super) fn status_pre_DamageFlyReflectD(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_DamageFlyReflectDEv"]
    pub(super) fn status_DamageFlyReflectD(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_DamageFlyReflectD_MainEv"]
    pub(super) fn status_DamageFlyReflectD_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_DamageFlyReflectDEv"]
    pub(super) fn status_end_DamageFlyReflectD(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_damage_fly_reflect_d_uniq_process_initEv"]
    pub(super) fn sub_damage_fly_reflect_d_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41FighterStatusUniqProcessDamage_leave_stopEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessDamage_leave_stop(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessDamageFly_check_attackEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessDamageFly_check_attack(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36local_func__fighter_status_damage_17Ev"]
    pub(super) fn local_func__fighter_status_damage_17(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36local_func__fighter_status_damage_18Ev"]
    pub(super) fn local_func__fighter_status_damage_18(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36local_func__fighter_status_damage_19Ev"]
    pub(super) fn local_func__fighter_status_damage_19(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_dead_up_star_scaleEv"]
    pub(super) fn sub_dead_up_star_scale(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22calc_camera_follow_posEN3lib8L2CValueES2_S2_"]
    pub(super) fn calc_camera_follow_pos(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_DeadEv"]
    pub(super) fn status_pre_Dead(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_StandbyEv"]
    pub(super) fn status_pre_Standby(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_DeadEv"]
    pub(super) fn status_Dead(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_DeadSubEv"]
    pub(super) fn status_DeadSub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Dead_MainEv"]
    pub(super) fn status_Dead_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_StandbyEv"]
    pub(super) fn status_Standby(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_Standby_MainEv"]
    pub(super) fn status_Standby_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_DeadEv"]
    pub(super) fn status_end_Dead(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_StandbyEv"]
    pub(super) fn status_end_Standby(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_get_dead_quake_kindEN3lib8L2CValueE"]
    pub(super) fn sub_get_dead_quake_kind(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_dead_uniq_process_initEv"]
    pub(super) fn sub_dead_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_dead_uniq_process_execEv"]
    pub(super) fn sub_dead_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_dead_up_fall_processEN3lib8L2CValueE"]
    pub(super) fn sub_dead_up_fall_process(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_dead_up_galaga_processEN3lib8L2CValueE"]
    pub(super) fn sub_dead_up_galaga_process(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon10sub_to_endEv"]
    pub(super) fn sub_to_end(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_dead_uniq_process_fix_cameraEv"]
    pub(super) fn sub_dead_uniq_process_fix_camera(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_dead_uniq_process_exitEv"]
    pub(super) fn sub_dead_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_DemoEv"]
    pub(super) fn status_pre_Demo(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_demo_uniq_process_initEv"]
    pub(super) fn sub_demo_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_DemoEv"]
    pub(super) fn status_Demo(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Demo_MainEv"]
    pub(super) fn status_Demo_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_DemoEv"]
    pub(super) fn status_end_Demo(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_DownEv"]
    pub(super) fn status_pre_Down(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_DownEv"]
    pub(super) fn status_Down(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_down_common_preEv"]
    pub(super) fn sub_down_common_pre(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Down_MainEv"]
    pub(super) fn status_Down_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15sub_down_commonEv"]
    pub(super) fn sub_down_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_down_chk_reflect_wallEv"]
    pub(super) fn sub_down_chk_reflect_wall(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_down_chk_reflect_wall_initEv"]
    pub(super) fn sub_down_chk_reflect_wall_init(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_DownEv"]
    pub(super) fn status_end_Down(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_DownSpotEv"]
    pub(super) fn status_pre_DownSpot(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_DownSpotEv"]
    pub(super) fn status_DownSpot(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_DownSpot_MainEv"]
    pub(super) fn status_DownSpot_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_DownSpotEv"]
    pub(super) fn status_end_DownSpot(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_down_spot_uniq_process_initEv"]
    pub(super) fn sub_down_spot_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_DownContinueEv"]
    pub(super) fn status_pre_DownContinue(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_DownContinueEv"]
    pub(super) fn status_DownContinue(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_DownContinue_MainEv"]
    pub(super) fn status_DownContinue_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_DownContinueEv"]
    pub(super) fn status_end_DownContinue(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_DownWaitEv"]
    pub(super) fn status_pre_DownWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_DownWaitEv"]
    pub(super) fn status_DownWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_down_wait_common_preEv"]
    pub(super) fn sub_down_wait_common_pre(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_down_wait_commonEv"]
    pub(super) fn sub_down_wait_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_down_wait_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_down_wait_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_DownWait_MainEv"]
    pub(super) fn status_DownWait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_DownWaitEv"]
    pub(super) fn status_end_DownWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_DownWaitContinueEv"]
    pub(super) fn status_pre_DownWaitContinue(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_DownWaitContinueEv"]
    pub(super) fn status_DownWaitContinue(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_DownWaitContinue_MainEv"]
    pub(super) fn status_DownWaitContinue_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_DownWaitContinueEv"]
    pub(super) fn status_end_DownWaitContinue(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_DownStandEv"]
    pub(super) fn status_pre_DownStand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_DownStandEv"]
    pub(super) fn status_DownStand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_DownStand_MainEv"]
    pub(super) fn status_DownStand_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_DownStandEv"]
    pub(super) fn status_end_DownStand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_DownStandFbEv"]
    pub(super) fn status_pre_DownStandFb(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_DownStandFbEv"]
    pub(super) fn status_DownStandFb(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_DownStandFb_MainEv"]
    pub(super) fn status_DownStandFb_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_DownStandFbEv"]
    pub(super) fn status_end_DownStandFb(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_DownStandAttackEv"]
    pub(super) fn status_pre_DownStandAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_DownStandAttackEv"]
    pub(super) fn status_DownStandAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_DownStandAttack_MainEv"]
    pub(super) fn status_DownStandAttack_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_DownStandAttackEv"]
    pub(super) fn status_end_DownStandAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_DownDamageEv"]
    pub(super) fn status_pre_DownDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_DownDamageEv"]
    pub(super) fn status_DownDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_DownDamage_MainEv"]
    pub(super) fn status_DownDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_down_damage_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_down_damage_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_DownDamageEv"]
    pub(super) fn status_end_DownDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_down_damage_uniq_process_initEv"]
    pub(super) fn sub_down_damage_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_down_damage_uniq_process_mainEv"]
    pub(super) fn sub_down_damage_uniq_process_main(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_down_damage_uniq_process_exitEv"]
    pub(super) fn sub_down_damage_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_DownReflectLREv"]
    pub(super) fn status_pre_DownReflectLR(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_DownReflectLREv"]
    pub(super) fn status_DownReflectLR(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_DownReflectLR_MainEv"]
    pub(super) fn status_DownReflectLR_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_DownReflectLREv"]
    pub(super) fn status_end_DownReflectLR(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_down_reflect_uniq_process_initEv"]
    pub(super) fn sub_down_reflect_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56sub_ftStatusUniqProcessDownDamage_initNormalDamageCommonEv"]
    pub(super) fn sub_ftStatusUniqProcessDownDamage_initNormalDamageCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_down_reflect_uniq_process_mainEv"]
    pub(super) fn sub_down_reflect_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56sub_ftStatusUniqProcessDownDamage_execNormalDamageCommonEv"]
    pub(super) fn sub_ftStatusUniqProcessDownDamage_execNormalDamageCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_down_uniq_process_initEv"]
    pub(super) fn sub_down_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_LayDownEv"]
    pub(super) fn status_pre_LayDown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_LayDownEv"]
    pub(super) fn status_LayDown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_LayDown_MainEv"]
    pub(super) fn status_LayDown_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_LayDownEv"]
    pub(super) fn status_end_LayDown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_DownEatEv"]
    pub(super) fn status_pre_DownEat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_DownEatEv"]
    pub(super) fn status_DownEat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_DownEat_MainEv"]
    pub(super) fn status_DownEat_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_DownEatEv"]
    pub(super) fn status_end_DownEat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_pre_EntryEv"]
    pub(super) fn status_pre_Entry(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12status_EntryEv"]
    pub(super) fn status_Entry(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_Entry_MainEv"]
    pub(super) fn status_Entry_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_entry_remove_articleEv"]
    pub(super) fn sub_entry_remove_article(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15sub_wait_commonEv"]
    pub(super) fn sub_wait_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_wait_motion_mtransEv"]
    pub(super) fn sub_wait_motion_mtrans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_entry_uniq_process_exec_fix_posEv"]
    pub(super) fn sub_entry_uniq_process_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_is_through_wait_motion_mtransEv"]
    pub(super) fn sub_is_through_wait_motion_mtrans(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_wait_item_motion_mtransEv"]
    pub(super) fn sub_wait_item_motion_mtrans(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_end_EntryEv"]
    pub(super) fn status_end_Entry(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_entry_uniq_process_initEv"]
    pub(super) fn sub_entry_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_entry_uniq_process_exitEv"]
    pub(super) fn sub_entry_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_pre_EscapeEv"]
    pub(super) fn status_pre_Escape(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_EscapeFEv"]
    pub(super) fn status_pre_EscapeF(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17sub_pre_escape_fbEv"]
    pub(super) fn sub_pre_escape_fb(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_EscapeBEv"]
    pub(super) fn status_pre_EscapeB(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_EscapeAirEv"]
    pub(super) fn status_pre_EscapeAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_Escape_commonEv"]
    pub(super) fn status_Escape_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15sub_escape_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_escape_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13status_EscapeEv"]
    pub(super) fn status_Escape(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_Escape_MainEv"]
    pub(super) fn status_Escape_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_escape_check_rumbleEv"]
    pub(super) fn sub_escape_check_rumble(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_EscapeF_SubEv"]
    pub(super) fn status_EscapeF_Sub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_escape_fb_commonEv"]
    pub(super) fn sub_escape_fb_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_EscapeFEv"]
    pub(super) fn status_EscapeF(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_escape_fb_mainEv"]
    pub(super) fn sub_escape_fb_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_EscapeF_MainEv"]
    pub(super) fn status_EscapeF_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_EscapeBEv"]
    pub(super) fn status_EscapeB(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_EscapeB_MainEv"]
    pub(super) fn status_EscapeB_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_EscapeAirEv"]
    pub(super) fn status_EscapeAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_escape_air_commonEv"]
    pub(super) fn sub_escape_air_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_escape_air_common_mainEv"]
    pub(super) fn sub_escape_air_common_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_escape_air_common_strans_mainEv"]
    pub(super) fn sub_escape_air_common_strans_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_escape_air_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_escape_air_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21exec_escape_air_slideEv"]
    pub(super) fn exec_escape_air_slide(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_EscapeAir_MainEv"]
    pub(super) fn status_EscapeAir_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_end_EscapeEv"]
    pub(super) fn status_end_Escape(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_EscapeFEv"]
    pub(super) fn status_end_EscapeF(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_status_end_EscaleFBEv"]
    pub(super) fn sub_status_end_EscaleFB(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_EscapeBEv"]
    pub(super) fn status_end_EscapeB(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_EscapeAirEv"]
    pub(super) fn status_end_EscapeAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_escape_uniq_process_common_initStatusEv"]
    pub(super) fn sub_escape_uniq_process_common_initStatus(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48sub_escape_uniq_process_common_initStatus_commonEN3lib8L2CValueES2_"]
    pub(super) fn sub_escape_uniq_process_common_initStatus_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22setup_escape_air_slideEv"]
    pub(super) fn setup_escape_air_slide(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29setup_escape_air_slide_commonEN3lib8L2CValueES2_"]
    pub(super) fn setup_escape_air_slide_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_escape_uniq_process_initStatusEv"]
    pub(super) fn sub_escape_uniq_process_initStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon65sub_escape_air_inherit_jump_aerial_motion_uniq_process_initStatusEv"]
    pub(super) fn sub_escape_air_inherit_jump_aerial_motion_uniq_process_initStatus(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon67sub_escape_air_inherit_jump_aerial_motion_uniq_process_exec_fix_posEv"]
    pub(super) fn sub_escape_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44FighterStatusUniqProcessEscapeAir_calc_paramEv"]
    pub(super) fn FighterStatusUniqProcessEscapeAir_calc_param(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_fall_specialEv"]
    pub(super) fn status_pre_fall_special(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12sub_pre_fallEv"]
    pub(super) fn sub_pre_fall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_pre_fall_paramEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_pre_fall_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_landing_fall_specialEv"]
    pub(super) fn status_pre_landing_fall_special(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38status_pre_landing_fall_special_commonEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_pre_landing_fall_special_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_fall_specialEv"]
    pub(super) fn status_fall_special(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_fall_special_mainEv"]
    pub(super) fn status_fall_special_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon8sub_fallEv"]
    pub(super) fn sub_fall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_landing_fall_specialEv"]
    pub(super) fn status_landing_fall_special(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_landing_fall_special_subEv"]
    pub(super) fn status_landing_fall_special_sub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_status_landing_fall_special_mainEv"]
    pub(super) fn sub_status_landing_fall_special_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_status_landing_fall_special_main_cmnEv"]
    pub(super) fn sub_status_landing_fall_special_main_cmn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44status_landing_fall_special_air_fall_specialEv"]
    pub(super) fn status_landing_fall_special_air_fall_special(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49status_landing_fall_special_air_fall_special_mainEv"]
    pub(super) fn status_landing_fall_special_air_fall_special_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_landing_fall_special_mainEv"]
    pub(super) fn status_landing_fall_special_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_fall_specialEv"]
    pub(super) fn status_end_fall_special(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_landing_fall_specialEv"]
    pub(super) fn status_end_landing_fall_special(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_FistDownEv"]
    pub(super) fn status_pre_FistDown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_FistDownEv"]
    pub(super) fn status_FistDown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_status_SavingDamage_Main_commonEv"]
    pub(super) fn sub_status_SavingDamage_Main_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_FistDownEv"]
    pub(super) fn status_end_FistDown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_fist_down_uniq_process_initEv"]
    pub(super) fn sub_fist_down_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_saving_damage_uniq_process_initEv"]
    pub(super) fn sub_saving_damage_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_fist_down_uniq_process_mainEv"]
    pub(super) fn sub_fist_down_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_saving_damage_uniq_process_main_commonEN3lib8L2CValueES2_S2_S2_S2_S2_S2_S2_"]
    pub(super) fn sub_saving_damage_uniq_process_main_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack, arg8: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_FistDown2Ev"]
    pub(super) fn status_pre_FistDown2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_FistDown2Ev"]
    pub(super) fn status_FistDown2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_status_SavingDamage_commonEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_status_SavingDamage_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_FistDown2Ev"]
    pub(super) fn status_end_FistDown2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_fist_down2_uniq_process_initEv"]
    pub(super) fn sub_fist_down2_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_fist_down2_uniq_process_mainEv"]
    pub(super) fn sub_fist_down2_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_FistDown3Ev"]
    pub(super) fn status_pre_FistDown3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_FistDown3Ev"]
    pub(super) fn status_FistDown3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_FistDown3Ev"]
    pub(super) fn status_end_FistDown3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_fist_down3_uniq_process_initEv"]
    pub(super) fn sub_fist_down3_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_fist_down3_uniq_process_mainEv"]
    pub(super) fn sub_fist_down3_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_FreeMoveEv"]
    pub(super) fn status_pre_FreeMove(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_FreeMoveEv"]
    pub(super) fn status_FreeMove(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_FreeMove_MainEv"]
    pub(super) fn status_FreeMove_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32uniq_process_Gimmick_init_statusEv"]
    pub(super) fn uniq_process_Gimmick_init_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32uniq_process_Gimmick_exit_statusEN3lib8L2CValueE"]
    pub(super) fn uniq_process_Gimmick_exit_status(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessGimmick_notify_event_gimmickEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmick_notify_event_gimmick(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_GimmickBarrelEv"]
    pub(super) fn status_pre_GimmickBarrel(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_GimmickBarrelEv"]
    pub(super) fn status_GimmickBarrel(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_GimmickBarrel_MainEv"]
    pub(super) fn status_GimmickBarrel_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34uniq_process_GimmickBarrel_fix_posEv"]
    pub(super) fn uniq_process_GimmickBarrel_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_GimmickBarrelEv"]
    pub(super) fn status_end_GimmickBarrel(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38uniq_process_GimmickBarrel_init_statusEv"]
    pub(super) fn uniq_process_GimmickBarrel_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38uniq_process_GimmickBarrel_exec_statusEv"]
    pub(super) fn uniq_process_GimmickBarrel_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessGimmickBarrel_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickBarrel_exec_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38uniq_process_GimmickBarrel_exit_statusEv"]
    pub(super) fn uniq_process_GimmickBarrel_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessGimmickBarrel_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickBarrel_exit_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58FighterStatusUniqProcessGimmickBarrel_notify_event_gimmickEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickBarrel_notify_event_gimmick(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_GimmickDoorEv"]
    pub(super) fn status_pre_GimmickDoor(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_GimmickDoorEv"]
    pub(super) fn status_GimmickDoor(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_GimmickDoor_MainEv"]
    pub(super) fn status_GimmickDoor_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_GimmickDoorEv"]
    pub(super) fn status_end_GimmickDoor(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36uniq_process_GimmickDoor_init_statusEv"]
    pub(super) fn uniq_process_GimmickDoor_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36uniq_process_GimmickDoor_exec_statusEv"]
    pub(super) fn uniq_process_GimmickDoor_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36uniq_process_GimmickDoor_exit_statusEv"]
    pub(super) fn uniq_process_GimmickDoor_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_GimmickSpringEv"]
    pub(super) fn status_pre_GimmickSpring(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_GimmickSpringEv"]
    pub(super) fn status_GimmickSpring(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_GimmickSpring_CommonEv"]
    pub(super) fn status_GimmickSpring_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_GimmickSpring_MainEv"]
    pub(super) fn status_GimmickSpring_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_GimmickSpringEv"]
    pub(super) fn status_end_GimmickSpring(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38uniq_process_GimmickSpring_init_statusEv"]
    pub(super) fn uniq_process_GimmickSpring_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38uniq_process_GimmickSpring_exec_statusEv"]
    pub(super) fn uniq_process_GimmickSpring_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38uniq_process_GimmickSpring_exit_statusEv"]
    pub(super) fn uniq_process_GimmickSpring_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58FighterStatusUniqProcessGimmickSpring_set_pos_and_safe_posEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickSpring_set_pos_and_safe_pos(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58FighterStatusUniqProcessGimmickSpring_notify_event_gimmickEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickSpring_notify_event_gimmick(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_GimmickSpringJumpEv"]
    pub(super) fn status_pre_GimmickSpringJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_GimmickSpringJumpEv"]
    pub(super) fn status_GimmickSpringJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_JumpEv"]
    pub(super) fn status_Jump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_jump_item_rocketbeltEv"]
    pub(super) fn sub_jump_item_rocketbelt(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_Jump_subEN3lib8L2CValueES2_"]
    pub(super) fn status_Jump_sub(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Jump_MainEv"]
    pub(super) fn status_Jump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_GimmickSpringJumpEv"]
    pub(super) fn status_end_GimmickSpringJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_GimmickSpringBackEv"]
    pub(super) fn status_pre_GimmickSpringBack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_GimmickSpringBackEv"]
    pub(super) fn status_GimmickSpringBack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_GimmickSpringBack_MainEv"]
    pub(super) fn status_GimmickSpringBack_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_GimmickSpringBackEv"]
    pub(super) fn status_end_GimmickSpringBack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_GimmickEatenEv"]
    pub(super) fn status_pre_GimmickEaten(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_GimmickEatenEv"]
    pub(super) fn status_GimmickEaten(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_GimmickEaten_MainEv"]
    pub(super) fn status_GimmickEaten_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_GimmickEatenEv"]
    pub(super) fn status_end_GimmickEaten(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37uniq_process_GimmickEaten_init_statusEv"]
    pub(super) fn uniq_process_GimmickEaten_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37uniq_process_GimmickEaten_exit_statusEv"]
    pub(super) fn uniq_process_GimmickEaten_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessGimmickEaten_notify_event_gimmickEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickEaten_notify_event_gimmick(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_GimmickPipeEv"]
    pub(super) fn status_pre_GimmickPipe(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_GimmickPipeEv"]
    pub(super) fn status_GimmickPipe(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_GimmickPipe_MainEv"]
    pub(super) fn status_GimmickPipe_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44FighterStatusUniqProcessGimmickPipe_is_movedEv"]
    pub(super) fn FighterStatusUniqProcessGimmickPipe_is_moved(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessGimmickPipe_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessGimmickPipe_exec_fix_pos(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_GimmickPipeEv"]
    pub(super) fn status_end_GimmickPipe(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36uniq_process_GimmickPipe_init_statusEv"]
    pub(super) fn uniq_process_GimmickPipe_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessGimmickPipe_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickPipe_init_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36uniq_process_GimmickPipe_exit_statusEv"]
    pub(super) fn uniq_process_GimmickPipe_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessGimmickPipe_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickPipe_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessGimmickPipe_notify_event_gimmickEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickPipe_notify_event_gimmick(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusUniqProcessGimmickPipe_moveEN3lib8L2CValueES2_S2_S2_S2_S2_S2_S2_S2_"]
    pub(super) fn FighterStatusUniqProcessGimmickPipe_move(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack, arg8: lib::L2CValueHack, arg9: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_GimmickTornadoEv"]
    pub(super) fn status_pre_GimmickTornado(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_GimmickTornadoEv"]
    pub(super) fn status_GimmickTornado(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_GimmickTornado_MainEv"]
    pub(super) fn status_GimmickTornado_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_GimmickTornado_damageFlyRollEndEv"]
    pub(super) fn sub_GimmickTornado_damageFlyRollEnd(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_GimmickTornadoEv"]
    pub(super) fn status_end_GimmickTornado(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39uniq_process_GimmickTornado_init_statusEv"]
    pub(super) fn uniq_process_GimmickTornado_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessGimmickTornado_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickTornado_init_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39uniq_process_GimmickTornado_exit_statusEv"]
    pub(super) fn uniq_process_GimmickTornado_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessGimmickTornado_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickTornado_exit_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44FighterStatusUniqProcessGimmickTornado_shootEv"]
    pub(super) fn FighterStatusUniqProcessGimmickTornado_shoot(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43FighterStatusUniqProcessGimmickTornado_moveEv"]
    pub(super) fn FighterStatusUniqProcessGimmickTornado_move(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon62FighterStatusUniqProcessGimmickTornado_move_for_hyrule_tornadoEv"]
    pub(super) fn FighterStatusUniqProcessGimmickTornado_move_for_hyrule_tornado(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessGimmickTornado_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickTornado_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessGimmickTornado_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessGimmickTornado_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon59FighterStatusUniqProcessGimmickTornado_notify_event_gimmickEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickTornado_notify_event_gimmick(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_GimmickFishCaptureEv"]
    pub(super) fn status_pre_GimmickFishCapture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_GimmickFishCaptureEv"]
    pub(super) fn status_GimmickFishCapture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_GimmickFishCapture_MainEv"]
    pub(super) fn status_GimmickFishCapture_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_GimmickFishCaptureEv"]
    pub(super) fn status_end_GimmickFishCapture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43uniq_process_GimmickFishCapture_init_statusEv"]
    pub(super) fn uniq_process_GimmickFishCapture_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43uniq_process_GimmickFishCapture_exit_statusEv"]
    pub(super) fn uniq_process_GimmickFishCapture_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessGimmickFishCapture_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickFishCapture_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessGimmickFishCapture_update_postureEv"]
    pub(super) fn FighterStatusUniqProcessGimmickFishCapture_update_posture(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessGimmickFishCapture_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessGimmickFishCapture_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_GimmickDrumEv"]
    pub(super) fn status_pre_GimmickDrum(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_GimmickDrumEv"]
    pub(super) fn status_GimmickDrum(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_GimmickDrum_MainEv"]
    pub(super) fn status_GimmickDrum_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessGimmickDrum_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessGimmickDrum_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_GimmickDrumEv"]
    pub(super) fn status_end_GimmickDrum(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36uniq_process_GimmickDrum_init_statusEv"]
    pub(super) fn uniq_process_GimmickDrum_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36uniq_process_GimmickDrum_exit_statusEv"]
    pub(super) fn uniq_process_GimmickDrum_exit_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessGimmickDrum_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickDrum_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessGimmickDrum_notify_event_gimmickEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickDrum_notify_event_gimmick(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_GimmickOdinSlashedEv"]
    pub(super) fn status_pre_GimmickOdinSlashed(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_GimmickOdinSlashedEv"]
    pub(super) fn status_GimmickOdinSlashed(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_GimmickOdinSlashed_MainEv"]
    pub(super) fn status_GimmickOdinSlashed_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_GimmickOdinSlashedEv"]
    pub(super) fn status_end_GimmickOdinSlashed(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessGimmickOdinSlashed_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickOdinSlashed_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessGimmickOdinSlashed_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickOdinSlashed_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon63FighterStatusUniqProcessGimmickOdinSlashed_notify_event_gimmickEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickOdinSlashed_notify_event_gimmick(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_DollyStageDeadEv"]
    pub(super) fn status_pre_DollyStageDead(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_DollyStageDeadEv"]
    pub(super) fn status_DollyStageDead(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_DollyStageDead_MainEv"]
    pub(super) fn status_DollyStageDead_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24exec_stop_DollyStageDeadEv"]
    pub(super) fn exec_stop_DollyStageDead(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_DollyStageDeadEv"]
    pub(super) fn status_end_DollyStageDead(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_GimmickJumpBoardEv"]
    pub(super) fn status_pre_GimmickJumpBoard(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_GimmickJumpBoardEv"]
    pub(super) fn status_GimmickJumpBoard(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_GimmickJumpBoard_CommonEv"]
    pub(super) fn status_GimmickJumpBoard_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_GimmickJumpBoard_MainEv"]
    pub(super) fn status_GimmickJumpBoard_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_GimmickJumpBoardEv"]
    pub(super) fn status_end_GimmickJumpBoard(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41uniq_process_GimmickJumpBoard_init_statusEv"]
    pub(super) fn uniq_process_GimmickJumpBoard_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41uniq_process_GimmickJumpBoard_exec_statusEv"]
    pub(super) fn uniq_process_GimmickJumpBoard_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41uniq_process_GimmickJumpBoard_exit_statusEv"]
    pub(super) fn uniq_process_GimmickJumpBoard_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon61FighterStatusUniqProcessGimmickJumpBoard_set_pos_and_safe_posEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickJumpBoard_set_pos_and_safe_pos(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_GimmickJumpBoardJumpEv"]
    pub(super) fn status_pre_GimmickJumpBoardJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_GimmickJumpBoardJumpEv"]
    pub(super) fn status_GimmickJumpBoardJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_JumpBoardJump_subEv"]
    pub(super) fn status_JumpBoardJump_sub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_GimmickJumpBoardJump_MainEv"]
    pub(super) fn status_GimmickJumpBoardJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45uniq_process_GimmickJumpBoardJump_exec_statusEv"]
    pub(super) fn uniq_process_GimmickJumpBoardJump_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_GimmickJumpBoardJumpEv"]
    pub(super) fn status_end_GimmickJumpBoardJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_pre_DamageFlyReflectJumpBoardEv"]
    pub(super) fn status_pre_DamageFlyReflectJumpBoard(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_DamageFlyReflectJumpBoardEv"]
    pub(super) fn status_DamageFlyReflectJumpBoard(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37status_DamageFlyReflectJumpBoard_MainEv"]
    pub(super) fn status_DamageFlyReflectJumpBoard_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_end_DamageFlyReflectJumpBoardEv"]
    pub(super) fn status_end_DamageFlyReflectJumpBoard(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51sub_damage_fly_reflect_jump_board_uniq_process_initEv"]
    pub(super) fn sub_damage_fly_reflect_jump_board_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36FighterStatusGuard__set_shield_scaleEN3lib8L2CValueE"]
    pub(super) fn FighterStatusGuard__set_shield_scale(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_GuardOnEv"]
    pub(super) fn status_pre_GuardOn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_guard_cont_preEv"]
    pub(super) fn sub_guard_cont_pre(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_status_guard_on_commonEv"]
    pub(super) fn sub_status_guard_on_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_ftStatusUniqProcessGuardFunc_updateShieldEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessGuardFunc_updateShield(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17sub_guard_on_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_guard_on_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_GuardOnEv"]
    pub(super) fn status_GuardOn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_GuardOn_MainEv"]
    pub(super) fn status_GuardOn_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_status_guard_on_main_air_commonEv"]
    pub(super) fn sub_status_guard_on_main_air_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14sub_guard_contEv"]
    pub(super) fn sub_guard_cont(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_guard_main_commonEv"]
    pub(super) fn status_guard_main_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16check_guard_holdEv"]
    pub(super) fn check_guard_hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29check_guard_attack_special_hiEN3lib8L2CValueE"]
    pub(super) fn check_guard_attack_special_hi(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_status_end_guard_on_commonEN3lib8L2CValueE"]
    pub(super) fn sub_status_end_guard_on_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_GuardOnEv"]
    pub(super) fn status_end_GuardOn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_ftStatusUniqProcessGuardOn_initStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardOn_initStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48sub_ftStatusUniqProcessGuardOn_initStatus_commonEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardOn_initStatus_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_ftStatusUniqProcessGuardOn_execStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardOn_execStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48sub_ftStatusUniqProcessGuardOn_execStatus_commonEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardOn_execStatus_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_ftStatusUniqProcessGuardOn_exitStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardOn_exitStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48sub_ftStatusUniqProcessGuardOn_exitStatus_commonEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardOn_exitStatus_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_ftStatusUniqProcessGuardOn_execStopEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardOn_execStop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_ftStatusUniqProcessGuardOn_execStop_InnerEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessGuardOn_execStop_Inner(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_pre_GuardEv"]
    pub(super) fn status_pre_Guard(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_status_guard_commonEv"]
    pub(super) fn sub_status_guard_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14sub_guard_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_guard_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12status_GuardEv"]
    pub(super) fn status_Guard(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_Guard_MainEv"]
    pub(super) fn status_Guard_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_guard_main_common_airEv"]
    pub(super) fn status_guard_main_common_air(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_end_GuardEv"]
    pub(super) fn status_end_Guard(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_ftStatusUniqProcessGuard_initStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessGuard_initStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46sub_ftStatusUniqProcessGuard_initStatus_commonEv"]
    pub(super) fn sub_ftStatusUniqProcessGuard_initStatus_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_ftStatusUniqProcessGuard_execStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessGuard_execStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46sub_ftStatusUniqProcessGuard_execStatus_commonEv"]
    pub(super) fn sub_ftStatusUniqProcessGuard_execStatus_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_ftStatusUniqProcessGuard_execStopEv"]
    pub(super) fn sub_ftStatusUniqProcessGuard_execStop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44sub_ftStatusUniqProcessGuard_execStop_commonEv"]
    pub(super) fn sub_ftStatusUniqProcessGuard_execStop_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_ftStatusUniqProcessGuard_exitStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessGuard_exitStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46sub_ftStatusUniqProcessGuard_exitStatus_commonEv"]
    pub(super) fn sub_ftStatusUniqProcessGuard_exitStatus_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_GuardOffEv"]
    pub(super) fn status_pre_GuardOff(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_GuardOff_CommonEv"]
    pub(super) fn status_GuardOff_Common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_guard_off_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_guard_off_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_GuardOffEv"]
    pub(super) fn status_GuardOff(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_GuardOff_MainEv"]
    pub(super) fn status_GuardOff_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_GuardOff_Main_commonEv"]
    pub(super) fn status_GuardOff_Main_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_status_guard_off_main_common_cancelEv"]
    pub(super) fn sub_status_guard_off_main_common_cancel(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_status_guard_off_main_common_airEv"]
    pub(super) fn sub_status_guard_off_main_common_air(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_status_guard_off_main_common_controlEv"]
    pub(super) fn sub_status_guard_off_main_common_control(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_GuardOffEv"]
    pub(super) fn status_end_GuardOff(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_ftStatusUniqProcessGuardOff_initStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardOff_initStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_ftStatusUniqProcessGuardOff_execStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardOff_execStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_ftStatusUniqProcessGuardOff_execStopEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardOff_execStop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_ftStatusUniqProcessGuardOff_exitStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardOff_exitStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_GuardDamageEv"]
    pub(super) fn status_pre_GuardDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_GuardDamage_commonEN3lib8L2CValueE"]
    pub(super) fn status_GuardDamage_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_GuardDamageUniqEN3lib8L2CValueE"]
    pub(super) fn sub_GuardDamageUniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_GuardDamageEv"]
    pub(super) fn status_GuardDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_GuardDamage_MainEv"]
    pub(super) fn status_GuardDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_guard_damage_main_common_airEv"]
    pub(super) fn status_guard_damage_main_common_air(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_guard_damage_main_commonEv"]
    pub(super) fn status_guard_damage_main_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_GuardDamageEv"]
    pub(super) fn status_end_GuardDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_ftStatusUniqProcessGuardDamage_initStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardDamage_initStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51sub_ftStatusUniqProcessGuardDamage_initStatus_InnerEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardDamage_initStatus_Inner(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_ftStatusUniqProcessGuardDamage_execStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardDamage_execStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52sub_ftStatusUniqProcessGuardDamage_execStatus_commonEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardDamage_execStatus_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_ftStatusUniqProcessGuardDamage_execStopEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardDamage_execStop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50sub_ftStatusUniqProcessGuardDamage_execStop_commonEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardDamage_execStop_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_ftStatusUniqProcessGuardDamage_exitStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardDamage_exitStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52sub_ftStatusUniqProcessGuardDamage_exitStatus_commonEv"]
    pub(super) fn sub_ftStatusUniqProcessGuardDamage_exitStatus_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessGuardDamage_leave_stopEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessGuardDamage_leave_stop(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_ShieldBreakFlyEv"]
    pub(super) fn status_pre_ShieldBreakFly(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_status_shield_break_fly_commonEN3lib8L2CValueE"]
    pub(super) fn sub_status_shield_break_fly_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_ShieldBreakFlyEv"]
    pub(super) fn status_ShieldBreakFly(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_ShieldBreakFly_MainEv"]
    pub(super) fn status_ShieldBreakFly_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_ShieldBreakFlyEv"]
    pub(super) fn status_end_ShieldBreakFly(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48sub_ftStatusUniqProcessShieldBreakFly_initStatusEv"]
    pub(super) fn sub_ftStatusUniqProcessShieldBreakFly_initStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessShieldBreak_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessShieldBreak_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_ShieldBreakFallEv"]
    pub(super) fn status_pre_ShieldBreakFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_ShieldBreakFallEv"]
    pub(super) fn status_ShieldBreakFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_ShieldBreakFall_MainEv"]
    pub(super) fn status_ShieldBreakFall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_ShieldBreakFallEv"]
    pub(super) fn status_end_ShieldBreakFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_ShieldBreakDownEv"]
    pub(super) fn status_pre_ShieldBreakDown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_ShieldBreakDownEv"]
    pub(super) fn status_ShieldBreakDown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_ShieldBreakDown_MainEv"]
    pub(super) fn status_ShieldBreakDown_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_ShieldBreakDownEv"]
    pub(super) fn status_end_ShieldBreakDown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_FuraFuraStandEv"]
    pub(super) fn status_pre_FuraFuraStand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_FuraFuraStandEv"]
    pub(super) fn status_FuraFuraStand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_furafura_stand_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_furafura_stand_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_FuraFuraStand_MainEv"]
    pub(super) fn status_FuraFuraStand_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_FuraFuraStandEv"]
    pub(super) fn status_end_FuraFuraStand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_FuraFuraEv"]
    pub(super) fn status_pre_FuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_FuraFuraEv"]
    pub(super) fn status_FuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_FuraFura_MainEv"]
    pub(super) fn status_FuraFura_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_FuraFuraEv"]
    pub(super) fn status_end_FuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_FuraFuraEndEv"]
    pub(super) fn status_pre_FuraFuraEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_FuraFuraEndEv"]
    pub(super) fn status_FuraFuraEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_FuraFuraEnd_MainEv"]
    pub(super) fn status_FuraFuraEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_FuraFuraEndEv"]
    pub(super) fn status_end_FuraFuraEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_ItemLightPickupEv"]
    pub(super) fn status_pre_ItemLightPickup(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_ItemLightPickupEv"]
    pub(super) fn status_ItemLightPickup(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_ItemLightPickup_MainEv"]
    pub(super) fn status_ItemLightPickup_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_ItemLightPickupEv"]
    pub(super) fn status_end_ItemLightPickup(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17sub_pre_ItemThrowEv"]
    pub(super) fn sub_pre_ItemThrow(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_ItemThrowEv"]
    pub(super) fn status_pre_ItemThrow(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28ItemThrowLightMotionDecisionEv"]
    pub(super) fn ItemThrowLightMotionDecision(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31ItemThrowLightMotionDecisionAirEv"]
    pub(super) fn ItemThrowLightMotionDecisionAir(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_ItemThrowCommonEv"]
    pub(super) fn status_ItemThrowCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_ItemThrow_LoopEv"]
    pub(super) fn status_ItemThrow_Loop(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_ItemThrowEv"]
    pub(super) fn status_ItemThrow(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_ItemThrow_MainEv"]
    pub(super) fn status_ItemThrow_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_item_throw_uniq_process_fix_posEv"]
    pub(super) fn sub_item_throw_uniq_process_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_ItemThrowEv"]
    pub(super) fn status_end_ItemThrow(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_ItemThrowDashEv"]
    pub(super) fn status_pre_ItemThrowDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_ItemThrowDashEv"]
    pub(super) fn status_ItemThrowDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17ItemThrowDashUniqEN3lib8L2CValueE"]
    pub(super) fn ItemThrowDashUniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_ItemThrowDash_MainEv"]
    pub(super) fn status_ItemThrowDash_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_ItemThrowDashEv"]
    pub(super) fn status_end_ItemThrowDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_ItemHeavyPickupEv"]
    pub(super) fn status_pre_ItemHeavyPickup(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_ItemHeavyPickupEv"]
    pub(super) fn status_ItemHeavyPickup(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_ItemHeavyPickup_MainEv"]
    pub(super) fn status_ItemHeavyPickup_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_ItemHeavyPickupEv"]
    pub(super) fn status_end_ItemHeavyPickup(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14ItemLiftCommonEv"]
    pub(super) fn ItemLiftCommon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_ItemLiftTurnEv"]
    pub(super) fn status_pre_ItemLiftTurn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_ItemLiftTurnEv"]
    pub(super) fn status_ItemLiftTurn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_ItemLiftTurn_MainEv"]
    pub(super) fn status_ItemLiftTurn_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_ItemLiftTurnEv"]
    pub(super) fn status_end_ItemLiftTurn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_ItemLiftWalkEv"]
    pub(super) fn status_pre_ItemLiftWalk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32FL_sub_get_item_walk_start_frameEv"]
    pub(super) fn FL_sub_get_item_walk_start_frame(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_ItemLiftWalkEv"]
    pub(super) fn status_ItemLiftWalk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_ItemLiftWalk_MainEv"]
    pub(super) fn status_ItemLiftWalk_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_ItemLiftWalkEv"]
    pub(super) fn status_end_ItemLiftWalk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_ItemLiftWaitEv"]
    pub(super) fn status_pre_ItemLiftWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_ItemLiftWaitEv"]
    pub(super) fn status_ItemLiftWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_ItemLiftWait_MainEv"]
    pub(super) fn status_ItemLiftWait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_ItemLiftWaitEv"]
    pub(super) fn status_end_ItemLiftWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20ItemThrowHeavyCommonEv"]
    pub(super) fn ItemThrowHeavyCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_ItemThrowHeavyEv"]
    pub(super) fn status_pre_ItemThrowHeavy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_ItemThrowHeavyEv"]
    pub(super) fn status_ItemThrowHeavy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_ItemThrowHeavy_MainEv"]
    pub(super) fn status_ItemThrowHeavy_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_ItemThrowHeavyEv"]
    pub(super) fn status_end_ItemThrowHeavy(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30item_lift_walk_set_motion_rateEv"]
    pub(super) fn item_lift_walk_set_motion_rate(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15init_move_speedEN3lib8L2CValueES2_S2_S2_S2_S2_"]
    pub(super) fn init_move_speed(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15set_speed_ratioEN3lib8L2CValueES2_S2_"]
    pub(super) fn set_speed_ratio(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_uniq_process_item_lift_initEv"]
    pub(super) fn sub_uniq_process_item_lift_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_uniq_process_item_lift_execEv"]
    pub(super) fn sub_uniq_process_item_lift_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15calc_walk_speedEN3lib8L2CValueES2_S2_S2_S2_S2_S2_S2_S2_"]
    pub(super) fn calc_walk_speed(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack, arg8: lib::L2CValueHack, arg9: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_uniq_process_item_lift_exitEv"]
    pub(super) fn sub_uniq_process_item_lift_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_ItemAssistHoistEv"]
    pub(super) fn status_pre_ItemAssistHoist(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_ItemAssistHoistEv"]
    pub(super) fn status_ItemAssistHoist(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_ItemAssistHoist_MainEv"]
    pub(super) fn status_ItemAssistHoist_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_ItemAssistHoistEv"]
    pub(super) fn status_end_ItemAssistHoist(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_WarpStarEv"]
    pub(super) fn status_pre_WarpStar(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_WarpStarEv"]
    pub(super) fn status_WarpStar(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_WarpStar_MainEv"]
    pub(super) fn status_WarpStar_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_WarpStarEv"]
    pub(super) fn status_end_WarpStar(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_WarpStarJumpEv"]
    pub(super) fn status_pre_WarpStarJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_WarpStarJumpEv"]
    pub(super) fn status_WarpStarJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_WarpStarJump_MainEv"]
    pub(super) fn status_WarpStarJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_WarpStarJumpEv"]
    pub(super) fn status_end_WarpStarJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_DragoonRideEv"]
    pub(super) fn status_pre_DragoonRide(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_DragoonRideEv"]
    pub(super) fn status_DragoonRide(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_DragoonRide_MainEv"]
    pub(super) fn status_DragoonRide_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_DragoonRide_Main_TermEv"]
    pub(super) fn status_DragoonRide_Main_Term(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_DragoonRideEv"]
    pub(super) fn status_end_DragoonRide(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43FighterStatusUniqProcessDragoon_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessDragoon_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43FighterStatusUniqProcessDragoon_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessDragoon_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_pre_KillerEv"]
    pub(super) fn status_pre_Killer(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13status_KillerEv"]
    pub(super) fn status_Killer(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_KillerMainEv"]
    pub(super) fn status_KillerMain(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_end_KillerEv"]
    pub(super) fn status_end_Killer(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusUniqProcessKiller_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessKiller_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusUniqProcessKiller_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessKiller_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusUniqProcessKiller_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessKiller_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusUniqProcessKiller_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessKiller_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_KillerJumpEv"]
    pub(super) fn status_pre_KillerJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_KillerJumpEv"]
    pub(super) fn status_KillerJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_KillerJump_MainEv"]
    pub(super) fn status_KillerJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_KillerJumpEv"]
    pub(super) fn status_end_KillerJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35uniq_process_KillerJump_init_statusEv"]
    pub(super) fn uniq_process_KillerJump_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_ItemRocketbeltHop_set_kineticEN3lib8L2CValueES2_"]
    pub(super) fn sub_ItemRocketbeltHop_set_kinetic(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35uniq_process_KillerJump_exec_statusEv"]
    pub(super) fn uniq_process_KillerJump_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33uniq_process_KillerJump_exec_stopEv"]
    pub(super) fn uniq_process_KillerJump_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35uniq_process_KillerJump_exit_statusEv"]
    pub(super) fn uniq_process_KillerJump_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15hammer_jump_chkEv"]
    pub(super) fn hammer_jump_chk(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12sub_pass_chkEv"]
    pub(super) fn sub_pass_chk(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12sub_set_passEv"]
    pub(super) fn sub_set_pass(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22hammer_ground_uniq_chkEv"]
    pub(super) fn hammer_ground_uniq_chk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17hammer_set_motionEv"]
    pub(super) fn hammer_set_motion(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15sub_hammer_exitEv"]
    pub(super) fn sub_hammer_exit(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_hammer_turn_exitEv"]
    pub(super) fn sub_hammer_turn_exit(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43FighterStatusUniqProcessHammer_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessHammer_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_HammerWaitEv"]
    pub(super) fn status_pre_HammerWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_HammerWaitEv"]
    pub(super) fn status_HammerWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_HammerWait_MainEv"]
    pub(super) fn status_HammerWait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_uniq_process_hammer_exec_fix_pos_mainEv"]
    pub(super) fn sub_uniq_process_hammer_exec_fix_pos_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_set_hammer_bgmEv"]
    pub(super) fn sub_set_hammer_bgm(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon44sub_uniq_process_hammer_exec_fix_pos_counterEv"]
    pub(super) fn sub_uniq_process_hammer_exec_fix_pos_counter(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_HammerWaitEv"]
    pub(super) fn status_end_HammerWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_HammerWalkEv"]
    pub(super) fn status_pre_HammerWalk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_HammerWalkEv"]
    pub(super) fn status_HammerWalk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_HammerWalk_MainEv"]
    pub(super) fn status_HammerWalk_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_HammerWalkEv"]
    pub(super) fn status_end_HammerWalk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_HammerTurnEv"]
    pub(super) fn status_pre_HammerTurn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_HammerTurnEv"]
    pub(super) fn status_HammerTurn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_HammerTurn_MainEv"]
    pub(super) fn status_HammerTurn_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26hammer_jump_squat_uniq_chkEv"]
    pub(super) fn hammer_jump_squat_uniq_chk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_HammerTurnEv"]
    pub(super) fn status_end_HammerTurn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_HammerJumpSquatEv"]
    pub(super) fn status_pre_HammerJumpSquat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_HammerJumpSquatEv"]
    pub(super) fn status_HammerJumpSquat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_HammerJumpSquat_MainEv"]
    pub(super) fn status_HammerJumpSquat_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_HammerJumpSquatEv"]
    pub(super) fn status_end_HammerJumpSquat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_HammerJumpEv"]
    pub(super) fn status_pre_HammerJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_HammerJumpEv"]
    pub(super) fn status_HammerJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_HammerJump_MainEv"]
    pub(super) fn status_HammerJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_HammerJumpEv"]
    pub(super) fn status_end_HammerJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_HammerFallEv"]
    pub(super) fn status_pre_HammerFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_HammerFallEv"]
    pub(super) fn status_HammerFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_HammerFall_MainEv"]
    pub(super) fn status_HammerFall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_HammerFallEv"]
    pub(super) fn status_end_HammerFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_HammerLandingEv"]
    pub(super) fn status_pre_HammerLanding(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_HammerLandingEv"]
    pub(super) fn status_HammerLanding(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_HammerLanding_MainEv"]
    pub(super) fn status_HammerLanding_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23hammer_landing_uniq_chkEN3lib8L2CValueE"]
    pub(super) fn hammer_landing_uniq_chk(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_HammerLandingEv"]
    pub(super) fn status_end_HammerLanding(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_hammer_init_statusEv"]
    pub(super) fn sub_uniq_process_hammer_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_hammer_exec_statusEv"]
    pub(super) fn sub_uniq_process_hammer_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_uniq_process_hammer_exec_fix_posEv"]
    pub(super) fn sub_uniq_process_hammer_exec_fix_pos(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_hammer_exit_statusEv"]
    pub(super) fn sub_uniq_process_hammer_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_uniq_process_hammer_check_damageEv"]
    pub(super) fn sub_uniq_process_hammer_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_big_small_common_mtransEv"]
    pub(super) fn sub_big_small_common_mtrans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_BigSmallCommonEv"]
    pub(super) fn sub_BigSmallCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_BigSmallCommon_MainEv"]
    pub(super) fn sub_BigSmallCommon_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_BigSmallExitCommonEv"]
    pub(super) fn sub_BigSmallExitCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_pre_BigEv"]
    pub(super) fn status_pre_Big(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon10status_BigEv"]
    pub(super) fn status_Big(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_Big_MainEv"]
    pub(super) fn status_Big_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_end_BigEv"]
    pub(super) fn status_end_Big(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_pre_SmallEv"]
    pub(super) fn status_pre_Small(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12status_SmallEv"]
    pub(super) fn status_Small(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_Small_MainEv"]
    pub(super) fn status_Small_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_end_SmallEv"]
    pub(super) fn status_end_Small(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusUniqProcessKinoko_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessKinoko_init_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27get_kinoko_scale_table_sizeEN3lib8L2CValueE"]
    pub(super) fn get_kinoko_scale_table_size(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusUniqProcessKinoko_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessKinoko_exec_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16get_kinoko_scaleEN3lib8L2CValueES2_"]
    pub(super) fn get_kinoko_scale(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusUniqProcessKinoko_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessKinoko_exit_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_ItemGrassPullEv"]
    pub(super) fn status_pre_ItemGrassPull(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_ItemGrassPullEv"]
    pub(super) fn status_ItemGrassPull(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_ItemGrassPull_MainEv"]
    pub(super) fn status_ItemGrassPull_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58FighterStatusUniqProcessItemGrassPull_exec_fix_pos_counterEv"]
    pub(super) fn FighterStatusUniqProcessItemGrassPull_exec_fix_pos_counter(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessItemGrassPull_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessItemGrassPull_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33local_func__fighter_status_item_3Ev"]
    pub(super) fn local_func__fighter_status_item_3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessItemGrassPull_get_itemEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessItemGrassPull_get_item(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessItemGrassPull_fall_itemEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn FighterStatusUniqProcessItemGrassPull_fall_item(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33local_func__fighter_status_item_6Ev"]
    pub(super) fn local_func__fighter_status_item_6(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_ItemGrassPullEv"]
    pub(super) fn status_end_ItemGrassPull(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessItemGrassPull_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessItemGrassPull_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon70FighterStatusUniqProcessItemGrassPull_update_grasspickup_kinetic_speedEv"]
    pub(super) fn FighterStatusUniqProcessItemGrassPull_update_grasspickup_kinetic_speed(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33local_func__fighter_status_item_4Ev"]
    pub(super) fn local_func__fighter_status_item_4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33local_func__fighter_status_item_5Ev"]
    pub(super) fn local_func__fighter_status_item_5(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessItemGrassPull_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessItemGrassPull_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessItemGrassPull_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessItemGrassPull_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_CaptureItemEv"]
    pub(super) fn status_pre_CaptureItem(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_CaptureItemEv"]
    pub(super) fn status_CaptureItem(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CaptureItem_MainEv"]
    pub(super) fn status_CaptureItem_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_CaptureItemEv"]
    pub(super) fn status_end_CaptureItem(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessCaptureItem_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureItem_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_CaptureBeetleEv"]
    pub(super) fn status_pre_CaptureBeetle(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_CaptureBeetleEv"]
    pub(super) fn status_CaptureBeetle(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CaptureBeetle_MainEv"]
    pub(super) fn status_CaptureBeetle_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_CaptureBeetleEv"]
    pub(super) fn status_end_CaptureBeetle(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessCaptureBeetle_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureBeetle_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_CaptureBossgalagaEv"]
    pub(super) fn status_pre_CaptureBossgalaga(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_CaptureBossgalagaEv"]
    pub(super) fn status_CaptureBossgalaga(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_CaptureBossgalaga_MainEv"]
    pub(super) fn status_CaptureBossgalaga_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_CaptureBossgalagaEv"]
    pub(super) fn status_end_CaptureBossgalaga(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessCaptureBossgalaga_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureBossgalaga_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_CaptureBeitcraneEv"]
    pub(super) fn status_pre_CaptureBeitcrane(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_CaptureBeitcraneEv"]
    pub(super) fn status_CaptureBeitcrane(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_CaptureBeitcrane_MainEv"]
    pub(super) fn status_CaptureBeitcrane_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_CaptureBeitcraneEv"]
    pub(super) fn status_end_CaptureBeitcrane(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessCaptureBeitcrane_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureBeitcrane_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_CaptureKawasakiEv"]
    pub(super) fn status_pre_CaptureKawasaki(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_CaptureKawasakiEv"]
    pub(super) fn status_CaptureKawasaki(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_CaptureKawasaki_MainEv"]
    pub(super) fn status_CaptureKawasaki_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessCaptureKawasaki_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureKawasaki_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_CaptureKawasakiEv"]
    pub(super) fn status_end_CaptureKawasaki(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessCaptureKawasaki_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureKawasaki_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_CaptureDriverEv"]
    pub(super) fn status_pre_CaptureDriver(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_CaptureDriverEv"]
    pub(super) fn status_CaptureDriver(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_CaptureDriver_MainEv"]
    pub(super) fn status_CaptureDriver_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessCaptureDriver_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessCaptureDriver_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessCaptureDriver_exec_fix_pos_slowEv"]
    pub(super) fn FighterStatusUniqProcessCaptureDriver_exec_fix_pos_slow(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_CaptureDriverEv"]
    pub(super) fn status_end_CaptureDriver(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessCaptureDriver_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureDriver_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_CaptureMimikkyuEv"]
    pub(super) fn status_pre_CaptureMimikkyu(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_CaptureMimikkyuEv"]
    pub(super) fn status_CaptureMimikkyu(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_CaptureMimikkyu_MainEv"]
    pub(super) fn status_CaptureMimikkyu_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_CaptureMimikkyuEv"]
    pub(super) fn status_end_CaptureMimikkyu(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessCaptureMimikkyu_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureMimikkyu_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessCaptureClaptrap_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessCaptureClaptrap_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_KaseyWarpEv"]
    pub(super) fn status_pre_KaseyWarp(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_KaseyWarpEv"]
    pub(super) fn status_KaseyWarp(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_KaseyWarp_MainEv"]
    pub(super) fn status_KaseyWarp_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_KaseyWarpEv"]
    pub(super) fn status_end_KaseyWarp(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessItemRocketbeltHover_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessItemRocketbeltHover_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33local_func__fighter_status_item_7Ev"]
    pub(super) fn local_func__fighter_status_item_7(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon71FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_fire_startEv"]
    pub(super) fn FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_fire_start(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon72FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_smoke_startEv"]
    pub(super) fn FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_smoke_start(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33local_func__fighter_status_item_8Ev"]
    pub(super) fn local_func__fighter_status_item_8(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessItemRocketbeltHover_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessItemRocketbeltHover_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33local_func__fighter_status_item_9Ev"]
    pub(super) fn local_func__fighter_status_item_9(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon61FighterStatusUniqProcessItemRocketbeltHover_button_push_checkEv"]
    pub(super) fn FighterStatusUniqProcessItemRocketbeltHover_button_push_check(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58FighterStatusUniqProcessItemRocketbeltHover_control_burnerEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessItemRocketbeltHover_control_burner(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34local_func__fighter_status_item_11Ev"]
    pub(super) fn local_func__fighter_status_item_11(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon70FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_smoke_endEv"]
    pub(super) fn FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_smoke_end(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon69FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_fire_endEv"]
    pub(super) fn FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_fire_end(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58FighterStatusUniqProcessItemRocketbeltHover_control_effectEv"]
    pub(super) fn FighterStatusUniqProcessItemRocketbeltHover_control_effect(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58FighterStatusUniqProcessItemRocketbeltHover_control_rumbleEv"]
    pub(super) fn FighterStatusUniqProcessItemRocketbeltHover_control_rumble(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessItemRocketbeltHover_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessItemRocketbeltHover_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34local_func__fighter_status_item_10Ev"]
    pub(super) fn local_func__fighter_status_item_10(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessItemRocketbeltHover_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessItemRocketbeltHover_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_ItemRocketbeltHoverEv"]
    pub(super) fn status_pre_ItemRocketbeltHover(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_ItemRocketbeltHoverEv"]
    pub(super) fn status_ItemRocketbeltHover(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_ItemRocketbeltHover_MainEv"]
    pub(super) fn status_ItemRocketbeltHover_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_ItemRocketbeltHoverEv"]
    pub(super) fn status_end_ItemRocketbeltHover(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_ItemRocketbeltHoverKeepEv"]
    pub(super) fn status_pre_ItemRocketbeltHoverKeep(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_ItemRocketbeltHoverKeepEv"]
    pub(super) fn status_ItemRocketbeltHoverKeep(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_ItemRocketbeltHoverKeep_MainEv"]
    pub(super) fn status_ItemRocketbeltHoverKeep_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_ItemRocketbeltHoverKeepEv"]
    pub(super) fn status_end_ItemRocketbeltHoverKeep(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_ItemRocketbeltHopEv"]
    pub(super) fn status_pre_ItemRocketbeltHop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_ItemRocketbeltHopEv"]
    pub(super) fn status_ItemRocketbeltHop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_ItemRocketbeltHop_MainEv"]
    pub(super) fn status_ItemRocketbeltHop_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_ItemRocketbeltHopEv"]
    pub(super) fn status_end_ItemRocketbeltHop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42uniq_process_ItemRocketbeltHop_init_statusEv"]
    pub(super) fn uniq_process_ItemRocketbeltHop_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_ItemSpecialflagHoistEv"]
    pub(super) fn status_pre_ItemSpecialflagHoist(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_ItemSpecialflagHoistEv"]
    pub(super) fn status_ItemSpecialflagHoist(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_item_specialflag_hoist_uniqEv"]
    pub(super) fn sub_item_specialflag_hoist_uniq(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_ItemSpecialflagHoist_MainEv"]
    pub(super) fn status_ItemSpecialflagHoist_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_ItemSpecialflagHoistEv"]
    pub(super) fn status_end_ItemSpecialflagHoist(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48sub_ftStatusUniqProcessShoot_isShootGroundStatusEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessShoot_isShootGroundStatus(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_ftStatusUniqProcessShoot_isShootAirStatusEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessShoot_isShootAirStatus(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_ftStatusUniqProcessShoot_initShoot_CommonEN3lib8L2CValueES2_"]
    pub(super) fn sub_ftStatusUniqProcessShoot_initShoot_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15init_walk_speedEN3lib8L2CValueES2_S2_S2_S2_S2_S2_"]
    pub(super) fn init_walk_speed(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16init_walk_motionEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn init_walk_motion(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20get_walk_motion_kindEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn get_walk_motion_kind(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23FL_get_walk_motion_rateEN3lib8L2CValueES2_S2_S2_S2_S2_"]
    pub(super) fn FL_get_walk_motion_rate(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_ftStatusUniqProcessShoot_execShoot_CommonEv"]
    pub(super) fn sub_ftStatusUniqProcessShoot_execShoot_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15set_walk_motionEN3lib8L2CValueES2_S2_S2_S2_S2_"]
    pub(super) fn set_walk_motion(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_ftStatusUniqProcessShoot_isShootStatusEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessShoot_isShootStatus(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_ftStatusUniqProcessShoot_exitShoot_CommonEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessShoot_exitShoot_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_ItemShootWait_CommonEN3lib8L2CValueE"]
    pub(super) fn sub_ItemShootWait_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ItemShootWait_Common_MainEv"]
    pub(super) fn sub_ItemShootWait_Common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_ItemShootWalkF_CommonEN3lib8L2CValueE"]
    pub(super) fn sub_ItemShootWalkF_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_ItemShootWalkF_Common_MainEv"]
    pub(super) fn sub_ItemShootWalkF_Common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_ItemShootWalkB_CommonEN3lib8L2CValueE"]
    pub(super) fn sub_ItemShootWalkB_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_ItemShootWalkB_Common_MainEv"]
    pub(super) fn sub_ItemShootWalkB_Common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_ItemShootWalkFBrake_CommonEv"]
    pub(super) fn sub_ItemShootWalkFBrake_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ItemShootWalkFBrake_Common_MainEv"]
    pub(super) fn sub_ItemShootWalkFBrake_Common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_ItemShootWalkBBrake_CommonEv"]
    pub(super) fn sub_ItemShootWalkBBrake_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ItemShootWalkBBrake_Common_MainEv"]
    pub(super) fn sub_ItemShootWalkBBrake_Common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_ItemShootDashF_CommonEv"]
    pub(super) fn sub_ItemShootDashF_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_ItemShootDashF_Common_MainEv"]
    pub(super) fn sub_ItemShootDashF_Common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_ItemShootDashB_CommonEv"]
    pub(super) fn sub_ItemShootDashB_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_ItemShootDashB_Common_MainEv"]
    pub(super) fn sub_ItemShootDashB_Common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_ItemShootAir_CommonEv"]
    pub(super) fn sub_ItemShootAir_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_ItemShootAir_Common_MainEv"]
    pub(super) fn sub_ItemShootAir_Common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_item_shoot_jump_squat_uniq_checkEv"]
    pub(super) fn sub_item_shoot_jump_squat_uniq_check(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_jump_squat_uniq_check_subEN3lib8L2CValueE"]
    pub(super) fn sub_jump_squat_uniq_check_sub(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ItemShootJumpSquat_CommonEv"]
    pub(super) fn sub_ItemShootJumpSquat_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootJumpSquat_Common_MainEv"]
    pub(super) fn sub_ItemShootJumpSquat_Common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_ItemShootLanding_CommonEN3lib8L2CValueE"]
    pub(super) fn sub_ItemShootLanding_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_ItemShootLanding_Common_MainEv"]
    pub(super) fn sub_ItemShootLanding_Common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50sub_ftStatusUniqProcessItemShoot_execFixPos_CommonEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPos_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_ItemShootJump_enable_aerialEv"]
    pub(super) fn sub_ItemShootJump_enable_aerial(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_ItemShootJumpAerial_CommonEv"]
    pub(super) fn sub_ItemShootJumpAerial_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_ItemShootJumpCommon_MainEv"]
    pub(super) fn sub_ItemShootJumpCommon_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_ItemShootAir_beforeEv"]
    pub(super) fn status_ItemShootAir_before(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33get_jump_aerial_motion_common_NewEN3lib8L2CValueES2_S2_S2_S2_S2_S2_"]
    pub(super) fn get_jump_aerial_motion_common_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_ItemShootCommon_Main_NewEv"]
    pub(super) fn sub_ItemShootCommon_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ItemShootJump_enable_aerial_NewEv"]
    pub(super) fn sub_ItemShootJump_enable_aerial_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_ItemShootJumpCommon_Main_NewEv"]
    pub(super) fn sub_ItemShootJumpCommon_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_pre_ItemShootWait_common_NewEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_pre_ItemShootWait_common_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_ItemShootWait_NewEv"]
    pub(super) fn status_pre_ItemShootWait_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32FT_UTIL_IS_ITEM_SHOOT_STATUS_NEWEN3lib8L2CValueE"]
    pub(super) fn FT_UTIL_IS_ITEM_SHOOT_STATUS_NEW(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_ItemShootWait_NewEv"]
    pub(super) fn status_ItemShootWait_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_ItemShootWait_LGun_NewEv"]
    pub(super) fn sub_ItemShootWait_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ItemShootWait_FFlower_NewEv"]
    pub(super) fn sub_ItemShootWait_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_ItemShootWait_SScope_NewEv"]
    pub(super) fn sub_ItemShootWait_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_ItemShootWait_GenesisSet_NewEv"]
    pub(super) fn sub_ItemShootWait_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_ItemShootWait_MagicPot_NewEv"]
    pub(super) fn sub_ItemShootWait_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_ItemShootWait_Main_NewEv"]
    pub(super) fn status_ItemShootWait_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_ItemShootWait_LGun_Main_NewEv"]
    pub(super) fn sub_ItemShootWait_LGun_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootWait_FFlower_Main_NewEv"]
    pub(super) fn sub_ItemShootWait_FFlower_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_ItemShootWait_SScope_Main_NewEv"]
    pub(super) fn sub_ItemShootWait_SScope_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_ItemShootWait_GenesisSet_Main_NewEv"]
    pub(super) fn sub_ItemShootWait_GenesisSet_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ItemShootWait_MagicPot_Main_NewEv"]
    pub(super) fn sub_ItemShootWait_MagicPot_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54sub_ftStatusUniqProcessItemShoot_execFixPosCounter_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPosCounter_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47sub_ftStatusUniqProcessItemShoot_execFixPos_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPos_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54sub_ftStatusUniqProcessItemShoot_execFixPos_Common_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPos_Common_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon61sub_ftStatusUniqProcessItemShoot_execFixPosCounter_SScope_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPosCounter_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon62sub_ftStatusUniqProcessItemShoot_execFixPosCounter_FFlower_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPosCounter_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon63sub_ftStatusUniqProcessItemShoot_execFixPosCounter_MagicPot_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPosCounter_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57sub_ftStatusUniqProcessItemShoot_isMotionEnd_MagicPot_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_isMotionEnd_MagicPot_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52sub_ftStatusUniqProcessItemShoot_toHold_MagicPot_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_toHold_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51sub_ftStatusUniqProcessItemShoot_toEnd_MagicPot_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_toEnd_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50sub_ftStatusUniqProcessItemShoot_toEnd_FFlower_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_toEnd_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49sub_ftStatusUniqProcessShoot_isShootAirStatus_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessShoot_isShootAirStatus_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55sub_ftStatusUniqProcessItemShoot_isMotionEnd_SScope_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_isMotionEnd_SScope_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51sub_ftStatusUniqProcessItemShoot_toRapid_SScope_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_toRapid_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50sub_ftStatusUniqProcessItemShoot_toHold_SScope_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_toHold_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49sub_ftStatusUniqProcessItemShoot_toEnd_SScope_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_toEnd_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54sub_ftStatusUniqProcessItemShoot_loopMotion_SScope_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_loopMotion_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54sub_ftStatusUniqProcessItemShoot_toHoldFire_SScope_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_toHoldFire_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_ItemShootWait_Common_Main_NewEv"]
    pub(super) fn sub_ItemShootWait_Common_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_ItemShootWait_Common_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ItemShootWait_Common_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_ItemShootWait_NewEv"]
    pub(super) fn status_end_ItemShootWait_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_ItemShootWalkF_NewEv"]
    pub(super) fn status_pre_ItemShootWalkF_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_ItemShootWalkF_NewEv"]
    pub(super) fn status_ItemShootWalkF_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_ItemShootWalkF_LGun_NewEv"]
    pub(super) fn sub_ItemShootWalkF_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_ItemShootWalkF_FFlower_NewEv"]
    pub(super) fn sub_ItemShootWalkF_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ItemShootWalkF_SScope_NewEv"]
    pub(super) fn sub_ItemShootWalkF_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_ItemShootWalkF_GenesisSet_NewEv"]
    pub(super) fn sub_ItemShootWalkF_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_ItemShootWalkF_MagicPot_NewEv"]
    pub(super) fn sub_ItemShootWalkF_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_ItemShootWalkF_Main_NewEv"]
    pub(super) fn status_ItemShootWalkF_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_ItemShootWalkF_LGun_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkF_LGun_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ItemShootWalkF_FFlower_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkF_FFlower_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootWalkF_SScope_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkF_SScope_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_ItemShootWalkF_GenesisSet_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkF_GenesisSet_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_ItemShootWalkF_MagicPot_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkF_MagicPot_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootWalkF_Common_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkF_Common_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ItemShootWalkF_Common_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ItemShootWalkF_Common_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_ItemShootWalkF_NewEv"]
    pub(super) fn status_end_ItemShootWalkF_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_ItemShootWalkFBrake_NewEv"]
    pub(super) fn status_pre_ItemShootWalkFBrake_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_ItemShootWalkFBrake_NewEv"]
    pub(super) fn status_ItemShootWalkFBrake_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootWalkFBrake_Common_NewEv"]
    pub(super) fn sub_ItemShootWalkFBrake_Common_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_ItemShootWalkFBrake_Main_NewEv"]
    pub(super) fn status_ItemShootWalkFBrake_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_ItemShootWalkFBrake_Common_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkFBrake_Common_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_ItemShootWalkBrakeF_LGun_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeF_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ItemShootWalkBrakeF_FFlower_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeF_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootWalkBrakeF_SScope_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeF_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_ItemShootWalkBrakeF_GenesisSet_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeF_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_ItemShootWalkBrakeF_MagicPot_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeF_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_ItemShootWalkBrakeF_LGun_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeF_LGun_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_ItemShootWalkBrakeF_FFlower_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeF_FFlower_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_ItemShootWalkBrakeF_SScope_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeF_SScope_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_ItemShootWalkBrakeF_GenesisSet_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeF_GenesisSet_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_ItemShootWalkBrakeF_MagicPot_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeF_MagicPot_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_ItemShootWalkFBrake_NewEv"]
    pub(super) fn status_end_ItemShootWalkFBrake_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_ItemShootWalkB_NewEv"]
    pub(super) fn status_pre_ItemShootWalkB_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_ItemShootWalkB_NewEv"]
    pub(super) fn status_ItemShootWalkB_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_ItemShootWalkB_LGun_NewEv"]
    pub(super) fn sub_ItemShootWalkB_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_ItemShootWalkB_FFlower_NewEv"]
    pub(super) fn sub_ItemShootWalkB_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ItemShootWalkB_SScope_NewEv"]
    pub(super) fn sub_ItemShootWalkB_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_ItemShootWalkB_GenesisSet_NewEv"]
    pub(super) fn sub_ItemShootWalkB_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_ItemShootWalkB_MagicPot_NewEv"]
    pub(super) fn sub_ItemShootWalkB_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_ItemShootWalkB_Main_NewEv"]
    pub(super) fn status_ItemShootWalkB_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_ItemShootWalkB_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkB_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_ItemShootWalkB_LGun_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkB_LGun_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ItemShootWalkB_FFlower_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkB_FFlower_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootWalkB_SScope_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkB_SScope_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_ItemShootWalkB_GenesisSet_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkB_GenesisSet_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_ItemShootWalkB_MagicPot_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkB_MagicPot_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootWalkB_Common_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkB_Common_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ItemShootWalkB_Common_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ItemShootWalkB_Common_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_ItemShootWalkB_NewEv"]
    pub(super) fn status_end_ItemShootWalkB_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_ItemShootWalkBBrake_NewEv"]
    pub(super) fn status_pre_ItemShootWalkBBrake_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_ItemShootWalkBBrake_NewEv"]
    pub(super) fn status_ItemShootWalkBBrake_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootWalkBBrake_Common_NewEv"]
    pub(super) fn sub_ItemShootWalkBBrake_Common_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_ItemShootWalkBBrake_Main_NewEv"]
    pub(super) fn status_ItemShootWalkBBrake_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_ItemShootWalkBBrake_Common_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkBBrake_Common_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_ItemShootWalkBrakeB_LGun_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeB_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ItemShootWalkBrakeB_FFlower_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeB_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootWalkBrakeB_SScope_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeB_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_ItemShootWalkBrakeB_GenesisSet_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeB_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_ItemShootWalkBrakeB_MagicPot_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeB_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_ItemShootWalkBrakeB_LGun_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeB_LGun_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_ItemShootWalkBrakeB_FFlower_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeB_FFlower_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_ItemShootWalkBrakeB_SScope_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeB_SScope_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_ItemShootWalkBrakeB_MagicPot_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeB_MagicPot_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_ItemShootWalkBBrake_NewEv"]
    pub(super) fn status_end_ItemShootWalkBBrake_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_ItemShootAir_NewEv"]
    pub(super) fn status_pre_ItemShootAir_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_ItemShootAir_before_NewEv"]
    pub(super) fn status_ItemShootAir_before_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_ItemShootAir_after_NewEv"]
    pub(super) fn status_ItemShootAir_after_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_ItemShootAir_SScope_NewEv"]
    pub(super) fn sub_ItemShootAir_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ItemShootAir_MagicPot_NewEv"]
    pub(super) fn sub_ItemShootAir_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_ItemShootAir_LGun_NewEv"]
    pub(super) fn sub_ItemShootAir_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_ItemShootAir_FFlower_NewEv"]
    pub(super) fn sub_ItemShootAir_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_ItemShootAir_GenesisSet_NewEv"]
    pub(super) fn sub_ItemShootAir_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_ItemShootAirCommon_NewEv"]
    pub(super) fn status_ItemShootAirCommon_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_ItemShootAir_NewEv"]
    pub(super) fn status_ItemShootAir_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_ItemShootAir_Main_NewEv"]
    pub(super) fn status_ItemShootAir_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_ItemShootAir_LGun_Main_NewEv"]
    pub(super) fn sub_ItemShootAir_LGun_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_ItemShootAir_FFlower_Main_NewEv"]
    pub(super) fn sub_ItemShootAir_FFlower_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_ItemShootAir_SScope_Main_NewEv"]
    pub(super) fn sub_ItemShootAir_SScope_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_ItemShootAir_GenesisSet_Main_NewEv"]
    pub(super) fn sub_ItemShootAir_GenesisSet_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootAir_MagicPot_Main_NewEv"]
    pub(super) fn sub_ItemShootAir_MagicPot_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_ItemShootAir_Common_Main_NewEv"]
    pub(super) fn sub_ItemShootAir_Common_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_ItemShootAir_NewEv"]
    pub(super) fn status_end_ItemShootAir_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_ItemShootJumpSquat_NewEv"]
    pub(super) fn status_pre_ItemShootJumpSquat_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_ItemShootJumpSquat_NewEv"]
    pub(super) fn status_ItemShootJumpSquat_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_ItemShootJumpSquat_Common_NewEv"]
    pub(super) fn sub_ItemShootJumpSquat_Common_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_ItemShootJumpSquat_Main_NewEv"]
    pub(super) fn status_ItemShootJumpSquat_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_ItemShootJumpSquat_Common_Main_NewEv"]
    pub(super) fn sub_ItemShootJumpSquat_Common_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_ItemShootJumpSquat_LGun_NewEv"]
    pub(super) fn sub_ItemShootJumpSquat_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootJumpSquat_FFlower_NewEv"]
    pub(super) fn sub_ItemShootJumpSquat_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_ItemShootJumpSquat_SScope_NewEv"]
    pub(super) fn sub_ItemShootJumpSquat_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_ItemShootJumpSquat_GenesisSet_NewEv"]
    pub(super) fn sub_ItemShootJumpSquat_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ItemShootJumpSquat_MagicPot_NewEv"]
    pub(super) fn sub_ItemShootJumpSquat_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_item_shoot_jump_squat_uniq_check_NewEv"]
    pub(super) fn sub_item_shoot_jump_squat_uniq_check_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_ItemShootJumpSquat_LGun_Main_NewEv"]
    pub(super) fn sub_ItemShootJumpSquat_LGun_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_ItemShootJumpSquat_FFlower_Main_NewEv"]
    pub(super) fn sub_ItemShootJumpSquat_FFlower_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_ItemShootJumpSquat_SScope_Main_NewEv"]
    pub(super) fn sub_ItemShootJumpSquat_SScope_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_ItemShootJumpSquat_GenesisSet_Main_NewEv"]
    pub(super) fn sub_ItemShootJumpSquat_GenesisSet_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_ItemShootJumpSquat_MagicPot_Main_NewEv"]
    pub(super) fn sub_ItemShootJumpSquat_MagicPot_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_ItemShootJumpSquat_NewEv"]
    pub(super) fn status_end_ItemShootJumpSquat_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_ItemShootJump_NewEv"]
    pub(super) fn status_pre_ItemShootJump_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_ItemShootJump_NewEv"]
    pub(super) fn status_ItemShootJump_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_ItemShootJump_SScope_NewEv"]
    pub(super) fn sub_ItemShootJump_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_ItemShootJump_GenesisSet_NewEv"]
    pub(super) fn sub_ItemShootJump_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_ItemShootJump_MagicPot_NewEv"]
    pub(super) fn sub_ItemShootJump_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_ItemShootJump_Main_NewEv"]
    pub(super) fn status_ItemShootJump_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_ItemShootJump_LGun_Main_NewEv"]
    pub(super) fn sub_ItemShootJump_LGun_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootJump_FFlower_Main_NewEv"]
    pub(super) fn sub_ItemShootJump_FFlower_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_ItemShootJump_SScope_Main_NewEv"]
    pub(super) fn sub_ItemShootJump_SScope_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_ItemShootJump_GenesisSet_Main_NewEv"]
    pub(super) fn sub_ItemShootJump_GenesisSet_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ItemShootJump_MagicPot_Main_NewEv"]
    pub(super) fn sub_ItemShootJump_MagicPot_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_ItemShootJump_LGun_NewEv"]
    pub(super) fn sub_ItemShootJump_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ItemShootJump_FFlower_NewEv"]
    pub(super) fn sub_ItemShootJump_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_ItemShootJump_NewEv"]
    pub(super) fn status_end_ItemShootJump_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_ItemShootJumpAerial_NewEv"]
    pub(super) fn status_pre_ItemShootJumpAerial_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_ItemShootJumpAerial_NewEv"]
    pub(super) fn status_ItemShootJumpAerial_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_ItemShootJumpAerial_NewSubEv"]
    pub(super) fn status_ItemShootJumpAerial_NewSub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_ItemShootJumpAerial_Main_NewEv"]
    pub(super) fn status_ItemShootJumpAerial_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootJumpAerial_Common_NewEv"]
    pub(super) fn sub_ItemShootJumpAerial_Common_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootJumpAerial_SScope_NewEv"]
    pub(super) fn sub_ItemShootJumpAerial_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_ItemShootJumpAerial_GenesisSet_NewEv"]
    pub(super) fn sub_ItemShootJumpAerial_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_ItemShootJumpAerial_MagicPot_NewEv"]
    pub(super) fn sub_ItemShootJumpAerial_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ItemShootJumpAerial_FFlower_NewEv"]
    pub(super) fn sub_ItemShootJumpAerial_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_ItemShootJumpAerial_NewEv"]
    pub(super) fn status_end_ItemShootJumpAerial_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_ItemShootFly_NewEv"]
    pub(super) fn status_pre_ItemShootFly_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_ItemShootFly_NewEv"]
    pub(super) fn status_ItemShootFly_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_ItemShootFly_Main_NewEv"]
    pub(super) fn status_ItemShootFly_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_ItemShootFly_NewEv"]
    pub(super) fn status_end_ItemShootFly_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_ItemShootLanding_NewEv"]
    pub(super) fn status_pre_ItemShootLanding_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_ItemShootLanding_NewEv"]
    pub(super) fn status_ItemShootLanding_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_ItemShootLanding_Common_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ItemShootLanding_Common_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_ItemShootLanding_Main_NewEv"]
    pub(super) fn status_ItemShootLanding_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_ItemShootLanding_Common_Main_NewEv"]
    pub(super) fn sub_ItemShootLanding_Common_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_ItemShootLanding_GenesisSet_Main_NewEv"]
    pub(super) fn sub_ItemShootLanding_GenesisSet_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ItemShootLanding_LGun_NewEv"]
    pub(super) fn sub_ItemShootLanding_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_ItemShootLanding_FFlower_NewEv"]
    pub(super) fn sub_ItemShootLanding_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_ItemShootLanding_SScope_NewEv"]
    pub(super) fn sub_ItemShootLanding_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ItemShootLanding_GenesisSet_NewEv"]
    pub(super) fn sub_ItemShootLanding_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_ItemShootLanding_MagicPot_NewEv"]
    pub(super) fn sub_ItemShootLanding_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootLanding_LGun_Main_NewEv"]
    pub(super) fn sub_ItemShootLanding_LGun_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_ItemShootLanding_FFlower_Main_NewEv"]
    pub(super) fn sub_ItemShootLanding_FFlower_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_ItemShootLanding_SScope_Main_NewEv"]
    pub(super) fn sub_ItemShootLanding_SScope_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_ItemShootLanding_MagicPot_Main_NewEv"]
    pub(super) fn sub_ItemShootLanding_MagicPot_Main_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_ItemShootLanding_NewEv"]
    pub(super) fn status_end_ItemShootLanding_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ItemShootDashF_Common_NewEv"]
    pub(super) fn sub_ItemShootDashF_Common_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootDashF_Common_Main_NewEv"]
    pub(super) fn sub_ItemShootDashF_Common_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ItemShootDashB_Common_NewEv"]
    pub(super) fn sub_ItemShootDashB_Common_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_ItemShootDashB_Common_Main_NewEv"]
    pub(super) fn sub_ItemShootDashB_Common_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_ItemShootAir_Common_NewEv"]
    pub(super) fn sub_ItemShootAir_Common_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43sub_ItemShootWalkBrakeB_GenesisSet_Main_NewEv"]
    pub(super) fn sub_ItemShootWalkBrakeB_GenesisSet_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_ItemShootTurn_GenesisSet_NewEv"]
    pub(super) fn sub_ItemShootTurn_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_ItemShootTurn_Common_NewEv"]
    pub(super) fn sub_ItemShootTurn_Common_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_ItemShootTurn_GenesisSet_Main_NewEv"]
    pub(super) fn sub_ItemShootTurn_GenesisSet_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_ItemShootTurn_Common_Main_NewEv"]
    pub(super) fn sub_ItemShootTurn_Common_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_GenesisGet_NewEv"]
    pub(super) fn status_pre_GenesisGet_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_GenesisGet_NewEv"]
    pub(super) fn status_GenesisGet_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_genesis_get_mtrans_NewEv"]
    pub(super) fn sub_genesis_get_mtrans_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_GenesisGet_Main_NewEv"]
    pub(super) fn status_GenesisGet_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_genesis_get_uniq_NewEN3lib8L2CValueE"]
    pub(super) fn sub_genesis_get_uniq_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_GenesisGet_NewEv"]
    pub(super) fn status_end_GenesisGet_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_GenesisShootStart_NewEv"]
    pub(super) fn status_pre_GenesisShootStart_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_GenesisShootStart_NewEv"]
    pub(super) fn status_GenesisShootStart_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_GenesisShootStart_Main_NewEv"]
    pub(super) fn status_GenesisShootStart_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_genesis_shoot_mtrans_NewEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_genesis_shoot_mtrans_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_GenesisShootUniq_NewEN3lib8L2CValueE"]
    pub(super) fn sub_GenesisShootUniq_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_GenesisShootStart_NewEv"]
    pub(super) fn status_end_GenesisShootStart_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_GenesisShoot_NewEv"]
    pub(super) fn status_pre_GenesisShoot_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_GenesisShoot_NewEv"]
    pub(super) fn status_GenesisShoot_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_GenesisShoot_Main_NewEv"]
    pub(super) fn status_GenesisShoot_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_GenesisShoot_NewEv"]
    pub(super) fn status_end_GenesisShoot_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_GenesisShootEnd_NewEv"]
    pub(super) fn status_pre_GenesisShootEnd_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_GenesisShootEnd_NewEv"]
    pub(super) fn status_GenesisShootEnd_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_GenesisShootEnd_Main_NewEv"]
    pub(super) fn status_GenesisShootEnd_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_GenesisShootEnd_NewEv"]
    pub(super) fn status_end_GenesisShootEnd_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46sub_ftStatusUniqProcessShoot_isShootStatus_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessShoot_isShootStatus_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52sub_ftStatusUniqProcessShoot_isShootGroundStatus_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessShoot_isShootGroundStatus_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35item_shoot_walk_set_motion_rate_NewEN3lib8L2CValueE"]
    pub(super) fn item_shoot_walk_set_motion_rate_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49sub_ftStatusUniqProcessShoot_initShoot_Common_NewEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_ftStatusUniqProcessShoot_initShoot_Common_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57sub_ftStatusUniqProcessShoot_initShoot_CommonAirUpper_NewEN3lib8L2CValueES2_"]
    pub(super) fn sub_ftStatusUniqProcessShoot_initShoot_CommonAirUpper_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49sub_ftStatusUniqProcessShoot_execShoot_Common_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessShoot_execShoot_Common_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49sub_ftStatusUniqProcessShoot_exitShoot_Common_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessShoot_exitShoot_Common_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54sub_ftStatusUniqProcessItemShoot_initStatus_common_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_initStatus_common_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52sub_ftStatusUniqProcessItemShoot_initStatus_LGun_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_initStatus_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54sub_ftStatusUniqProcessItemShoot_initStatus_SScope_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_initStatus_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55sub_ftStatusUniqProcessItemShoot_initStatus_FFlower_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_initStatus_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58sub_ftStatusUniqProcessItemShoot_initStatus_GenesisSet_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_initStatus_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53sub_ftStatusUniqProcessItemShoot_initStatus_Drill_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_initStatus_Drill_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56sub_ftStatusUniqProcessItemShoot_initStatus_MagicPot_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_initStatus_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58sub_ftStatusUniqProcessItemShoot_initStatus_SteelDiver_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_initStatus_SteelDiver_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53sub_ftStatusUniqProcessItemShoot_initStatus_Staff_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_initStatus_Staff_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon62sub_ftStatusUniqProcessItemShoot_initStatus_Revengeshooter_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_initStatus_Revengeshooter_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51sub_ftStatusUniqProcessItemShoot_detachHoldAnim_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_detachHoldAnim_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon60sub_ftStatusUniqProcessItemShoot_setup_status_kind_table_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_setup_status_kind_table_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47sub_ftStatusUniqProcessItemShoot_initStatus_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_initStatus_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51sub_ftStatusUniqProcessItemShoot_attachHoldAnim_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_attachHoldAnim_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47sub_ftStatusUniqProcessItemShoot_execStatus_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execStatus_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54sub_ftStatusUniqProcessItemShoot_execStatus_SScope_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execStatus_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55sub_ftStatusUniqProcessItemShoot_execStatus_FFlower_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execStatus_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56sub_ftStatusUniqProcessItemShoot_execStatus_MagicPot_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execStatus_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53sub_ftStatusUniqProcessItemShoot_execStatus_Staff_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execStatus_Staff_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon62sub_ftStatusUniqProcessItemShoot_execStatus_Revengeshooter_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execStatus_Revengeshooter_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51sub_ftStatusUniqProcessItemShoot_toHold_FFlower_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_toHold_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52sub_ftStatusUniqProcessItemShoot_execStatus_LGun_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execStatus_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58sub_ftStatusUniqProcessItemShoot_execStatus_GenesisSet_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execStatus_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon59sub_ftStatusUniqProcessItemShoot_execFixPosCounter_LGun_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPosCounter_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon65sub_ftStatusUniqProcessItemShoot_execFixPosCounter_GenesisSet_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPosCounter_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52sub_ftStatusUniqProcessItemShoot_execFixPos_LGun_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPos_LGun_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54sub_ftStatusUniqProcessItemShoot_execFixPos_SScope_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPos_SScope_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55sub_ftStatusUniqProcessItemShoot_execFixPos_FFlower_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPos_FFlower_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58sub_ftStatusUniqProcessItemShoot_execFixPos_GenesisSet_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPos_GenesisSet_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56sub_ftStatusUniqProcessItemShoot_execFixPos_MagicPot_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_execFixPos_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47sub_ftStatusUniqProcessItemShoot_exitStatus_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_exitStatus_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46sub_ftStatusUniqProcessItemShoot_exit_LGun_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_exit_LGun_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48sub_ftStatusUniqProcessItemShoot_exit_SScope_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_exit_SScope_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49sub_ftStatusUniqProcessItemShoot_exit_FFlower_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_exit_FFlower_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50sub_ftStatusUniqProcessItemShoot_exit_MagicPot_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_exit_MagicPot_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52sub_ftStatusUniqProcessItemShoot_exit_GenesisSet_NewEN3lib8L2CValueE"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_exit_GenesisSet_New(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56sub_ftStatusUniqProcessItemShoot_loopMotion_MagicPot_NewEv"]
    pub(super) fn sub_ftStatusUniqProcessItemShoot_loopMotion_MagicPot_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessItemShootWalkF_calc_param_NewEv"]
    pub(super) fn FighterStatusUniqProcessItemShootWalkF_calc_param_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessItemShootWalkB_calc_param_NewEv"]
    pub(super) fn FighterStatusUniqProcessItemShootWalkB_calc_param_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_ItemShootTurn_NewEv"]
    pub(super) fn status_pre_ItemShootTurn_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_status_ItemShootTurn_NewEv"]
    pub(super) fn sub_status_ItemShootTurn_New(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_ItemShootTurn_NewEv"]
    pub(super) fn status_ItemShootTurn_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_ItemShootTurn_Main_NewEv"]
    pub(super) fn status_ItemShootTurn_Main_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_ItemShootTurn_NewEv"]
    pub(super) fn status_end_ItemShootTurn_New(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_ItemStarringEv"]
    pub(super) fn status_pre_ItemStarring(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_ItemStarringEv"]
    pub(super) fn status_ItemStarring(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42local_func__fighter_status_item_starring_1Ev"]
    pub(super) fn local_func__fighter_status_item_starring_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_ItemStarring_MainEv"]
    pub(super) fn status_ItemStarring_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42local_func__fighter_status_item_starring_2Ev"]
    pub(super) fn local_func__fighter_status_item_starring_2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42local_func__fighter_status_item_starring_3Ev"]
    pub(super) fn local_func__fighter_status_item_starring_3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42local_func__fighter_status_item_starring_4Ev"]
    pub(super) fn local_func__fighter_status_item_starring_4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_ItemStarringEv"]
    pub(super) fn status_end_ItemStarring(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_ItemStarringShootEv"]
    pub(super) fn status_pre_ItemStarringShoot(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_ItemStarringShootEv"]
    pub(super) fn status_ItemStarringShoot(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_ItemStarringShoot_MainEv"]
    pub(super) fn status_ItemStarringShoot_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_ItemStarringShootEv"]
    pub(super) fn status_end_ItemStarringShoot(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessItemStarringShoot_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessItemStarringShoot_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessItemStarringShoot_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessItemStarringShoot_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42local_func__fighter_status_item_starring_5Ev"]
    pub(super) fn local_func__fighter_status_item_starring_5(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessItemStarringShoot_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessItemStarringShoot_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessItemStarringShoot_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessItemStarringShoot_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40local_func__fighter_status_item_swing_11EN3lib8L2CValueES2_S2_S2_S2_S2_S2_S2_S2_"]
    pub(super) fn local_func__fighter_status_item_swing_11(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack, arg8: lib::L2CValueHack, arg9: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25get_swing_log_attack_kindEN3lib8L2CValueE"]
    pub(super) fn get_swing_log_attack_kind(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40local_func__fighter_status_item_swing_12Ev"]
    pub(super) fn local_func__fighter_status_item_swing_12(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40local_func__fighter_status_item_swing_13Ev"]
    pub(super) fn local_func__fighter_status_item_swing_13(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_ItemSwingEv"]
    pub(super) fn status_pre_ItemSwing(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17item_swing_motionEv"]
    pub(super) fn item_swing_motion(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22item_swing_motion_rateEv"]
    pub(super) fn item_swing_motion_rate(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_item_swing_uniq_checkEN3lib8L2CValueE"]
    pub(super) fn sub_item_swing_uniq_check(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_ItemSwingEv"]
    pub(super) fn status_ItemSwing(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_ItemSwing_MainEv"]
    pub(super) fn status_ItemSwing_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15ItemSwingCommonEv"]
    pub(super) fn ItemSwingCommon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessItemSwing_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessItemSwing_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon71FighterStatusUniqProcessItemSwing_update_attack_num_by_firebar_ball_numEv"]
    pub(super) fn FighterStatusUniqProcessItemSwing_update_attack_num_by_firebar_ball_num(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40local_func__fighter_status_item_swing_14Ev"]
    pub(super) fn local_func__fighter_status_item_swing_14(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_ItemSwingEv"]
    pub(super) fn status_end_ItemSwing(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_ItemSwingS3Ev"]
    pub(super) fn status_pre_ItemSwingS3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18item_swing3_motionEv"]
    pub(super) fn item_swing3_motion(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_ItemSwingS3Ev"]
    pub(super) fn status_ItemSwingS3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_ItemSwingS3_MainEv"]
    pub(super) fn status_ItemSwingS3_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_ItemSwingS3Ev"]
    pub(super) fn status_end_ItemSwingS3(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_ItemSwingS4StartEv"]
    pub(super) fn status_pre_ItemSwingS4Start(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18item_swing_motion4Ev"]
    pub(super) fn item_swing_motion4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_ItemSwingS4StartEv"]
    pub(super) fn status_ItemSwingS4Start(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_ItemSwingS4Start_MainEv"]
    pub(super) fn status_ItemSwingS4Start_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_item_swing_smash_start_uniqEv"]
    pub(super) fn sub_item_swing_smash_start_uniq(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_ItemSwingS4StartEv"]
    pub(super) fn status_end_ItemSwingS4Start(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_ItemSwingS4HoldEv"]
    pub(super) fn status_pre_ItemSwingS4Hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_ItemSwingS4Hold_CommonEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_pre_ItemSwingS4Hold_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_ItemSwingS4HoldEv"]
    pub(super) fn status_ItemSwingS4Hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_item_swing_smash_hold_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_item_swing_smash_hold_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_ItemSwingS4Hold_MainEv"]
    pub(super) fn status_ItemSwingS4Hold_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_ItemSwingS4HoldEv"]
    pub(super) fn status_end_ItemSwingS4Hold(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_ItemSwingS4Ev"]
    pub(super) fn status_pre_ItemSwingS4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_ItemSwingS4Ev"]
    pub(super) fn status_ItemSwingS4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15ItemSwingS4UniqEv"]
    pub(super) fn ItemSwingS4Uniq(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_ItemSwingS4_MainEv"]
    pub(super) fn status_ItemSwingS4_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_ItemSwingS4Ev"]
    pub(super) fn status_end_ItemSwingS4(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_ItemSwingDashEv"]
    pub(super) fn status_pre_ItemSwingDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22item_swing_dash_motionEv"]
    pub(super) fn item_swing_dash_motion(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_ItemSwingDashEv"]
    pub(super) fn status_ItemSwingDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_ItemSwingDash_MainEv"]
    pub(super) fn status_ItemSwingDash_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_ItemSwingDashEv"]
    pub(super) fn status_end_ItemSwingDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_item_swing_initEv"]
    pub(super) fn sub_uniq_process_item_swing_init(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_uniq_process_item_swing_exit_statusEv"]
    pub(super) fn sub_uniq_process_item_swing_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45FighterStatusUniqProcessItemSwing_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessItemSwing_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45FighterStatusUniqProcessItemSwing_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessItemSwing_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessItemSwing_check_attackEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessItemSwing_check_attack(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_JumpSquatEv"]
    pub(super) fn status_pre_JumpSquat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_JumpSquat_paramEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn status_pre_JumpSquat_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_Jump_subEv"]
    pub(super) fn status_pre_Jump_sub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_Jump_sub_paramEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn status_pre_Jump_sub_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_JumpEv"]
    pub(super) fn status_pre_Jump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_JumpAerial_subEv"]
    pub(super) fn status_pre_JumpAerial_sub(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_JumpAerial_sub_paramEN3lib8L2CValueE"]
    pub(super) fn status_pre_JumpAerial_sub_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_JumpAerialEv"]
    pub(super) fn status_pre_JumpAerial(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_JumpAerial_paramEN3lib8L2CValueES2_S2_S2_S2_S2_"]
    pub(super) fn status_pre_JumpAerial_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_pre_FlyEv"]
    pub(super) fn status_pre_Fly(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_FallEv"]
    pub(super) fn status_pre_Fall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_Fall_mainEv"]
    pub(super) fn status_pre_Fall_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_Fall_main_paramEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn status_pre_Fall_main_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_FallAerialEv"]
    pub(super) fn status_pre_FallAerial(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_FallAerial_mainEv"]
    pub(super) fn status_pre_FallAerial_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_FallAerial_main_paramEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn status_pre_FallAerial_main_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_LandingEv"]
    pub(super) fn status_pre_Landing(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15sub_pre_landingEv"]
    pub(super) fn sub_pre_landing(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_Landing_paramEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn status_pre_Landing_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_LandingLightEv"]
    pub(super) fn status_pre_LandingLight(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_LandingLight_paramEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn status_pre_LandingLight_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_LandingAttackAirEv"]
    pub(super) fn status_pre_LandingAttackAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_LandingDamageLightEv"]
    pub(super) fn status_pre_LandingDamageLight(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_GlideStartEv"]
    pub(super) fn status_pre_GlideStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_pre_GlideEv"]
    pub(super) fn status_pre_Glide(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_GlideLandingEv"]
    pub(super) fn status_pre_GlideLanding(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_GlideAttackEv"]
    pub(super) fn status_pre_GlideAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_GlideEndEv"]
    pub(super) fn status_pre_GlideEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_JumpSquat_commonEN3lib8L2CValueE"]
    pub(super) fn status_JumpSquat_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42sub_status_JumpSquat_check_stick_lr_updateEv"]
    pub(super) fn sub_status_JumpSquat_check_stick_lr_update(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_JumpSquatEv"]
    pub(super) fn status_JumpSquat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_JumpSquat_MainEv"]
    pub(super) fn status_JumpSquat_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34uniq_process_JumpSquat_exec_statusEv"]
    pub(super) fn uniq_process_JumpSquat_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40uniq_process_JumpSquat_exec_status_paramEN3lib8L2CValueE"]
    pub(super) fn uniq_process_JumpSquat_exec_status_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_jump_squat_uniq_check_sub_mini_attackEv"]
    pub(super) fn sub_jump_squat_uniq_check_sub_mini_attack(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_jump_squat_uniq_checkEv"]
    pub(super) fn sub_jump_squat_uniq_check(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_jump_aerial_item_rocketbeltEv"]
    pub(super) fn sub_jump_aerial_item_rocketbelt(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_JumpAerialEv"]
    pub(super) fn status_JumpAerial(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon10status_FlyEv"]
    pub(super) fn status_Fly(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13status_FlySubEv"]
    pub(super) fn status_FlySub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_Fly_MainEv"]
    pub(super) fn status_Fly_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16sub_getFlyMotionEv"]
    pub(super) fn sub_getFlyMotion(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12sub_fly_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_fly_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33local_func__fighter_status_jump_1Ev"]
    pub(super) fn local_func__fighter_status_jump_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_FallEv"]
    pub(super) fn status_Fall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_FallSubEN3lib8L2CValueE"]
    pub(super) fn status_FallSub(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Fall_MainEv"]
    pub(super) fn status_Fall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_FallSub_paramEN3lib8L2CValueES2_"]
    pub(super) fn status_FallSub_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_FallAerialEv"]
    pub(super) fn status_FallAerial(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_FallAerialSub_paramEN3lib8L2CValueE"]
    pub(super) fn status_FallAerialSub_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_FallAerial_MainEv"]
    pub(super) fn status_FallAerial_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_FallAerialSubEv"]
    pub(super) fn status_FallAerialSub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_LandingEv"]
    pub(super) fn status_Landing(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_landing_start_check_damage_faceEv"]
    pub(super) fn sub_landing_start_check_damage_face(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_LandingLightEv"]
    pub(super) fn status_LandingLight(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_LandingLightSub_paramEN3lib8L2CValueE"]
    pub(super) fn status_LandingLightSub_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_LandingLight_MainEv"]
    pub(super) fn status_LandingLight_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_LandingLightSubEv"]
    pub(super) fn status_LandingLightSub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_landing_uniq_check_attack_airEv"]
    pub(super) fn sub_landing_uniq_check_attack_air(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_LandingAttackAirEv"]
    pub(super) fn status_LandingAttackAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_LandingAttackAirSubEv"]
    pub(super) fn status_LandingAttackAirSub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_LandingAttackAir_MainEv"]
    pub(super) fn status_LandingAttackAir_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_LandingDamageLightEv"]
    pub(super) fn status_LandingDamageLight(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_GlideStartEv"]
    pub(super) fn status_GlideStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_GlideStart_MainEv"]
    pub(super) fn status_GlideStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12status_GlideEv"]
    pub(super) fn status_Glide(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_Glide_MainEv"]
    pub(super) fn status_Glide_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_GlideLandingEv"]
    pub(super) fn status_GlideLanding(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_GlideLanding_MainEv"]
    pub(super) fn status_GlideLanding_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_GlideAttackEv"]
    pub(super) fn status_GlideAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_GlideAttack_MainEv"]
    pub(super) fn status_GlideAttack_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_GlideEndEv"]
    pub(super) fn status_GlideEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_GlideEnd_MainEv"]
    pub(super) fn status_GlideEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_JumpSquatEv"]
    pub(super) fn status_end_JumpSquat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_JumpEv"]
    pub(super) fn status_end_Jump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_JumpAerialEv"]
    pub(super) fn status_end_JumpAerial(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_JumpAerial_paramEN3lib8L2CValueE"]
    pub(super) fn status_end_JumpAerial_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_end_FlyEv"]
    pub(super) fn status_end_Fly(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_FallEv"]
    pub(super) fn status_end_Fall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_FallAerialEv"]
    pub(super) fn status_end_FallAerial(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_LandingEv"]
    pub(super) fn status_end_Landing(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_LandingLightEv"]
    pub(super) fn status_end_LandingLight(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_LandingAttackAirEv"]
    pub(super) fn status_end_LandingAttackAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_LandingDamageLightEv"]
    pub(super) fn status_end_LandingDamageLight(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_GlideStartEv"]
    pub(super) fn status_end_GlideStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_end_GlideEv"]
    pub(super) fn status_end_Glide(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_GlideLandingEv"]
    pub(super) fn status_end_GlideLanding(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_GlideAttackEv"]
    pub(super) fn status_end_GlideAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_GlideEndEv"]
    pub(super) fn status_end_GlideEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15sub_glide_checkEv"]
    pub(super) fn sub_glide_check(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_ItemScrewJumpEv"]
    pub(super) fn status_pre_ItemScrewJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_ItemScrewJumpEv"]
    pub(super) fn status_ItemScrewJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_ItemScrewJump_MainEv"]
    pub(super) fn status_ItemScrewJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_ItemScrewJumpEv"]
    pub(super) fn status_end_ItemScrewJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_ItemScrewJumpAerialEv"]
    pub(super) fn status_pre_ItemScrewJumpAerial(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_ItemScrewJumpAerialEv"]
    pub(super) fn status_ItemScrewJumpAerial(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_ItemScrewJumpAerialSubEv"]
    pub(super) fn status_ItemScrewJumpAerialSub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_ItemScrewJumpAerial_MainEv"]
    pub(super) fn status_ItemScrewJumpAerial_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_ItemScrewJumpAerialEv"]
    pub(super) fn status_end_ItemScrewJumpAerial(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_ItemScrewFallEv"]
    pub(super) fn status_pre_ItemScrewFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_ItemScrewFallEv"]
    pub(super) fn status_ItemScrewFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_ItemScrewFall_MainEv"]
    pub(super) fn status_ItemScrewFall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_ItemScrewFallEv"]
    pub(super) fn status_end_ItemScrewFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_fall_uniq_process_initEv"]
    pub(super) fn sub_fall_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_fall_uniq_process_init_paramEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_fall_uniq_process_init_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_fall_uniq_process_execEv"]
    pub(super) fn sub_fall_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_fall_uniq_process_exitEv"]
    pub(super) fn sub_fall_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_fall_uniq_process_exit_paramEN3lib8L2CValueE"]
    pub(super) fn sub_fall_uniq_process_exit_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_jump_squat_uniq_process_initEv"]
    pub(super) fn sub_jump_squat_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_jump_squat_uniq_process_init_paramEN3lib8L2CValueE"]
    pub(super) fn sub_jump_squat_uniq_process_init_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_fly_uniq_process_initEv"]
    pub(super) fn sub_fly_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_calc_landing_motion_rateEN3lib8L2CValueES2_"]
    pub(super) fn sub_calc_landing_motion_rate(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_landing_uniq_process_init_mainEN3lib8L2CValueE"]
    pub(super) fn sub_landing_uniq_process_init_main(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_landing_uniq_process_init_main_paramEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn sub_landing_uniq_process_init_main_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_landing_attack_air_initEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_landing_attack_air_init(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_landing_fall_special_initEN3lib8L2CValueE"]
    pub(super) fn sub_landing_fall_special_init(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_get_landing_motion_rateEN3lib8L2CValueES2_"]
    pub(super) fn sub_get_landing_motion_rate(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_landing_uniq_process_initEv"]
    pub(super) fn sub_landing_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_landing_attack_air_commonEN3lib8L2CValueE"]
    pub(super) fn sub_landing_attack_air_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_landing_uniq_process_exec_mainEN3lib8L2CValueE"]
    pub(super) fn sub_landing_uniq_process_exec_main(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_landing_uniq_process_exec_main_paramEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_landing_uniq_process_exec_main_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_landing_uniq_process_execEv"]
    pub(super) fn sub_landing_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_landing_uniq_process_exitEv"]
    pub(super) fn sub_landing_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_FinalJumpEndEv"]
    pub(super) fn status_pre_FinalJumpEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_FinalJumpEndEv"]
    pub(super) fn status_FinalJumpEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_FinalJumpEndUniqEN3lib8L2CValueE"]
    pub(super) fn sub_FinalJumpEndUniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_FinalJumpEnd_MainEv"]
    pub(super) fn status_FinalJumpEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_FinalJumpEndEv"]
    pub(super) fn status_end_FinalJumpEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_KamuiPierceEv"]
    pub(super) fn status_pre_KamuiPierce(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_KamuiPierceEv"]
    pub(super) fn status_KamuiPierce(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_KamuiPierce_MainEv"]
    pub(super) fn status_KamuiPierce_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_KamuiPierceEv"]
    pub(super) fn status_end_KamuiPierce(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_KoopaDivedEv"]
    pub(super) fn status_pre_KoopaDived(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_KoopaDivedEv"]
    pub(super) fn status_KoopaDived(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_KoopaDivedEv"]
    pub(super) fn status_end_KoopaDived(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_KoopaDived_initEv"]
    pub(super) fn sub_uniq_process_KoopaDived_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_KoopaDived_execEv"]
    pub(super) fn sub_uniq_process_KoopaDived_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_KoopaDived_exitEv"]
    pub(super) fn sub_uniq_process_KoopaDived_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessKoopaDived_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessKoopaDived_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_LadderCatchEv"]
    pub(super) fn status_pre_LadderCatch(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_LadderCatchEv"]
    pub(super) fn status_LadderCatch(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_status_LadderCatch_mtransEv"]
    pub(super) fn sub_status_LadderCatch_mtrans(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_LadderCatch_MainEv"]
    pub(super) fn status_LadderCatch_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessGimmickLadder_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_catchEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_catch(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon65FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_catch_bottomEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_catch_bottom(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessGimmickLadder_fix_pos_ladderEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_fix_pos_ladder(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_endEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_end(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_paramEN3lib8L2CValueES2_S2_"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_LadderCatchEv"]
    pub(super) fn status_end_LadderCatch(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_LadderCatchBottomEv"]
    pub(super) fn status_pre_LadderCatchBottom(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_LadderCatchBottomEv"]
    pub(super) fn status_LadderCatchBottom(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_LadderCatchBottom_commonEv"]
    pub(super) fn sub_LadderCatchBottom_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_LadderCatchBottom_MainEv"]
    pub(super) fn status_LadderCatchBottom_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_LadderCatchBottomEv"]
    pub(super) fn status_end_LadderCatchBottom(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_pre_LadderEv"]
    pub(super) fn status_pre_Ladder(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13status_LadderEv"]
    pub(super) fn status_Ladder(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17sub_LadderUniqChkEN3lib8L2CValueE"]
    pub(super) fn sub_LadderUniqChk(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_Ladder_MainEv"]
    pub(super) fn status_Ladder_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_end_LadderEv"]
    pub(super) fn status_end_Ladder(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_LadderAttackEv"]
    pub(super) fn status_pre_LadderAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_LadderAttack_commonEv"]
    pub(super) fn status_LadderAttack_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_LadderAttack_common_paramEN3lib8L2CValueE"]
    pub(super) fn status_LadderAttack_common_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_LadderAttackEv"]
    pub(super) fn status_LadderAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_LadderAttack_MainEv"]
    pub(super) fn status_LadderAttack_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_LadderAttackEv"]
    pub(super) fn status_end_LadderAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_LadderEndEv"]
    pub(super) fn status_pre_LadderEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_LadderEndEv"]
    pub(super) fn status_LadderEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_LadderEnd_MainEv"]
    pub(super) fn status_LadderEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_LadderEndEv"]
    pub(super) fn status_end_LadderEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessGimmickLadder_correct_xEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_correct_x(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessGimmickLadder_correct_yEN3lib8L2CValueES2_S2_"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_correct_y(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon60FighterStatusUniqProcessGimmickLadder_adjust_ladder_movementEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_adjust_ladder_movement(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon62FighterStatusUniqProcessGimmickLadder_init_ladder_catch_bottomEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_init_ladder_catch_bottom(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessGimmickLadder_init_ladderEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_init_ladder(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessGimmickLadder_init_ladder_attackEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_init_ladder_attack(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessGimmickLadder_init_ladder_endEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_init_ladder_end(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessGimmickLadder_exec_ladder_catchEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_exec_ladder_catch(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon62FighterStatusUniqProcessGimmickLadder_exec_ladder_catch_bottomEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_exec_ladder_catch_bottom(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessGimmickLadder_exec_ladderEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_exec_ladder(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessGimmickLadder_exec_ladder_endEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_exec_ladder_end(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon63FighterStatusUniqProcessGimmickLadder_exec_ladder_flip_rotationEv"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_exec_ladder_flip_rotation(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessGimmickLadder_exec_stop_ladderEv"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_exec_stop_ladder(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessGimmickLadder_exit_ladderEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_exit_ladder(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessGimmickLadder_exit_ladder_attackEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_exit_ladder_attack(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessGimmickLadder_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessGimmickLadder_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessGimmickLadderAttack_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickLadderAttack_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessGimmickLadder_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessGimmickLadder_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessGimmickLadder_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_MewtwoThrownEv"]
    pub(super) fn status_pre_MewtwoThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_MewtwoThrownEv"]
    pub(super) fn status_MewtwoThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42local_func__fighter_status_mewtwo_thrown_1Ev"]
    pub(super) fn local_func__fighter_status_mewtwo_thrown_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_MewtwoThrownEv"]
    pub(super) fn status_end_MewtwoThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessMewtwoThrown_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessMewtwoThrown_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessMewtwoThrown_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessMewtwoThrown_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_MiifighterCounterThrownEv"]
    pub(super) fn status_pre_MiifighterCounterThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_MiifighterCounterThrownEv"]
    pub(super) fn status_MiifighterCounterThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_MiifighterCounterThrown_MainEv"]
    pub(super) fn status_MiifighterCounterThrown_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_MiifighterCounterThrownEv"]
    pub(super) fn status_end_MiifighterCounterThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon60FighterStatusUniqProcessMiifighterCounterThrown_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessMiifighterCounterThrown_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_MiifighterSuplexThrownEv"]
    pub(super) fn status_pre_MiifighterSuplexThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_MiifighterSuplexThrownEv"]
    pub(super) fn status_MiifighterSuplexThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_MiifighterSuplexThrown_MainEv"]
    pub(super) fn status_MiifighterSuplexThrown_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20update_suplex_offsetEv"]
    pub(super) fn update_suplex_offset(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_MiifighterSuplexThrownEv"]
    pub(super) fn status_end_MiifighterSuplexThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon59FighterStatusUniqProcessMiifighterSuplexThrown_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessMiifighterSuplexThrown_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38status_pre_MiifighterSuplexAirCapturedEv"]
    pub(super) fn status_pre_MiifighterSuplexAirCaptured(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_MiifighterSuplexAirCapturedEv"]
    pub(super) fn status_MiifighterSuplexAirCaptured(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39status_MiifighterSuplexAirCaptured_MainEv"]
    pub(super) fn status_MiifighterSuplexAirCaptured_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38status_end_MiifighterSuplexAirCapturedEv"]
    pub(super) fn status_end_MiifighterSuplexAirCaptured(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon64FighterStatusUniqProcessMiifighterSuplexAirCaptured_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessMiifighterSuplexAirCaptured_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_MiifighterSuplexAirFallEv"]
    pub(super) fn status_pre_MiifighterSuplexAirFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_MiifighterSuplexAirFallEv"]
    pub(super) fn status_MiifighterSuplexAirFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_MiifighterSuplexAirFall_MainEv"]
    pub(super) fn status_MiifighterSuplexAirFall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_MiifighterSuplexAirFallEv"]
    pub(super) fn status_end_MiifighterSuplexAirFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon60FighterStatusUniqProcessMiifighterSuplexAirFall_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessMiifighterSuplexAirFall_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37status_pre_MiifighterSuplexAirLandingEv"]
    pub(super) fn status_pre_MiifighterSuplexAirLanding(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_MiifighterSuplexAirLandingEv"]
    pub(super) fn status_MiifighterSuplexAirLanding(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38status_MiifighterSuplexAirLanding_MainEv"]
    pub(super) fn status_MiifighterSuplexAirLanding_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37status_end_MiifighterSuplexAirLandingEv"]
    pub(super) fn status_end_MiifighterSuplexAirLanding(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon63FighterStatusUniqProcessMiifighterSuplexAirLanding_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessMiifighterSuplexAirLanding_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_pre_MiiswordsmanCounterDamageEv"]
    pub(super) fn status_pre_MiiswordsmanCounterDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_MiiswordsmanCounterDamageEv"]
    pub(super) fn status_MiiswordsmanCounterDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_end_MiiswordsmanCounterDamageEv"]
    pub(super) fn status_end_MiiswordsmanCounterDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_OttottoEv"]
    pub(super) fn status_pre_Ottotto(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_ottotto_common_preEv"]
    pub(super) fn sub_ottotto_common_pre(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_ottotto_commonEv"]
    pub(super) fn sub_ottotto_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_OttottoEv"]
    pub(super) fn status_Ottotto(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_Ottotto_MainEv"]
    pub(super) fn status_Ottotto_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_OttottoWaitEv"]
    pub(super) fn status_OttottoWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_Ottotto_Wait_MainEv"]
    pub(super) fn status_Ottotto_Wait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_ottotto_uniq_process_exec_fix_posEv"]
    pub(super) fn sub_ottotto_uniq_process_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ottotto_uniq_process_mainEv"]
    pub(super) fn sub_ottotto_uniq_process_main(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_OttottoEv"]
    pub(super) fn status_end_Ottotto(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ottotto_uniq_process_initEv"]
    pub(super) fn sub_ottotto_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_ottotto_uniq_process_exec_statusEv"]
    pub(super) fn sub_ottotto_uniq_process_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_ottotto_uniq_process_exitEv"]
    pub(super) fn sub_ottotto_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_PassiveEv"]
    pub(super) fn status_pre_Passive(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_PassiveEv"]
    pub(super) fn status_Passive(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_Passive_MainEv"]
    pub(super) fn status_Passive_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_PassiveEv"]
    pub(super) fn status_end_Passive(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_PassiveFBEv"]
    pub(super) fn status_pre_PassiveFB(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_PassiveFBEv"]
    pub(super) fn status_PassiveFB(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_PassiveFB_MainEv"]
    pub(super) fn status_PassiveFB_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_PassiveFBEv"]
    pub(super) fn status_end_PassiveFB(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_PassiveWallEv"]
    pub(super) fn status_pre_PassiveWall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_PassiveWallEv"]
    pub(super) fn status_PassiveWall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15PassiveWallUniqEN3lib8L2CValueE"]
    pub(super) fn PassiveWallUniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_PassiveWall_MainEv"]
    pub(super) fn status_PassiveWall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_PassiveWallEv"]
    pub(super) fn status_end_PassiveWall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_PassiveWallJumpEv"]
    pub(super) fn status_pre_PassiveWallJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_PassiveWallJumpEv"]
    pub(super) fn status_PassiveWallJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19PassiveWallJumpUniqEN3lib8L2CValueE"]
    pub(super) fn PassiveWallJumpUniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_PassiveWallJump_MainEv"]
    pub(super) fn status_PassiveWallJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_PassiveWallJumpEv"]
    pub(super) fn status_end_PassiveWallJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_PassiveCeilEv"]
    pub(super) fn status_pre_PassiveCeil(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_PassiveCeilEv"]
    pub(super) fn status_PassiveCeil(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_PassiveCeil_MainEv"]
    pub(super) fn status_PassiveCeil_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_PassiveCeilEv"]
    pub(super) fn status_end_PassiveCeil(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14FL_get_wall_lrEv"]
    pub(super) fn FL_get_wall_lr(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18set_passive_effectEN3lib8L2CValueE"]
    pub(super) fn set_passive_effect(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_uniq_process_Passive_initEv"]
    pub(super) fn sub_uniq_process_Passive_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_uniq_process_Passive_exec_stopEv"]
    pub(super) fn sub_uniq_process_Passive_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_uniq_process_Passive_exec_statusEv"]
    pub(super) fn sub_uniq_process_Passive_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_uniq_process_Passive_exit_statusEv"]
    pub(super) fn sub_uniq_process_Passive_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_PsychobreakEv"]
    pub(super) fn status_pre_Psychobreak(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_PsychobreakEv"]
    pub(super) fn status_Psychobreak(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_Psychobreak_MainEv"]
    pub(super) fn status_Psychobreak_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_PsychobreakEv"]
    pub(super) fn status_end_Psychobreak(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessPsychobreak_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessPsychobreak_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_RebirthEv"]
    pub(super) fn status_pre_Rebirth(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_RebirthEv"]
    pub(super) fn status_Rebirth(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_rebirth_common_preEv"]
    pub(super) fn sub_rebirth_common_pre(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_Rebirth_MainEv"]
    pub(super) fn status_Rebirth_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_rebirth_commonEv"]
    pub(super) fn sub_rebirth_common(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15sub_wait_motionEN3lib8L2CValueE"]
    pub(super) fn sub_wait_motion(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_rebirth_uniq_process_exec_fix_posEv"]
    pub(super) fn sub_rebirth_uniq_process_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17sub_update_effectEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_update_effect(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17sub_remove_effectEv"]
    pub(super) fn sub_remove_effect(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17sub_attach_effectEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_attach_effect(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_rebirth_uniq_checkEN3lib8L2CValueE"]
    pub(super) fn sub_rebirth_uniq_check(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_rebirth_uniq_process_mainEv"]
    pub(super) fn sub_rebirth_uniq_process_main(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_Rebirth_commonEv"]
    pub(super) fn status_end_Rebirth_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_RebirthEv"]
    pub(super) fn status_end_Rebirth(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_rebirth_uniq_process_initEv"]
    pub(super) fn sub_rebirth_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_rebirth_uniq_process_init_coreEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_rebirth_uniq_process_init_core(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43FighterStatusUniqProcessRebirth_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessRebirth_init_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessRebirth_check_discretion_finalEv"]
    pub(super) fn FighterStatusUniqProcessRebirth_check_discretion_final(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessRebirth_is_operation_cpuEv"]
    pub(super) fn FighterStatusUniqProcessRebirth_is_operation_cpu(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessRebirth_is_item_appearance_offEv"]
    pub(super) fn FighterStatusUniqProcessRebirth_is_item_appearance_off(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessRebirth_is_entry_countEv"]
    pub(super) fn FighterStatusUniqProcessRebirth_is_entry_count(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41FighterStatusUniqProcessRebirth_is_bottomEv"]
    pub(super) fn FighterStatusUniqProcessRebirth_is_bottom(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_check_discretion_finalEN3lib8L2CValueE"]
    pub(super) fn sub_check_discretion_final(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12sub_end_moveEv"]
    pub(super) fn sub_end_move(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_rebirth_uniq_process_exec_statusEv"]
    pub(super) fn sub_rebirth_uniq_process_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_rebirth_uniq_process_exitEv"]
    pub(super) fn sub_rebirth_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45FighterStatusUniqProcessRebirth_is_final_usedEv"]
    pub(super) fn FighterStatusUniqProcessRebirth_is_final_used(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessRebirth_is_last_dead_suicideEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessRebirth_is_last_dead_suicide(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_pre_WinEv"]
    pub(super) fn status_pre_Win(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon10status_WinEv"]
    pub(super) fn status_Win(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_Win_MainEv"]
    pub(super) fn status_Win_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_end_WinEv"]
    pub(super) fn status_end_Win(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_LoseEv"]
    pub(super) fn status_pre_Lose(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_LoseEv"]
    pub(super) fn status_Lose(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Lose_MainEv"]
    pub(super) fn status_Lose_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_RouletteEv"]
    pub(super) fn status_pre_Roulette(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_roulette_set_stepEN3lib8L2CValueE"]
    pub(super) fn sub_roulette_set_step(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_get_wait_motionEv"]
    pub(super) fn sub_get_wait_motion(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12sub_RouletteEv"]
    pub(super) fn sub_Roulette(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_RouletteEv"]
    pub(super) fn status_Roulette(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_Roulette_MainEv"]
    pub(super) fn status_Roulette_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25set_roulette_shoot_motionEv"]
    pub(super) fn set_roulette_shoot_motion(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_RouletteEv"]
    pub(super) fn status_end_Roulette(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_RouletteFuraFuraEv"]
    pub(super) fn status_pre_RouletteFuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_RouletteFuraFuraEv"]
    pub(super) fn status_RouletteFuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_RouletteFuraFura_MainEv"]
    pub(super) fn status_RouletteFuraFura_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_RouletteFuraFuraEv"]
    pub(super) fn status_end_RouletteFuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusRouletteFuraFura_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusRouletteFuraFura_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_SavingDamageEv"]
    pub(super) fn status_pre_SavingDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_SavingDamageEv"]
    pub(super) fn status_SavingDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_SavingDamage_MainEv"]
    pub(super) fn status_SavingDamage_Main(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_SavingDamageEv"]
    pub(super) fn status_end_SavingDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_saving_damage_uniq_process_mainEv"]
    pub(super) fn sub_saving_damage_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_ShoulderedDonkeyStartEv"]
    pub(super) fn status_pre_ShoulderedDonkeyStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_ShoulderedDonkeyStartEv"]
    pub(super) fn status_ShoulderedDonkeyStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_ShoulderedDonkeyStart_MainEv"]
    pub(super) fn status_ShoulderedDonkeyStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34uniq_process_ShoulderedDonkey_mainEv"]
    pub(super) fn uniq_process_ShoulderedDonkey_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50uniq_process_ShoulderedDonkey_exec_fix_pos_counterEv"]
    pub(super) fn uniq_process_ShoulderedDonkey_exec_fix_pos_counter(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_ShoulderedDonkeyStartEv"]
    pub(super) fn status_end_ShoulderedDonkeyStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_ShoulderedDonkeyEv"]
    pub(super) fn status_pre_ShoulderedDonkey(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_shouldered_donkey_uniqEv"]
    pub(super) fn sub_shouldered_donkey_uniq(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_ShoulderedDonkeyEv"]
    pub(super) fn status_ShoulderedDonkey(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_ShoulderedDonkey_MainEv"]
    pub(super) fn status_ShoulderedDonkey_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_ShoulderedDonkeyEv"]
    pub(super) fn status_end_ShoulderedDonkey(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_ShoulderedDonkeyThrownEv"]
    pub(super) fn status_pre_ShoulderedDonkeyThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_ShoulderedDonkeyThrownEv"]
    pub(super) fn status_ShoulderedDonkeyThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40uniq_process_ShoulderedDonkeyThrown_mainEv"]
    pub(super) fn uniq_process_ShoulderedDonkeyThrown_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_ShoulderedDonkeyThrown_MainEv"]
    pub(super) fn status_ShoulderedDonkeyThrown_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_ShoulderedDonkeyThrownEv"]
    pub(super) fn status_end_ShoulderedDonkeyThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41uniq_process_ShoulderedDonkey_init_statusEv"]
    pub(super) fn uniq_process_ShoulderedDonkey_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon42uniq_process_ShoulderedDonkey_exec_fix_posEv"]
    pub(super) fn uniq_process_ShoulderedDonkey_exec_fix_pos(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41uniq_process_ShoulderedDonkey_exit_statusEv"]
    pub(super) fn uniq_process_ShoulderedDonkey_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessShoulderedDonkey_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessShoulderedDonkey_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47uniq_process_ShoulderedDonkeyThrown_init_statusEv"]
    pub(super) fn uniq_process_ShoulderedDonkeyThrown_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56uniq_process_ShoulderedDonkeyThrown_exec_fix_pos_counterEv"]
    pub(super) fn uniq_process_ShoulderedDonkeyThrown_exec_fix_pos_counter(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48uniq_process_ShoulderedDonkeyThrown_exec_fix_posEv"]
    pub(super) fn uniq_process_ShoulderedDonkeyThrown_exec_fix_pos(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47uniq_process_ShoulderedDonkeyThrown_exit_statusEv"]
    pub(super) fn uniq_process_ShoulderedDonkeyThrown_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon59FighterStatusUniqProcessShoulderedDonkeyThrown_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessShoulderedDonkeyThrown_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14sub_SlipCommonEv"]
    pub(super) fn sub_SlipCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_SlipCommon_MainEv"]
    pub(super) fn sub_SlipCommon_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_SlipDownCommonEv"]
    pub(super) fn sub_SlipDownCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_SlipDownCommon_MainEv"]
    pub(super) fn sub_SlipDownCommon_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_SlipStandCommonEv"]
    pub(super) fn sub_SlipStandCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_SlipStandCommon_MainEv"]
    pub(super) fn sub_SlipStandCommon_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_SlipEv"]
    pub(super) fn status_pre_Slip(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_SlipEv"]
    pub(super) fn status_Slip(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Slip_MainEv"]
    pub(super) fn status_Slip_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_SlipEv"]
    pub(super) fn status_end_Slip(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_SlipDamageEv"]
    pub(super) fn status_pre_SlipDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_SlipDamageEv"]
    pub(super) fn status_SlipDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_SlipDamage_MainEv"]
    pub(super) fn status_SlipDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_SlipWait_Common_MainEv"]
    pub(super) fn status_SlipWait_Common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_SlipDamageEv"]
    pub(super) fn status_end_SlipDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_SlipWaitEv"]
    pub(super) fn status_pre_SlipWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_SlipWaitEv"]
    pub(super) fn status_SlipWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16sub_SlipWaitUniqEN3lib8L2CValueE"]
    pub(super) fn sub_SlipWaitUniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_SlipWait_MainEv"]
    pub(super) fn status_SlipWait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_SlipWaitEv"]
    pub(super) fn status_end_SlipWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_SlipStandEv"]
    pub(super) fn status_pre_SlipStand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_SlipStandEv"]
    pub(super) fn status_SlipStand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_SlipStand_MainEv"]
    pub(super) fn status_SlipStand_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_SlipStandEv"]
    pub(super) fn status_end_SlipStand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_SlipStandAttackEv"]
    pub(super) fn status_pre_SlipStandAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_SlipStandAttackEv"]
    pub(super) fn status_SlipStandAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_SlipStandAttack_MainEv"]
    pub(super) fn status_SlipStandAttack_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_SlipStandAttackEv"]
    pub(super) fn status_end_SlipStandAttack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_SlipStandFEv"]
    pub(super) fn status_pre_SlipStandF(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_SlipStandFEv"]
    pub(super) fn status_SlipStandF(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_SlipStandF_MainEv"]
    pub(super) fn status_SlipStandF_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_SlipStandFEv"]
    pub(super) fn status_end_SlipStandF(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_SlipStandBEv"]
    pub(super) fn status_pre_SlipStandB(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_SlipStandBEv"]
    pub(super) fn status_SlipStandB(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_SlipStandB_MainEv"]
    pub(super) fn status_SlipStandB_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_SlipStandBEv"]
    pub(super) fn status_end_SlipStandB(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_pre_SquatEv"]
    pub(super) fn status_pre_Squat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_Squat_paramEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn status_pre_Squat_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43status_pre_SquatWait_common_check_interruptEv"]
    pub(super) fn status_pre_SquatWait_common_check_interrupt(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_SquatWait_commonEN3lib8L2CValueE"]
    pub(super) fn status_pre_SquatWait_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_SquatWait_common_paramEN3lib8L2CValueES2_S2_S2_S2_S2_"]
    pub(super) fn status_pre_SquatWait_common_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_SquatWaitEv"]
    pub(super) fn status_pre_SquatWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_pre_SquatFEv"]
    pub(super) fn status_pre_SquatF(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_pre_SquatBEv"]
    pub(super) fn status_pre_SquatB(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_SquatRvEv"]
    pub(super) fn status_pre_SquatRv(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_SquatRv_paramEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn status_pre_SquatRv_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12status_SquatEv"]
    pub(super) fn status_Squat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Squat_subEv"]
    pub(super) fn status_Squat_sub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_Squat_MainEv"]
    pub(super) fn status_Squat_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_squat_common_MainEv"]
    pub(super) fn sub_squat_common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_Squat_sub_paramEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_Squat_sub_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_squat_common_paramEN3lib8L2CValueES2_"]
    pub(super) fn sub_squat_common_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_SquatWait_commonEN3lib8L2CValueE"]
    pub(super) fn status_SquatWait_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_SquatWait_common_paramEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_SquatWait_common_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16sub_squat_commonEv"]
    pub(super) fn sub_squat_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_squat_wait_mtrans_loop_paramEN3lib8L2CValueE"]
    pub(super) fn sub_squat_wait_mtrans_loop_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_squat_check_front_preEv"]
    pub(super) fn sub_squat_check_front_pre(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_squat_check_back_preEv"]
    pub(super) fn sub_squat_check_back_pre(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_SquatWaitEv"]
    pub(super) fn status_SquatWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_SquatWait_MainEv"]
    pub(super) fn status_SquatWait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_squat_check_frontEv"]
    pub(super) fn sub_squat_check_front(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_squat_check_backEv"]
    pub(super) fn sub_squat_check_back(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_squat_wait_mtrans_conditionsEv"]
    pub(super) fn sub_squat_wait_mtrans_conditions(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_squat_wait_mtrans_loopEv"]
    pub(super) fn sub_squat_wait_mtrans_loop(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13status_SquatFEv"]
    pub(super) fn status_SquatF(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_SquatF_MainEv"]
    pub(super) fn status_SquatF_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13status_SquatBEv"]
    pub(super) fn status_SquatB(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_SquatB_MainEv"]
    pub(super) fn status_SquatB_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_SquatRv_paramEN3lib8L2CValueES2_S2_"]
    pub(super) fn status_SquatRv_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11SquatRvUniqEv"]
    pub(super) fn SquatRvUniq(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_SquatRvEv"]
    pub(super) fn status_SquatRv(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_SquatRv_MainEv"]
    pub(super) fn status_SquatRv_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_end_SquatEv"]
    pub(super) fn status_end_Squat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_SquatWaitEv"]
    pub(super) fn status_end_SquatWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_end_SquatFEv"]
    pub(super) fn status_end_SquatF(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_end_SquatBEv"]
    pub(super) fn status_end_SquatB(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_SquatRvEv"]
    pub(super) fn status_end_SquatRv(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_squat_uniq_process_initEv"]
    pub(super) fn sub_squat_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_squat_uniq_process_init_paramEN3lib8L2CValueES2_"]
    pub(super) fn sub_squat_uniq_process_init_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_squat_uniq_process_mainEv"]
    pub(super) fn sub_squat_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_squat_uniq_process_main_paramEN3lib8L2CValueE"]
    pub(super) fn sub_squat_uniq_process_main_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_squat_uniq_process_exitEv"]
    pub(super) fn sub_squat_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_squat_walk_uniq_process_initEv"]
    pub(super) fn sub_squat_walk_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27FL_get_squat_walk_max_speedEv"]
    pub(super) fn FL_get_squat_walk_max_speed(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15get_motion_kindEv"]
    pub(super) fn get_motion_kind(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_squat_walk_uniq_process_mainEv"]
    pub(super) fn sub_squat_walk_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19FL_get_stick_x_rateEv"]
    pub(super) fn FL_get_stick_x_rate(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26set_squat_walk_motion_rateEv"]
    pub(super) fn set_squat_walk_motion_rate(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_squat_walk_uniq_process_exitEv"]
    pub(super) fn sub_squat_walk_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_squat_walk_uniq_process_calc_paramEv"]
    pub(super) fn sub_squat_walk_uniq_process_calc_param(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_StabbedDamageEv"]
    pub(super) fn status_pre_StabbedDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_StabbedDamageEv"]
    pub(super) fn status_StabbedDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_StabbedDamageEv"]
    pub(super) fn status_end_StabbedDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_stabbed_damage_uniq_process_initEv"]
    pub(super) fn sub_stabbed_damage_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_stabbed_damage_uniq_process_mainEv"]
    pub(super) fn sub_stabbed_damage_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_StabbedRidleyEv"]
    pub(super) fn status_pre_StabbedRidley(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_StabbedRidleyEv"]
    pub(super) fn status_StabbedRidley(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_StabbedRidley_MainEv"]
    pub(super) fn status_StabbedRidley_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_StabbedRidleyEv"]
    pub(super) fn status_end_StabbedRidley(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_StabbedRidley_initEv"]
    pub(super) fn sub_uniq_process_StabbedRidley_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_StabbedRidley_execEv"]
    pub(super) fn sub_uniq_process_StabbedRidley_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_uniq_process_StabbedRidley_exitEv"]
    pub(super) fn sub_uniq_process_StabbedRidley_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessStabbedRidley_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessStabbedRidley_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_StopWallEv"]
    pub(super) fn status_pre_StopWall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_StopWallEv"]
    pub(super) fn status_StopWall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_StopWall_MainEv"]
    pub(super) fn status_StopWall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_StopWallEv"]
    pub(super) fn status_end_StopWall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_ground_check_stop_wallEv"]
    pub(super) fn sub_ground_check_stop_wall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_StopCeilEv"]
    pub(super) fn status_pre_StopCeil(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_StopCeilEv"]
    pub(super) fn status_StopCeil(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17sub_StopCeil_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_StopCeil_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_StopCeil_MainEv"]
    pub(super) fn status_StopCeil_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_StopCeilEv"]
    pub(super) fn status_end_StopCeil(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21super_jump_punch_uniqEN3lib8L2CValueE"]
    pub(super) fn super_jump_punch_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16super_jump_punchEN3lib8L2CValueE"]
    pub(super) fn super_jump_punch(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21super_jump_punch_mainEv"]
    pub(super) fn super_jump_punch_main(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39super_jump_punch_reset_common_conditionEv"]
    pub(super) fn super_jump_punch_reset_common_condition(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20super_jump_punch_endEN3lib8L2CValueE"]
    pub(super) fn super_jump_punch_end(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_SwallowedEv"]
    pub(super) fn status_pre_Swallowed(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_SwallowedEv"]
    pub(super) fn status_Swallowed(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_Swallowed_MainEv"]
    pub(super) fn status_Swallowed_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessSwallowed_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessSwallowed_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_SwallowedEv"]
    pub(super) fn status_end_Swallowed(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45FighterStatusUniqProcessSwallowed_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowed_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43FighterStatusUniqProcessSwallowed_getOffsetEv"]
    pub(super) fn FighterStatusUniqProcessSwallowed_getOffset(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessSwallowed_getOffsetKindEv"]
    pub(super) fn FighterStatusUniqProcessSwallowed_getOffsetKind(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45FighterStatusUniqProcessSwallowed_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowed_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43FighterStatusUniqProcessSwallowed_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessSwallowed_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45FighterStatusUniqProcessSwallowed_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowed_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusUniqProcessSwallowed_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessSwallowed_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_SwallowedDrinkEv"]
    pub(super) fn status_pre_SwallowedDrink(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_SwallowedDrinkEv"]
    pub(super) fn status_SwallowedDrink(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_SwallowedDrink_MainEv"]
    pub(super) fn status_SwallowedDrink_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessSwallowedDrink_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedDrink_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_SwallowedDrinkEv"]
    pub(super) fn status_end_SwallowedDrink(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessSwallowedDrink_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedDrink_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessSwallowedDrink_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedDrink_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_SwallowedCaptureEv"]
    pub(super) fn status_pre_SwallowedCapture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_SwallowedCaptureEv"]
    pub(super) fn status_SwallowedCapture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessSwallowedCapture_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedCapture_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_SwallowedCapture_MainEv"]
    pub(super) fn status_SwallowedCapture_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_SwallowedCaptureEv"]
    pub(super) fn status_end_SwallowedCapture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessSwallowedCapture_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedCapture_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessSwallowedCapture_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedCapture_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessSwallowedCapture_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedCapture_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessSwallowedCapture_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessSwallowedCapture_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_SwallowedThrownStarEv"]
    pub(super) fn status_pre_SwallowedThrownStar(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_SwallowedThrownStarEv"]
    pub(super) fn status_SwallowedThrownStar(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessSwallowedThrownStar_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedThrownStar_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_SwallowedThrownStar_MainEv"]
    pub(super) fn status_SwallowedThrownStar_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_SwallowedThrownStarEv"]
    pub(super) fn status_end_SwallowedThrownStar(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessSwallowedThrownStar_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedThrownStar_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessSwallowedThrownStar_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedThrownStar_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessSwallowedThrownStar_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedThrownStar_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessSwallowedThrownStar_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessSwallowedThrownStar_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_SwallowedThrownEv"]
    pub(super) fn status_pre_SwallowedThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_SwallowedThrownEv"]
    pub(super) fn status_SwallowedThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessSwallowedThrown_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedThrown_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_SwallowedThrown_MainEv"]
    pub(super) fn status_SwallowedThrown_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_SwallowedThrownEv"]
    pub(super) fn status_end_SwallowedThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessSwallowedThrown_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedThrown_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_SwallowedCancelEv"]
    pub(super) fn status_pre_SwallowedCancel(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_SwallowedCancelEv"]
    pub(super) fn status_SwallowedCancel(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessSwallowedCancel_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedCancel_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_SwallowedCancel_MainEv"]
    pub(super) fn status_SwallowedCancel_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_SwallowedCancelEv"]
    pub(super) fn status_end_SwallowedCancel(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessSwallowedCancel_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedCancel_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessSwallowedCancel_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwallowedCancel_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_SwimDiveEv"]
    pub(super) fn status_pre_SwimDive(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_SwimDiveEv"]
    pub(super) fn status_SwimDive(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_SwimDropItemChkEv"]
    pub(super) fn sub_SwimDropItemChk(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_SwimDive_MainEv"]
    pub(super) fn status_SwimDive_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusUniqProcessSwim_exec_fix_pos_counterEv"]
    pub(super) fn FighterStatusUniqProcessSwim_exec_fix_pos_counter(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41FighterStatusUniqProcessSwim_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessSwim_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessSwim_get_water_surface_offset_posEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessSwim_get_water_surface_offset_pos(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_SwimDiveEv"]
    pub(super) fn status_end_SwimDive(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_SwimRiseEv"]
    pub(super) fn status_pre_SwimRise(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_SwimRiseEv"]
    pub(super) fn status_SwimRise(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_SwimRise_MainEv"]
    pub(super) fn status_SwimRise_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_SwimRiseEv"]
    pub(super) fn status_end_SwimRise(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_pre_SwimUpEv"]
    pub(super) fn status_pre_SwimUp(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13status_SwimUpEv"]
    pub(super) fn status_SwimUp(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_SwimUp_MainEv"]
    pub(super) fn status_SwimUp_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_end_SwimUpEv"]
    pub(super) fn status_end_SwimUp(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_SwimWaitEv"]
    pub(super) fn status_pre_SwimWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_SwimWaitEv"]
    pub(super) fn status_SwimWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_pre_SwimDrownChkEv"]
    pub(super) fn sub_pre_SwimDrownChk(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_SwimWait_MainEv"]
    pub(super) fn status_SwimWait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15sub_SwimJumpChkEv"]
    pub(super) fn sub_SwimJumpChk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16sub_SwimDrownChkEv"]
    pub(super) fn sub_SwimDrownChk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_SwimWaitEv"]
    pub(super) fn status_end_SwimWait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_SwimEv"]
    pub(super) fn status_pre_Swim(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_SwimEv"]
    pub(super) fn status_Swim(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Swim_MainEv"]
    pub(super) fn status_Swim_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_SwimEv"]
    pub(super) fn status_end_Swim(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_SwimEndEv"]
    pub(super) fn status_pre_SwimEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_SwimEndEv"]
    pub(super) fn status_SwimEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_SwimEnd_MainEv"]
    pub(super) fn status_SwimEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_SwimEndEv"]
    pub(super) fn status_end_SwimEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_SwimTurnEv"]
    pub(super) fn status_pre_SwimTurn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_SwimTurnEv"]
    pub(super) fn status_SwimTurn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_SwimTurn_MainEv"]
    pub(super) fn status_SwimTurn_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_SwimTurnEv"]
    pub(super) fn status_end_SwimTurn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_SwimJumpEv"]
    pub(super) fn status_pre_SwimJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_SwimJumpEv"]
    pub(super) fn status_SwimJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_SwimJump_MainEv"]
    pub(super) fn status_SwimJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16sub_SwimJumpUniqEN3lib8L2CValueE"]
    pub(super) fn sub_SwimJumpUniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_SwimJumpEv"]
    pub(super) fn status_end_SwimJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_SwimDrownEv"]
    pub(super) fn status_pre_SwimDrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_SwimDrownEv"]
    pub(super) fn status_SwimDrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_SwimDrown_MainEv"]
    pub(super) fn status_SwimDrown_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17sub_SwimDrownUniqEv"]
    pub(super) fn sub_SwimDrownUniq(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_SwimDrownEv"]
    pub(super) fn status_end_SwimDrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_SwimDrownOutEv"]
    pub(super) fn status_pre_SwimDrownOut(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_SwimDrownOutEv"]
    pub(super) fn status_SwimDrownOut(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_SwimDrownOut_MainEv"]
    pub(super) fn status_SwimDrownOut_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_SwimDrownOutEv"]
    pub(super) fn status_end_SwimDrownOut(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusUniqProcessSwim_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwim_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusUniqProcessSwim_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwim_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusUniqProcessSwim_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessSwim_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_SwingGaogaenCatchedEv"]
    pub(super) fn status_pre_SwingGaogaenCatched(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_SwingGaogaenCatchedEv"]
    pub(super) fn status_SwingGaogaenCatched(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36set_swing_special_state_SwingGaogaenEv"]
    pub(super) fn set_swing_special_state_SwingGaogaen(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_SwingGaogaenCatched_MainEv"]
    pub(super) fn status_SwingGaogaenCatched_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_SwingGaogaenCatchedEv"]
    pub(super) fn status_end_SwingGaogaenCatched(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusCapture_set_invalid_capture_SwingGaogaenEv"]
    pub(super) fn FighterStatusCapture_set_invalid_capture_SwingGaogaen(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_SwingGaogaenThrownEv"]
    pub(super) fn status_pre_SwingGaogaenThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_SwingGaogaenThrownEv"]
    pub(super) fn status_SwingGaogaenThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23sub_get_rope_run_motionEv"]
    pub(super) fn sub_get_rope_run_motion(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_SwingGaogaenThrown_MainEv"]
    pub(super) fn status_SwingGaogaenThrown_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_SwingGaogaenThrownEv"]
    pub(super) fn status_end_SwingGaogaenThrown(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_SwingGaogaenAttachRopeEv"]
    pub(super) fn status_pre_SwingGaogaenAttachRope(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_SwingGaogaenAttachRopeEv"]
    pub(super) fn status_SwingGaogaenAttachRope(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_SwingGaogaenAttachRope_MainEv"]
    pub(super) fn status_SwingGaogaenAttachRope_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_SwingGaogaenAttachRopeEv"]
    pub(super) fn status_end_SwingGaogaenAttachRope(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_SwingGaogaenReturnEv"]
    pub(super) fn status_pre_SwingGaogaenReturn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_SwingGaogaenReturnEv"]
    pub(super) fn status_SwingGaogaenReturn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_SwingGaogaenReturn_MainEv"]
    pub(super) fn status_SwingGaogaenReturn_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_SwingGaogaenReturnEv"]
    pub(super) fn status_end_SwingGaogaenReturn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_SwingGaogaenLariatEv"]
    pub(super) fn status_pre_SwingGaogaenLariat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_SwingGaogaenLariatEv"]
    pub(super) fn status_SwingGaogaenLariat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_SwingGaogaenLariat_MainEv"]
    pub(super) fn status_SwingGaogaenLariat_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_SwingGaogaenLariatEv"]
    pub(super) fn status_end_SwingGaogaenLariat(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessSwingGaogaenLariat_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessSwingGaogaenLariat_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_SwingGaogaenShoulderEv"]
    pub(super) fn status_pre_SwingGaogaenShoulder(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_SwingGaogaenShoulderEv"]
    pub(super) fn status_SwingGaogaenShoulder(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_SwingGaogaenShoulder_MainEv"]
    pub(super) fn status_SwingGaogaenShoulder_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_SwingGaogaenShoulderEv"]
    pub(super) fn status_end_SwingGaogaenShoulder(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessSwingGaogaenShoulder_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessSwingGaogaenShoulder_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_SwingGaogaenFailureEv"]
    pub(super) fn status_pre_SwingGaogaenFailure(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_SwingGaogaenFailureEv"]
    pub(super) fn status_SwingGaogaenFailure(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_SwingGaogaenFailure_MainEv"]
    pub(super) fn status_SwingGaogaenFailure_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_SwingGaogaenFailureEv"]
    pub(super) fn status_end_SwingGaogaenFailure(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessSwingGaogaenFailure_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessSwingGaogaenFailure_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_ThrowKirbyEv"]
    pub(super) fn status_pre_ThrowKirby(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_ThrowKirbyEv"]
    pub(super) fn status_ThrowKirby(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_ThrowKirby_UniqEN3lib8L2CValueE"]
    pub(super) fn status_ThrowKirby_Uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_ThrowKirby_MainEv"]
    pub(super) fn status_ThrowKirby_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_status_uniq_process_ThrowKirby_execFixPosEv"]
    pub(super) fn sub_status_uniq_process_ThrowKirby_execFixPos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_ThrowKirbyEv"]
    pub(super) fn status_end_ThrowKirby(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_status_uniq_process_ThrowKirby_initStatusEv"]
    pub(super) fn sub_status_uniq_process_ThrowKirby_initStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon45sub_status_uniq_process_ThrowKirby_exitStatusEv"]
    pub(super) fn sub_status_uniq_process_ThrowKirby_exitStatus(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_TreadDamageEv"]
    pub(super) fn status_pre_TreadDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_TreadDamageEv"]
    pub(super) fn status_TreadDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_TreadDamage_MainEv"]
    pub(super) fn status_TreadDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_TreadDamageEv"]
    pub(super) fn status_end_TreadDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_tread_damage_unique_process_exitEv"]
    pub(super) fn sub_tread_damage_unique_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_TreadDamageAirEv"]
    pub(super) fn status_pre_TreadDamageAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_TreadDamageAirEv"]
    pub(super) fn status_TreadDamageAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_TreadDamageAir_MainEv"]
    pub(super) fn status_TreadDamageAir_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_TreadDamageAirEv"]
    pub(super) fn status_end_TreadDamageAir(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_tread_damage_air_unique_process_exitEv"]
    pub(super) fn sub_tread_damage_air_unique_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_TreadDamageRvEv"]
    pub(super) fn status_pre_TreadDamageRv(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_TreadDamageRvEv"]
    pub(super) fn status_TreadDamageRv(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_TreadDamageRv_MainEv"]
    pub(super) fn status_TreadDamageRv_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_TreadDamageRvEv"]
    pub(super) fn status_end_TreadDamageRv(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_TreadFallEv"]
    pub(super) fn status_pre_TreadFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_tread_fall_uniq_checkEv"]
    pub(super) fn sub_tread_fall_uniq_check(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_TreadFallPassiveEv"]
    pub(super) fn sub_TreadFallPassive(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_TreadFallEv"]
    pub(super) fn status_TreadFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_TreadFall_MainEv"]
    pub(super) fn status_TreadFall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_TreadFallEv"]
    pub(super) fn status_end_TreadFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_tread_fall_unique_process_initEv"]
    pub(super) fn sub_tread_fall_unique_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_tread_fall_unique_process_execEv"]
    pub(super) fn sub_tread_fall_unique_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_TreadJumpEv"]
    pub(super) fn status_pre_TreadJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_tread_jump_uniq_checkEv"]
    pub(super) fn sub_tread_jump_uniq_check(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_TreadJumpEv"]
    pub(super) fn status_TreadJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_tread_jump_unique_process_init_innerEv"]
    pub(super) fn sub_tread_jump_unique_process_init_inner(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_TreadJump_MainEv"]
    pub(super) fn status_TreadJump_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_TreadJumpEv"]
    pub(super) fn status_end_TreadJump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_tread_jump_unique_process_initEv"]
    pub(super) fn sub_tread_jump_unique_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_tread_jump_unique_process_exitEv"]
    pub(super) fn sub_tread_jump_unique_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_Wait_check_interruptEv"]
    pub(super) fn status_pre_Wait_check_interrupt(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_Wait_mainEv"]
    pub(super) fn status_pre_Wait_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_Wait_main_paramEN3lib8L2CValueE"]
    pub(super) fn status_pre_Wait_main_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_WaitEv"]
    pub(super) fn status_pre_Wait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_WaitEv"]
    pub(super) fn status_Wait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Wait_MainEv"]
    pub(super) fn status_Wait_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20sub_wait_common_MainEv"]
    pub(super) fn sub_wait_common_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_ground_check_ottottoEv"]
    pub(super) fn sub_ground_check_ottotto(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_WaitEv"]
    pub(super) fn status_end_Wait(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_PassEv"]
    pub(super) fn status_pre_Pass(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_Pass_commonEv"]
    pub(super) fn status_Pass_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_PassEv"]
    pub(super) fn status_Pass(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_Pass_Main_subEN3lib8L2CValueE"]
    pub(super) fn status_Pass_Main_sub(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15end_pass_groundEv"]
    pub(super) fn end_pass_ground(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Pass_MainEv"]
    pub(super) fn status_Pass_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_PassEv"]
    pub(super) fn status_end_Pass(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_uniq_process_Pass_initEv"]
    pub(super) fn sub_uniq_process_Pass_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_uniq_process_Pass_exec_statusEv"]
    pub(super) fn sub_uniq_process_Pass_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_Appeal_commonEN3lib8L2CValueE"]
    pub(super) fn status_pre_Appeal_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_pre_AppealEv"]
    pub(super) fn status_pre_Appeal(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_Appeal_CommonEv"]
    pub(super) fn status_Appeal_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_Appeal_common_uniqEN3lib8L2CValueE"]
    pub(super) fn status_Appeal_common_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13status_AppealEv"]
    pub(super) fn status_Appeal(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_Appeal_MainEv"]
    pub(super) fn status_Appeal_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_end_AppealEv"]
    pub(super) fn status_end_Appeal(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_SmashAppealEv"]
    pub(super) fn status_pre_SmashAppeal(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_SmashAppealEv"]
    pub(super) fn status_SmashAppeal(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_ground_check_ottotto_motion_endEv"]
    pub(super) fn sub_ground_check_ottotto_motion_end(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_MetamonOutEv"]
    pub(super) fn status_pre_MetamonOut(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_MetamonOutEv"]
    pub(super) fn status_MetamonOut(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_MetamonOut_MainEv"]
    pub(super) fn status_MetamonOut_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_end_MetamonOutEv"]
    pub(super) fn status_end_MetamonOut(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_SuicideBombEv"]
    pub(super) fn status_pre_SuicideBomb(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_SuicideBombEv"]
    pub(super) fn status_SuicideBomb(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_SuicideBomb_MainEv"]
    pub(super) fn status_SuicideBomb_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_SuicideBombEv"]
    pub(super) fn status_end_SuicideBomb(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28check_turn_attack_s4_pad_revEv"]
    pub(super) fn check_turn_attack_s4_pad_rev(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_WalkEv"]
    pub(super) fn status_pre_Walk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon12sub_pre_WalkEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn sub_pre_Walk(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_pre_Walk_paramEN3lib8L2CValueES2_S2_S2_S2_S2_S2_S2_"]
    pub(super) fn sub_pre_Walk_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack, arg8: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_WalkBrakeEv"]
    pub(super) fn status_pre_WalkBrake(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_WalkBrake_paramEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn status_pre_WalkBrake_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_DashCommonEv"]
    pub(super) fn status_pre_DashCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_DashEv"]
    pub(super) fn status_pre_Dash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_pre_RunEv"]
    pub(super) fn status_pre_Run(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_RunBrakeEv"]
    pub(super) fn status_pre_RunBrake(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_pre_TurnCommonEv"]
    pub(super) fn status_pre_TurnCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_TurnCommonEv"]
    pub(super) fn status_TurnCommon(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_Turn_CommonEN3lib8L2CValueE"]
    pub(super) fn status_pre_Turn_Common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_pre_TurnEv"]
    pub(super) fn status_pre_Turn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_TurnDashEv"]
    pub(super) fn status_pre_TurnDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_pre_TurnRunEv"]
    pub(super) fn status_pre_TurnRun(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_pre_TurnRunBrakeEv"]
    pub(super) fn status_pre_TurnRunBrake(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_Walk_CommonEv"]
    pub(super) fn status_Walk_Common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_WalkEv"]
    pub(super) fn status_Walk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_walk_uniq_checkEv"]
    pub(super) fn sub_walk_uniq_check(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Walk_MainEv"]
    pub(super) fn status_Walk_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_Walk_Main_commonEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn status_Walk_Main_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22reset_walk_speed_ratioEN3lib8L2CValueES2_S2_S2_S2_S2_"]
    pub(super) fn reset_walk_speed_ratio(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39FighterStatusUniqProcessWalk_calc_paramEv"]
    pub(super) fn FighterStatusUniqProcessWalk_calc_param(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_WalkBrake_commonEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn status_WalkBrake_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_WalkBrakeEv"]
    pub(super) fn status_WalkBrake(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_WalkBrake_MainEv"]
    pub(super) fn status_WalkBrake_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_DashEv"]
    pub(super) fn status_Dash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_Dash_SubEv"]
    pub(super) fn status_Dash_Sub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_Dash_Main_commonEN3lib8L2CValueE"]
    pub(super) fn status_Dash_Main_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17status_DashCommonEv"]
    pub(super) fn status_DashCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Dash_MainEv"]
    pub(super) fn status_Dash_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_Run_SubEv"]
    pub(super) fn status_Run_Sub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon10status_RunEv"]
    pub(super) fn status_Run(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_Run_MainEv"]
    pub(super) fn status_Run_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23check_run_brake_l_frameEN3lib8L2CValueES2_S2_"]
    pub(super) fn check_run_brake_l_frame(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38FighterStatusUniqProcessRun_calc_paramEv"]
    pub(super) fn FighterStatusUniqProcessRun_calc_param(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19sub_status_RunBrakeEv"]
    pub(super) fn sub_status_RunBrake(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24sub_run_brake_uniq_checkEN3lib8L2CValueE"]
    pub(super) fn sub_run_brake_uniq_check(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_RunBrakeEv"]
    pub(super) fn status_RunBrake(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_RunBrake_MainEv"]
    pub(super) fn status_RunBrake_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon11status_TurnEv"]
    pub(super) fn status_Turn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_Turn_MainEv"]
    pub(super) fn status_Turn_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_TurnDash_SubEv"]
    pub(super) fn status_TurnDash_Sub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_TurnDashEv"]
    pub(super) fn status_TurnDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_TurnDash_MainEv"]
    pub(super) fn status_TurnDash_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_TurnRunEv"]
    pub(super) fn status_TurnRun(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_TurnRun_SubEN3lib8L2CValueES2_"]
    pub(super) fn status_TurnRun_Sub(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_TurnRun_MainEv"]
    pub(super) fn status_TurnRun_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon17sub_turn_run_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_turn_run_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22sub_turn_run_uniq_mainEN3lib8L2CValueES2_S2_"]
    pub(super) fn sub_turn_run_uniq_main(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_TurnRunBrake_SubEv"]
    pub(super) fn status_TurnRunBrake_Sub(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_TurnRunBrakeEv"]
    pub(super) fn status_TurnRunBrake(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_TurnRunBrake_MainEv"]
    pub(super) fn status_TurnRunBrake_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_WalkEv"]
    pub(super) fn status_end_Walk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_WalkBrakeEv"]
    pub(super) fn status_end_WalkBrake(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_DashEv"]
    pub(super) fn status_end_Dash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon14status_end_RunEv"]
    pub(super) fn status_end_Run(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_RunBrakeEv"]
    pub(super) fn status_end_RunBrake(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon15status_end_TurnEv"]
    pub(super) fn status_end_Turn(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon13sub_exit_TurnEv"]
    pub(super) fn sub_exit_Turn(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_TurnDashEv"]
    pub(super) fn status_end_TurnDash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_end_TurnRunEv"]
    pub(super) fn status_end_TurnRun(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_end_TurnRunBrakeEv"]
    pub(super) fn status_end_TurnRunBrake(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_dash_uniq_process_initEv"]
    pub(super) fn sub_dash_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_dash_uniq_process_mainEv"]
    pub(super) fn sub_dash_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_dash_uniq_process_main_internalEN3lib8L2CValueE"]
    pub(super) fn sub_dash_uniq_process_main_internal(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_dash_uniq_process_exit_commonEN3lib8L2CValueE"]
    pub(super) fn sub_dash_uniq_process_exit_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_dash_uniq_process_exitEv"]
    pub(super) fn sub_dash_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19run_set_motion_rateEv"]
    pub(super) fn run_set_motion_rate(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_run_uniq_process_initEv"]
    pub(super) fn sub_run_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_run_uniq_process_mainEv"]
    pub(super) fn sub_run_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34calc_jostle_speed_attenuation_rateEv"]
    pub(super) fn calc_jostle_speed_attenuation_rate(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_run_uniq_process_exitEv"]
    pub(super) fn sub_run_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_run_uniq_process_on_change_lrEN3lib8L2CValueES2_"]
    pub(super) fn sub_run_uniq_process_on_change_lr(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_run_brake_uniq_process_initEv"]
    pub(super) fn sub_run_brake_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_run_brake_uniq_process_mainEv"]
    pub(super) fn sub_run_brake_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_run_brake_uniq_process_exitEv"]
    pub(super) fn sub_run_brake_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_turn_uniq_process_initEv"]
    pub(super) fn sub_turn_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_turn_dash_uniq_process_initEv"]
    pub(super) fn sub_turn_dash_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_turn_uniq_process_mainEv"]
    pub(super) fn sub_turn_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_turn_dash_uniq_process_mainEv"]
    pub(super) fn sub_turn_dash_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_turn_uniq_process_exitEv"]
    pub(super) fn sub_turn_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_turn_dash_uniq_process_exitEv"]
    pub(super) fn sub_turn_dash_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_turn_run_uniq_process_initEv"]
    pub(super) fn sub_turn_run_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_turn_run_uniq_process_mainEv"]
    pub(super) fn sub_turn_run_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30sub_turn_run_uniq_process_exitEv"]
    pub(super) fn sub_turn_run_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34init_walk_motion_middle_fast_blendEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn init_walk_motion_middle_fast_blend(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38get_walk_motion_kind_middle_fast_blendEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn get_walk_motion_kind_middle_fast_blend(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41FL_get_walk_motion_rate_middle_fast_blendEN3lib8L2CValueES2_S2_S2_S2_S2_"]
    pub(super) fn FL_get_walk_motion_rate_middle_fast_blend(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_walk_uniq_process_init_common_paramEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn sub_walk_uniq_process_init_common_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_walk_uniq_process_init_commonEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn sub_walk_uniq_process_init_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_walk_uniq_process_initEv"]
    pub(super) fn sub_walk_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_walk_uniq_process_main_common_paramEN3lib8L2CValueES2_S2_S2_S2_S2_S2_"]
    pub(super) fn sub_walk_uniq_process_main_common_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33set_walk_motion_middle_fast_blendEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn set_walk_motion_middle_fast_blend(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29sub_walk_fix_pos_counter_execEv"]
    pub(super) fn sub_walk_fix_pos_counter_exec(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21sub_walk_fix_pos_execEv"]
    pub(super) fn sub_walk_fix_pos_exec(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_walk_uniq_process_main_commonEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn sub_walk_uniq_process_main_common(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_walk_uniq_process_mainEv"]
    pub(super) fn sub_walk_uniq_process_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_walk_uniq_process_exitEv"]
    pub(super) fn sub_walk_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_walk_uniq_process_on_change_lrEv"]
    pub(super) fn sub_walk_uniq_process_on_change_lr(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_walk_brake_uniq_process_initEv"]
    pub(super) fn sub_walk_brake_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_walk_brake_uniq_process_execEv"]
    pub(super) fn sub_walk_brake_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_walk_brake_uniq_process_exitEv"]
    pub(super) fn sub_walk_brake_uniq_process_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_pre_WalkBackEv"]
    pub(super) fn status_pre_WalkBack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_WalkBack_commonEv"]
    pub(super) fn status_WalkBack_common(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19status_end_WalkBackEv"]
    pub(super) fn status_end_WalkBack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_walk_back_uniq_process_exitEv"]
    pub(super) fn sub_walk_back_uniq_process_exit(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_walk_back_uniq_process_initEv"]
    pub(super) fn sub_walk_back_uniq_process_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_walk_back_uniq_process_execEv"]
    pub(super) fn sub_walk_back_uniq_process_exec(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon43FighterStatusUniqProcessWalkBack_calc_paramEv"]
    pub(super) fn FighterStatusUniqProcessWalkBack_calc_param(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_WalkBrakeBackEv"]
    pub(super) fn status_WalkBrakeBack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_WalkBrakeBackEv"]
    pub(super) fn status_end_WalkBrakeBack(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_pre_wall_jumpEv"]
    pub(super) fn status_pre_wall_jump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16status_wall_jumpEv"]
    pub(super) fn status_wall_jump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_wall_jump_uniqEN3lib8L2CValueE"]
    pub(super) fn sub_wall_jump_uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_wall_jump_mainEv"]
    pub(super) fn status_wall_jump_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_end_wall_jumpEv"]
    pub(super) fn status_end_wall_jump(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon76FighterStatusUniqProcessBayonettaFinalTarget_get_final_actor_module_accessorEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTarget_get_final_actor_module_accessor(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_pre_BayonettaFinalTargetStartEv"]
    pub(super) fn status_pre_BayonettaFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_BayonettaFinalTargetStartEv"]
    pub(super) fn status_BayonettaFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37status_BayonettaFinalTargetStart_MainEv"]
    pub(super) fn status_BayonettaFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_end_BayonettaFinalTargetStartEv"]
    pub(super) fn status_end_BayonettaFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_BayonettaFinalTargetStart_init_statusEv"]
    pub(super) fn sub_BayonettaFinalTargetStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon62FighterStatusUniqProcessBayonettaFinalTargetStart_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetStart_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37status_pre_BayonettaFinalTargetDamageEv"]
    pub(super) fn status_pre_BayonettaFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_BayonettaFinalTargetDamageEv"]
    pub(super) fn status_BayonettaFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38status_BayonettaFinalTargetDamage_MainEv"]
    pub(super) fn status_BayonettaFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37status_end_BayonettaFinalTargetDamageEv"]
    pub(super) fn status_end_BayonettaFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon62FighterStatusUniqProcessBayonettaFinalTargetDamage_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage_exit_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon62FighterStatusUniqProcessBayonettaFinalTargetDamage_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon62FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon60FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_coreEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_core(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon85FighterStatusUniqProcessBayonettaFinalTargetDamage_get_gomorrah_joint_global_positionEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage_get_gomorrah_joint_global_position(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon70FighterStatusUniqProcessBayonettaFinalTargetDamage_set_damage_reactionEN3lib8L2CValueES2_"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage_set_damage_reaction(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon72FighterStatusUniqProcessBayonettaFinalTargetDamageget_damage_motion_randEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamageget_damage_motion_rand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon60FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_stopEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_stop(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon66FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_fix_cameraEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_fix_camera(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon63FighterStatusUniqProcessBayonettaFinalTargetDamage_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon73FighterStatusUniqProcessBayonettaFinalTargetDamage_get_damage_motion_randEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage_get_damage_motion_rand(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon63FighterStatusUniqProcessBayonettaFinalTargetDamage_set_init_posEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage_set_init_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37status_pre_BayonettaFinalTargetStart2Ev"]
    pub(super) fn status_pre_BayonettaFinalTargetStart2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_BayonettaFinalTargetStart2Ev"]
    pub(super) fn status_BayonettaFinalTargetStart2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38status_BayonettaFinalTargetStart2_MainEv"]
    pub(super) fn status_BayonettaFinalTargetStart2_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37status_end_BayonettaFinalTargetStart2Ev"]
    pub(super) fn status_end_BayonettaFinalTargetStart2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon63FighterStatusUniqProcessBayonettaFinalTargetStart2_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetStart2_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38status_pre_BayonettaFinalTargetDamage2Ev"]
    pub(super) fn status_pre_BayonettaFinalTargetDamage2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_BayonettaFinalTargetDamage2Ev"]
    pub(super) fn status_BayonettaFinalTargetDamage2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39status_BayonettaFinalTargetDamage2_MainEv"]
    pub(super) fn status_BayonettaFinalTargetDamage2_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38status_end_BayonettaFinalTargetDamage2Ev"]
    pub(super) fn status_end_BayonettaFinalTargetDamage2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon63FighterStatusUniqProcessBayonettaFinalTargetDamage2_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage2_exit_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon63FighterStatusUniqProcessBayonettaFinalTargetDamage2_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage2_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon64FighterStatusUniqProcessBayonettaFinalTargetDamage2_set_init_posEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage2_set_init_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon64FighterStatusUniqProcessBayonettaFinalTargetDamage2_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetDamage2_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_BayonettaFinalTargetEndEv"]
    pub(super) fn status_pre_BayonettaFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_BayonettaFinalTargetEndEv"]
    pub(super) fn status_BayonettaFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_BayonettaFinalTargetEnd_MainEv"]
    pub(super) fn status_BayonettaFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_BayonettaFinalTargetEndEv"]
    pub(super) fn status_end_BayonettaFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon59FighterStatusUniqProcessBayonettaFinalTargetEnd_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessBayonettaFinalTargetEnd_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_CaptainFinalStartEv"]
    pub(super) fn status_CaptainFinalStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_CaptainFinalStart_MainEv"]
    pub(super) fn status_CaptainFinalStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_CaptainFinalStartEv"]
    pub(super) fn status_end_CaptainFinalStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_CaptainFinalStart_init_statusEv"]
    pub(super) fn sub_CaptainFinalStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_CaptainFinalFuraFuraEv"]
    pub(super) fn status_pre_CaptainFinalFuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_CaptainFinalFuraFuraEv"]
    pub(super) fn status_CaptainFinalFuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_CaptainFinalFuraFura_MainEv"]
    pub(super) fn status_CaptainFinalFuraFura_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_CaptainFinalFuraFuraEv"]
    pub(super) fn status_end_CaptainFinalFuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_CaptainFinalClashEv"]
    pub(super) fn status_pre_CaptainFinalClash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_CaptainFinalClashEv"]
    pub(super) fn status_CaptainFinalClash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_CaptainFinalClash_MainEv"]
    pub(super) fn status_CaptainFinalClash_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_CaptainFinalClashEv"]
    pub(super) fn status_end_CaptainFinalClash(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_CaptainFinalEndEv"]
    pub(super) fn status_pre_CaptainFinalEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_CaptainFinalEndEv"]
    pub(super) fn status_CaptainFinalEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_CaptainFinalEnd_MainEv"]
    pub(super) fn status_CaptainFinalEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_CaptainFinalEndEv"]
    pub(super) fn status_end_CaptainFinalEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_ChromFinalTargetDamageEv"]
    pub(super) fn status_pre_ChromFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_ChromFinalTargetDamageEv"]
    pub(super) fn status_ChromFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_ChromFinalTargetDamage_MainEv"]
    pub(super) fn status_ChromFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_ChromFinalTargetDamageEv"]
    pub(super) fn status_end_ChromFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusChromFinalTargetDamage_check_damageEv"]
    pub(super) fn FighterStatusChromFinalTargetDamage_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_DededeFinalTargetStartEv"]
    pub(super) fn status_DededeFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_DededeFinalTargetStart_MainEv"]
    pub(super) fn status_DededeFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_DededeFinalTargetStartEv"]
    pub(super) fn status_end_DededeFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon38sub_DededeFinalTargetStart_init_statusEv"]
    pub(super) fn sub_DededeFinalTargetStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_DededeFinalTargetDamageEv"]
    pub(super) fn status_pre_DededeFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_DededeFinalTargetDamageEv"]
    pub(super) fn status_DededeFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_DededeFinalTargetDamage_MainEv"]
    pub(super) fn status_DededeFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_DededeFinalTargetDamageEv"]
    pub(super) fn status_end_DededeFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_DededeFinalTargetEndEv"]
    pub(super) fn status_pre_DededeFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_DededeFinalTargetEndEv"]
    pub(super) fn status_DededeFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_DededeFinalTargetEnd_MainEv"]
    pub(super) fn status_DededeFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_DededeFinalTargetEndEv"]
    pub(super) fn status_end_DededeFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_DuckhuntFinalStartEv"]
    pub(super) fn status_DuckhuntFinalStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_DuckhuntFinalStart_MainEv"]
    pub(super) fn status_DuckhuntFinalStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_DuckhuntFinalStartEv"]
    pub(super) fn status_end_DuckhuntFinalStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_DuckhuntFinalStart_init_statusEv"]
    pub(super) fn sub_DuckhuntFinalStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_DuckhuntFinalFuraFuraEv"]
    pub(super) fn status_pre_DuckhuntFinalFuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_DuckhuntFinalFuraFuraEv"]
    pub(super) fn status_DuckhuntFinalFuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_DuckhuntFinalFuraFura_MainEv"]
    pub(super) fn status_DuckhuntFinalFuraFura_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_DuckhuntFinalFuraFuraEv"]
    pub(super) fn status_end_DuckhuntFinalFuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_DuckhuntFinalEndEv"]
    pub(super) fn status_pre_DuckhuntFinalEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_DuckhuntFinalEndEv"]
    pub(super) fn status_DuckhuntFinalEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_DuckhuntFinalEnd_MainEv"]
    pub(super) fn status_DuckhuntFinalEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_DuckhuntFinalEndEv"]
    pub(super) fn status_end_DuckhuntFinalEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_FalcoFinalTargetStartEv"]
    pub(super) fn status_FalcoFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_FalcoFinalTargetStart_MainEv"]
    pub(super) fn status_FalcoFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_FalcoFinalTargetStartEv"]
    pub(super) fn status_end_FalcoFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_FalcoFinalTargetStart_init_statusEv"]
    pub(super) fn sub_FalcoFinalTargetStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_FalcoFinalTargetDamageEv"]
    pub(super) fn status_pre_FalcoFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_FalcoFinalTargetDamageEv"]
    pub(super) fn status_FalcoFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_FalcoFinalTargetDamage_MainEv"]
    pub(super) fn status_FalcoFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_FalcoFinalTargetDamageEv"]
    pub(super) fn status_end_FalcoFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_FalcoFinalTargetEndEv"]
    pub(super) fn status_pre_FalcoFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_FalcoFinalTargetEndEv"]
    pub(super) fn status_FalcoFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_FalcoFinalTargetEnd_MainEv"]
    pub(super) fn status_FalcoFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_FalcoFinalTargetEndEv"]
    pub(super) fn status_end_FalcoFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_FinalDamageFlyMainEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn status_pre_FinalDamageFlyMain(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_FinalDamageFlyEv"]
    pub(super) fn status_pre_FinalDamageFly(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_FinalDamageFlyCloudEv"]
    pub(super) fn status_pre_FinalDamageFlyCloud(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_FinalDamageFlyLittlemacEv"]
    pub(super) fn status_pre_FinalDamageFlyLittlemac(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_FinalDamageFlyCommonEv"]
    pub(super) fn status_FinalDamageFlyCommon(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_FinalDamageFlyEv"]
    pub(super) fn status_FinalDamageFly(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_FinalDamageFlyRefletEv"]
    pub(super) fn status_FinalDamageFlyReflet(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_FinalDamageFly_MainEv"]
    pub(super) fn status_FinalDamageFly_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_end_FinalDamageFlyEv"]
    pub(super) fn status_end_FinalDamageFly(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessFinalDamageFly_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFly_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessFinalDamageFly_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFly_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessFinalDamageFly_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFly_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessFinalDamageFly_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFly_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessFinalDamageFly_exec_fix_cameraEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFly_exec_fix_camera(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessFinalDamageFly_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFly_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_pre_FinalDamageEv"]
    pub(super) fn status_pre_FinalDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18status_FinalDamageEv"]
    pub(super) fn status_FinalDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessFinalDamage_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamage_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_FinalDamageCloudEv"]
    pub(super) fn status_FinalDamageCloud(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_FinalDamageCloud_MainEv"]
    pub(super) fn status_FinalDamageCloud_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_FinalDamage_MainEv"]
    pub(super) fn status_FinalDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_end_FinalDamageEv"]
    pub(super) fn status_end_FinalDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_FinalDamageCloudEv"]
    pub(super) fn status_end_FinalDamageCloud(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon47FighterStatusUniqProcessFinalDamage_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamage_exec_status(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessFinalDamage_exec_fix_cameraEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamage_exec_fix_camera(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusUniqProcessFinalDamage_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessFinalDamage_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_FinalDamageFallEv"]
    pub(super) fn status_pre_FinalDamageFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_FinalDamageFallEv"]
    pub(super) fn status_FinalDamageFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_FinalDamageFallCommonEN3lib8L2CValueE"]
    pub(super) fn status_FinalDamageFallCommon(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessFinalDamageFall_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFall_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_FinalDamageFall_MainEv"]
    pub(super) fn status_FinalDamageFall_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_FinalDamageFallEv"]
    pub(super) fn status_end_FinalDamageFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessFinalDamageFall_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFall_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessFinalDamageFall_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFall_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessFinalDamageFall_exec_fix_cameraEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFall_exec_fix_camera(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessFinalDamageFall_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFall_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_KirbyFinalDamageEv"]
    pub(super) fn status_pre_KirbyFinalDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessKirbyFinalDamage_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessKirbyFinalDamage_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_FinalDamageFlyCloudEv"]
    pub(super) fn status_FinalDamageFlyCloud(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessFinalDamageFlyCloud_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyCloud_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_FinalDamageFlyCloud_MainEv"]
    pub(super) fn status_FinalDamageFlyCloud_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon52FighterStatusUniqProcessFinalDamageCloud_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageCloud_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessFinalDamageFlyCloud_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyCloud_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessFinalDamageFlyCloud_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyCloud_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessFinalDamageFlyCloud_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyCloud_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_FinalDamageFallCloudEv"]
    pub(super) fn status_pre_FinalDamageFallCloud(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_FinalDamageFallCloudEv"]
    pub(super) fn status_FinalDamageFallCloud(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_Final2DamageFallCloud_MainEv"]
    pub(super) fn status_Final2DamageFallCloud_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessFinalDamageFlyIke_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyIke_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon58FighterStatusUniqProcessFinalDamageFlyIke_init_status_mainEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyIke_init_status_main(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessFinalDamageFlyIke_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyIke_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessFinalDamageIke_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageIke_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessFinalDamageIke_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageIke_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessFinalDamageFlyKoopa_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyKoopa_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessFinalDamageFlyKoopa_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyKoopa_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_FinalDamageFlyLittlemacEv"]
    pub(super) fn status_FinalDamageFlyLittlemac(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_FinalDamageFlyLittlemac_MainEv"]
    pub(super) fn status_FinalDamageFlyLittlemac_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_FinalDamageLittlemacEv"]
    pub(super) fn status_FinalDamageLittlemac(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_FinalDamageLittlemacEv"]
    pub(super) fn status_end_FinalDamageLittlemac(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon59FighterStatusUniqProcessFinalDamageFlyLittlemac_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyLittlemac_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon59FighterStatusUniqProcessFinalDamageFlyLittlemac_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyLittlemac_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon59FighterStatusUniqProcessFinalDamageFlyLittlemac_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyLittlemac_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon56FighterStatusUniqProcessFinalDamageLittlemac_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageLittlemac_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_FinalDamageFlyPikachuEv"]
    pub(super) fn status_pre_FinalDamageFlyPikachu(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_FinalDamageFlyPikachuEv"]
    pub(super) fn status_FinalDamageFlyPikachu(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31is_finish_FinalDamageFlyPikachuEv"]
    pub(super) fn is_finish_FinalDamageFlyPikachu(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_FinalDamageFlyPikachu_MainEv"]
    pub(super) fn status_FinalDamageFlyPikachu_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_FinalDamageFlyPikachuEv"]
    pub(super) fn status_end_FinalDamageFlyPikachu(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_pre_FinalDamageRyuEv"]
    pub(super) fn status_pre_FinalDamageRyu(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon21status_FinalDamageRyuEv"]
    pub(super) fn status_FinalDamageRyu(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessFinalDamageRyu_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageRyu_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_FinalDamageRyu_MainEv"]
    pub(super) fn status_FinalDamageRyu_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_FinalDamageFlyRyuEv"]
    pub(super) fn status_pre_FinalDamageFlyRyu(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_FinalDamageFlyRyuEv"]
    pub(super) fn status_FinalDamageFlyRyu(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessFinalDamageFlyRyu_exec_fix_posEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyRyu_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_FinalDamageFlyRyu_MainEv"]
    pub(super) fn status_FinalDamageFlyRyu_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessFinalDamageRyu_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageRyu_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessFinalDamageRyu_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageRyu_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon50FighterStatusUniqProcessFinalDamageRyu_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageRyu_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessFinalDamageRyu_exec_fix_cameraEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageRyu_exec_fix_camera(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon51FighterStatusUniqProcessFinalDamageRyu_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessFinalDamageRyu_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessFinalDamageFlyRyu_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyRyu_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon53FighterStatusUniqProcessFinalDamageFlyRyu_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyRyu_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessFinalDamageFlyRyu_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyRyu_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessFinalDamageFlyZoroark_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyZoroark_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessFinalDamageFlyZoroark_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyZoroark_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessFinalDamageZoroark_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageZoroark_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessFinalDamageZoroark_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageZoroark_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon54FighterStatusUniqProcessFinalDamageZoroark_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageZoroark_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon55FighterStatusUniqProcessFinalDamageZoroark_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessFinalDamageZoroark_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_FinalTargetStartEv"]
    pub(super) fn status_pre_FinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_FinalTargetStartShulkEv"]
    pub(super) fn status_pre_FinalTargetStartShulk(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_FinalVisualAttackOtherEv"]
    pub(super) fn status_pre_FinalVisualAttackOther(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39status_pre_FinalVisualAttackOther_paramEN3lib8L2CValueE"]
    pub(super) fn status_pre_FinalVisualAttackOther_param(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_FinalVisualAttackOtherEv"]
    pub(super) fn status_FinalVisualAttackOther(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_captain_finalother_uniqEv"]
    pub(super) fn sub_captain_finalother_uniq(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_FinalVisualAttackOther_MainEv"]
    pub(super) fn status_FinalVisualAttackOther_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_FinalVisualAttackOtherEv"]
    pub(super) fn status_end_FinalVisualAttackOther(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35FinalVisualAttackOther_check_damageEv"]
    pub(super) fn FinalVisualAttackOther_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_FoxFinalTargetStartEv"]
    pub(super) fn status_FoxFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_FoxFinalTargetStart_MainEv"]
    pub(super) fn status_FoxFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_FoxFinalTargetStartEv"]
    pub(super) fn status_end_FoxFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_FoxFinalTargetStart_init_statusEv"]
    pub(super) fn sub_FoxFinalTargetStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_FoxFinalTargetDamageEv"]
    pub(super) fn status_pre_FoxFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_FoxFinalTargetDamageEv"]
    pub(super) fn status_FoxFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_FoxFinalTargetDamage_MainEv"]
    pub(super) fn status_FoxFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_FoxFinalTargetDamageEv"]
    pub(super) fn status_end_FoxFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_FoxFinalTargetEndEv"]
    pub(super) fn status_pre_FoxFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_FoxFinalTargetEndEv"]
    pub(super) fn status_FoxFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_FoxFinalTargetEnd_MainEv"]
    pub(super) fn status_FoxFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_FoxFinalTargetEndEv"]
    pub(super) fn status_end_FoxFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_GaogaenFinalTargetStartEv"]
    pub(super) fn status_GaogaenFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_GaogaenFinalTargetStart_MainEv"]
    pub(super) fn status_GaogaenFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_GaogaenFinalTargetStartEv"]
    pub(super) fn status_end_GaogaenFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_GaogaenFinalTargetStart_init_statusEv"]
    pub(super) fn sub_GaogaenFinalTargetStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon60FighterStatusUniqProcessGaogaenFinalTargetStart_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessGaogaenFinalTargetStart_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_pre_GaogaenFinalTargetDamageEv"]
    pub(super) fn status_pre_GaogaenFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_GaogaenFinalTargetDamageEv"]
    pub(super) fn status_GaogaenFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20update_thrown_offsetEv"]
    pub(super) fn update_thrown_offset(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_GaogaenFinalTargetDamage_MainEv"]
    pub(super) fn status_GaogaenFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_end_GaogaenFinalTargetDamageEv"]
    pub(super) fn status_end_GaogaenFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_GaogaenFinalTargetEndEv"]
    pub(super) fn status_pre_GaogaenFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_GaogaenFinalTargetEndEv"]
    pub(super) fn status_GaogaenFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_GaogaenFinalTargetEnd_MainEv"]
    pub(super) fn status_GaogaenFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_GaogaenFinalTargetEndEv"]
    pub(super) fn status_end_GaogaenFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_GekkougaFinalDamageEv"]
    pub(super) fn status_pre_GekkougaFinalDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_FinalDamageFlyGekkougaEv"]
    pub(super) fn status_FinalDamageFlyGekkouga(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_FinalDamageFlyGekkouga_MainEv"]
    pub(super) fn status_FinalDamageFlyGekkouga_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_FinalDamageFlyGekkougaEv"]
    pub(super) fn status_end_FinalDamageFlyGekkouga(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_GekkougaFinalDamageFallEv"]
    pub(super) fn status_pre_GekkougaFinalDamageFall(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_KamuiFinalStartEv"]
    pub(super) fn status_KamuiFinalStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_KamuiFinalStart_MainEv"]
    pub(super) fn status_KamuiFinalStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_KamuiFinalStartEv"]
    pub(super) fn status_end_KamuiFinalStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_KamuiFinalTargetStart_init_statusEv"]
    pub(super) fn sub_KamuiFinalTargetStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_KamuiFinalDamageEv"]
    pub(super) fn status_pre_KamuiFinalDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_KamuiFinalDamageEv"]
    pub(super) fn status_KamuiFinalDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_KamuiFinalDamage_MainEv"]
    pub(super) fn status_KamuiFinalDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_KamuiFinalDamageEv"]
    pub(super) fn status_end_KamuiFinalDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_KamuiFinalEndEv"]
    pub(super) fn status_pre_KamuiFinalEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_KamuiFinalEndEv"]
    pub(super) fn status_KamuiFinalEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_KamuiFinalEnd_MainEv"]
    pub(super) fn status_KamuiFinalEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_KamuiFinalEndEv"]
    pub(super) fn status_end_KamuiFinalEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_KroolFinalTargetStartEv"]
    pub(super) fn status_KroolFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_KroolFinalTargetStart_MainEv"]
    pub(super) fn status_KroolFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_KroolFinalTargetStartEv"]
    pub(super) fn status_end_KroolFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_KroolFinalTargetStart_init_statusEv"]
    pub(super) fn sub_KroolFinalTargetStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_KroolFinalTargetDamageEv"]
    pub(super) fn status_pre_KroolFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_KroolFinalTargetDamageEv"]
    pub(super) fn status_KroolFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_KroolFinalTargetDamage_MainEv"]
    pub(super) fn status_KroolFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_KroolFinalTargetDamageEv"]
    pub(super) fn status_end_KroolFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_KroolFinalTargetEndEv"]
    pub(super) fn status_pre_KroolFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_KroolFinalTargetEndEv"]
    pub(super) fn status_KroolFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_KroolFinalTargetEnd_MainEv"]
    pub(super) fn status_KroolFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_KroolFinalTargetEndEv"]
    pub(super) fn status_end_KroolFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_LuigiFinalVacuumEv"]
    pub(super) fn status_pre_LuigiFinalVacuum(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_LuigiFinalVacuumEv"]
    pub(super) fn status_LuigiFinalVacuum(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_LuigiFinalVacuum_MainEv"]
    pub(super) fn status_LuigiFinalVacuum_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_LuigiFinalVacuumEv"]
    pub(super) fn status_end_LuigiFinalVacuum(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_pre_LuigiFinalShootEv"]
    pub(super) fn status_pre_LuigiFinalShoot(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_LuigiFinalShootEv"]
    pub(super) fn status_LuigiFinalShoot(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_LuigiFinalShoot_MainEv"]
    pub(super) fn status_LuigiFinalShoot_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_LuigiFinalShoot_exec_fix_posEv"]
    pub(super) fn status_LuigiFinalShoot_exec_fix_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_LuigiFinalShootEv"]
    pub(super) fn status_end_LuigiFinalShoot(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon60FighterStatusUniqProcessFinalDamageFlyMetaknight_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyMetaknight_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon60FighterStatusUniqProcessFinalDamageFlyMetaknight_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFlyMetaknight_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessFinalDamageMetaknight_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageMetaknight_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_FinalDamageMetaknightEv"]
    pub(super) fn status_FinalDamageMetaknight(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessFinalDamageMetaknight_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageMetaknight_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessFinalDamageMetaknight_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageMetaknight_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon61FighterStatusUniqProcessFinalDamageFallMetaknight_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFallMetaknight_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon61FighterStatusUniqProcessFinalDamageFallMetaknight_exit_statusEv"]
    pub(super) fn FighterStatusUniqProcessFinalDamageFallMetaknight_exit_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_MurabitoFinalCaptureEv"]
    pub(super) fn status_pre_MurabitoFinalCapture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_MurabitoFinalCaptureEv"]
    pub(super) fn status_MurabitoFinalCapture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_MurabitoFinalCapture_MainEv"]
    pub(super) fn status_MurabitoFinalCapture_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_MurabitoFinalCaptureEv"]
    pub(super) fn status_end_MurabitoFinalCapture(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_PacmanFinalEatenEv"]
    pub(super) fn status_pre_PacmanFinalEaten(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_PacmanFinalEatenEv"]
    pub(super) fn status_PacmanFinalEaten(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_PacmanFinalEaten_MainEv"]
    pub(super) fn status_PacmanFinalEaten_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_PacmanFinalEatenEv"]
    pub(super) fn status_end_PacmanFinalEaten(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_PalutenaFinalBlackholeEv"]
    pub(super) fn status_pre_PalutenaFinalBlackhole(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_PalutenaFinalBlackholeEv"]
    pub(super) fn status_PalutenaFinalBlackhole(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_PalutenaFinalBlackhole_MainEv"]
    pub(super) fn status_PalutenaFinalBlackhole_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_PalutenaFinalBlackholeEv"]
    pub(super) fn status_end_PalutenaFinalBlackhole(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_RefletFinalDamageEv"]
    pub(super) fn status_pre_RefletFinalDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_RidleyFinalTargetStartEv"]
    pub(super) fn status_RidleyFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_RidleyFinalTargetStart_MainEv"]
    pub(super) fn status_RidleyFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_RidleyFinalTargetStartEv"]
    pub(super) fn status_end_RidleyFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_pre_RidleyFinalTargetDamageEv"]
    pub(super) fn status_pre_RidleyFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_RidleyFinalTargetDamageEv"]
    pub(super) fn status_RidleyFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35status_RidleyFinalTargetDamage_MainEv"]
    pub(super) fn status_RidleyFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_end_RidleyFinalTargetDamageEv"]
    pub(super) fn status_end_RidleyFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusRidleyFinalTargetDamage_check_damageEv"]
    pub(super) fn FighterStatusRidleyFinalTargetDamage_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_pre_RidleyFinalTargetEndEv"]
    pub(super) fn status_pre_RidleyFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_RidleyFinalTargetEndEv"]
    pub(super) fn status_RidleyFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_RidleyFinalTargetEnd_MainEv"]
    pub(super) fn status_RidleyFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_end_RidleyFinalTargetEndEv"]
    pub(super) fn status_end_RidleyFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon74FighterStatusUniqProcessRockmanFinalTarget_get_final_actor_module_accessorEv"]
    pub(super) fn FighterStatusUniqProcessRockmanFinalTarget_get_final_actor_module_accessor(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_pre_RockmanFinalTargetScene01Ev"]
    pub(super) fn status_pre_RockmanFinalTargetScene01(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_RockmanFinalTargetScene01Ev"]
    pub(super) fn status_RockmanFinalTargetScene01(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37status_RockmanFinalTargetScene01_MainEv"]
    pub(super) fn status_RockmanFinalTargetScene01_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_end_RockmanFinalTargetScene01Ev"]
    pub(super) fn status_end_RockmanFinalTargetScene01(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon61FighterStatusUniqProcessRockmanFinalTargetScene01_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessRockmanFinalTargetScene01_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessRockmanFinalTargetScene01_set_posEv"]
    pub(super) fn FighterStatusUniqProcessRockmanFinalTargetScene01_set_pos(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_pre_RockmanFinalTargetScene02Ev"]
    pub(super) fn status_pre_RockmanFinalTargetScene02(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_RockmanFinalTargetScene02Ev"]
    pub(super) fn status_RockmanFinalTargetScene02(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37status_RockmanFinalTargetScene02_MainEv"]
    pub(super) fn status_RockmanFinalTargetScene02_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_end_RockmanFinalTargetScene02Ev"]
    pub(super) fn status_end_RockmanFinalTargetScene02(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon61FighterStatusUniqProcessRockmanFinalTargetScene02_exec_statusEv"]
    pub(super) fn FighterStatusUniqProcessRockmanFinalTargetScene02_exec_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon69FighterStatusUniqProcessRockmanFinalTargetScene02_set_damage_reactionEv"]
    pub(super) fn FighterStatusUniqProcessRockmanFinalTargetScene02_set_damage_reaction(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon72FighterStatusUniqProcessRockmanFinalTargetScene02_get_damage_motion_randEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessRockmanFinalTargetScene02_get_damage_motion_rand(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon62FighterStatusUniqProcessRockmanFinalTargetScene02_check_damageEN3lib8L2CValueE"]
    pub(super) fn FighterStatusUniqProcessRockmanFinalTargetScene02_check_damage(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_pre_RockmanFinalTargetEndEv"]
    pub(super) fn status_pre_RockmanFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_RockmanFinalTargetEndEv"]
    pub(super) fn status_RockmanFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_RockmanFinalTargetEnd_MainEv"]
    pub(super) fn status_RockmanFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_RockmanFinalTargetEndEv"]
    pub(super) fn status_end_RockmanFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon57FighterStatusUniqProcessRockmanFinalTargetEnd_init_statusEv"]
    pub(super) fn FighterStatusUniqProcessRockmanFinalTargetEnd_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_pre_SheikFinalDamageEv"]
    pub(super) fn status_pre_SheikFinalDamage(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon23status_SheikFinalDamageEv"]
    pub(super) fn status_SheikFinalDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_SheikFinalDamage_UniqEN3lib8L2CValueE"]
    pub(super) fn status_SheikFinalDamage_Uniq(this: *mut L2CFighterCommon, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_SheikFinalDamage_MainEv"]
    pub(super) fn status_SheikFinalDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_end_SheikFinalDamageEv"]
    pub(super) fn status_end_SheikFinalDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon22status_ShulkFinalStartEv"]
    pub(super) fn status_ShulkFinalStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27status_ShulkFinalStart_MainEv"]
    pub(super) fn status_ShulkFinalStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_end_ShulkFinalStartEv"]
    pub(super) fn status_end_ShulkFinalStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_ShulkFinalStart_init_statusEv"]
    pub(super) fn sub_ShulkFinalStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_pre_ShulkFinalFuraFuraEv"]
    pub(super) fn status_pre_ShulkFinalFuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_ShulkFinalFuraFuraEv"]
    pub(super) fn status_ShulkFinalFuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_ShulkFinalFuraFura_MainEv"]
    pub(super) fn status_ShulkFinalFuraFura_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_ShulkFinalFuraFuraEv"]
    pub(super) fn status_end_ShulkFinalFuraFura(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_pre_ShulkFinalEndEv"]
    pub(super) fn status_pre_ShulkFinalEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon20status_ShulkFinalEndEv"]
    pub(super) fn status_ShulkFinalEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25status_ShulkFinalEnd_MainEv"]
    pub(super) fn status_ShulkFinalEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_end_ShulkFinalEndEv"]
    pub(super) fn status_end_ShulkFinalEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_SimonFinalTargetStartEv"]
    pub(super) fn status_SimonFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_SimonFinalTargetStart_MainEv"]
    pub(super) fn status_SimonFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_SimonFinalTargetStartEv"]
    pub(super) fn status_end_SimonFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_SimonFinalTargetDamageEv"]
    pub(super) fn status_pre_SimonFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_SimonFinalTargetDamageEv"]
    pub(super) fn status_SimonFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_SimonFinalTargetDamage_MainEv"]
    pub(super) fn status_SimonFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_SimonFinalTargetDamageEv"]
    pub(super) fn status_end_SimonFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_SimonFinalTargetEndEv"]
    pub(super) fn status_pre_SimonFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_SimonFinalTargetEndEv"]
    pub(super) fn status_SimonFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_SimonFinalTargetEnd_MainEv"]
    pub(super) fn status_SimonFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_SimonFinalTargetEndEv"]
    pub(super) fn status_end_SimonFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_uniq_process_pre_link_finalEv"]
    pub(super) fn sub_uniq_process_pre_link_final(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_link_final_initEv"]
    pub(super) fn sub_uniq_process_link_final_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_link_final_mainEv"]
    pub(super) fn sub_uniq_process_link_final_main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_uniq_process_link_final_exec_fix_posEv"]
    pub(super) fn sub_uniq_process_link_final_exec_fix_pos(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon16sub_check_cancelEv"]
    pub(super) fn sub_check_cancel(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48sub_uniq_process_link_final_exec_fix_pos_counterEv"]
    pub(super) fn sub_uniq_process_link_final_exec_fix_pos_counter(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32sub_uniq_process_link_final_exitEv"]
    pub(super) fn sub_uniq_process_link_final_exit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_get_random_damage_motionEv"]
    pub(super) fn sub_get_random_damage_motion(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26sub_is_link_final_capturedEv"]
    pub(super) fn sub_is_link_final_captured(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_pre_LinkFinalArrowHitEv"]
    pub(super) fn status_pre_LinkFinalArrowHit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon24status_LinkFinalArrowHitEv"]
    pub(super) fn status_LinkFinalArrowHit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_LinkFinalArrowHit_MainEv"]
    pub(super) fn status_LinkFinalArrowHit_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_end_LinkFinalArrowHitEv"]
    pub(super) fn status_end_LinkFinalArrowHit(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_WarioFinalTargetStartEv"]
    pub(super) fn status_WarioFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_WarioFinalTargetStart_MainEv"]
    pub(super) fn status_WarioFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_WarioFinalTargetStartEv"]
    pub(super) fn status_end_WarioFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_WarioFinalTargetStart_init_statusEv"]
    pub(super) fn sub_WarioFinalTargetStart_init_status(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_WarioFinalTargetDamageEv"]
    pub(super) fn status_pre_WarioFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_WarioFinalTargetDamageEv"]
    pub(super) fn status_WarioFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_WarioFinalTargetDamage_MainEv"]
    pub(super) fn status_WarioFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_WarioFinalTargetDamageEv"]
    pub(super) fn status_end_WarioFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusWarioFinalTargetDamage_check_damageEv"]
    pub(super) fn FighterStatusWarioFinalTargetDamage_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_WarioFinalTargetEndEv"]
    pub(super) fn status_pre_WarioFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_WarioFinalTargetEndEv"]
    pub(super) fn status_WarioFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_WarioFinalTargetEnd_MainEv"]
    pub(super) fn status_WarioFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_WarioFinalTargetEndEv"]
    pub(super) fn status_end_WarioFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_YoshiFinalTargetStartEv"]
    pub(super) fn status_YoshiFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_YoshiFinalTargetStart_MainEv"]
    pub(super) fn status_YoshiFinalTargetStart_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon32status_end_YoshiFinalTargetStartEv"]
    pub(super) fn status_end_YoshiFinalTargetStart(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_pre_YoshiFinalTargetDamageEv"]
    pub(super) fn status_pre_YoshiFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_YoshiFinalTargetDamageEv"]
    pub(super) fn status_YoshiFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34status_YoshiFinalTargetDamage_MainEv"]
    pub(super) fn status_YoshiFinalTargetDamage_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33status_end_YoshiFinalTargetDamageEv"]
    pub(super) fn status_end_YoshiFinalTargetDamage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusYoshiFinalTargetDamage_check_damageEv"]
    pub(super) fn FighterStatusYoshiFinalTargetDamage_check_damage(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_pre_YoshiFinalTargetEndEv"]
    pub(super) fn status_pre_YoshiFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon26status_YoshiFinalTargetEndEv"]
    pub(super) fn status_YoshiFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31status_YoshiFinalTargetEnd_MainEv"]
    pub(super) fn status_YoshiFinalTargetEnd_Main(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon30status_end_YoshiFinalTargetEndEv"]
    pub(super) fn status_end_YoshiFinalTargetEnd(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon27sub_fighter_common_settingsEv"]
    pub(super) fn sub_fighter_common_settings(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon19global_fighter_initEv"]
    pub(super) fn global_fighter_init(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37local_func__fighter_global_variable_1Ev"]
    pub(super) fn local_func__fighter_global_variable_1(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37local_func__fighter_global_variable_2Ev"]
    pub(super) fn local_func__fighter_global_variable_2(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sub_set_fighter_common_tableEv"]
    pub(super) fn sub_set_fighter_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_set_status_line_msc_common_tableEv"]
    pub(super) fn sub_set_status_line_msc_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_set_status_pre_msc_common_tableEv"]
    pub(super) fn sub_set_status_pre_msc_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_set_status_main_msc_common_tableEv"]
    pub(super) fn sub_set_status_main_msc_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_set_status_end_msc_common_tableEv"]
    pub(super) fn sub_set_status_end_msc_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_set_init_status_msc_common_tableEv"]
    pub(super) fn sub_set_init_status_msc_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_set_exec_status_msc_common_tableEv"]
    pub(super) fn sub_set_exec_status_msc_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon34sub_set_exec_stop_msc_common_tableEv"]
    pub(super) fn sub_set_exec_stop_msc_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_set_exec_status_post_msc_common_tableEv"]
    pub(super) fn sub_set_exec_status_post_msc_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36sub_set_exit_status_msc_common_tableEv"]
    pub(super) fn sub_set_exit_status_msc_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon39sub_set_map_correction_msc_common_tableEv"]
    pub(super) fn sub_set_map_correction_msc_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon37sub_set_fix_pos_slow_msc_common_tableEv"]
    pub(super) fn sub_set_fix_pos_slow_msc_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon35sub_set_fix_camera_msc_common_tableEv"]
    pub(super) fn sub_set_fix_camera_msc_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_set_check_damage_common_tableEv"]
    pub(super) fn sub_set_check_damage_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_set_check_attack_common_tableEv"]
    pub(super) fn sub_set_check_attack_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon33sub_set_on_change_lr_common_tableEv"]
    pub(super) fn sub_set_on_change_lr_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_set_leave_stop_common_tableEv"]
    pub(super) fn sub_set_leave_stop_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon41sub_set_notify_event_gimmick_common_tableEv"]
    pub(super) fn sub_set_notify_event_gimmick_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon40sub_set_status_uniq_process_common_tableEv"]
    pub(super) fn sub_set_status_uniq_process_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon46sub_set_uniq_process_final_damage_common_tableEv"]
    pub(super) fn sub_set_uniq_process_final_damage_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sub_set_calc_param_common_tableEv"]
    pub(super) fn sub_set_calc_param_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon18sub_set_item_shootEv"]
    pub(super) fn sub_set_item_shoot(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon25sub_set_func_common_tableEv"]
    pub(super) fn sub_set_func_common_table(this: *mut L2CFighterCommon);

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28sys_line_waza_customize_initEv"]
    pub(super) fn sys_line_waza_customize_init(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon31sys_line_waza_customize_controlEv"]
    pub(super) fn sys_line_waza_customize_control(this: *mut L2CFighterCommon) -> lib::L2CValueHack;

}