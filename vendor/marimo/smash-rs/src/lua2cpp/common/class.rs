use std::ops::{Deref, DerefMut};

use crate::*;
use super::cpp::*;

#[repr(C)]
pub struct L2CFighterCommon {
    fighter_base:                     lua2cpp::L2CFighterBase,
    pub attack_info:                  lib::L2CValue,
    pub damage_info1:                 lib::L2CValue,
    pub damage_info2:                 lib::L2CValue,
    pub guard_info:                   lib::L2CValue,
    pub item_info1:                   lib::L2CValue,
    pub item_info2:                   lib::L2CValue,
    pub ladder_info1:                 lib::L2CValue,
    pub ladder_info2:                 lib::L2CValue,
    pub swallowed_info1:              lib::L2CValue,
    pub swallowed_info2:              lib::L2CValue,
    pub swallowed_info3:              lib::L2CValue,
    pub swim_info1:                   lib::L2CValue,
    pub swim_info2:                   lib::L2CValue,
    pub bayonetta_final_target_info1: lib::L2CValue,
    pub bayonetta_final_target_info2: lib::L2CValue,
    pub final_damage_info:            lib::L2CValue,
    pub rockman_final_target_info:    lib::L2CValue
}

impl Deref for L2CFighterCommon {
    type Target = lua2cpp::L2CFighterBase;
    
    fn deref(&self) -> &Self::Target {
        &self.fighter_base
    }
}

impl DerefMut for L2CFighterCommon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fighter_base
    }
}

impl L2CFighterCommon {
    pub fn main_func__fighter_status_attack(&mut self) {
        unsafe {
            main_func__fighter_status_attack(self)
        }
    }

    pub fn main_func__fighter_status_damage(&mut self) {
        unsafe {
            main_func__fighter_status_damage(self)
        }
    }

    pub fn main_func__fighter_status_guard(&mut self) {
        unsafe {
            main_func__fighter_status_guard(self)
        }
    }

    pub fn main_func__fighter_status_item(&mut self) {
        unsafe {
            main_func__fighter_status_item(self)
        }
    }

    pub fn main_func__fighter_status_ladder(&mut self) {
        unsafe {
            main_func__fighter_status_ladder(self)
        }
    }

    pub fn main_func__fighter_status_swallowed(&mut self) {
        unsafe {
            main_func__fighter_status_swallowed(self)
        }
    }

    pub fn main_func__fighter_status_swim(&mut self) {
        unsafe {
            main_func__fighter_status_swim(self)
        }
    }

    pub fn main_func__fighter_status_bayonetta_final_target(&mut self) {
        unsafe {
            main_func__fighter_status_bayonetta_final_target(self)
        }
    }

    pub fn local_func__fighter_status_final_damage_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_final_damage_1(self).into()
        }
    }

    pub fn local_func__fighter_status_rockman_final_target_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_rockman_final_target_1(self).into()
        }
    }

    pub fn local_func__fighter_status_bayonetta_final_target_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_bayonetta_final_target_1(self).into()
        }
    }

    pub fn local_func__fighter_status_bayonetta_final_target_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_bayonetta_final_target_2(self).into()
        }
    }

    pub fn local_func__fighter_status_item_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_1(self).into()
        }
    }

    pub fn local_func__fighter_status_item_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_2(self).into()
        }
    }

    pub fn FighterStatusGuard__landing_effect_control(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusGuard__landing_effect_control(self).into()
        }
    }

    pub fn FighterStatusGuard__check_hit_stop_delay_flick(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusGuard__check_hit_stop_delay_flick(self, arg1.into()).into()
        }
    }

    pub fn local_func__fighter_status_guard_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_guard_2(self).into()
        }
    }

    pub fn FighterStatusGuard__check_hit_stop_delay(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusGuard__check_hit_stop_delay(self, arg1.into()).into()
        }
    }

    pub fn local_func__fighter_status_guard_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_guard_1(self).into()
        }
    }

    pub fn FighterStatusGuard__is_continue_just_shield_count(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusGuard__is_continue_just_shield_count(self).into()
        }
    }

    pub fn FighterStatusGuard__set_just_shield_scale(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusGuard__set_just_shield_scale(self).into()
        }
    }

    pub fn FighterStatusGuard__calc_shield_scale(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusGuard__calc_shield_scale(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusGuard__set_guard_blend_motion(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusGuard__set_guard_blend_motion(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into()).into()
        }
    }

    pub fn FighterStatusGuard__set_guard_blend_motion_angle(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusGuard__set_guard_blend_motion_angle(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn FighterStatusDamage__effect_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__effect_exit_status(self).into()
        }
    }

    pub fn FighterStatusDamage__req_fly_roll_smoke_first(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__req_fly_roll_smoke_first(self).into()
        }
    }

    pub fn sub_update_damage_fly_effect(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_update_damage_fly_effect(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into()).into()
        }
    }

    pub fn FighterStatusDamage__check_smoke_effect(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__check_smoke_effect(self).into()
        }
    }

    pub fn FighterStatusDamage__is_enable_damage_fly_effect(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__is_enable_damage_fly_effect(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn FighterStatusDamage__get_dolly_stadium_wall_break_speed(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__get_dolly_stadium_wall_break_speed(self).into()
        }
    }

    pub fn FighterStatusDamage__check_dolly_stadium_wall_break_to_death(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__check_dolly_stadium_wall_break_to_death(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_2(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_1(self).into()
        }
    }

    pub fn FighterStatusDamage__set_delay_motion_interpolation(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__set_delay_motion_interpolation(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusDamage__get_damage_stop_motion_intp_frame(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__get_damage_stop_motion_intp_frame(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusDamage__is_elec_or_paralyze_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__is_elec_or_paralyze_damage(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusDamage__requestVectorAdjustEffect(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__requestVectorAdjustEffect(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into()).into()
        }
    }

    pub fn FighterStatusDamage__correctDamageVectorEffect(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__correctDamageVectorEffect(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusDamage__correctDamageVectorCommon(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__correctDamageVectorCommon(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusDamage__correctDamageVector(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusDamage__correctDamageVector(self).into()
        }
    }

    pub fn local_func__fighter_status_attack_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_attack_1(self).into()
        }
    }

    pub fn sys_line_system_init(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_system_init(self).into()
        }
    }

    pub fn sub_begin_added_lines(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_begin_added_lines(self, arg1.into())
        }
    }

    pub fn sys_line_status_end_control(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_status_end_control(self).into()
        }
    }

    pub fn sub_end_added_lines(&mut self) {
        unsafe {
            sub_end_added_lines(self)
        }
    }

    pub fn RESET(&mut self) {
        unsafe {
            RESET(self)
        }
    }

    pub fn global_fighter_reset(&mut self) {
        unsafe {
            global_fighter_reset(self)
        }
    }

    pub fn sub_fighter_pre_end_status(&mut self) {
        unsafe {
            sub_fighter_pre_end_status(self)
        }
    }

    pub fn sys_line_system_control_fighter(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_system_control_fighter(self).into()
        }
    }

    pub fn call_calc_param(&mut self) -> lib::L2CValue {
        unsafe {
            call_calc_param(self).into()
        }
    }

    pub fn call_notify_event_gimmick(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            call_notify_event_gimmick(self, arg1.into()).into()
        }
    }

    pub fn call_leave_stop(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            call_leave_stop(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn call_on_change_lr(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            call_on_change_lr(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn call_check_attack(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue, arg8: lib::L2CValue, arg9: lib::L2CValue, arg10: lib::L2CValue, arg11: lib::L2CValue, arg12: lib::L2CValue, arg13: lib::L2CValue, arg14: lib::L2CValue, arg15: lib::L2CValue, arg16: lib::L2CValue, arg17: lib::L2CValue, arg18: lib::L2CValue, arg19: lib::L2CValue, arg20: lib::L2CValue, arg21: lib::L2CValue, arg22: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            call_check_attack(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into(), arg8.into(), arg9.into(), arg10.into(), arg11.into(), arg12.into(), arg13.into(), arg14.into(), arg15.into(), arg16.into(), arg17.into(), arg18.into(), arg19.into(), arg20.into(), arg21.into(), arg22.into()).into()
        }
    }

    pub fn local_func__fighter_common_line_system_2(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue, arg8: lib::L2CValue, arg9: lib::L2CValue, arg10: lib::L2CValue, arg11: lib::L2CValue, arg12: lib::L2CValue, arg13: lib::L2CValue, arg14: lib::L2CValue, arg15: lib::L2CValue, arg16: lib::L2CValue, arg17: lib::L2CValue, arg18: lib::L2CValue, arg19: lib::L2CValue, arg20: lib::L2CValue, arg21: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__fighter_common_line_system_2(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into(), arg8.into(), arg9.into(), arg10.into(), arg11.into(), arg12.into(), arg13.into(), arg14.into(), arg15.into(), arg16.into(), arg17.into(), arg18.into(), arg19.into(), arg20.into(), arg21.into()).into()
        }
    }

    pub fn local_func__fighter_common_line_system_1(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__fighter_common_line_system_1(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn call_check_damage(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue, arg8: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            call_check_damage(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into(), arg8.into()).into()
        }
    }

    pub fn main_func__fighter_status_final_damage(&mut self) {
        unsafe {
            main_func__fighter_status_final_damage(self)
        }
    }

    pub fn main_func__fighter_status_rockman_final_target(&mut self) {
        unsafe {
            main_func__fighter_status_rockman_final_target(self)
        }
    }

    pub fn status_pre_PackunFinalCapture(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_PackunFinalCapture(self).into()
        }
    }

    pub fn status_PackunFinalCapture(&mut self) -> lib::L2CValue {
        unsafe {
            status_PackunFinalCapture(self).into()
        }
    }

    pub fn sub_shift_status_main(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_shift_status_main(self, arg1.into()).into()
        }
    }

    pub fn status_PackunFinalCapture_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_PackunFinalCapture_Main(self).into()
        }
    }

    pub fn status_end_PackunFinalCapture(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_PackunFinalCapture(self).into()
        }
    }

    pub fn FighterStatusPackunFinalCapture_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusPackunFinalCapture_check_damage(self).into()
        }
    }

    pub fn sub_get_motion_kind_CaptureJackWire(&mut self) -> lib::L2CValue {
        unsafe {
            sub_get_motion_kind_CaptureJackWire(self).into()
        }
    }

    pub fn status_pre_CaptureJackWire(&mut self) {
        unsafe {
            status_pre_CaptureJackWire(self)
        }
    }

    pub fn status_pre_CapturePulled(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CapturePulled(self).into()
        }
    }

    pub fn status_pre_CapturePulled_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_CapturePulled_common(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn status_CaptureJackWire(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureJackWire(self).into()
        }
    }

    pub fn CapturePulledCommon(&mut self) {
        unsafe {
            CapturePulledCommon(self)
        }
    }

    pub fn status_CaptureJackWire_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureJackWire_Main(self).into()
        }
    }

    pub fn sub_check_capture_cut(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_check_capture_cut(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn FighterStatusCaptrue_set_correct_ground(&mut self) {
        unsafe {
            FighterStatusCaptrue_set_correct_ground(self)
        }
    }

    pub fn sub_CaptureJackWire_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_CaptureJackWire_uniq_process_init(self).into()
        }
    }

    pub fn sub_capture_pulled_uniq_process_init_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_capture_pulled_uniq_process_init_common(self, arg1.into())
        }
    }

    pub fn local_func__fighter_status_capture_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_1(self).into()
        }
    }

    pub fn sub_capture_pulled_uniq_process_init_core(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            sub_capture_pulled_uniq_process_init_core(self, arg1.into(), arg2.into())
        }
    }

    pub fn getMotionKind(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            getMotionKind(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn sub_capture_pulled_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_pulled_uniq_process_init(self).into()
        }
    }

    pub fn sub_CaptureJackWire_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_CaptureJackWire_uniq_process_exec(self).into()
        }
    }

    pub fn sub_capture_uniq_process_exec_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) {
        unsafe {
            sub_capture_uniq_process_exec_common(self, arg1.into(), arg2.into(), arg3.into(), arg4.into())
        }
    }

    pub fn CaptureCommonJumpButtonCount(&mut self) {
        unsafe {
            CaptureCommonJumpButtonCount(self)
        }
    }

    pub fn status_end_CaptureJackWire(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureJackWire(self).into()
        }
    }

    pub fn status_end_CapturePulled(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CapturePulled(self).into()
        }
    }

    pub fn status_pre_JackFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_JackFinalTargetStart(self).into()
        }
    }

    pub fn status_pre_FinalTargetStartMain(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_FinalTargetStartMain(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn status_JackFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_JackFinalTargetStart(self).into()
        }
    }

    pub fn status_JackFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_JackFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_JackFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_JackFinalTargetStart(self).into()
        }
    }

    pub fn status_pre_JackFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_JackFinalTargetDamage(self).into()
        }
    }

    pub fn sub_JackFinalTargetDamage_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_JackFinalTargetDamage_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_JackFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_JackFinalTargetDamage(self).into()
        }
    }

    pub fn status_JackFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_JackFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_JackFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_JackFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusJackFinalTargetDamage_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusJackFinalTargetDamage_check_damage(self).into()
        }
    }

    pub fn status_pre_JackFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_JackFinalTargetEnd(self).into()
        }
    }

    pub fn status_JackFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_JackFinalTargetEnd(self).into()
        }
    }

    pub fn status_JackFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_JackFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_JackFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_JackFinalTargetEnd(self).into()
        }
    }

    pub fn status_BraveFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_BraveFinalTargetStart(self).into()
        }
    }

    pub fn status_BraveFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BraveFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_BraveFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BraveFinalTargetStart(self).into()
        }
    }

    pub fn status_pre_BraveFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BraveFinalTargetDamage(self).into()
        }
    }

    pub fn status_BraveFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_BraveFinalTargetDamage(self).into()
        }
    }

    pub fn status_BraveFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BraveFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_BraveFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BraveFinalTargetDamage(self).into()
        }
    }

    pub fn status_pre_BraveFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BraveFinalTargetEnd(self).into()
        }
    }

    pub fn status_BuddyFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_BuddyFinalTargetStart(self).into()
        }
    }

    pub fn status_BuddyFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BuddyFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_BuddyFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BuddyFinalTargetStart(self).into()
        }
    }

    pub fn sub_BuddyFinalTargetStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_BuddyFinalTargetStart_init_status(self).into()
        }
    }

    pub fn status_pre_BuddyFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BuddyFinalTargetDamage(self).into()
        }
    }

    pub fn status_BuddyFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_BuddyFinalTargetDamage(self).into()
        }
    }

    pub fn status_BuddyFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BuddyFinalTargetDamage_Main(self).into()
        }
    }

    pub fn FighterStatusBuddyFinalTargetDamage_exec_fix_camera(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusBuddyFinalTargetDamage_exec_fix_camera(self).into()
        }
    }

    pub fn status_end_BuddyFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BuddyFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusBuddyFinalTargetDamage_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusBuddyFinalTargetDamage_check_damage(self).into()
        }
    }

    pub fn status_pre_BuddyFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BuddyFinalTargetEnd(self).into()
        }
    }

    pub fn status_BuddyFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_BuddyFinalTargetEnd(self).into()
        }
    }

    pub fn status_BuddyFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BuddyFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_BuddyFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BuddyFinalTargetEnd(self).into()
        }
    }

    pub fn status_pre_DollySuperSpecial2Capture(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DollySuperSpecial2Capture(self).into()
        }
    }

    pub fn status_DollySuperSpecial2Capture(&mut self) -> lib::L2CValue {
        unsafe {
            status_DollySuperSpecial2Capture(self).into()
        }
    }

    pub fn status_DollySuperSpecial2Capture_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DollySuperSpecial2Capture_Main(self).into()
        }
    }

    pub fn sub_DollySuperSpecial2Capture_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_DollySuperSpecial2Capture_uniq_process_init(self).into()
        }
    }

    pub fn sub_DollySuperSpecial2Capture_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_DollySuperSpecial2Capture_uniq_process_exec(self).into()
        }
    }

    pub fn status_end_DollySuperSpecial2Capture(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DollySuperSpecial2Capture(self).into()
        }
    }

    pub fn sub_DollySuperSpecial2Capture_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_DollySuperSpecial2Capture_uniq_process_exit(self).into()
        }
    }

    pub fn sub_DollySuperSpecial2Capture_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_DollySuperSpecial2Capture_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_DollyFinalTargetWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DollyFinalTargetWait(self).into()
        }
    }

    pub fn status_DollyFinalTargetWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_DollyFinalTargetWait(self).into()
        }
    }

    pub fn status_DollyFinalTargetWait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DollyFinalTargetWait_Main(self).into()
        }
    }

    pub fn status_end_DollyFinalTargetWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DollyFinalTargetWait(self).into()
        }
    }

    pub fn FighterStatusDollyFinalTargetWait_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusDollyFinalTargetWait_check_damage(self).into()
        }
    }

    pub fn status_pre_DollyFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DollyFinalTargetStart(self).into()
        }
    }

    pub fn status_DollyFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_DollyFinalTargetStart(self).into()
        }
    }

    pub fn status_DollyFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DollyFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_DollyFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DollyFinalTargetStart(self).into()
        }
    }

    pub fn status_pre_DollyFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DollyFinalTargetDamage(self).into()
        }
    }

    pub fn status_DollyFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_DollyFinalTargetDamage(self).into()
        }
    }

    pub fn status_DollyFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DollyFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_DollyFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DollyFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusDollyFinalTargetDamage_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusDollyFinalTargetDamage_check_damage(self).into()
        }
    }

    pub fn status_pre_DollyFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DollyFinalTargetEnd(self).into()
        }
    }

    pub fn status_DollyFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_DollyFinalTargetEnd(self).into()
        }
    }

    pub fn status_DollyFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DollyFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_DollyFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DollyFinalTargetEnd(self).into()
        }
    }

    pub fn status_pre_CaptureMasterSword(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureMasterSword(self).into()
        }
    }

    pub fn status_CaptureMasterSword(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureMasterSword(self).into()
        }
    }

    pub fn status_CaptureMasterSword_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureMasterSword_Main(self).into()
        }
    }

    pub fn status_end_CaptureMasterSword(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureMasterSword(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureMasterSword_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureMasterSword_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_MasterFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_MasterFinalTargetStart(self).into()
        }
    }

    pub fn status_MasterFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_MasterFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_MasterFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MasterFinalTargetStart(self).into()
        }
    }

    pub fn sub_MasterFinalTargetStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_MasterFinalTargetStart_init_status(self).into()
        }
    }

    pub fn status_pre_MasterFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_MasterFinalTargetDamage(self).into()
        }
    }

    pub fn status_MasterFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_MasterFinalTargetDamage(self).into()
        }
    }

    pub fn status_MasterFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_MasterFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_MasterFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MasterFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusMasterFinalTargetDamage_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusMasterFinalTargetDamage_check_damage(self).into()
        }
    }

    pub fn status_pre_MasterFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_MasterFinalTargetEnd(self).into()
        }
    }

    pub fn status_MasterFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_MasterFinalTargetEnd(self).into()
        }
    }

    pub fn status_MasterFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_MasterFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_MasterFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MasterFinalTargetEnd(self).into()
        }
    }

    pub fn local_func__fighter_status_tantan_final_target_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_tantan_final_target_1(self).into()
        }
    }

    pub fn local_func__fighter_status_tantan_final_target_1_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_tantan_final_target_1_1(self).into()
        }
    }

    pub fn local_func__fighter_status_tantan_final_target_2_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_tantan_final_target_2_1(self).into()
        }
    }

    pub fn local_func__fighter_status_tantan_final_target_3_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_tantan_final_target_3_1(self).into()
        }
    }

    pub fn status_pre_TantanFinalTargetStart(&mut self) {
        unsafe {
            status_pre_TantanFinalTargetStart(self)
        }
    }

    pub fn status_pre_FinalTargetStartFox(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalTargetStartFox(self).into()
        }
    }

    pub fn status_TantanFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_TantanFinalTargetStart(self).into()
        }
    }

    pub fn status_TantanFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TantanFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_TantanFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TantanFinalTargetStart(self).into()
        }
    }

    pub fn sub_TantanFinalTargetStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_TantanFinalTargetStart_init_status(self).into()
        }
    }

    pub fn status_pre_TantanFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TantanFinalTargetDamage(self).into()
        }
    }

    pub fn status_TantanFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_TantanFinalTargetDamage(self).into()
        }
    }

    pub fn status_TantanFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TantanFinalTargetDamage_Main(self).into()
        }
    }

    pub fn local_func__fighter_status_tantan_final_target_4_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_tantan_final_target_4_1(self).into()
        }
    }

    pub fn status_end_TantanFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TantanFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusTantanFinalTargetDamage_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusTantanFinalTargetDamage_check_damage(self).into()
        }
    }

    pub fn status_pre_TantanFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TantanFinalTargetEnd(self).into()
        }
    }

    pub fn status_TantanFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_TantanFinalTargetEnd(self).into()
        }
    }

    pub fn status_TantanFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TantanFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_TantanFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TantanFinalTargetEnd(self).into()
        }
    }

    pub fn status_pre_CapturePulledPickel(&mut self) {
        unsafe {
            status_pre_CapturePulledPickel(self)
        }
    }

    pub fn status_CapturePulledPickel(&mut self) -> lib::L2CValue {
        unsafe {
            status_CapturePulledPickel(self).into()
        }
    }

    pub fn CapturePulledCommon_Main(&mut self) {
        unsafe {
            CapturePulledCommon_Main(self)
        }
    }

    pub fn flip_have_item_capture(&mut self) {
        unsafe {
            flip_have_item_capture(self)
        }
    }

    pub fn status_CapturePulled_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CapturePulled_Main(self).into()
        }
    }

    pub fn is_capture_pulled_special_fighter(&mut self) -> lib::L2CValue {
        unsafe {
            is_capture_pulled_special_fighter(self).into()
        }
    }

    pub fn status_end_CapturePulledPickel(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CapturePulledPickel(self).into()
        }
    }

    pub fn status_pre_CatchedPickelTrolley(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchedPickelTrolley(self).into()
        }
    }

    pub fn status_CatchedPickelTrolley(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedPickelTrolley(self).into()
        }
    }

    pub fn status_CatchedPickelTrolley_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedPickelTrolley_Main(self).into()
        }
    }

    pub fn status_end_CatchedPickelTrolley(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchedPickelTrolley(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCatchedPickelTrolley_set_scale(&mut self) {
        unsafe {
            FighterStatusUniqProcessCatchedPickelTrolley_set_scale(self)
        }
    }

    pub fn FighterStatusUniqProcessCatchedPickelTrolley_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCatchedPickelTrolley_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCatchedPickelTrolley_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCatchedPickelTrolley_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCatchedPickelTrolley_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCatchedPickelTrolley_exec_stop(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCatchedPickelTrolley_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCatchedPickelTrolley_exit_status(self).into()
        }
    }

    pub fn status_pre_PickelFinalTargetWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_PickelFinalTargetWait(self).into()
        }
    }

    pub fn sub_PickelFinalTargetWait_mtrans(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_PickelFinalTargetWait_mtrans(self, arg1.into())
        }
    }

    pub fn sub_PickelFinalTargetWait_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_PickelFinalTargetWait_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_PickelFinalTargetWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_PickelFinalTargetWait(self).into()
        }
    }

    pub fn status_PickelFinalTargetWait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_PickelFinalTargetWait_Main(self).into()
        }
    }

    pub fn status_end_PickelFinalTargetWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_PickelFinalTargetWait(self).into()
        }
    }

    pub fn FighterStatusPickelFinalTargetWait_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusPickelFinalTargetWait_check_damage(self).into()
        }
    }

    pub fn sub_req_PickelFinalTargetStart_smoke_effect(&mut self) {
        unsafe {
            sub_req_PickelFinalTargetStart_smoke_effect(self)
        }
    }

    pub fn status_PickelFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_PickelFinalTargetStart(self).into()
        }
    }

    pub fn status_PickelFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_PickelFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_PickelFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_PickelFinalTargetStart(self).into()
        }
    }

    pub fn status_pre_PickelFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_PickelFinalTargetDamage(self).into()
        }
    }

    pub fn status_PickelFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_PickelFinalTargetDamage(self).into()
        }
    }

    pub fn sub_set_PickelFinalTargetDamage_rot_angle_y(&mut self) {
        unsafe {
            sub_set_PickelFinalTargetDamage_rot_angle_y(self)
        }
    }

    pub fn status_PickelFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_PickelFinalTargetDamage_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessPickelFinalTargetDamage_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessPickelFinalTargetDamage_exec_stop(self).into()
        }
    }

    pub fn FighterStatusUniqProcessPickelFinalTargetDamage_exec_fix_pos_slow(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessPickelFinalTargetDamage_exec_fix_pos_slow(self).into()
        }
    }

    pub fn status_end_PickelFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_PickelFinalTargetDamage(self).into()
        }
    }

    pub fn status_pre_ElementFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ElementFinalTargetStart(self).into()
        }
    }

    pub fn status_ElementFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_ElementFinalTargetStart(self).into()
        }
    }

    pub fn status_ElementFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ElementFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_ElementFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ElementFinalTargetStart(self).into()
        }
    }

    pub fn status_pre_ElementFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ElementFinalTargetDamage(self).into()
        }
    }

    pub fn status_ElementFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_ElementFinalTargetDamage(self).into()
        }
    }

    pub fn status_ElementFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ElementFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_ElementFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ElementFinalTargetDamage(self).into()
        }
    }

    pub fn status_pre_ElementFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ElementFinalTargetEnd(self).into()
        }
    }

    pub fn status_ElementFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_ElementFinalTargetEnd(self).into()
        }
    }

    pub fn status_ElementFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ElementFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_pre_DemonDived(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DemonDived(self).into()
        }
    }

    pub fn status_DemonDived(&mut self) -> lib::L2CValue {
        unsafe {
            status_DemonDived(self).into()
        }
    }

    pub fn status_DemonDived_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DemonDived_Main(self).into()
        }
    }

    pub fn status_end_DemonDived(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DemonDived(self).into()
        }
    }

    pub fn sub_uniq_process_DemonDived_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_DemonDived_init(self).into()
        }
    }

    pub fn sub_uniq_process_DemonDived_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_DemonDived_exec(self).into()
        }
    }

    pub fn sub_uniq_process_DemonDived_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_DemonDived_exit(self).into()
        }
    }

    pub fn sub_uniq_process_DemonDived_exec_fix_camera(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_DemonDived_exec_fix_camera(self).into()
        }
    }

    pub fn FighterStatusUniqProcessDemonDived_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessDemonDived_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_DemonFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DemonFinalTargetStart(self).into()
        }
    }

    pub fn status_DemonFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_DemonFinalTargetStart(self).into()
        }
    }

    pub fn status_DemonFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DemonFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_DemonFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DemonFinalTargetStart(self).into()
        }
    }

    pub fn status_pre_DemonFinalDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DemonFinalDamage(self).into()
        }
    }

    pub fn status_DemonFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_DemonFinalTargetDamage(self).into()
        }
    }

    pub fn status_FinalDamageCommon(&mut self) {
        unsafe {
            status_FinalDamageCommon(self)
        }
    }

    pub fn status_DemonFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DemonFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_DemonFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DemonFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusUniqProcessDemonFinalDamage_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessDemonFinalDamage_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamage_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamage_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamage_get_damage_motion_rand(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamage_get_damage_motion_rand(self).into()
        }
    }

    pub fn FighterStatusUniqProcessDemonFinalDamage_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessDemonFinalDamage_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamage_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamage_exit_status(self).into()
        }
    }

    pub fn status_pre_DemonFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DemonFinalTargetEnd(self).into()
        }
    }

    pub fn status_DemonFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_DemonFinalTargetEnd(self).into()
        }
    }

    pub fn status_DemonFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DemonFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_pre_EdgeFinalTargetStart(&mut self) {
        unsafe {
            status_pre_EdgeFinalTargetStart(self)
        }
    }

    pub fn status_EdgeFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_EdgeFinalTargetStart(self).into()
        }
    }

    pub fn status_EdgeFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_EdgeFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_EdgeFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_EdgeFinalTargetStart(self).into()
        }
    }

    pub fn sub_EdgeFinalTargetStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_EdgeFinalTargetStart_init_status(self).into()
        }
    }

    pub fn status_pre_EdgeFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_EdgeFinalTargetDamage(self).into()
        }
    }

    pub fn status_EdgeFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_EdgeFinalTargetDamage(self).into()
        }
    }

    pub fn status_EdgeFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_EdgeFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_EdgeFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_EdgeFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusEdgeFinalTargetDamage_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusEdgeFinalTargetDamage_check_damage(self).into()
        }
    }

    pub fn status_pre_EdgeFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_EdgeFinalTargetEnd(self).into()
        }
    }

    pub fn status_EdgeFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_EdgeFinalTargetEnd(self).into()
        }
    }

    pub fn status_EdgeFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_EdgeFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_EdgeFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_EdgeFinalTargetEnd(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_trail_keyhole_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_trail_keyhole_1(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_trail_keyhole_1_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_trail_keyhole_1_1(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_trail_keyhole_2_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_trail_keyhole_2_1(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_trail_keyhole_3_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_trail_keyhole_3_1(self).into()
        }
    }

    pub fn status_pre_CaptureTrailKeyhole(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureTrailKeyhole(self).into()
        }
    }

    pub fn status_CaptureTrailKeyhole(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureTrailKeyhole(self).into()
        }
    }

    pub fn status_CaptureTrailKeyhole_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureTrailKeyhole_Main(self).into()
        }
    }

    pub fn status_end_CaptureTrailKeyhole(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureTrailKeyhole(self).into()
        }
    }

    pub fn sub_capture_trail_ease_in_quad(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_capture_trail_ease_in_quad(self, arg1.into())
        }
    }

    pub fn sub_capture_trail_ease_out_quad(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_capture_trail_ease_out_quad(self, arg1.into()).into()
        }
    }

    pub fn sub_capture_trail_ease_in_cubic(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_capture_trail_ease_in_cubic(self, arg1.into()).into()
        }
    }

    pub fn sub_capture_trail_ease_out_cubic(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_capture_trail_ease_out_cubic(self, arg1.into()).into()
        }
    }

    pub fn sub_capture_trail_ease_in_quint(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_capture_trail_ease_in_quint(self, arg1.into()).into()
        }
    }

    pub fn sub_capture_trail_ease_out_quint(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_capture_trail_ease_out_quint(self, arg1.into()).into()
        }
    }

    pub fn sub_capture_trail_ease_in_circ(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_capture_trail_ease_in_circ(self, arg1.into()).into()
        }
    }

    pub fn sub_capture_trail_ease_out_circ(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_capture_trail_ease_out_circ(self, arg1.into()).into()
        }
    }

    pub fn sub_capture_trail_keyhole_status(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_capture_trail_keyhole_status(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessCaptureTrailKeyhole_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureTrailKeyhole_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureTrailKeyhole_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureTrailKeyhole_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureTrailKeyhole_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureTrailKeyhole_exit_status(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_trail_keyhole_4_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_trail_keyhole_4_1(self).into()
        }
    }

    pub fn status_pre_TrailRebound(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TrailRebound(self).into()
        }
    }

    pub fn status_TrailRebound(&mut self) -> lib::L2CValue {
        unsafe {
            status_TrailRebound(self).into()
        }
    }

    pub fn status_TrailRebound_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TrailRebound_Main(self).into()
        }
    }

    pub fn sub_wait_ground_check_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_wait_ground_check_common(self, arg1.into()).into()
        }
    }

    pub fn sub_air_check_fall_common(&mut self) -> lib::L2CValue {
        unsafe {
            sub_air_check_fall_common(self).into()
        }
    }

    pub fn sub_trail_change_motion_by_situation(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            sub_trail_change_motion_by_situation(self, arg1.into(), arg2.into())
        }
    }

    pub fn sub_transition_group_check_air_landing(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_landing(self).into()
        }
    }

    pub fn sub_transition_group_check_air_cliff(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_cliff(self).into()
        }
    }

    pub fn sub_rocketbelt_hover_check(&mut self) -> lib::L2CValue {
        unsafe {
            sub_rocketbelt_hover_check(self).into()
        }
    }

    pub fn sub_transition_group_check_air_special(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_special(self).into()
        }
    }

    pub fn sub_transition_group_check_air_item_throw(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_item_throw(self).into()
        }
    }

    pub fn sub_transition_group_check_air_lasso(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_lasso(self).into()
        }
    }

    pub fn sub_transition_group_check_air_escape(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_escape(self).into()
        }
    }

    pub fn sub_transition_group_check_air_attack(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_attack(self).into()
        }
    }

    pub fn sub_transition_group_check_air_tread_jump(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_tread_jump(self).into()
        }
    }

    pub fn sub_transition_group_check_air_wall_jump(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_wall_jump(self).into()
        }
    }

    pub fn sub_transition_group_check_air_jump_aerial(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_jump_aerial(self).into()
        }
    }

    pub fn sub_is_fly_next(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_is_fly_next(self, arg1.into()).into()
        }
    }

    pub fn sub_fighter_general_term_is_can_attach_wall(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fighter_general_term_is_can_attach_wall(self).into()
        }
    }

    pub fn sub_fighter_general_term_is_can_wall_jump(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fighter_general_term_is_can_wall_jump(self).into()
        }
    }

    pub fn FL_sub_fighter_float_next_tread_speed_y(&mut self) -> lib::L2CValue {
        unsafe {
            FL_sub_fighter_float_next_tread_speed_y(self).into()
        }
    }

    pub fn sub_transition_group_check_air_jump_attack(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_jump_attack(self).into()
        }
    }

    pub fn sub_is_item_shoot_air(&mut self) -> lib::L2CValue {
        unsafe {
            sub_is_item_shoot_air(self).into()
        }
    }

    pub fn sub_is_fly_next_jump_trigger(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_is_fly_next_jump_trigger(self, arg1.into()).into()
        }
    }

    pub fn can_entry_cliff_air_lasso(&mut self) -> lib::L2CValue {
        unsafe {
            can_entry_cliff_air_lasso(self).into()
        }
    }

    pub fn sub_AIRChkDropItemImm(&mut self) -> lib::L2CValue {
        unsafe {
            sub_AIRChkDropItemImm(self).into()
        }
    }

    pub fn sub_AIRChkGetItemImm(&mut self) {
        unsafe {
            sub_AIRChkGetItemImm(self)
        }
    }

    pub fn sub_GetLightItemImm(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_GetLightItemImm(self, arg1.into())
        }
    }

    pub fn sub_transition_group_check_special_command(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_special_command(self).into()
        }
    }

    pub fn sub_transition_group_check_ground_jump_mini_attack(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_ground_jump_mini_attack(self).into()
        }
    }

    pub fn sub_transition_group_check_ground_item(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_ground_item(self).into()
        }
    }

    pub fn sub_transition_group_check_ground_catch(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_ground_catch(self).into()
        }
    }

    pub fn sub_transition_group_check_ground_escape(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_ground_escape(self).into()
        }
    }

    pub fn sub_transition_group_check_ground_guard(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_ground_guard(self).into()
        }
    }

    pub fn sub_transition_group_check_ground_special(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_ground_special(self).into()
        }
    }

    pub fn sub_transition_group_check_ground_attack(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_ground_attack(self).into()
        }
    }

    pub fn sub_transition_group_check_ground_jump(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_ground_jump(self).into()
        }
    }

    pub fn sub_transition_group_check_ground(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_ground(self, arg1.into()).into()
        }
    }

    pub fn sub_check_command_squat(&mut self) -> lib::L2CValue {
        unsafe {
            sub_check_command_squat(self).into()
        }
    }

    pub fn sub_check_command_walk(&mut self) -> lib::L2CValue {
        unsafe {
            sub_check_command_walk(self).into()
        }
    }

    pub fn sub_check_button_jump(&mut self) -> lib::L2CValue {
        unsafe {
            sub_check_button_jump(self).into()
        }
    }

    pub fn sub_check_button_frick(&mut self) -> lib::L2CValue {
        unsafe {
            sub_check_button_frick(self).into()
        }
    }

    pub fn sub_transition_specialflag_hoist(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_specialflag_hoist(self).into()
        }
    }

    pub fn sub_check_command_guard(&mut self) -> lib::L2CValue {
        unsafe {
            sub_check_command_guard(self).into()
        }
    }

    pub fn change_status_jump_mini_attack(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            change_status_jump_mini_attack(self, arg1.into()).into()
        }
    }

    pub fn change_status_jump_mini_attack_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            change_status_jump_mini_attack_common(self, arg1.into(), arg2.into())
        }
    }

    pub fn status_end_TrailRebound(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TrailRebound(self).into()
        }
    }

    pub fn FighterStatusUniqProcessTrailRebound_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessTrailRebound_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessTrailRebound_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessTrailRebound_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessTrailRebound_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessTrailRebound_exit_status(self).into()
        }
    }

    pub fn status_pre_TrailFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TrailFinalTargetDamage(self).into()
        }
    }

    pub fn status_TrailFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_TrailFinalTargetDamage(self).into()
        }
    }

    pub fn status_TrailFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TrailFinalTargetDamage_Main(self).into()
        }
    }

    pub fn FighterStatusTrailFinalTargetDamage_exec_fix_camera(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusTrailFinalTargetDamage_exec_fix_camera(self).into()
        }
    }

    pub fn status_end_TrailFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TrailFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusTrailFinalTargetDamage_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusTrailFinalTargetDamage_check_damage(self).into()
        }
    }

    pub fn status_pre_TrailFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TrailFinalTargetEnd(self).into()
        }
    }

    pub fn status_TrailFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_TrailFinalTargetEnd(self).into()
        }
    }

    pub fn status_TrailFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TrailFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_TrailFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TrailFinalTargetEnd(self).into()
        }
    }

    pub fn sub_wait_ground_check_common_pre(&mut self) {
        unsafe {
            sub_wait_ground_check_common_pre(self)
        }
    }

    pub fn sub_air_check_fall_common_pre(&mut self) {
        unsafe {
            sub_air_check_fall_common_pre(self)
        }
    }

    pub fn sub_check_damage_knock_out(&mut self) -> lib::L2CValue {
        unsafe {
            sub_check_damage_knock_out(self).into()
        }
    }

    pub fn sub_transition_group_disguise(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_transition_group_disguise(self, arg1.into()).into()
        }
    }

    pub fn sub_transition_term_id_cont_disguise(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_transition_term_id_cont_disguise(self, arg1.into()).into()
        }
    }

    pub fn sub_check_jump_in_charging(&mut self) -> lib::L2CValue {
        unsafe {
            sub_check_jump_in_charging(self).into()
        }
    }

    pub fn sub_check_jump_in_charging_for_cancel_status(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_check_jump_in_charging_for_cancel_status(self, arg1.into()).into()
        }
    }

    pub fn sub_check_fly_in_charging_for_cancel_status(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_check_fly_in_charging_for_cancel_status(self, arg1.into()).into()
        }
    }

    pub fn sub_check_charge_cancel_jump_mini_attack(&mut self) {
        unsafe {
            sub_check_charge_cancel_jump_mini_attack(self)
        }
    }

    pub fn sub_transition_group_check_air_cliff_force(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_cliff_force(self).into()
        }
    }

    pub fn sub_status_pre_SpecialNCommon(&mut self) {
        unsafe {
            sub_status_pre_SpecialNCommon(self)
        }
    }

    pub fn sub_status_pre_FinalCommon(&mut self) {
        unsafe {
            sub_status_pre_FinalCommon(self)
        }
    }

    pub fn delete_clatter_input(&mut self) {
        unsafe {
            delete_clatter_input(self)
        }
    }

    pub fn sub_fighter_change_status(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_fighter_change_status(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn sub_fighter_check_abnormal_fail_in_attack(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_fighter_check_abnormal_fail_in_attack(self, arg1.into()).into()
        }
    }

    pub fn sub_fighter_change_status_shift(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fighter_change_status_shift(self).into()
        }
    }

    pub fn sub_fighter_remake_control_command(&mut self) {
        unsafe {
            sub_fighter_remake_control_command(self)
        }
    }

    pub fn sub_fighter_line_status_system(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fighter_line_status_system(self).into()
        }
    }

    pub fn sub_fighter_general_term_is_cliff_check_pos(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fighter_general_term_is_cliff_check_pos(self).into()
        }
    }

    pub fn sub_fighter_cliff_check(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_fighter_cliff_check(self, arg1.into())
        }
    }

    pub fn status_utility_mediate_motion(&mut self) {
        unsafe {
            status_utility_mediate_motion(self)
        }
    }

    pub fn sub_fighter_final_end_to_fall_common(&mut self) {
        unsafe {
            sub_fighter_final_end_to_fall_common(self)
        }
    }

    pub fn sub_fighter_do_control_passable(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fighter_do_control_passable(self).into()
        }
    }

    pub fn sub_get_adjust_rate_from_cancel_frame(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_get_adjust_rate_from_cancel_frame(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn sub_get_adjust_rate_from_cancel_frame_inv(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_get_adjust_rate_from_cancel_frame_inv(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn sub_set_meteor_hit_id_to_work(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) {
        unsafe {
            sub_set_meteor_hit_id_to_work(self, arg1.into(), arg2.into(), arg3.into(), arg4.into())
        }
    }

    pub fn sub_off_passive_opponent(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_off_passive_opponent(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn sub_set_reaction_frame_by_current_motion(&mut self) {
        unsafe {
            sub_set_reaction_frame_by_current_motion(self)
        }
    }

    pub fn sub_set_reaction_frame_by_motion(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_set_reaction_frame_by_motion(self, arg1.into())
        }
    }

    pub fn sub_set_special_start_common_kinetic_setting(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_set_special_start_common_kinetic_setting(self, arg1.into())
        }
    }

    pub fn sub_set_special_start_inherit_common_kinetic_setting(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_set_special_start_inherit_common_kinetic_setting(self, arg1.into())
        }
    }

    pub fn sub_exec_special_start_common_kinetic_setting_inner(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            sub_exec_special_start_common_kinetic_setting_inner(self, arg1.into(), arg2.into())
        }
    }

    pub fn sub_exec_special_start_common_kinetic_setting_gravity_func(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_exec_special_start_common_kinetic_setting_gravity_func(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn sub_exec_special_start_common_kinetic_setting(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_exec_special_start_common_kinetic_setting(self, arg1.into())
        }
    }

    pub fn sub_change_motion_by_situation_innner(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_change_motion_by_situation_innner(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn sub_change_motion_by_situation(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_change_motion_by_situation(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn sub_change_motion_by_situation_kirby_copy(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_change_motion_by_situation_kirby_copy(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn sub_set_ground_correct_by_situation(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_set_ground_correct_by_situation(self, arg1.into()).into()
        }
    }

    pub fn sub_change_kinetic_type_by_situation(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_change_kinetic_type_by_situation(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn sub_remove_exist_article_at_status_end(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            sub_remove_exist_article_at_status_end(self, arg1.into(), arg2.into())
        }
    }

    pub fn sub_is_body_up(&mut self) -> lib::L2CValue {
        unsafe {
            sub_is_body_up(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLassoCommon_notify_event_link(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLassoCommon_notify_event_link(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLassoCommon_change_motion_sync_shoulder(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessAirLassoCommon_change_motion_sync_shoulder(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessAirLassoCommon_exec_map_correction(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLassoCommon_exec_map_correction(self).into()
        }
    }

    pub fn status_pre_AirLasso(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AirLasso(self).into()
        }
    }

    pub fn sub_air_lasso_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_air_lasso_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_AirLasso(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_AirLasso(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn status_air_lasso_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_air_lasso_main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLasso_check_attack(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLasso_check_attack(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn remove_exist_article2(&mut self) {
        unsafe {
            remove_exist_article2(self)
        }
    }

    pub fn status_end_AirLasso(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AirLasso(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLasso_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLasso_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLasso_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLasso_exit_status(self).into()
        }
    }

    pub fn status_pre_AirLassoReach(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AirLassoReach(self).into()
        }
    }

    pub fn status_AirLassoReach(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_AirLassoReach(self, arg1.into()).into()
        }
    }

    pub fn status_air_lasso_reach_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_air_lasso_reach_main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLassoReach_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessAirLassoReach_exec_fix_pos(self)
        }
    }

    pub fn FighterStatusUniqProcessAirLassoReach_exec_fix_pos_common(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLassoReach_exec_fix_pos_common(self).into()
        }
    }

    pub fn status_end_AirLassoReach(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AirLassoReach(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLassoReach_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLassoReach_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLassoReach_exec_fix_pos_slow(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLassoReach_exec_fix_pos_slow(self).into()
        }
    }

    pub fn status_pre_AirLassoHang(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AirLassoHang(self).into()
        }
    }

    pub fn sub_air_lasso_hang_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_air_lasso_hang_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_AirLassoHang(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_AirLassoHang(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn status_air_lasso_hang_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_air_lasso_hang_main(self).into()
        }
    }

    pub fn sub_transition_group_check_air_cliff_air_lasso(&mut self) -> lib::L2CValue {
        unsafe {
            sub_transition_group_check_air_cliff_air_lasso(self).into()
        }
    }

    pub fn sub_check_over_cliff(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_check_over_cliff(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into()).into()
        }
    }

    pub fn sub_air_lasso_landing_limit_speed(&mut self) {
        unsafe {
            sub_air_lasso_landing_limit_speed(self)
        }
    }

    pub fn status_end_AirLassoHang(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AirLassoHang(self).into()
        }
    }

    pub fn status_end_AirLassoHangCommon(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            status_end_AirLassoHangCommon(self, arg1.into(), arg2.into())
        }
    }

    pub fn FighterStatusUniqProcessAirLassoHang_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLassoHang_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLassoHang_exec_map_correction(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLassoHang_exec_map_correction(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLassoHang_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLassoHang_exit_status(self).into()
        }
    }

    pub fn status_pre_AirLassoRewind(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AirLassoRewind(self).into()
        }
    }

    pub fn sub_air_lasso_rewind_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_air_lasso_rewind_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_AirLassoRewind(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_AirLassoRewind(self, arg1.into()).into()
        }
    }

    pub fn status_air_lasso_rewind_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_air_lasso_rewind_main(self).into()
        }
    }

    pub fn status_end_AirLassoRewind(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AirLassoRewind(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLassoRewind_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLassoRewind_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLassoRewind_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLassoRewind_exit_status(self).into()
        }
    }

    pub fn status_pre_AirLassoFailure(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AirLassoFailure(self).into()
        }
    }

    pub fn sub_air_lasso_failure_uniq(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_air_lasso_failure_uniq(self, arg1.into())
        }
    }

    pub fn status_AirLassoFailure(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_AirLassoFailure(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn status_AirLassoFailureCommon(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_AirLassoFailureCommon(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn status_air_lasso_failure_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_air_lasso_failure_main(self).into()
        }
    }

    pub fn status_end_AirLassoFailure(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AirLassoFailure(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAirLassoFailure_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAirLassoFailure_init_status(self).into()
        }
    }

    pub fn status_pre_AirLassoLanding(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AirLassoLanding(self).into()
        }
    }

    pub fn sub_pre_landing_kinetic_type(&mut self) -> lib::L2CValue {
        unsafe {
            sub_pre_landing_kinetic_type(self).into()
        }
    }

    pub fn status_AirLassoLanding(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_AirLassoLanding(self, arg1.into()).into()
        }
    }

    pub fn status_LandingSub(&mut self) {
        unsafe {
            status_LandingSub(self)
        }
    }

    pub fn status_LandingStiffness(&mut self) {
        unsafe {
            status_LandingStiffness(self)
        }
    }

    pub fn status_air_lasso_landing_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_air_lasso_landing_main(self).into()
        }
    }

    pub fn status_Landing_MainSub(&mut self) -> lib::L2CValue {
        unsafe {
            status_Landing_MainSub(self).into()
        }
    }

    pub fn sub_landing_cancel_check_damage_face(&mut self) {
        unsafe {
            sub_landing_cancel_check_damage_face(self)
        }
    }

    pub fn sub_landing_cancel_damage_face(&mut self) {
        unsafe {
            sub_landing_cancel_damage_face(self)
        }
    }

    pub fn sub_landing_uniq_check_strans(&mut self) -> lib::L2CValue {
        unsafe {
            sub_landing_uniq_check_strans(self).into()
        }
    }

    pub fn sub_landing_ground_check_common(&mut self) -> lib::L2CValue {
        unsafe {
            sub_landing_ground_check_common(self).into()
        }
    }

    pub fn FL_get_LandingStiffness(&mut self) -> lib::L2CValue {
        unsafe {
            FL_get_LandingStiffness(self).into()
        }
    }

    pub fn sub_landing_ground_check_common_pre(&mut self) {
        unsafe {
            sub_landing_ground_check_common_pre(self)
        }
    }

    pub fn sub_landing_uniq_check(&mut self) -> lib::L2CValue {
        unsafe {
            sub_landing_uniq_check(self).into()
        }
    }

    pub fn status_Landing_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Landing_Main(self).into()
        }
    }

    pub fn status_end_AirLassoLanding(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AirLassoLanding(self).into()
        }
    }

    pub fn status_pre_attach_wall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_attach_wall(self).into()
        }
    }

    pub fn status_attach_wall(&mut self) -> lib::L2CValue {
        unsafe {
            status_attach_wall(self).into()
        }
    }

    pub fn sub_attach_wall_uniq(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attach_wall_uniq(self).into()
        }
    }

    pub fn status_attach_wall_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_attach_wall_main(self).into()
        }
    }

    pub fn sub_uniq_process_Passive_exec_fix_pos(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_Passive_exec_fix_pos(self).into()
        }
    }

    pub fn status_end_attach_wall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_attach_wall(self).into()
        }
    }

    pub fn status_pre_attach_wall_wait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_attach_wall_wait(self).into()
        }
    }

    pub fn status_attach_wall_wait(&mut self) -> lib::L2CValue {
        unsafe {
            status_attach_wall_wait(self).into()
        }
    }

    pub fn sub_attach_wall_wait_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_attach_wall_wait_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_attach_wall_wait_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_attach_wall_wait_main(self).into()
        }
    }

    pub fn status_end_attach_wall_wait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_attach_wall_wait(self).into()
        }
    }

    pub fn status_pre_DetachWall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DetachWall(self).into()
        }
    }

    pub fn status_DetachWall(&mut self) -> lib::L2CValue {
        unsafe {
            status_DetachWall(self).into()
        }
    }

    pub fn status_DetachWall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DetachWall_Main(self).into()
        }
    }

    pub fn status_end_DetachWall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DetachWall(self).into()
        }
    }

    pub fn status_pre_DetachWallFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DetachWallFall(self).into()
        }
    }

    pub fn status_DetachWallFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_DetachWallFall(self).into()
        }
    }

    pub fn status_DetachWallFall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DetachWallFall_Main(self).into()
        }
    }

    pub fn status_end_DetachWallFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DetachWallFall(self).into()
        }
    }

    pub fn status_pre_DetachWallJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DetachWallJump(self).into()
        }
    }

    pub fn status_DetachWallJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_DetachWallJump(self).into()
        }
    }

    pub fn status_JumpAerialSub(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_JumpAerialSub(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn status_JumpAerial_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_JumpAerial_Main(self).into()
        }
    }

    pub fn sub_air_check_stop_ceil(&mut self) -> lib::L2CValue {
        unsafe {
            sub_air_check_stop_ceil(self).into()
        }
    }

    pub fn sub_air_check_superleaf_fall_slowly(&mut self) {
        unsafe {
            sub_air_check_superleaf_fall_slowly(self)
        }
    }

    pub fn sub_is_fall_slowly(&mut self) -> lib::L2CValue {
        unsafe {
            sub_is_fall_slowly(self).into()
        }
    }

    pub fn sub_jump_aerial_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_jump_aerial_uniq(self, arg1.into()).into()
        }
    }

    pub fn sub_fall_common_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_fall_common_uniq(self, arg1.into()).into()
        }
    }

    pub fn sub_glide_stick_check_uniq(&mut self) {
        unsafe {
            sub_glide_stick_check_uniq(self)
        }
    }

    pub fn sub_air_check_dive(&mut self) {
        unsafe {
            sub_air_check_dive(self)
        }
    }

    pub fn sub_is_dive(&mut self) -> lib::L2CValue {
        unsafe {
            sub_is_dive(self).into()
        }
    }

    pub fn check_mach_stamp(&mut self) {
        unsafe {
            check_mach_stamp(self)
        }
    }

    pub fn status_DetachWallJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DetachWallJump_Main(self).into()
        }
    }

    pub fn status_end_DetachWallJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DetachWallJump(self).into()
        }
    }

    pub fn get_mini_jump_attack_data_log_info(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_mini_jump_attack_data_log_info(self, arg1.into()).into()
        }
    }

    pub fn get_mini_jump_attack_data_cancel_function(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_mini_jump_attack_data_cancel_function(self, arg1.into()).into()
        }
    }

    pub fn is_smash_hold(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_smash_hold(self, arg1.into()).into()
        }
    }

    pub fn status_pre_Attack(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Attack(self).into()
        }
    }

    pub fn sub_status_AttackCommon(&mut self) {
        unsafe {
            sub_status_AttackCommon(self)
        }
    }

    pub fn sub_status_AttackComboCommon(&mut self) {
        unsafe {
            sub_status_AttackComboCommon(self)
        }
    }

    pub fn sub_status_AttackComboCommon_button(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_status_AttackComboCommon_button(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn attack_combo_uniq_chk(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            attack_combo_uniq_chk(self, arg1.into()).into()
        }
    }

    pub fn attack_combo_uniq_chk_button(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            attack_combo_uniq_chk_button(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn attack_uniq_chk_command(&mut self, arg1: lib::L2CValue) {
        unsafe {
            attack_uniq_chk_command(self, arg1.into())
        }
    }

    pub fn attack_combo_none_uniq_chk(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            attack_combo_none_uniq_chk(self, arg1.into()).into()
        }
    }

    pub fn attack_combo_none_uniq_chk_button(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            attack_combo_none_uniq_chk_button(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn status_Attack(&mut self) -> lib::L2CValue {
        unsafe {
            status_Attack(self).into()
        }
    }

    pub fn check_attack_mtrans(&mut self) -> lib::L2CValue {
        unsafe {
            check_attack_mtrans(self).into()
        }
    }

    pub fn status_Attack_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Attack_Main(self).into()
        }
    }

    pub fn status_Attack_Main_button(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_Attack_Main_button(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn check_100_count_button(&mut self, arg1: lib::L2CValue) {
        unsafe {
            check_100_count_button(self, arg1.into())
        }
    }

    pub fn attack_mtrans_pre_process(&mut self) {
        unsafe {
            attack_mtrans_pre_process(self)
        }
    }

    pub fn attack_mtrans_post_process(&mut self) {
        unsafe {
            attack_mtrans_post_process(self)
        }
    }

    pub fn check_100_count(&mut self) {
        unsafe {
            check_100_count(self)
        }
    }

    pub fn attack_uniq_chk(&mut self) {
        unsafe {
            attack_uniq_chk(self)
        }
    }

    pub fn attack_mtrans(&mut self) {
        unsafe {
            attack_mtrans(self)
        }
    }

    pub fn status_end_Attack(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Attack(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAttack_check_attack(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAttack_check_attack(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn local_func__fighter_status_attack_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_attack_2(self).into()
        }
    }

    pub fn local_func__fighter_status_attack_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_attack_3(self).into()
        }
    }

    pub fn status_pre_Attack100(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Attack100(self).into()
        }
    }

    pub fn sub_status_Attack100_common(&mut self) {
        unsafe {
            sub_status_Attack100_common(self)
        }
    }

    pub fn status_Attack100(&mut self) -> lib::L2CValue {
        unsafe {
            status_Attack100(self).into()
        }
    }

    pub fn status_Attack100_Main_uniq_func(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_Attack100_Main_uniq_func(self, arg1.into())
        }
    }

    pub fn attack_100_start_uniq_chk(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            attack_100_start_uniq_chk(self, arg1.into()).into()
        }
    }

    pub fn sub_attack_100_uniq_check(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_attack_100_uniq_check(self, arg1.into()).into()
        }
    }

    pub fn sub_attack_100_uniq_check_button(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            sub_attack_100_uniq_check_button(self, arg1.into(), arg2.into())
        }
    }

    pub fn status_Attack100_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Attack100_Main(self).into()
        }
    }

    pub fn status_end_Attack100(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Attack100(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAttack100_check_attack(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAttack100_check_attack(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn status_pre_AttackDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackDash(self).into()
        }
    }

    pub fn sub_attack_dash_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_attack_dash_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_AttackDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackDash(self).into()
        }
    }

    pub fn status_AttackDash_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackDash_Main(self).into()
        }
    }

    pub fn status_end_AttackDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackDash(self).into()
        }
    }

    pub fn status_pre_AttackS3_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_pre_AttackS3_common(self, arg1.into())
        }
    }

    pub fn status_pre_AttackS3(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackS3(self).into()
        }
    }

    pub fn sub_check_attack_lw_status_func_common(&mut self) -> lib::L2CValue {
        unsafe {
            sub_check_attack_lw_status_func_common(self).into()
        }
    }

    pub fn sub_attack3_uniq_check_is_button(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attack3_uniq_check_is_button(self).into()
        }
    }

    pub fn sub_attack3_uniq_check(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_attack3_uniq_check(self, arg1.into()).into()
        }
    }

    pub fn sub_attack3_uniq_check_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_attack3_uniq_check_param(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn status_AttackS3Common(&mut self) {
        unsafe {
            status_AttackS3Common(self)
        }
    }

    pub fn status_AttackS3(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackS3(self).into()
        }
    }

    pub fn status_AttackS3_Main_param(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_AttackS3_Main_param(self, arg1.into()).into()
        }
    }

    pub fn attack_s3_mtrans_param(&mut self, arg1: lib::L2CValue) {
        unsafe {
            attack_s3_mtrans_param(self, arg1.into())
        }
    }

    pub fn attack_s3_mtrans(&mut self) {
        unsafe {
            attack_s3_mtrans(self)
        }
    }

    pub fn status_AttackS3_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackS3_Main(self).into()
        }
    }

    pub fn status_end_AttackS3(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackS3(self).into()
        }
    }

    pub fn status_pre_AttackHi3(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackHi3(self).into()
        }
    }

    pub fn status_AttackHi3_Common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            status_AttackHi3_Common(self, arg1.into(), arg2.into())
        }
    }

    pub fn status_AttackHi3_Common_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            status_AttackHi3_Common_param(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn status_AttackHi3(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackHi3(self).into()
        }
    }

    pub fn status_AttackHi3_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackHi3_Main(self).into()
        }
    }

    pub fn status_AttackHi3_Main_minjump(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackHi3_Main_minjump(self).into()
        }
    }

    pub fn status_end_AttackHi3(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackHi3(self).into()
        }
    }

    pub fn status_pre_AttackLw3_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_pre_AttackLw3_common(self, arg1.into())
        }
    }

    pub fn status_pre_AttackLw3(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackLw3(self).into()
        }
    }

    pub fn status_AttackLw3_common(&mut self) {
        unsafe {
            status_AttackLw3_common(self)
        }
    }

    pub fn status_AttackLw3_common_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            status_AttackLw3_common_param(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn status_AttackLw3(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackLw3(self).into()
        }
    }

    pub fn status_AttackLw3_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackLw3_Main(self).into()
        }
    }

    pub fn status_end_AttackLw3(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackLw3(self).into()
        }
    }

    pub fn FighterStatusUniqProcessAttackLw3_check_attack(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessAttackLw3_check_attack(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn local_func__fighter_status_attack_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_attack_4(self).into()
        }
    }

    pub fn local_func__fighter_status_attack_5(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_attack_5(self).into()
        }
    }

    pub fn status_pre_AttackS4Start(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackS4Start(self).into()
        }
    }

    pub fn sub_smash_hold_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_smash_hold_uniq(self, arg1.into()).into()
        }
    }

    pub fn sub_attack_s4_turn_rev(&mut self) {
        unsafe {
            sub_attack_s4_turn_rev(self)
        }
    }

    pub fn sub_end_attack_s4_turn_rev(&mut self) {
        unsafe {
            sub_end_attack_s4_turn_rev(self)
        }
    }

    pub fn status_AttackS4Start_Common(&mut self) {
        unsafe {
            status_AttackS4Start_Common(self)
        }
    }

    pub fn status_AttackS4Start(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackS4Start(self).into()
        }
    }

    pub fn status_AttackS4Start_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackS4Start_Main(self).into()
        }
    }

    pub fn status_end_AttackXX4Start(&mut self) {
        unsafe {
            status_end_AttackXX4Start(self)
        }
    }

    pub fn status_end_AttackS4Start(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackS4Start(self).into()
        }
    }

    pub fn status_pre_AttackS4Hold(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackS4Hold(self).into()
        }
    }

    pub fn status_pre_AttackS4Hold_Common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            status_pre_AttackS4Hold_Common(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn status_AttackS4Hold(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackS4Hold(self).into()
        }
    }

    pub fn status_AttackS4Hold_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackS4Hold_main(self).into()
        }
    }

    pub fn status_end_AttackS4Hold(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackS4Hold(self).into()
        }
    }

    pub fn status_pre_AttackS4(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackS4(self).into()
        }
    }

    pub fn status_pre_AttackS4_Common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_AttackS4_Common(self, arg1.into()).into()
        }
    }

    pub fn status_AttackS4(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackS4(self).into()
        }
    }

    pub fn sub_AttackS4(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_AttackS4(self, arg1.into())
        }
    }

    pub fn status_AttackS4_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackS4_Main(self).into()
        }
    }

    pub fn attack_s4_mtrans(&mut self) {
        unsafe {
            attack_s4_mtrans(self)
        }
    }

    pub fn status_end_AttackS4(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackS4(self).into()
        }
    }

    pub fn sub_attack_s4_common_uniq_process_init(&mut self) {
        unsafe {
            sub_attack_s4_common_uniq_process_init(self)
        }
    }

    pub fn sub_attack_xx4_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attack_xx4_uniq_process_init(self).into()
        }
    }

    pub fn sub_set_attack_s4_power_mul(&mut self) {
        unsafe {
            sub_set_attack_s4_power_mul(self)
        }
    }

    pub fn sub_attack_xx4_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attack_xx4_uniq_process_exec(self).into()
        }
    }

    pub fn sub_attack_xx4_common_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attack_xx4_common_uniq_process_exit(self).into()
        }
    }

    pub fn sub_attack_s4_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attack_s4_uniq_process_exit(self).into()
        }
    }

    pub fn item_swing_motion4_hold(&mut self) -> lib::L2CValue {
        unsafe {
            item_swing_motion4_hold(self).into()
        }
    }

    pub fn get_swing_motion_data(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_swing_motion_data(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_1(self).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_2(self).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_3(self).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_4(self).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_5(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_5(self).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_6(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_6(self).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_7(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_7(self).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_8(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_8(self).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_9(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_9(self).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_10(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_10(self).into()
        }
    }

    pub fn status_pre_AttackHi4Start(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackHi4Start(self).into()
        }
    }

    pub fn status_pre_AttackHi4Start_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_AttackHi4Start_common(self, arg1.into()).into()
        }
    }

    pub fn sub_status_AttackHi4Start_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_status_AttackHi4Start_common(self, arg1.into())
        }
    }

    pub fn status_AttackHi4Start_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_AttackHi4Start_common(self, arg1.into()).into()
        }
    }

    pub fn status_AttackHi4Start_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackHi4Start_Main(self).into()
        }
    }

    pub fn status_AttackHi4Start(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackHi4Start(self).into()
        }
    }

    pub fn status_end_AttackHi4Start(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackHi4Start(self).into()
        }
    }

    pub fn status_pre_AttackHi4Hold(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackHi4Hold(self).into()
        }
    }

    pub fn status_pre_AttackHi4Hold_Common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_AttackHi4Hold_Common(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn status_AttackHi4Hold_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_AttackHi4Hold_common(self, arg1.into()).into()
        }
    }

    pub fn status_AttackHi4Hold_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackHi4Hold_main(self).into()
        }
    }

    pub fn status_AttackHi4Hold(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackHi4Hold(self).into()
        }
    }

    pub fn status_end_AttackHi4Hold(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackHi4Hold(self).into()
        }
    }

    pub fn status_pre_AttackHi4(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackHi4(self).into()
        }
    }

    pub fn status_AttackHi4_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_AttackHi4_common(self, arg1.into())
        }
    }

    pub fn status_AttackHi4(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackHi4(self).into()
        }
    }

    pub fn status_AttackHi4_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackHi4_Main(self).into()
        }
    }

    pub fn status_end_AttackHi4(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackHi4(self).into()
        }
    }

    pub fn status_pre_AttackLw4Start(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackLw4Start(self).into()
        }
    }

    pub fn status_AttackLw4Start_common(&mut self) {
        unsafe {
            status_AttackLw4Start_common(self)
        }
    }

    pub fn status_AttackLw4Start(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackLw4Start(self).into()
        }
    }

    pub fn status_AttackLw4Start_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackLw4Start_Main(self).into()
        }
    }

    pub fn status_end_AttackLw4Start(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackLw4Start(self).into()
        }
    }

    pub fn status_pre_AttackLw4Hold(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackLw4Hold(self).into()
        }
    }

    pub fn status_pre_AttackLw4Hold_Common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_AttackLw4Hold_Common(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn status_AttackLw4Hold_common(&mut self) {
        unsafe {
            status_AttackLw4Hold_common(self)
        }
    }

    pub fn status_AttackLw4Hold(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackLw4Hold(self).into()
        }
    }

    pub fn status_AttackLw4Hold_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackLw4Hold_main(self).into()
        }
    }

    pub fn status_end_AttackLw4Hold(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackLw4Hold(self).into()
        }
    }

    pub fn status_pre_AttackLw4(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackLw4(self).into()
        }
    }

    pub fn status_AttackLw4_common(&mut self) {
        unsafe {
            status_AttackLw4_common(self)
        }
    }

    pub fn attack_lw4_mtrans_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            attack_lw4_mtrans_common(self, arg1.into())
        }
    }

    pub fn attack_lw4_mtrans(&mut self) {
        unsafe {
            attack_lw4_mtrans(self)
        }
    }

    pub fn status_AttackLw4(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackLw4(self).into()
        }
    }

    pub fn status_AttackLw4_Main_param(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_AttackLw4_Main_param(self, arg1.into())
        }
    }

    pub fn status_AttackLw4_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackLw4_Main(self).into()
        }
    }

    pub fn status_end_AttackLw4(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackLw4(self).into()
        }
    }

    pub fn status_pre_AttackAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_AttackAir(self).into()
        }
    }

    pub fn sub_attack_air_kind(&mut self) {
        unsafe {
            sub_attack_air_kind(self)
        }
    }

    pub fn sub_attack_air_kind_set_log_info(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attack_air_kind_set_log_info(self).into()
        }
    }

    pub fn sub_attack_air_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_attack_air_common(self, arg1.into())
        }
    }

    pub fn attack_air_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            attack_air_uniq(self, arg1.into()).into()
        }
    }

    pub fn sub_attack_air(&mut self) {
        unsafe {
            sub_attack_air(self)
        }
    }

    pub fn status_AttackAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackAir(self).into()
        }
    }

    pub fn status_AttackAir_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackAir_Main(self).into()
        }
    }

    pub fn status_AttackAir_Main_common(&mut self) -> lib::L2CValue {
        unsafe {
            status_AttackAir_Main_common(self).into()
        }
    }

    pub fn attack_air_common_strans(&mut self) -> lib::L2CValue {
        unsafe {
            attack_air_common_strans(self).into()
        }
    }

    pub fn sub_attack_air_uniq_process_exec_fix_pos(&mut self) {
        unsafe {
            sub_attack_air_uniq_process_exec_fix_pos(self)
        }
    }

    pub fn status_end_AttackAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_AttackAir(self).into()
        }
    }

    pub fn sub_attack_air_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attack_air_uniq_process_init(self).into()
        }
    }

    pub fn sub_attack_air_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attack_air_uniq_process_exec(self).into()
        }
    }

    pub fn sub_attack_air_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attack_air_uniq_process_exit(self).into()
        }
    }

    pub fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_init(self).into()
        }
    }

    pub fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec(self).into()
        }
    }

    pub fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos(&mut self) {
        unsafe {
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos(self)
        }
    }

    pub fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_exit(self).into()
        }
    }

    pub fn status_pre_ReboundStop(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ReboundStop(self).into()
        }
    }

    pub fn status_ReboundStop_common(&mut self) {
        unsafe {
            status_ReboundStop_common(self)
        }
    }

    pub fn status_ReboundStop(&mut self) -> lib::L2CValue {
        unsafe {
            status_ReboundStop(self).into()
        }
    }

    pub fn status_ReboundStop_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ReboundStop_Main(self).into()
        }
    }

    pub fn status_end_ReboundStop(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ReboundStop(self).into()
        }
    }

    pub fn status_pre_Rebound(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Rebound(self).into()
        }
    }

    pub fn status_Rebound(&mut self) -> lib::L2CValue {
        unsafe {
            status_Rebound(self).into()
        }
    }

    pub fn status_Rebound_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Rebound_Main(self).into()
        }
    }

    pub fn status_end_Rebound(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Rebound(self).into()
        }
    }

    pub fn status_pre_ReboundJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ReboundJump(self).into()
        }
    }

    pub fn status_pre_Jump_Common_param(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_Jump_Common_param(self, arg1.into()).into()
        }
    }

    pub fn status_pre_Jump_Common(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Jump_Common(self).into()
        }
    }

    pub fn status_ReboundJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_ReboundJump(self).into()
        }
    }

    pub fn rebound_jump_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            rebound_jump_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_ReboundJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ReboundJump_Main(self).into()
        }
    }

    pub fn status_end_ReboundJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ReboundJump(self).into()
        }
    }

    pub fn status_pre_BittenWario(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BittenWario(self).into()
        }
    }

    pub fn status_BittenWario(&mut self) -> lib::L2CValue {
        unsafe {
            status_BittenWario(self).into()
        }
    }

    pub fn sub_status_BittenWario_uniq(&mut self) -> lib::L2CValue {
        unsafe {
            sub_status_BittenWario_uniq(self).into()
        }
    }

    pub fn status_BittenWario_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BittenWario_Main(self).into()
        }
    }

    pub fn check_common_BittenWario(&mut self) -> lib::L2CValue {
        unsafe {
            check_common_BittenWario(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBittenWario_exec_fix_pos_counter(&mut self) {
        unsafe {
            FighterStatusUniqProcessBittenWario_exec_fix_pos_counter(self)
        }
    }

    pub fn FighterStatusUniqProcessBittenWario_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessBittenWario_exec_fix_pos(self)
        }
    }

    pub fn FighterStatusUniqProcessBittenWario_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBittenWario_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBittenWario_init_status_sub(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessBittenWario_init_status_sub(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessBittenWario_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBittenWario_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBittenWario_exec_fix_pos_slow(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBittenWario_exec_fix_pos_slow(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBittenWario_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBittenWario_exit_status(self).into()
        }
    }

    pub fn local_func__fighter_status_bitten_wario_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_bitten_wario_1(self).into()
        }
    }

    pub fn FighterStatusCapture_set_invalid_capture(&mut self) {
        unsafe {
            FighterStatusCapture_set_invalid_capture(self)
        }
    }

    pub fn FighterStatusUniqProcessBittenWario_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBittenWario_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_BittenWarioEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BittenWarioEnd(self).into()
        }
    }

    pub fn status_BittenWarioEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_BittenWarioEnd(self).into()
        }
    }

    pub fn status_BittenWarioEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BittenWarioEnd_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBittenWarioEnd_exec_fix_pos_counter(&mut self) {
        unsafe {
            FighterStatusUniqProcessBittenWarioEnd_exec_fix_pos_counter(self)
        }
    }

    pub fn FighterStatusUniqProcessBittenWarioEnd_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessBittenWarioEnd_exec_fix_pos(self)
        }
    }

    pub fn FighterStatusUniqProcessBittenWarioEnd_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBittenWarioEnd_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBittenWarioEnd_init_status_sub(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessBittenWarioEnd_init_status_sub(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessBittenWarioEnd_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBittenWarioEnd_exit_status(self).into()
        }
    }

    pub fn local_func__fighter_status_bitten_wario_end_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_bitten_wario_end_1(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBittenWarioEnd_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBittenWarioEnd_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_BittenWarioStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BittenWarioStart(self).into()
        }
    }

    pub fn status_BittenWarioStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_BittenWarioStart(self).into()
        }
    }

    pub fn status_BittenWarioStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BittenWarioStart_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBittenWarioStart_exec_fix_pos_counter(&mut self) {
        unsafe {
            FighterStatusUniqProcessBittenWarioStart_exec_fix_pos_counter(self)
        }
    }

    pub fn FighterStatusUniqProcessBittenWarioStart_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessBittenWarioStart_exec_fix_pos(self)
        }
    }

    pub fn FighterStatusUniqProcessBittenWarioStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBittenWarioStart_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBittenWarioStart_init_status_sub(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessBittenWarioStart_init_status_sub(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessBittenWarioStart_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBittenWarioStart_exit_status(self).into()
        }
    }

    pub fn local_func__fighter_status_bitten_wario_start_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_bitten_wario_start_1(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBittenWarioStart_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBittenWarioStart_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_BossEntry(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BossEntry(self).into()
        }
    }

    pub fn status_BossEntry(&mut self) -> lib::L2CValue {
        unsafe {
            status_BossEntry(self).into()
        }
    }

    pub fn status_BossEntry_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BossEntry_Main(self).into()
        }
    }

    pub fn status_end_BossEntry(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BossEntry(self).into()
        }
    }

    pub fn is_turn_motion_bossdeadsubfunc(&mut self) -> lib::L2CValue {
        unsafe {
            is_turn_motion_bossdeadsubfunc(self).into()
        }
    }

    pub fn is_turned_frame_bossdeadsubfunc(&mut self) -> lib::L2CValue {
        unsafe {
            is_turned_frame_bossdeadsubfunc(self).into()
        }
    }

    pub fn reverse_lr_bossdeadsubfunc(&mut self) {
        unsafe {
            reverse_lr_bossdeadsubfunc(self)
        }
    }

    pub fn status_pre_BossDead(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BossDead(self).into()
        }
    }

    pub fn status_BossDead(&mut self) -> lib::L2CValue {
        unsafe {
            status_BossDead(self).into()
        }
    }

    pub fn status_BossDead_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BossDead_Main(self).into()
        }
    }

    pub fn status_end_BossDead(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BossDead(self).into()
        }
    }

    pub fn sub_exit_common(&mut self) {
        unsafe {
            sub_exit_common(self)
        }
    }

    pub fn sub_update_bury_shake(&mut self) {
        unsafe {
            sub_update_bury_shake(self)
        }
    }

    pub fn sub_update_bury_effect(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_update_bury_effect(self, arg1.into())
        }
    }

    pub fn BuryRecovery(&mut self) {
        unsafe {
            BuryRecovery(self)
        }
    }

    pub fn check_damage_bury(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            check_damage_bury(self, arg1.into()).into()
        }
    }

    pub fn status_pre_Bury(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Bury(self).into()
        }
    }

    pub fn sub_bury_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_bury_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_Bury(&mut self) -> lib::L2CValue {
        unsafe {
            status_Bury(self).into()
        }
    }

    pub fn status_Bury_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Bury_Main(self).into()
        }
    }

    pub fn status_end_Bury(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Bury(self).into()
        }
    }

    pub fn uniq_process_Bury_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_Bury_init_status(self).into()
        }
    }

    pub fn local_func__fighter_status_bury_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_bury_1(self).into()
        }
    }

    pub fn FighterStatusUniqProcessDamage_init_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessDamage_init_status(self)
        }
    }

    pub fn FighterStatusUniqProcessDamage_check_hit_stop_delay(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessDamage_check_hit_stop_delay(self, arg1.into())
        }
    }

    pub fn local_func__fighter_status_damage_16(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_16(self).into()
        }
    }

    pub fn FighterStatusUniqProcessDamage_hit_stop_delay(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessDamage_hit_stop_delay(self, arg1.into(), arg2.into(), arg3.into(), arg4.into())
        }
    }

    pub fn local_func__fighter_status_damage_14(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_14(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_15(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_15(self).into()
        }
    }

    pub fn uniq_process_Bury_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_Bury_exec_status(self).into()
        }
    }

    pub fn uniq_process_Bury_exec_fix_pos_counter(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_Bury_exec_fix_pos_counter(self).into()
        }
    }

    pub fn uniq_process_Bury_exec_fix_pos(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_Bury_exec_fix_pos(self).into()
        }
    }

    pub fn uniq_process_Bury_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_Bury_exec_stop(self).into()
        }
    }

    pub fn uniq_process_Bury_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_Bury_exit_status(self).into()
        }
    }

    pub fn sub_damage_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_uniq_process_exit(self).into()
        }
    }

    pub fn status_pre_BuryWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BuryWait(self).into()
        }
    }

    pub fn status_BuryWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_BuryWait(self).into()
        }
    }

    pub fn status_BuryWait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BuryWait_Main(self).into()
        }
    }

    pub fn uniq_process_BuryWait_main(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryWait_main(self).into()
        }
    }

    pub fn uniq_process_BuryWait_exec_fix_pos_counter(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryWait_exec_fix_pos_counter(self).into()
        }
    }

    pub fn status_end_BuryWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BuryWait(self).into()
        }
    }

    pub fn uniq_process_BuryWait_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryWait_init_status(self).into()
        }
    }

    pub fn uniq_process_BuryWait_exec_fix_pos(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryWait_exec_fix_pos(self).into()
        }
    }

    pub fn uniq_process_BuryWait_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryWait_exec_status(self).into()
        }
    }

    pub fn uniq_process_BuryWait_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryWait_exec_stop(self).into()
        }
    }

    pub fn uniq_process_BuryWait_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryWait_exit_status(self).into()
        }
    }

    pub fn sub_BuryJumpUniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_BuryJumpUniq(self, arg1.into()).into()
        }
    }

    pub fn status_pre_BuryJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BuryJump(self).into()
        }
    }

    pub fn status_BuryJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_BuryJump(self).into()
        }
    }

    pub fn status_BuryJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BuryJump_Main(self).into()
        }
    }

    pub fn uniq_process_BuryJump_main(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryJump_main(self).into()
        }
    }

    pub fn status_end_BuryJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BuryJump(self).into()
        }
    }

    pub fn uniq_process_BuryJump_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryJump_init_status(self).into()
        }
    }

    pub fn uniq_process_BuryJump_exec_fix_pos_counter(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryJump_exec_fix_pos_counter(self).into()
        }
    }

    pub fn uniq_process_BuryJump_exec_fix_pos(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryJump_exec_fix_pos(self).into()
        }
    }

    pub fn uniq_process_BuryJump_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryJump_exec_status(self).into()
        }
    }

    pub fn uniq_process_BuryJump_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_BuryJump_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCapture_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCapture_check_damage(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessCapture_exit_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessCapture_exit_status(self)
        }
    }

    pub fn FighterStatusUniqProcessCapture_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCapture_exec_stop(self).into()
        }
    }

    pub fn status_CapturePulled(&mut self) -> lib::L2CValue {
        unsafe {
            status_CapturePulled(self).into()
        }
    }

    pub fn sub_capture_pulled_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_pulled_uniq_process_exec(self).into()
        }
    }

    pub fn sub_capture_pulled_uniq_process_exec_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_capture_pulled_uniq_process_exec_common(self, arg1.into())
        }
    }

    pub fn process_fix_camera_CapturePulled(&mut self) -> lib::L2CValue {
        unsafe {
            process_fix_camera_CapturePulled(self).into()
        }
    }

    pub fn sub_capture_pulled_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_pulled_uniq_process_exit(self).into()
        }
    }

    pub fn sub_capture_pulled_uniq_process_exit_core(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_capture_pulled_uniq_process_exit_core(self, arg1.into())
        }
    }

    pub fn status_pre_CaptureWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureWait(self).into()
        }
    }

    pub fn sub_capture_wait_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_wait_uniq_process_init(self).into()
        }
    }

    pub fn sub_capture_wait_uniq_process_init_core(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_capture_wait_uniq_process_init_core(self, arg1.into())
        }
    }

    pub fn status_CaptureWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureWait(self).into()
        }
    }

    pub fn status_CaptureWait_Loop(&mut self) {
        unsafe {
            status_CaptureWait_Loop(self)
        }
    }

    pub fn status_CaptureWait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureWait_Main(self).into()
        }
    }

    pub fn CaptureWaitCommon(&mut self) -> lib::L2CValue {
        unsafe {
            CaptureWaitCommon(self).into()
        }
    }

    pub fn sub_capture_wait_uniq_process_fix_pos(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_wait_uniq_process_fix_pos(self).into()
        }
    }

    pub fn sub_capture_wait_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_wait_uniq_process_exec(self).into()
        }
    }

    pub fn process_fix_camera_CaptureWait(&mut self) -> lib::L2CValue {
        unsafe {
            process_fix_camera_CaptureWait(self).into()
        }
    }

    pub fn status_end_CaptureWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureWait(self).into()
        }
    }

    pub fn status_pre_CaptureDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureDamage(self).into()
        }
    }

    pub fn sub_capture_damage_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_damage_uniq_process_init(self).into()
        }
    }

    pub fn status_CaptureDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureDamage(self).into()
        }
    }

    pub fn CaptureDamageCommon_Loop(&mut self) {
        unsafe {
            CaptureDamageCommon_Loop(self)
        }
    }

    pub fn status_CaptureDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureDamage_Main(self).into()
        }
    }

    pub fn CaptureDamageCommon(&mut self) -> lib::L2CValue {
        unsafe {
            CaptureDamageCommon(self).into()
        }
    }

    pub fn sub_capture_damage_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_damage_uniq_process_exec(self).into()
        }
    }

    pub fn process_fix_camera_CaptureDamage(&mut self) -> lib::L2CValue {
        unsafe {
            process_fix_camera_CaptureDamage(self).into()
        }
    }

    pub fn status_end_CaptureDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureDamage(self).into()
        }
    }

    pub fn status_pre_CaptureCut(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureCut(self).into()
        }
    }

    pub fn sub_capture_cut_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_cut_uniq_process_init(self).into()
        }
    }

    pub fn status_CaptureCut(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureCut(self).into()
        }
    }

    pub fn status_CaptureCut_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureCut_Main(self).into()
        }
    }

    pub fn status_end_CaptureCut(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureCut(self).into()
        }
    }

    pub fn status_pre_CaptureJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureJump(self).into()
        }
    }

    pub fn status_CaptureJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureJump(self).into()
        }
    }

    pub fn status_CaptureJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureJump_Main(self).into()
        }
    }

    pub fn status_end_CaptureJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureJump(self).into()
        }
    }

    pub fn status_pre_Thrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Thrown(self).into()
        }
    }

    pub fn sub_thrown_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_thrown_uniq_process_init(self).into()
        }
    }

    pub fn sub_Thrown_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_Thrown_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_Thrown_common(&mut self) -> lib::L2CValue {
        unsafe {
            status_Thrown_common(self).into()
        }
    }

    pub fn status_Thrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_Thrown(self).into()
        }
    }

    pub fn status_Thrown_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Thrown_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessThrown_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessThrown_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessThrown_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessThrown_exec_stop(self).into()
        }
    }

    pub fn sub_thrown_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_thrown_uniq_process_exit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessThrown_exit_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessThrown_exit_status(self)
        }
    }

    pub fn status_end_Thrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Thrown(self).into()
        }
    }

    pub fn FighterStatusUniqProcessThrown_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessThrown_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CaptureBlackhole(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureBlackhole(self).into()
        }
    }

    pub fn status_CaptureBlackhole(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureBlackhole(self).into()
        }
    }

    pub fn status_CaptureBlackhole_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureBlackhole_Main(self).into()
        }
    }

    pub fn sub_capture_blackhole_scale_animation(&mut self) {
        unsafe {
            sub_capture_blackhole_scale_animation(self)
        }
    }

    pub fn FighterStatusUniqProcessCaptureBlackhole_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessCaptureBlackhole_exec_fix_pos(self)
        }
    }

    pub fn status_end_CaptureBlackhole(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureBlackhole(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureBlackhole_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureBlackhole_init_status(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_blackhole_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_blackhole_1(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureBlackhole_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureBlackhole_exec_status(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_blackhole_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_blackhole_2(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_blackhole_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_blackhole_3(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_blackhole_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_blackhole_4(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_blackhole_5(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_blackhole_5(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureBlackhole_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureBlackhole_exit_status(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_blackhole_6(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_blackhole_6(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureBlackhole_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureBlackhole_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CaptureMastercore(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureMastercore(self).into()
        }
    }

    pub fn status_CaptureMastercore(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureMastercore(self).into()
        }
    }

    pub fn status_CaptureMastercore_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureMastercore_Main(self).into()
        }
    }

    pub fn status_end_CaptureMastercore(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureMastercore(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureMastercore_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureMastercore_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CaptureMasterhand(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureMasterhand(self).into()
        }
    }

    pub fn status_CaptureMasterhand(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureMasterhand(self).into()
        }
    }

    pub fn status_CaptureMasterhand_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureMasterhand_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureMasterhand_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessCaptureMasterhand_exec_fix_pos(self)
        }
    }

    pub fn status_end_CaptureMasterhand(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureMasterhand(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureMasterhand_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureMasterhand_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureMasterhand_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureMasterhand_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureMasterhand_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureMasterhand_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CaptureNabbit(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureNabbit(self).into()
        }
    }

    pub fn status_CaptureNabbit(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureNabbit(self).into()
        }
    }

    pub fn status_CaptureNabbit_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureNabbit_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureNabbit_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessCaptureNabbit_exec_fix_pos(self)
        }
    }

    pub fn status_end_CaptureNabbit(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureNabbit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureNabbit_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureNabbit_check_damage(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureNabbit_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureNabbit_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureNabbit_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureNabbit_exit_status(self).into()
        }
    }

    pub fn status_pre_CapturePulledFishingrod(&mut self) {
        unsafe {
            status_pre_CapturePulledFishingrod(self)
        }
    }

    pub fn status_CapturePulledFishingrod(&mut self) -> lib::L2CValue {
        unsafe {
            status_CapturePulledFishingrod(self).into()
        }
    }

    pub fn status_CapturePulledFishingrod_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CapturePulledFishingrod_Main(self).into()
        }
    }

    pub fn status_end_CapturePulledFishingrod(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CapturePulledFishingrod(self).into()
        }
    }

    pub fn sub_capture_octopus_init(&mut self) {
        unsafe {
            sub_capture_octopus_init(self)
        }
    }

    pub fn sub_capture_octopus_exec(&mut self) {
        unsafe {
            sub_capture_octopus_exec(self)
        }
    }

    pub fn sub_capture_octopus_exit(&mut self) {
        unsafe {
            sub_capture_octopus_exit(self)
        }
    }

    pub fn sub_capture_pulled_octopus_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_pulled_octopus_uniq_process_init(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_octopus_1(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_octopus_1(self, arg1.into()).into()
        }
    }

    pub fn sub_capture_pulled_octopus_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_pulled_octopus_uniq_process_exec(self).into()
        }
    }

    pub fn sub_capture_pulled_octopus_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_pulled_octopus_uniq_process_exit(self).into()
        }
    }

    pub fn status_pre_CapturePulledOctopus(&mut self) {
        unsafe {
            status_pre_CapturePulledOctopus(self)
        }
    }

    pub fn status_CapturePulledOctopus(&mut self) -> lib::L2CValue {
        unsafe {
            status_CapturePulledOctopus(self).into()
        }
    }

    pub fn status_CapturePulledOctopus_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CapturePulledOctopus_Main(self).into()
        }
    }

    pub fn status_end_CapturePulledOctopus(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CapturePulledOctopus(self).into()
        }
    }

    pub fn sub_capture_wait_octopus_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_wait_octopus_uniq_process_init(self).into()
        }
    }

    pub fn sub_capture_wait_octopus_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_wait_octopus_uniq_process_exec(self).into()
        }
    }

    pub fn sub_capture_wait_octopus_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_capture_wait_octopus_uniq_process_exit(self).into()
        }
    }

    pub fn status_pre_CaptureWaitOctopus(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureWaitOctopus(self).into()
        }
    }

    pub fn status_CaptureWaitOctopus(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureWaitOctopus(self).into()
        }
    }

    pub fn status_CaptureWaitOctopus_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureWaitOctopus_Main(self).into()
        }
    }

    pub fn status_end_CaptureWaitOctopus(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureWaitOctopus(self).into()
        }
    }

    pub fn status_pre_CaptureYoshi(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureYoshi(self).into()
        }
    }

    pub fn status_CaptureYoshi(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureYoshi(self).into()
        }
    }

    pub fn status_CaptureYoshi_mtrans(&mut self) {
        unsafe {
            status_CaptureYoshi_mtrans(self)
        }
    }

    pub fn status_CaptureYoshi_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureYoshi_Main(self).into()
        }
    }

    pub fn status_end_CaptureYoshi(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureYoshi(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureYoshi_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureYoshi_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureYoshi_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureYoshi_exit_status(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_yoshi_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_yoshi_1(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_yoshi_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_yoshi_2(self).into()
        }
    }

    pub fn sub_uniq_process_capture_yoshi_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_capture_yoshi_exec(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureYoshi_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureYoshi_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_YoshiEgg(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_YoshiEgg(self).into()
        }
    }

    pub fn status_YoshiEgg(&mut self) -> lib::L2CValue {
        unsafe {
            status_YoshiEgg(self).into()
        }
    }

    pub fn status_YoshiEgg_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_YoshiEgg_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessYoshiEgg_exec_fix_pos_counter(&mut self) {
        unsafe {
            FighterStatusUniqProcessYoshiEgg_exec_fix_pos_counter(self)
        }
    }

    pub fn FighterStatusUniqProcessYoshiEgg_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessYoshiEgg_exec_fix_pos(self)
        }
    }

    pub fn status_end_YoshiEgg(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_YoshiEgg(self).into()
        }
    }

    pub fn FighterStatusUniqProcessYoshiEgg_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessYoshiEgg_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessYoshiEgg_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessYoshiEgg_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessYoshiEgg_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessYoshiEgg_exec_stop(self).into()
        }
    }

    pub fn FighterStatusUniqProcessYoshiEgg_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessYoshiEgg_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessYoshiEgg_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessYoshiEgg_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CapturePulledYoshi(&mut self) {
        unsafe {
            status_pre_CapturePulledYoshi(self)
        }
    }

    pub fn status_CapturePulledYoshi(&mut self) -> lib::L2CValue {
        unsafe {
            status_CapturePulledYoshi(self).into()
        }
    }

    pub fn status_CapturePulledYoshi_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CapturePulledYoshi_Main(self).into()
        }
    }

    pub fn status_end_CapturePulledYoshi(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CapturePulledYoshi(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCapturePulledYoshi_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCapturePulledYoshi_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCapturePulledYoshi_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCapturePulledYoshi_exit_status(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_yoshi_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_yoshi_3(self).into()
        }
    }

    pub fn status_pre_CaptureWaitYoshi(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureWaitYoshi(self).into()
        }
    }

    pub fn status_CaptureWaitYoshi(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureWaitYoshi(self).into()
        }
    }

    pub fn status_CaptureWaitYoshi_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureWaitYoshi_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureWaitYoshi_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessCaptureWaitYoshi_exec_fix_pos(self)
        }
    }

    pub fn status_end_CaptureWaitYoshi(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureWaitYoshi(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureWaitYoshi_init_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessCaptureWaitYoshi_init_status(self)
        }
    }

    pub fn FighterStatusUniqProcessCaptureWaitYoshi_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureWaitYoshi_exit_status(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_yoshi_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_yoshi_4(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureWaitYoshi_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureWaitYoshi_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CaptureDamageYoshi(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureDamageYoshi(self).into()
        }
    }

    pub fn status_CaptureDamageYoshi(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureDamageYoshi(self).into()
        }
    }

    pub fn status_CaptureDamageYoshi_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureDamageYoshi_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureDamageYoshi_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessCaptureDamageYoshi_exec_fix_pos(self)
        }
    }

    pub fn status_end_CaptureDamageYoshi(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureDamageYoshi(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureDamageYoshi_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureDamageYoshi_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureDamageYoshi_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureDamageYoshi_exit_status(self).into()
        }
    }

    pub fn local_func__fighter_status_capture_yoshi_5(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_capture_yoshi_5(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureDamageYoshi_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureDamageYoshi_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_Catch(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Catch(self).into()
        }
    }

    pub fn status_pre_Catch_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_Catch_common(self, arg1.into()).into()
        }
    }

    pub fn sub_status_Catch(&mut self) {
        unsafe {
            sub_status_Catch(self)
        }
    }

    pub fn status_Catch(&mut self) -> lib::L2CValue {
        unsafe {
            status_Catch(self).into()
        }
    }

    pub fn status_Catch_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Catch_Main(self).into()
        }
    }

    pub fn status_end_Catch(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Catch(self).into()
        }
    }

    pub fn status_pre_CatchPull(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchPull(self).into()
        }
    }

    pub fn status_pre_CatchPull_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_CatchPull_common(self, arg1.into()).into()
        }
    }

    pub fn sub_catch_pull_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_catch_pull_uniq_process_init(self).into()
        }
    }

    pub fn status_CatchPull_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_CatchPull_common(self, arg1.into())
        }
    }

    pub fn status_CatchPull(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchPull(self).into()
        }
    }

    pub fn status_CatchPull_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchPull_Main(self).into()
        }
    }

    pub fn CatchCont(&mut self) -> lib::L2CValue {
        unsafe {
            CatchCont(self).into()
        }
    }

    pub fn IsThrowStick(&mut self) -> lib::L2CValue {
        unsafe {
            IsThrowStick(self).into()
        }
    }

    pub fn local_func__fighter_status_catch_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_catch_1(self).into()
        }
    }

    pub fn sub_is_not_throw_status_kind(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_is_not_throw_status_kind(self, arg1.into()).into()
        }
    }

    pub fn sub_catch_pull_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_catch_pull_uniq_process_exit(self).into()
        }
    }

    pub fn sub_is_throw_status_kind(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_is_throw_status_kind(self, arg1.into()).into()
        }
    }

    pub fn status_end_CatchPull(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchPull(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCatchPull_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCatchPull_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CatchDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchDash(self).into()
        }
    }

    pub fn status_pre_CatchDash_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_CatchDash_common(self, arg1.into()).into()
        }
    }

    pub fn sub_status_CatchDash(&mut self) {
        unsafe {
            sub_status_CatchDash(self)
        }
    }

    pub fn CatchDashUniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            CatchDashUniq(self, arg1.into()).into()
        }
    }

    pub fn status_CatchDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchDash(self).into()
        }
    }

    pub fn status_CatchDash_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchDash_Main(self).into()
        }
    }

    pub fn status_end_CatchDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchDash(self).into()
        }
    }

    pub fn status_pre_CatchDashPull(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchDashPull(self).into()
        }
    }

    pub fn status_CatchDashPull_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_CatchDashPull_common(self, arg1.into()).into()
        }
    }

    pub fn status_CatchDashPull(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchDashPull(self).into()
        }
    }

    pub fn status_CatchDashPull_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchDashPull_Main(self).into()
        }
    }

    pub fn status_end_CatchDashPull(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchDashPull(self).into()
        }
    }

    pub fn status_pre_CatchTurn(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchTurn(self).into()
        }
    }

    pub fn status_pre_CatchTurn_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_CatchTurn_common(self, arg1.into()).into()
        }
    }

    pub fn sub_status_CatchTurn(&mut self) {
        unsafe {
            sub_status_CatchTurn(self)
        }
    }

    pub fn status_CatchTurn(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchTurn(self).into()
        }
    }

    pub fn status_CatchTurn_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchTurn_Main(self).into()
        }
    }

    pub fn status_end_CatchTurn(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchTurn(self).into()
        }
    }

    pub fn status_pre_CatchWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchWait(self).into()
        }
    }

    pub fn sub_catch_wait_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_catch_wait_uniq_process_init(self).into()
        }
    }

    pub fn status_CatchWait_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_CatchWait_common(self, arg1.into()).into()
        }
    }

    pub fn status_CatchWait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchWait_Main(self).into()
        }
    }

    pub fn status_CatchWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchWait(self).into()
        }
    }

    pub fn sub_catch_wait_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_catch_wait_uniq_process_exit(self).into()
        }
    }

    pub fn status_end_CatchWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchWait(self).into()
        }
    }

    pub fn status_pre_CatchAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchAttack(self).into()
        }
    }

    pub fn status_CatchAttack_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_CatchAttack_common(self, arg1.into()).into()
        }
    }

    pub fn status_CatchAttack_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchAttack_Main(self).into()
        }
    }

    pub fn status_CatchAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchAttack(self).into()
        }
    }

    pub fn status_end_CatchAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchAttack(self).into()
        }
    }

    pub fn status_pre_CatchCut(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchCut(self).into()
        }
    }

    pub fn sub_catch_cut_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_catch_cut_uniq_process_init(self).into()
        }
    }

    pub fn status_CatchCut(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchCut(self).into()
        }
    }

    pub fn status_CatchCut_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchCut_Main(self).into()
        }
    }

    pub fn status_end_CatchCut(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchCut(self).into()
        }
    }

    pub fn status_pre_CatchJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchJump(self).into()
        }
    }

    pub fn status_CatchJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchJump(self).into()
        }
    }

    pub fn status_CatchJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchJump_Main(self).into()
        }
    }

    pub fn status_end_CatchJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchJump(self).into()
        }
    }

    pub fn status_pre_Throw(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Throw(self).into()
        }
    }

    pub fn status_Throw_Sub(&mut self) {
        unsafe {
            status_Throw_Sub(self)
        }
    }

    pub fn ThrowLogCommon(&mut self) {
        unsafe {
            ThrowLogCommon(self)
        }
    }

    pub fn ThrowUniq(&mut self) -> lib::L2CValue {
        unsafe {
            ThrowUniq(self).into()
        }
    }

    pub fn status_Throw(&mut self) -> lib::L2CValue {
        unsafe {
            status_Throw(self).into()
        }
    }

    pub fn status_Throw_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Throw_Main(self).into()
        }
    }

    pub fn status_end_Throw(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Throw(self).into()
        }
    }

    pub fn sub_throw_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_throw_uniq_process_init(self).into()
        }
    }

    pub fn sub_throw_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_throw_uniq_process_exit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessThrow_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessThrow_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CatchedGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchedGanon(self).into()
        }
    }

    pub fn status_CatchedGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedGanon(self).into()
        }
    }

    pub fn status_CatchedGanon_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedGanon_Main(self).into()
        }
    }

    pub fn status_end_CatchedGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchedGanon(self).into()
        }
    }

    pub fn sub_uniq_process_CatchedGanon_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_CatchedGanon_init(self).into()
        }
    }

    pub fn sub_uniq_process_CatchedGanon_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_CatchedGanon_exit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCatchedGanon_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCatchedGanon_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CatchedAirGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchedAirGanon(self).into()
        }
    }

    pub fn status_CatchedAirGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedAirGanon(self).into()
        }
    }

    pub fn status_CatchedAirGanon_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedAirGanon_Main(self).into()
        }
    }

    pub fn status_end_CatchedAirGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchedAirGanon(self).into()
        }
    }

    pub fn sub_uniq_process_CatchedAirGanon_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_CatchedAirGanon_init(self).into()
        }
    }

    pub fn sub_uniq_process_CatchedAirGanon_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_CatchedAirGanon_exit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCatchedAirGanon_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCatchedAirGanon_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CatchedAirFallGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchedAirFallGanon(self).into()
        }
    }

    pub fn status_CatchedAirFallGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedAirFallGanon(self).into()
        }
    }

    pub fn status_CatchedAirFallGanon_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedAirFallGanon_Main(self).into()
        }
    }

    pub fn status_end_CatchedAirFallGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchedAirFallGanon(self).into()
        }
    }

    pub fn sub_uniq_process_CatchedAirFallGanon_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_CatchedAirFallGanon_init(self).into()
        }
    }

    pub fn sub_uniq_process_CatchedAirFallGanon_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_CatchedAirFallGanon_exit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCatchedAirFallGanon_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCatchedAirFallGanon_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CatchedAirEndGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchedAirEndGanon(self).into()
        }
    }

    pub fn status_CatchedAirEndGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedAirEndGanon(self).into()
        }
    }

    pub fn status_CatchedAirEndGanon_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedAirEndGanon_Main(self).into()
        }
    }

    pub fn status_end_CatchedAirEndGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchedAirEndGanon(self).into()
        }
    }

    pub fn sub_uniq_process_CatchedAirEndGanon_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_CatchedAirEndGanon_init(self).into()
        }
    }

    pub fn sub_uniq_process_CatchedAirEndGanon_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_CatchedAirEndGanon_exit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCatchedAirEndGanon_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCatchedAirEndGanon_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CatchedCutGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchedCutGanon(self).into()
        }
    }

    pub fn status_CatchedCutGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedCutGanon(self).into()
        }
    }

    pub fn status_CatchedCutGanon_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedCutGanon_Main(self).into()
        }
    }

    pub fn status_end_CatchedCutGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchedCutGanon(self).into()
        }
    }

    pub fn status_pre_CatchedReflet(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchedReflet(self).into()
        }
    }

    pub fn status_CatchedReflet(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedReflet(self).into()
        }
    }

    pub fn status_CatchedReflet_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedReflet_Main(self).into()
        }
    }

    pub fn process_fix_camera_CatchedReflet(&mut self) -> lib::L2CValue {
        unsafe {
            process_fix_camera_CatchedReflet(self).into()
        }
    }

    pub fn status_end_CatchedReflet(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchedReflet(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCatchedReflet_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCatchedReflet_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CatchedRidley(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CatchedRidley(self).into()
        }
    }

    pub fn status_CatchedRidley(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedRidley(self).into()
        }
    }

    pub fn status_CatchedRidley_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CatchedRidley_Main(self).into()
        }
    }

    pub fn status_end_CatchedRidley(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CatchedRidley(self).into()
        }
    }

    pub fn sub_uniq_process_CatchedRidley_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_CatchedRidley_init(self).into()
        }
    }

    pub fn sub_uniq_process_CatchedRidley_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_CatchedRidley_exec(self).into()
        }
    }

    pub fn sub_uniq_process_CatchedRidley_exec_fix_camera(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_CatchedRidley_exec_fix_camera(self).into()
        }
    }

    pub fn sub_uniq_process_CatchedRidley_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_CatchedRidley_exit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCatchedRidley_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCatchedRidley_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_DraggedRidley(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DraggedRidley(self).into()
        }
    }

    pub fn status_DraggedRidley(&mut self) -> lib::L2CValue {
        unsafe {
            status_DraggedRidley(self).into()
        }
    }

    pub fn status_DraggedRidley_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DraggedRidley_Main(self).into()
        }
    }

    pub fn status_end_DraggedRidley(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DraggedRidley(self).into()
        }
    }

    pub fn sub_uniq_process_DraggedRidley_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_DraggedRidley_init(self).into()
        }
    }

    pub fn sub_uniq_process_DraggedRidley_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_DraggedRidley_exec(self).into()
        }
    }

    pub fn sub_uniq_process_DraggedRidley_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_DraggedRidley_exit(self).into()
        }
    }

    pub fn set_cliff_xlu_frame(&mut self, arg1: lib::L2CValue) {
        unsafe {
            set_cliff_xlu_frame(self, arg1.into())
        }
    }

    pub fn status_pre_CliffCatchMove(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CliffCatchMove(self).into()
        }
    }

    pub fn status_CliffCatchMove(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffCatchMove(self).into()
        }
    }

    pub fn status_CliffCatchMove_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffCatchMove_Main(self).into()
        }
    }

    pub fn status_end_CliffCatchMove(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CliffCatchMove(self).into()
        }
    }

    pub fn sub_cliff_catch_move_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_cliff_catch_move_uniq_process_init(self).into()
        }
    }

    pub fn sub_cliff_catch_move_uniq_process_init_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            sub_cliff_catch_move_uniq_process_init_common(self, arg1.into(), arg2.into())
        }
    }

    pub fn sub_cliff_catch_move_uniq_process_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_cliff_catch_move_uniq_process_exec_status(self).into()
        }
    }

    pub fn sub_cliff_catch_move_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_cliff_catch_move_uniq_process_exit(self).into()
        }
    }

    pub fn sub_is_leave_cliff(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_is_leave_cliff(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CliffCatch(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CliffCatch(self).into()
        }
    }

    pub fn sub_status_CliffCatchCommon(&mut self) {
        unsafe {
            sub_status_CliffCatchCommon(self)
        }
    }

    pub fn status_CliffCatch(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffCatch(self).into()
        }
    }

    pub fn sub_status_CliffCatch_MainCommon(&mut self) -> lib::L2CValue {
        unsafe {
            sub_status_CliffCatch_MainCommon(self).into()
        }
    }

    pub fn sub_cliff_uniq_process_main(&mut self) {
        unsafe {
            sub_cliff_uniq_process_main(self)
        }
    }

    pub fn sub_cliff_uniq_process_exec_fix_pos(&mut self) {
        unsafe {
            sub_cliff_uniq_process_exec_fix_pos(self)
        }
    }

    pub fn status_CliffCatchSub(&mut self) {
        unsafe {
            status_CliffCatchSub(self)
        }
    }

    pub fn status_CliffCatch_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffCatch_Main(self).into()
        }
    }

    pub fn status_end_CliffCatch(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CliffCatch(self).into()
        }
    }

    pub fn sub_cliff_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_cliff_uniq_process_init(self).into()
        }
    }

    pub fn get_cliff_wait_hit_xlu_frame(&mut self) -> lib::L2CValue {
        unsafe {
            get_cliff_wait_hit_xlu_frame(self).into()
        }
    }

    pub fn sub_cliff_common_input(&mut self) {
        unsafe {
            sub_cliff_common_input(self)
        }
    }

    pub fn sub_cliff_uniq_process_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_cliff_uniq_process_exec_status(self).into()
        }
    }

    pub fn sub_cliff_uniq_process_exit_Common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_cliff_uniq_process_exit_Common(self, arg1.into())
        }
    }

    pub fn sub_cliff_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_cliff_uniq_process_exit(self).into()
        }
    }

    pub fn sub_cliff_wait_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_cliff_wait_uniq_process_init(self).into()
        }
    }

    pub fn sub_cliff_wait_uniq_process_init_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_cliff_wait_uniq_process_init_common(self, arg1.into())
        }
    }

    pub fn status_pre_CliffWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CliffWait(self).into()
        }
    }

    pub fn enable_cliff_wait_stans(&mut self) {
        unsafe {
            enable_cliff_wait_stans(self)
        }
    }

    pub fn sub_cliff_wait_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_cliff_wait_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_CliffWait_Sub(&mut self) {
        unsafe {
            status_CliffWait_Sub(self)
        }
    }

    pub fn status_CliffWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffWait(self).into()
        }
    }

    pub fn status_CliffWait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffWait_Main(self).into()
        }
    }

    pub fn status_end_CliffWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CliffWait(self).into()
        }
    }

    pub fn status_pre_CliffAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CliffAttack(self).into()
        }
    }

    pub fn status_CliffAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffAttack(self).into()
        }
    }

    pub fn status_CliffAttack_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffAttack_Main(self).into()
        }
    }

    pub fn status_end_CliffAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CliffAttack(self).into()
        }
    }

    pub fn status_pre_CliffClimb(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CliffClimb(self).into()
        }
    }

    pub fn status_CliffClimb(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffClimb(self).into()
        }
    }

    pub fn status_CliffClimb_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffClimb_Main(self).into()
        }
    }

    pub fn status_end_CliffClimb(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CliffClimb(self).into()
        }
    }

    pub fn status_pre_CliffEscape(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CliffEscape(self).into()
        }
    }

    pub fn status_CliffEscape(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffEscape(self).into()
        }
    }

    pub fn status_CliffEscape_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffEscape_Main(self).into()
        }
    }

    pub fn status_end_CliffEscape(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CliffEscape(self).into()
        }
    }

    pub fn status_pre_CliffJump1(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CliffJump1(self).into()
        }
    }

    pub fn status_CliffJump1(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffJump1(self).into()
        }
    }

    pub fn status_CliffJump1_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffJump1_Main(self).into()
        }
    }

    pub fn status_end_CliffJump1(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CliffJump1(self).into()
        }
    }

    pub fn status_pre_CliffJump2(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CliffJump2(self).into()
        }
    }

    pub fn status_CliffJump2(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffJump2(self).into()
        }
    }

    pub fn sub_cliff_jump2_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_cliff_jump2_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_CliffJump2_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffJump2_Main(self).into()
        }
    }

    pub fn sub_air_check_stop_ceil_pre(&mut self) {
        unsafe {
            sub_air_check_stop_ceil_pre(self)
        }
    }

    pub fn status_end_CliffJump2(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CliffJump2(self).into()
        }
    }

    pub fn status_pre_CliffJump3(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CliffJump3(self).into()
        }
    }

    pub fn status_CliffJump3(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffJump3(self).into()
        }
    }

    pub fn status_CliffJump3_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffJump3_Main(self).into()
        }
    }

    pub fn status_end_CliffJump3(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CliffJump3(self).into()
        }
    }

    pub fn status_pre_CliffRobbed(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CliffRobbed(self).into()
        }
    }

    pub fn status_CliffRobbed(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffRobbed(self).into()
        }
    }

    pub fn sub_cliff_robbed_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_cliff_robbed_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_CliffRobbed_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CliffRobbed_Main(self).into()
        }
    }

    pub fn status_end_CliffRobbed(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CliffRobbed(self).into()
        }
    }

    pub fn status_pre_MissFoot(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_MissFoot(self).into()
        }
    }

    pub fn status_MissFoot(&mut self) -> lib::L2CValue {
        unsafe {
            status_MissFoot(self).into()
        }
    }

    pub fn status_MissFoot_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_MissFoot_Main(self).into()
        }
    }

    pub fn sub_AirChkPassive(&mut self) -> lib::L2CValue {
        unsafe {
            sub_AirChkPassive(self).into()
        }
    }

    pub fn is_enable_passive(&mut self) -> lib::L2CValue {
        unsafe {
            is_enable_passive(self).into()
        }
    }

    pub fn sub_check_passive_button(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_check_passive_button(self, arg1.into()).into()
        }
    }

    pub fn sub_MissFootPassive(&mut self) {
        unsafe {
            sub_MissFootPassive(self)
        }
    }

    pub fn status_end_MissFoot(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MissFoot(self).into()
        }
    }

    pub fn status_pre_ClungCaptain(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ClungCaptain(self).into()
        }
    }

    pub fn status_ClungCaptain(&mut self) -> lib::L2CValue {
        unsafe {
            status_ClungCaptain(self).into()
        }
    }

    pub fn status_ClungCaptain_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ClungCaptain_Main(self).into()
        }
    }

    pub fn status_end_ClungCaptain(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ClungCaptain(self).into()
        }
    }

    pub fn sub_uniq_process_ClungCaptain_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_ClungCaptain_init(self).into()
        }
    }

    pub fn sub_uniq_process_ClungCaptain_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_ClungCaptain_exit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungCaptain_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessClungCaptain_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_ClungDiddy(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ClungDiddy(self).into()
        }
    }

    pub fn status_ClungDiddy(&mut self) -> lib::L2CValue {
        unsafe {
            status_ClungDiddy(self).into()
        }
    }

    pub fn sub_clung_diddy_mtrans(&mut self) {
        unsafe {
            sub_clung_diddy_mtrans(self)
        }
    }

    pub fn status_ClungDiddy_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ClungDiddy_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungDiddy_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessClungDiddy_exec_fix_pos(self)
        }
    }

    pub fn status_end_ClungDiddy(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ClungDiddy(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungDiddy_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessClungDiddy_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungDiddy_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessClungDiddy_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungDiddy_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessClungDiddy_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungDiddy_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessClungDiddy_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_ClungDamageDiddy(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ClungDamageDiddy(self).into()
        }
    }

    pub fn status_ClungDamageDiddy(&mut self) -> lib::L2CValue {
        unsafe {
            status_ClungDamageDiddy(self).into()
        }
    }

    pub fn sub_clung_damage_diddy_mtrans(&mut self) {
        unsafe {
            sub_clung_damage_diddy_mtrans(self)
        }
    }

    pub fn status_ClungDamageDiddy_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ClungDamageDiddy_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungDamageDiddy_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessClungDamageDiddy_exec_fix_pos(self)
        }
    }

    pub fn status_end_ClungDamageDiddy(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ClungDamageDiddy(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungDamageDiddy_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessClungDamageDiddy_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungDamageDiddy_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessClungDamageDiddy_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungDamageDiddy_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessClungDamageDiddy_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_ClungThrownDiddy(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ClungThrownDiddy(self).into()
        }
    }

    pub fn clung_thrown_diddy_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            clung_thrown_diddy_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_ClungThrownDiddy(&mut self) -> lib::L2CValue {
        unsafe {
            status_ClungThrownDiddy(self).into()
        }
    }

    pub fn status_ClungThrownJumpDiddy_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ClungThrownJumpDiddy_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungThrownDiddy_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessClungThrownDiddy_exec_fix_pos(self)
        }
    }

    pub fn status_end_ClungThrownDiddy(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ClungThrownDiddy(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungThrownDiddy_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessClungThrownDiddy_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungThrownDiddy_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessClungThrownDiddy_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_ClungThrownBlankDiddy(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ClungThrownBlankDiddy(self).into()
        }
    }

    pub fn status_ClungThrownBlankDiddy(&mut self) -> lib::L2CValue {
        unsafe {
            status_ClungThrownBlankDiddy(self).into()
        }
    }

    pub fn status_ClungThrownBlankDiddy_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ClungThrownBlankDiddy_Main(self).into()
        }
    }

    pub fn status_end_ClungThrownBlankDiddy(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ClungThrownBlankDiddy(self).into()
        }
    }

    pub fn status_pre_ClungGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ClungGanon(self).into()
        }
    }

    pub fn status_ClungGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_ClungGanon(self).into()
        }
    }

    pub fn status_ClungGanon_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ClungGanon_Main(self).into()
        }
    }

    pub fn sub_uniq_process_ClungGanon_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_ClungGanon_main(self).into()
        }
    }

    pub fn status_end_ClungGanon(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ClungGanon(self).into()
        }
    }

    pub fn sub_uniq_process_ClungGanon_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_ClungGanon_init(self).into()
        }
    }

    pub fn sub_uniq_process_ClungGanon_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_ClungGanon_exec_status(self).into()
        }
    }

    pub fn sub_uniq_process_ClungGanon_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_ClungGanon_exec_stop(self).into()
        }
    }

    pub fn sub_uniq_process_ClungGanon_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_ClungGanon_exit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessClungGanon_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessClungGanon_check_damage(self, arg1.into()).into()
        }
    }

    pub fn sub_FighterStatusDamage_get_coll_stop_slidable_length(&mut self) -> lib::L2CValue {
        unsafe {
            sub_FighterStatusDamage_get_coll_stop_slidable_length(self).into()
        }
    }

    pub fn sub_FighterStatusDamage_get_damage_fly_angle_compose(&mut self) -> lib::L2CValue {
        unsafe {
            sub_FighterStatusDamage_get_damage_fly_angle_compose(self).into()
        }
    }

    pub fn sub_FighterStatusDamage_correctDamageVectorExecStop(&mut self) {
        unsafe {
            sub_FighterStatusDamage_correctDamageVectorExecStop(self)
        }
    }

    pub fn FighterStatusDamage_init_damage_speed_up(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            FighterStatusDamage_init_damage_speed_up(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn FighterStatusDamage__create_damage_log_table(&mut self) {
        unsafe {
            FighterStatusDamage__create_damage_log_table(self)
        }
    }

    pub fn status_pre_Damage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Damage(self).into()
        }
    }

    pub fn status_Damage(&mut self) -> lib::L2CValue {
        unsafe {
            status_Damage(self).into()
        }
    }

    pub fn damage_ground_chk_uniq(&mut self) -> lib::L2CValue {
        unsafe {
            damage_ground_chk_uniq(self).into()
        }
    }

    pub fn status_Damage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Damage_Main(self).into()
        }
    }

    pub fn status_end_Damage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Damage(self).into()
        }
    }

    pub fn status_pre_DamageAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageAir(self).into()
        }
    }

    pub fn status_DamageAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageAir(self).into()
        }
    }

    pub fn damage_air_chk_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            damage_air_chk_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_DamageAir_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageAir_Main(self).into()
        }
    }

    pub fn status_end_DamageAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageAir(self).into()
        }
    }

    pub fn check_ryu_final_damage_03(&mut self, arg1: lib::L2CValue) {
        unsafe {
            check_ryu_final_damage_03(self, arg1.into())
        }
    }

    pub fn status_pre_DamageFly(&mut self) {
        unsafe {
            status_pre_DamageFly(self)
        }
    }

    pub fn status_DamageFly(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFly(self).into()
        }
    }

    pub fn status_DamageFly_Common(&mut self) {
        unsafe {
            status_DamageFly_Common(self)
        }
    }

    pub fn status_DamageFly_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFly_Main(self).into()
        }
    }

    pub fn status_DamageFinishCamera_exec(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFinishCamera_exec(self).into()
        }
    }

    pub fn sub_DamageFlyCommon(&mut self) -> lib::L2CValue {
        unsafe {
            sub_DamageFlyCommon(self).into()
        }
    }

    pub fn sub_AirChkDamageReflectWall(&mut self) -> lib::L2CValue {
        unsafe {
            sub_AirChkDamageReflectWall(self).into()
        }
    }

    pub fn sub_AirChkDamageReflectCeil(&mut self) -> lib::L2CValue {
        unsafe {
            sub_AirChkDamageReflectCeil(self).into()
        }
    }

    pub fn sub_AirChkDamageReflectFloor(&mut self) -> lib::L2CValue {
        unsafe {
            sub_AirChkDamageReflectFloor(self).into()
        }
    }

    pub fn sub_AirChkPassiveWallJump(&mut self) -> lib::L2CValue {
        unsafe {
            sub_AirChkPassiveWallJump(self).into()
        }
    }

    pub fn sub_AirChkPassiveWall(&mut self) -> lib::L2CValue {
        unsafe {
            sub_AirChkPassiveWall(self).into()
        }
    }

    pub fn sub_AirChkPassiveCeil(&mut self) -> lib::L2CValue {
        unsafe {
            sub_AirChkPassiveCeil(self).into()
        }
    }

    pub fn sub_DamageFlyChkUniq(&mut self) -> lib::L2CValue {
        unsafe {
            sub_DamageFlyChkUniq(self).into()
        }
    }

    pub fn sub_AirChkDown(&mut self) -> lib::L2CValue {
        unsafe {
            sub_AirChkDown(self).into()
        }
    }

    pub fn sub_AirChkPassive_for_damage(&mut self) -> lib::L2CValue {
        unsafe {
            sub_AirChkPassive_for_damage(self).into()
        }
    }

    pub fn sub_check_passive_button_for_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_check_passive_button_for_damage(self, arg1.into()).into()
        }
    }

    pub fn status_DamageFinishCamera_init(&mut self) {
        unsafe {
            status_DamageFinishCamera_init(self)
        }
    }

    pub fn status_DamageFinishCamera_StartEndSlow(&mut self) {
        unsafe {
            status_DamageFinishCamera_StartEndSlow(self)
        }
    }

    pub fn ftStatusUniqProcessDamageFly_init(&mut self, arg1: lib::L2CValue) {
        unsafe {
            ftStatusUniqProcessDamageFly_init(self, arg1.into())
        }
    }

    pub fn sub_ftStatusUniqProcessDamageFly_getMotionKind(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessDamageFly_getMotionKind(self).into()
        }
    }

    pub fn virtual_ftStatusUniqProcessDamage_init_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            virtual_ftStatusUniqProcessDamage_init_common(self, arg1.into())
        }
    }

    pub fn sub_ftStatusUniqProcessDamage_getMotionKind(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessDamage_getMotionKind(self).into()
        }
    }

    pub fn set_damage_motion_rate(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            set_damage_motion_rate(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn calc_damage_motion_rate(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            calc_damage_motion_rate(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn reaction_frame_mul_speed_up(&mut self) -> lib::L2CValue {
        unsafe {
            reaction_frame_mul_speed_up(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessDamage_getMotionKindSub(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessDamage_getMotionKindSub(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn ftStatusUniqProcessDamageFly_init_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            ftStatusUniqProcessDamageFly_init_common(self, arg1.into())
        }
    }

    pub fn ftStatusUniqProcessDamage_init_common(&mut self) {
        unsafe {
            ftStatusUniqProcessDamage_init_common(self)
        }
    }

    pub fn sub_ftStatusUniqProcessDamageFly_getMotionKindSub(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessDamageFly_getMotionKindSub(self, arg1.into()).into()
        }
    }

    pub fn sub_DamageFly_setup_strans(&mut self) {
        unsafe {
            sub_DamageFly_setup_strans(self)
        }
    }

    pub fn sub_DamageFlyCommon_check_dead(&mut self) {
        unsafe {
            sub_DamageFlyCommon_check_dead(self)
        }
    }

    pub fn sub_DamageFlyCommon_init(&mut self) {
        unsafe {
            sub_DamageFlyCommon_init(self)
        }
    }

    pub fn damage_fly_chk_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            damage_fly_chk_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_end_DamageFly(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageFly(self).into()
        }
    }

    pub fn sub_end_damage_fly_roll_to_dead(&mut self) {
        unsafe {
            sub_end_damage_fly_roll_to_dead(self)
        }
    }

    pub fn status_pre_DamageFlyRoll(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageFlyRoll(self).into()
        }
    }

    pub fn status_DamageFlyRoll(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFlyRoll(self).into()
        }
    }

    pub fn status_DamageFlyRoll_Common(&mut self) {
        unsafe {
            status_DamageFlyRoll_Common(self)
        }
    }

    pub fn status_DamageFlyRoll_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFlyRoll_Main(self).into()
        }
    }

    pub fn status_end_DamageFlyRoll(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageFlyRoll(self).into()
        }
    }

    pub fn ftStatusUniqProcessDamageAir_init(&mut self, arg1: lib::L2CValue) {
        unsafe {
            ftStatusUniqProcessDamageAir_init(self, arg1.into())
        }
    }

    pub fn sub_damage_uniq_process_getMotionKindAir(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_uniq_process_getMotionKindAir(self).into()
        }
    }

    pub fn sub_damage_uniq_process_getMotionKindAirSub(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_damage_uniq_process_getMotionKindAirSub(self, arg1.into()).into()
        }
    }

    pub fn ftStatusUniqProcessDamageFly_exec_common(&mut self) -> lib::L2CValue {
        unsafe {
            ftStatusUniqProcessDamageFly_exec_common(self).into()
        }
    }

    pub fn ftStatusUniqProcessDamage_exec_common(&mut self) {
        unsafe {
            ftStatusUniqProcessDamage_exec_common(self)
        }
    }

    pub fn sub_ftStatusUniqProcessDamageFly_execFixPosCounter(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessDamageFly_execFixPosCounter(self)
        }
    }

    pub fn sub_PacmanFinal_end_eye(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_PacmanFinal_end_eye(self, arg1.into())
        }
    }

    pub fn ftStatusUniqProcessDamageFly_exec(&mut self) {
        unsafe {
            ftStatusUniqProcessDamageFly_exec(self)
        }
    }

    pub fn virtual_ftStatusUniqProcessDamage_exec_common(&mut self) {
        unsafe {
            virtual_ftStatusUniqProcessDamage_exec_common(self)
        }
    }

    pub fn sub_ftStatusUniqProcessDamageFly_exitStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessDamageFly_exitStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessDamageFlyRoll_exitStatus(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessDamageFlyRoll_exitStatus(self)
        }
    }

    pub fn ftStatusUniqProcessDamageFlyRoll_init(&mut self, arg1: lib::L2CValue) {
        unsafe {
            ftStatusUniqProcessDamageFlyRoll_init(self, arg1.into())
        }
    }

    pub fn ftStatusUniqProcessDamageFlyRoll_exec(&mut self) {
        unsafe {
            ftStatusUniqProcessDamageFlyRoll_exec(self)
        }
    }

    pub fn ftStatusUniqProcessDamageFlyMeteor_exec(&mut self) {
        unsafe {
            ftStatusUniqProcessDamageFlyMeteor_exec(self)
        }
    }

    pub fn sub_ftStatusUniqProcessDamageFlyRoll_execStop(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessDamageFlyRoll_execStop(self)
        }
    }

    pub fn status_pre_DamageFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageFall(self).into()
        }
    }

    pub fn sub_DamageFall_common(&mut self) {
        unsafe {
            sub_DamageFall_common(self)
        }
    }

    pub fn sub_damage_fall_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_damage_fall_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_DamageFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFall(self).into()
        }
    }

    pub fn status_DamageFall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFall_Main(self).into()
        }
    }

    pub fn check_damage_fall_transition(&mut self) -> lib::L2CValue {
        unsafe {
            check_damage_fall_transition(self).into()
        }
    }

    pub fn sub_damage_fall_uniq_process_exec_fix_pos(&mut self) {
        unsafe {
            sub_damage_fall_uniq_process_exec_fix_pos(self)
        }
    }

    pub fn set_damage_fall_transition(&mut self, arg1: lib::L2CValue) {
        unsafe {
            set_damage_fall_transition(self, arg1.into())
        }
    }

    pub fn sub_ItemShoot_GenesisCommon_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShoot_GenesisCommon_Main_New(self).into()
        }
    }

    pub fn sub_damage_fall_uniq_process_main(&mut self) {
        unsafe {
            sub_damage_fall_uniq_process_main(self)
        }
    }

    pub fn status_end_DamageFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageFall(self).into()
        }
    }

    pub fn sub_damage_fall_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_fall_uniq_process_init(self).into()
        }
    }

    pub fn sub_damage_fall_uniq_process_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_fall_uniq_process_exec_status(self).into()
        }
    }

    pub fn status_pre_Ice(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Ice(self).into()
        }
    }

    pub fn status_Ice(&mut self) -> lib::L2CValue {
        unsafe {
            status_Ice(self).into()
        }
    }

    pub fn sub_ice_mtrans(&mut self) {
        unsafe {
            sub_ice_mtrans(self)
        }
    }

    pub fn status_Ice_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Ice_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessIce_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessIce_exec_fix_pos(self)
        }
    }

    pub fn local_func__fighter_status_damage_10(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_10(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_11(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_11(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_12(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_12(self).into()
        }
    }

    pub fn sub_IceUniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_IceUniq(self, arg1.into()).into()
        }
    }

    pub fn status_end_Ice(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Ice(self).into()
        }
    }

    pub fn FighterStatusUniqProcessIce_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessIce_init_status(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_4(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_5(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_5(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_6(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_6(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_7(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_7(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_8(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_8(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_9(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_9(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_3(self).into()
        }
    }

    pub fn FighterStatusUniqProcessIce_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessIce_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessIce_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessIce_exec_stop(self).into()
        }
    }

    pub fn FighterStatusUniqProcessIce_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessIce_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessIce_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessIce_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_IceJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_IceJump(self).into()
        }
    }

    pub fn status_IceJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_IceJump(self).into()
        }
    }

    pub fn sub_IceJumpUniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_IceJumpUniq(self, arg1.into()).into()
        }
    }

    pub fn status_IceJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_IceJump_Main(self).into()
        }
    }

    pub fn status_end_IceJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_IceJump(self).into()
        }
    }

    pub fn status_pre_Bind(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Bind(self).into()
        }
    }

    pub fn status_Bind(&mut self) -> lib::L2CValue {
        unsafe {
            status_Bind(self).into()
        }
    }

    pub fn status_Bind_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Bind_Main(self).into()
        }
    }

    pub fn status_end_Bind(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Bind(self).into()
        }
    }

    pub fn status_pre_PitFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_PitFall(self).into()
        }
    }

    pub fn status_PitFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_PitFall(self).into()
        }
    }

    pub fn status_PitFall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_PitFall_Main(self).into()
        }
    }

    pub fn status_end_PitFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_PitFall(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessPitFall_initStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessPitFall_initStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessPitFall_exitStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessPitFall_exitStatus(self).into()
        }
    }

    pub fn status_pre_DamageSongStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageSongStart(self).into()
        }
    }

    pub fn status_DamageSongStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSongStart(self).into()
        }
    }

    pub fn status_DamageSongStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSongStart_Main(self).into()
        }
    }

    pub fn status_end_DamageSongStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageSongStart(self).into()
        }
    }

    pub fn sub_DamageSongExit(&mut self) {
        unsafe {
            sub_DamageSongExit(self)
        }
    }

    pub fn status_pre_DamageSong(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageSong(self).into()
        }
    }

    pub fn status_DamageSong(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSong(self).into()
        }
    }

    pub fn status_DamageSong_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSong_Main(self).into()
        }
    }

    pub fn status_end_DamageSong(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageSong(self).into()
        }
    }

    pub fn status_pre_DamageSongFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageSongFall(self).into()
        }
    }

    pub fn status_DamageSongFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSongFall(self).into()
        }
    }

    pub fn status_DamageSongFall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSongFall_Main(self).into()
        }
    }

    pub fn status_end_DamageSongFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageSongFall(self).into()
        }
    }

    pub fn status_pre_DamageSongEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageSongEnd(self).into()
        }
    }

    pub fn status_DamageSongEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSongEnd(self).into()
        }
    }

    pub fn status_DamageSongEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSongEnd_Main(self).into()
        }
    }

    pub fn status_end_DamageSongEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageSongEnd(self).into()
        }
    }

    pub fn status_pre_DamageSleepStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageSleepStart(self).into()
        }
    }

    pub fn status_DamageSleepStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSleepStart(self).into()
        }
    }

    pub fn status_DamageSleepStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSleepStart_Main(self).into()
        }
    }

    pub fn status_end_DamageSleepStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageSleepStart(self).into()
        }
    }

    pub fn status_pre_DamageSleep(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageSleep(self).into()
        }
    }

    pub fn status_DamageSleep(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSleep(self).into()
        }
    }

    pub fn status_DamageSleep_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSleep_Main(self).into()
        }
    }

    pub fn status_end_DamageSleep(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageSleep(self).into()
        }
    }

    pub fn status_pre_DamageSleepFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageSleepFall(self).into()
        }
    }

    pub fn status_DamageSleepFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSleepFall(self).into()
        }
    }

    pub fn status_DamageSleepFall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSleepFall_Main(self).into()
        }
    }

    pub fn status_end_DamageSleepFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageSleepFall(self).into()
        }
    }

    pub fn status_pre_DamageSleepEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageSleepEnd(self).into()
        }
    }

    pub fn status_DamageSleepEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSleepEnd(self).into()
        }
    }

    pub fn status_DamageSleepEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageSleepEnd_Main(self).into()
        }
    }

    pub fn status_end_DamageSleepEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageSleepEnd(self).into()
        }
    }

    pub fn sub_damage_uniq_process_init_with_cpp(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_uniq_process_init_with_cpp(self).into()
        }
    }

    pub fn sub_damage_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_uniq_process_init(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_13(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_13(self).into()
        }
    }

    pub fn sub_PacmanFinal_start_eye(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_PacmanFinal_start_eye(self, arg1.into())
        }
    }

    pub fn virtual_ftStatusUniqProcessDamage_init(&mut self, arg1: lib::L2CValue) {
        unsafe {
            virtual_ftStatusUniqProcessDamage_init(self, arg1.into())
        }
    }

    pub fn ftStatusUniqProcessDamage_init(&mut self, arg1: lib::L2CValue) {
        unsafe {
            ftStatusUniqProcessDamage_init(self, arg1.into())
        }
    }

    pub fn sub_damage_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_uniq_process_main(self).into()
        }
    }

    pub fn virtual_ftStatusUniqProcessDamage_exec(&mut self) {
        unsafe {
            virtual_ftStatusUniqProcessDamage_exec(self)
        }
    }

    pub fn sub_damage_uniq_process_post_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_uniq_process_post_main(self).into()
        }
    }

    pub fn exec_damage_elec_hit_stop(&mut self) {
        unsafe {
            exec_damage_elec_hit_stop(self)
        }
    }

    pub fn FighterStatusUniqProcessDamage_check_hit_stop_delay_flick(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessDamage_check_hit_stop_delay_flick(self, arg1.into())
        }
    }

    pub fn sub_damage_uniq_process_mainStop(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_uniq_process_mainStop(self).into()
        }
    }

    pub fn ftStatusUniqProcessDamage_exec(&mut self) {
        unsafe {
            ftStatusUniqProcessDamage_exec(self)
        }
    }

    pub fn status_pre_DamageFlyReflectLR(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageFlyReflectLR(self).into()
        }
    }

    pub fn status_DamageFlyReflectLR(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFlyReflectLR(self).into()
        }
    }

    pub fn status_DamageFlyReflectLR_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFlyReflectLR_Main(self).into()
        }
    }

    pub fn sub_AirChkDamageDown(&mut self) -> lib::L2CValue {
        unsafe {
            sub_AirChkDamageDown(self).into()
        }
    }

    pub fn status_end_DamageFlyReflectLR(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageFlyReflectLR(self).into()
        }
    }

    pub fn end_damage_reflect_stop(&mut self) {
        unsafe {
            end_damage_reflect_stop(self)
        }
    }

    pub fn sub_DamageFlyReflect_effect(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_DamageFlyReflect_effect(self, arg1.into())
        }
    }

    pub fn sub_ftStatusUniqProcessDamageFly_initReflect(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessDamageFly_initReflect(self)
        }
    }

    pub fn start_damage_reflect_stop(&mut self) {
        unsafe {
            start_damage_reflect_stop(self)
        }
    }

    pub fn sub_damage_fly_reflect_lr_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_fly_reflect_lr_uniq_process_init(self).into()
        }
    }

    pub fn sub_damage_fly_refect_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_fly_refect_uniq_process_main(self).into()
        }
    }

    pub fn status_pre_DamageFlyReflectU(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageFlyReflectU(self).into()
        }
    }

    pub fn status_DamageFlyReflectU(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFlyReflectU(self).into()
        }
    }

    pub fn status_DamageFlyReflectU_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFlyReflectU_Main(self).into()
        }
    }

    pub fn status_end_DamageFlyReflectU(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageFlyReflectU(self).into()
        }
    }

    pub fn sub_damage_fly_reflect_u_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_fly_reflect_u_uniq_process_init(self).into()
        }
    }

    pub fn status_pre_DamageFlyReflectD(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageFlyReflectD(self).into()
        }
    }

    pub fn status_DamageFlyReflectD(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFlyReflectD(self).into()
        }
    }

    pub fn status_DamageFlyReflectD_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFlyReflectD_Main(self).into()
        }
    }

    pub fn status_end_DamageFlyReflectD(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageFlyReflectD(self).into()
        }
    }

    pub fn sub_damage_fly_reflect_d_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_fly_reflect_d_uniq_process_init(self).into()
        }
    }

    pub fn FighterStatusUniqProcessDamage_leave_stop(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessDamage_leave_stop(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessDamageFly_check_attack(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessDamageFly_check_attack(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn local_func__fighter_status_damage_17(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_17(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_18(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_18(self).into()
        }
    }

    pub fn local_func__fighter_status_damage_19(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_damage_19(self).into()
        }
    }

    pub fn sub_dead_up_star_scale(&mut self) {
        unsafe {
            sub_dead_up_star_scale(self)
        }
    }

    pub fn calc_camera_follow_pos(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            calc_camera_follow_pos(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn status_pre_Dead(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Dead(self).into()
        }
    }

    pub fn status_pre_Standby(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Standby(self).into()
        }
    }

    pub fn status_Dead(&mut self) -> lib::L2CValue {
        unsafe {
            status_Dead(self).into()
        }
    }

    pub fn status_DeadSub(&mut self) {
        unsafe {
            status_DeadSub(self)
        }
    }

    pub fn status_Dead_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Dead_Main(self).into()
        }
    }

    pub fn status_Standby(&mut self) -> lib::L2CValue {
        unsafe {
            status_Standby(self).into()
        }
    }

    pub fn status_Standby_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Standby_Main(self).into()
        }
    }

    pub fn status_end_Dead(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Dead(self).into()
        }
    }

    pub fn status_end_Standby(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Standby(self).into()
        }
    }

    pub fn sub_get_dead_quake_kind(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_get_dead_quake_kind(self, arg1.into()).into()
        }
    }

    pub fn sub_dead_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_dead_uniq_process_init(self).into()
        }
    }

    pub fn sub_dead_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_dead_uniq_process_exec(self).into()
        }
    }

    pub fn sub_dead_up_fall_process(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_dead_up_fall_process(self, arg1.into()).into()
        }
    }

    pub fn sub_dead_up_galaga_process(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_dead_up_galaga_process(self, arg1.into()).into()
        }
    }

    pub fn sub_to_end(&mut self) -> lib::L2CValue {
        unsafe {
            sub_to_end(self).into()
        }
    }

    pub fn sub_dead_uniq_process_fix_camera(&mut self) -> lib::L2CValue {
        unsafe {
            sub_dead_uniq_process_fix_camera(self).into()
        }
    }

    pub fn sub_dead_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_dead_uniq_process_exit(self).into()
        }
    }

    pub fn status_pre_Demo(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Demo(self).into()
        }
    }

    pub fn sub_demo_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_demo_uniq_process_init(self).into()
        }
    }

    pub fn status_Demo(&mut self) -> lib::L2CValue {
        unsafe {
            status_Demo(self).into()
        }
    }

    pub fn status_Demo_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Demo_Main(self).into()
        }
    }

    pub fn status_end_Demo(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Demo(self).into()
        }
    }

    pub fn status_pre_Down(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Down(self).into()
        }
    }

    pub fn status_Down(&mut self) -> lib::L2CValue {
        unsafe {
            status_Down(self).into()
        }
    }

    pub fn sub_down_common_pre(&mut self) {
        unsafe {
            sub_down_common_pre(self)
        }
    }

    pub fn status_Down_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Down_Main(self).into()
        }
    }

    pub fn sub_down_common(&mut self) -> lib::L2CValue {
        unsafe {
            sub_down_common(self).into()
        }
    }

    pub fn sub_down_chk_reflect_wall(&mut self) {
        unsafe {
            sub_down_chk_reflect_wall(self)
        }
    }

    pub fn sub_down_chk_reflect_wall_init(&mut self) {
        unsafe {
            sub_down_chk_reflect_wall_init(self)
        }
    }

    pub fn status_end_Down(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Down(self).into()
        }
    }

    pub fn status_pre_DownSpot(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DownSpot(self).into()
        }
    }

    pub fn status_DownSpot(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownSpot(self).into()
        }
    }

    pub fn status_DownSpot_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownSpot_Main(self).into()
        }
    }

    pub fn status_end_DownSpot(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DownSpot(self).into()
        }
    }

    pub fn sub_down_spot_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_down_spot_uniq_process_init(self).into()
        }
    }

    pub fn status_pre_DownContinue(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DownContinue(self).into()
        }
    }

    pub fn status_DownContinue(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownContinue(self).into()
        }
    }

    pub fn status_DownContinue_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownContinue_Main(self).into()
        }
    }

    pub fn status_end_DownContinue(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DownContinue(self).into()
        }
    }

    pub fn status_pre_DownWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DownWait(self).into()
        }
    }

    pub fn status_DownWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownWait(self).into()
        }
    }

    pub fn sub_down_wait_common_pre(&mut self) {
        unsafe {
            sub_down_wait_common_pre(self)
        }
    }

    pub fn sub_down_wait_common(&mut self) -> lib::L2CValue {
        unsafe {
            sub_down_wait_common(self).into()
        }
    }

    pub fn sub_down_wait_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_down_wait_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_DownWait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownWait_Main(self).into()
        }
    }

    pub fn status_end_DownWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DownWait(self).into()
        }
    }

    pub fn status_pre_DownWaitContinue(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DownWaitContinue(self).into()
        }
    }

    pub fn status_DownWaitContinue(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownWaitContinue(self).into()
        }
    }

    pub fn status_DownWaitContinue_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownWaitContinue_Main(self).into()
        }
    }

    pub fn status_end_DownWaitContinue(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DownWaitContinue(self).into()
        }
    }

    pub fn status_pre_DownStand(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DownStand(self).into()
        }
    }

    pub fn status_DownStand(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownStand(self).into()
        }
    }

    pub fn status_DownStand_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownStand_Main(self).into()
        }
    }

    pub fn status_end_DownStand(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DownStand(self).into()
        }
    }

    pub fn status_pre_DownStandFb(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DownStandFb(self).into()
        }
    }

    pub fn status_DownStandFb(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownStandFb(self).into()
        }
    }

    pub fn status_DownStandFb_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownStandFb_Main(self).into()
        }
    }

    pub fn status_end_DownStandFb(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DownStandFb(self).into()
        }
    }

    pub fn status_pre_DownStandAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DownStandAttack(self).into()
        }
    }

    pub fn status_DownStandAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownStandAttack(self).into()
        }
    }

    pub fn status_DownStandAttack_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownStandAttack_Main(self).into()
        }
    }

    pub fn status_end_DownStandAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DownStandAttack(self).into()
        }
    }

    pub fn status_pre_DownDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DownDamage(self).into()
        }
    }

    pub fn status_DownDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownDamage(self).into()
        }
    }

    pub fn status_DownDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownDamage_Main(self).into()
        }
    }

    pub fn sub_down_damage_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_down_damage_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_end_DownDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DownDamage(self).into()
        }
    }

    pub fn sub_down_damage_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_down_damage_uniq_process_init(self).into()
        }
    }

    pub fn sub_down_damage_uniq_process_main(&mut self) {
        unsafe {
            sub_down_damage_uniq_process_main(self)
        }
    }

    pub fn sub_down_damage_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_down_damage_uniq_process_exit(self).into()
        }
    }

    pub fn status_pre_DownReflectLR(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DownReflectLR(self).into()
        }
    }

    pub fn status_DownReflectLR(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownReflectLR(self).into()
        }
    }

    pub fn status_DownReflectLR_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownReflectLR_Main(self).into()
        }
    }

    pub fn status_end_DownReflectLR(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DownReflectLR(self).into()
        }
    }

    pub fn sub_down_reflect_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_down_reflect_uniq_process_init(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessDownDamage_initNormalDamageCommon(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessDownDamage_initNormalDamageCommon(self)
        }
    }

    pub fn sub_down_reflect_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_down_reflect_uniq_process_main(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessDownDamage_execNormalDamageCommon(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessDownDamage_execNormalDamageCommon(self)
        }
    }

    pub fn sub_down_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_down_uniq_process_init(self).into()
        }
    }

    pub fn status_pre_LayDown(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_LayDown(self).into()
        }
    }

    pub fn status_LayDown(&mut self) -> lib::L2CValue {
        unsafe {
            status_LayDown(self).into()
        }
    }

    pub fn status_LayDown_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_LayDown_Main(self).into()
        }
    }

    pub fn status_end_LayDown(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_LayDown(self).into()
        }
    }

    pub fn status_pre_DownEat(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DownEat(self).into()
        }
    }

    pub fn status_DownEat(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownEat(self).into()
        }
    }

    pub fn status_DownEat_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DownEat_Main(self).into()
        }
    }

    pub fn status_end_DownEat(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DownEat(self).into()
        }
    }

    pub fn status_pre_Entry(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Entry(self).into()
        }
    }

    pub fn status_Entry(&mut self) -> lib::L2CValue {
        unsafe {
            status_Entry(self).into()
        }
    }

    pub fn status_Entry_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Entry_Main(self).into()
        }
    }

    pub fn sub_entry_remove_article(&mut self) {
        unsafe {
            sub_entry_remove_article(self)
        }
    }

    pub fn sub_wait_common(&mut self) {
        unsafe {
            sub_wait_common(self)
        }
    }

    pub fn sub_wait_motion_mtrans(&mut self) {
        unsafe {
            sub_wait_motion_mtrans(self)
        }
    }

    pub fn sub_entry_uniq_process_exec_fix_pos(&mut self) {
        unsafe {
            sub_entry_uniq_process_exec_fix_pos(self)
        }
    }

    pub fn sub_is_through_wait_motion_mtrans(&mut self) -> lib::L2CValue {
        unsafe {
            sub_is_through_wait_motion_mtrans(self).into()
        }
    }

    pub fn sub_wait_item_motion_mtrans(&mut self) -> lib::L2CValue {
        unsafe {
            sub_wait_item_motion_mtrans(self).into()
        }
    }

    pub fn status_end_Entry(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Entry(self).into()
        }
    }

    pub fn sub_entry_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_entry_uniq_process_init(self).into()
        }
    }

    pub fn sub_entry_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_entry_uniq_process_exit(self).into()
        }
    }

    pub fn status_pre_Escape(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Escape(self).into()
        }
    }

    pub fn status_pre_EscapeF(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_EscapeF(self).into()
        }
    }

    pub fn sub_pre_escape_fb(&mut self) -> lib::L2CValue {
        unsafe {
            sub_pre_escape_fb(self).into()
        }
    }

    pub fn status_pre_EscapeB(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_EscapeB(self).into()
        }
    }

    pub fn status_pre_EscapeAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_EscapeAir(self).into()
        }
    }

    pub fn status_Escape_common(&mut self) {
        unsafe {
            status_Escape_common(self)
        }
    }

    pub fn sub_escape_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_escape_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_Escape(&mut self) -> lib::L2CValue {
        unsafe {
            status_Escape(self).into()
        }
    }

    pub fn status_Escape_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Escape_Main(self).into()
        }
    }

    pub fn sub_escape_check_rumble(&mut self) {
        unsafe {
            sub_escape_check_rumble(self)
        }
    }

    pub fn status_EscapeF_Sub(&mut self) {
        unsafe {
            status_EscapeF_Sub(self)
        }
    }

    pub fn sub_escape_fb_common(&mut self) {
        unsafe {
            sub_escape_fb_common(self)
        }
    }

    pub fn status_EscapeF(&mut self) -> lib::L2CValue {
        unsafe {
            status_EscapeF(self).into()
        }
    }

    pub fn sub_escape_fb_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_escape_fb_main(self).into()
        }
    }

    pub fn status_EscapeF_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_EscapeF_Main(self).into()
        }
    }

    pub fn status_EscapeB(&mut self) -> lib::L2CValue {
        unsafe {
            status_EscapeB(self).into()
        }
    }

    pub fn status_EscapeB_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_EscapeB_Main(self).into()
        }
    }

    pub fn status_EscapeAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_EscapeAir(self).into()
        }
    }

    pub fn sub_escape_air_common(&mut self) {
        unsafe {
            sub_escape_air_common(self)
        }
    }

    pub fn sub_escape_air_common_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_escape_air_common_main(self).into()
        }
    }

    pub fn sub_escape_air_common_strans_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_escape_air_common_strans_main(self).into()
        }
    }

    pub fn sub_escape_air_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_escape_air_uniq(self, arg1.into()).into()
        }
    }

    pub fn exec_escape_air_slide(&mut self) {
        unsafe {
            exec_escape_air_slide(self)
        }
    }

    pub fn status_EscapeAir_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_EscapeAir_Main(self).into()
        }
    }

    pub fn status_end_Escape(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Escape(self).into()
        }
    }

    pub fn status_end_EscapeF(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_EscapeF(self).into()
        }
    }

    pub fn sub_status_end_EscaleFB(&mut self) {
        unsafe {
            sub_status_end_EscaleFB(self)
        }
    }

    pub fn status_end_EscapeB(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_EscapeB(self).into()
        }
    }

    pub fn status_end_EscapeAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_EscapeAir(self).into()
        }
    }

    pub fn sub_escape_uniq_process_common_initStatus(&mut self) {
        unsafe {
            sub_escape_uniq_process_common_initStatus(self)
        }
    }

    pub fn sub_escape_uniq_process_common_initStatus_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            sub_escape_uniq_process_common_initStatus_common(self, arg1.into(), arg2.into())
        }
    }

    pub fn setup_escape_air_slide(&mut self) {
        unsafe {
            setup_escape_air_slide(self)
        }
    }

    pub fn setup_escape_air_slide_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            setup_escape_air_slide_common(self, arg1.into(), arg2.into())
        }
    }

    pub fn sub_escape_uniq_process_initStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_escape_uniq_process_initStatus(self).into()
        }
    }

    pub fn sub_escape_air_inherit_jump_aerial_motion_uniq_process_initStatus(&mut self) {
        unsafe {
            sub_escape_air_inherit_jump_aerial_motion_uniq_process_initStatus(self)
        }
    }

    pub fn sub_escape_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos(&mut self) {
        unsafe {
            sub_escape_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos(self)
        }
    }

    pub fn FighterStatusUniqProcessEscapeAir_calc_param(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessEscapeAir_calc_param(self).into()
        }
    }

    pub fn status_pre_fall_special(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_fall_special(self).into()
        }
    }

    pub fn sub_pre_fall(&mut self) -> lib::L2CValue {
        unsafe {
            sub_pre_fall(self).into()
        }
    }

    pub fn sub_pre_fall_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_pre_fall_param(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn status_pre_landing_fall_special(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_landing_fall_special(self).into()
        }
    }

    pub fn status_pre_landing_fall_special_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_landing_fall_special_common(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn status_fall_special(&mut self) -> lib::L2CValue {
        unsafe {
            status_fall_special(self).into()
        }
    }

    pub fn status_fall_special_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_fall_special_main(self).into()
        }
    }

    pub fn sub_fall(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fall(self).into()
        }
    }

    pub fn status_landing_fall_special(&mut self) -> lib::L2CValue {
        unsafe {
            status_landing_fall_special(self).into()
        }
    }

    pub fn status_landing_fall_special_sub(&mut self) {
        unsafe {
            status_landing_fall_special_sub(self)
        }
    }

    pub fn sub_status_landing_fall_special_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_status_landing_fall_special_main(self).into()
        }
    }

    pub fn sub_status_landing_fall_special_main_cmn(&mut self) -> lib::L2CValue {
        unsafe {
            sub_status_landing_fall_special_main_cmn(self).into()
        }
    }

    pub fn status_landing_fall_special_air_fall_special(&mut self) -> lib::L2CValue {
        unsafe {
            status_landing_fall_special_air_fall_special(self).into()
        }
    }

    pub fn status_landing_fall_special_air_fall_special_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_landing_fall_special_air_fall_special_main(self).into()
        }
    }

    pub fn status_landing_fall_special_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_landing_fall_special_main(self).into()
        }
    }

    pub fn status_end_fall_special(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_fall_special(self).into()
        }
    }

    pub fn status_end_landing_fall_special(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_landing_fall_special(self).into()
        }
    }

    pub fn status_pre_FistDown(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FistDown(self).into()
        }
    }

    pub fn status_FistDown(&mut self) -> lib::L2CValue {
        unsafe {
            status_FistDown(self).into()
        }
    }

    pub fn sub_status_SavingDamage_Main_common(&mut self) -> lib::L2CValue {
        unsafe {
            sub_status_SavingDamage_Main_common(self).into()
        }
    }

    pub fn status_end_FistDown(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FistDown(self).into()
        }
    }

    pub fn sub_fist_down_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fist_down_uniq_process_init(self).into()
        }
    }

    pub fn sub_saving_damage_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_saving_damage_uniq_process_init(self).into()
        }
    }

    pub fn sub_fist_down_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fist_down_uniq_process_main(self).into()
        }
    }

    pub fn sub_saving_damage_uniq_process_main_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue, arg8: lib::L2CValue) {
        unsafe {
            sub_saving_damage_uniq_process_main_common(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into(), arg8.into())
        }
    }

    pub fn status_pre_FistDown2(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FistDown2(self).into()
        }
    }

    pub fn status_FistDown2(&mut self) -> lib::L2CValue {
        unsafe {
            status_FistDown2(self).into()
        }
    }

    pub fn sub_status_SavingDamage_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_status_SavingDamage_common(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn status_end_FistDown2(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FistDown2(self).into()
        }
    }

    pub fn sub_fist_down2_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fist_down2_uniq_process_init(self).into()
        }
    }

    pub fn sub_fist_down2_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fist_down2_uniq_process_main(self).into()
        }
    }

    pub fn status_pre_FistDown3(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FistDown3(self).into()
        }
    }

    pub fn status_FistDown3(&mut self) -> lib::L2CValue {
        unsafe {
            status_FistDown3(self).into()
        }
    }

    pub fn status_end_FistDown3(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FistDown3(self).into()
        }
    }

    pub fn sub_fist_down3_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fist_down3_uniq_process_init(self).into()
        }
    }

    pub fn sub_fist_down3_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fist_down3_uniq_process_main(self).into()
        }
    }

    pub fn status_pre_FreeMove(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FreeMove(self).into()
        }
    }

    pub fn status_FreeMove(&mut self) -> lib::L2CValue {
        unsafe {
            status_FreeMove(self).into()
        }
    }

    pub fn status_FreeMove_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FreeMove_Main(self).into()
        }
    }

    pub fn uniq_process_Gimmick_init_status(&mut self) {
        unsafe {
            uniq_process_Gimmick_init_status(self)
        }
    }

    pub fn uniq_process_Gimmick_exit_status(&mut self, arg1: lib::L2CValue) {
        unsafe {
            uniq_process_Gimmick_exit_status(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmick_notify_event_gimmick(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmick_notify_event_gimmick(self, arg1.into()).into()
        }
    }

    pub fn status_pre_GimmickBarrel(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickBarrel(self).into()
        }
    }

    pub fn status_GimmickBarrel(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickBarrel(self).into()
        }
    }

    pub fn status_GimmickBarrel_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickBarrel_Main(self).into()
        }
    }

    pub fn uniq_process_GimmickBarrel_fix_pos(&mut self) {
        unsafe {
            uniq_process_GimmickBarrel_fix_pos(self)
        }
    }

    pub fn status_end_GimmickBarrel(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickBarrel(self).into()
        }
    }

    pub fn uniq_process_GimmickBarrel_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickBarrel_init_status(self).into()
        }
    }

    pub fn uniq_process_GimmickBarrel_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickBarrel_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickBarrel_exec_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickBarrel_exec_status(self)
        }
    }

    pub fn uniq_process_GimmickBarrel_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickBarrel_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickBarrel_exit_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickBarrel_exit_status(self)
        }
    }

    pub fn FighterStatusUniqProcessGimmickBarrel_notify_event_gimmick(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickBarrel_notify_event_gimmick(self, arg1.into()).into()
        }
    }

    pub fn status_pre_GimmickDoor(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickDoor(self).into()
        }
    }

    pub fn status_GimmickDoor(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickDoor(self).into()
        }
    }

    pub fn status_GimmickDoor_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickDoor_Main(self).into()
        }
    }

    pub fn status_end_GimmickDoor(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickDoor(self).into()
        }
    }

    pub fn uniq_process_GimmickDoor_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickDoor_init_status(self).into()
        }
    }

    pub fn uniq_process_GimmickDoor_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickDoor_exec_status(self).into()
        }
    }

    pub fn uniq_process_GimmickDoor_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickDoor_exit_status(self).into()
        }
    }

    pub fn status_pre_GimmickSpring(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickSpring(self).into()
        }
    }

    pub fn status_GimmickSpring(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickSpring(self).into()
        }
    }

    pub fn status_GimmickSpring_Common(&mut self) {
        unsafe {
            status_GimmickSpring_Common(self)
        }
    }

    pub fn status_GimmickSpring_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickSpring_Main(self).into()
        }
    }

    pub fn status_end_GimmickSpring(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickSpring(self).into()
        }
    }

    pub fn uniq_process_GimmickSpring_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickSpring_init_status(self).into()
        }
    }

    pub fn uniq_process_GimmickSpring_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickSpring_exec_status(self).into()
        }
    }

    pub fn uniq_process_GimmickSpring_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickSpring_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickSpring_set_pos_and_safe_pos(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickSpring_set_pos_and_safe_pos(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickSpring_notify_event_gimmick(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickSpring_notify_event_gimmick(self, arg1.into()).into()
        }
    }

    pub fn status_pre_GimmickSpringJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickSpringJump(self).into()
        }
    }

    pub fn status_GimmickSpringJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickSpringJump(self).into()
        }
    }

    pub fn status_Jump(&mut self) -> lib::L2CValue {
        unsafe {
            status_Jump(self).into()
        }
    }

    pub fn sub_jump_item_rocketbelt(&mut self) {
        unsafe {
            sub_jump_item_rocketbelt(self)
        }
    }

    pub fn status_Jump_sub(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_Jump_sub(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn status_Jump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Jump_Main(self).into()
        }
    }

    pub fn status_end_GimmickSpringJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickSpringJump(self).into()
        }
    }

    pub fn status_pre_GimmickSpringBack(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickSpringBack(self).into()
        }
    }

    pub fn status_GimmickSpringBack(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickSpringBack(self).into()
        }
    }

    pub fn status_GimmickSpringBack_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickSpringBack_Main(self).into()
        }
    }

    pub fn status_end_GimmickSpringBack(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickSpringBack(self).into()
        }
    }

    pub fn status_pre_GimmickEaten(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickEaten(self).into()
        }
    }

    pub fn status_GimmickEaten(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickEaten(self).into()
        }
    }

    pub fn status_GimmickEaten_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickEaten_Main(self).into()
        }
    }

    pub fn status_end_GimmickEaten(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickEaten(self).into()
        }
    }

    pub fn uniq_process_GimmickEaten_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickEaten_init_status(self).into()
        }
    }

    pub fn uniq_process_GimmickEaten_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickEaten_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickEaten_notify_event_gimmick(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickEaten_notify_event_gimmick(self, arg1.into()).into()
        }
    }

    pub fn status_pre_GimmickPipe(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickPipe(self).into()
        }
    }

    pub fn status_GimmickPipe(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickPipe(self).into()
        }
    }

    pub fn status_GimmickPipe_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickPipe_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickPipe_is_moved(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickPipe_is_moved(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickPipe_exec_fix_pos(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickPipe_exec_fix_pos(self).into()
        }
    }

    pub fn status_end_GimmickPipe(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickPipe(self).into()
        }
    }

    pub fn uniq_process_GimmickPipe_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickPipe_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickPipe_init_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickPipe_init_status(self)
        }
    }

    pub fn uniq_process_GimmickPipe_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickPipe_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickPipe_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickPipe_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickPipe_notify_event_gimmick(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickPipe_notify_event_gimmick(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickPipe_move(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue, arg8: lib::L2CValue, arg9: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickPipe_move(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into(), arg8.into(), arg9.into())
        }
    }

    pub fn status_pre_GimmickTornado(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickTornado(self).into()
        }
    }

    pub fn status_GimmickTornado(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickTornado(self).into()
        }
    }

    pub fn status_GimmickTornado_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickTornado_Main(self).into()
        }
    }

    pub fn sub_GimmickTornado_damageFlyRollEnd(&mut self) {
        unsafe {
            sub_GimmickTornado_damageFlyRollEnd(self)
        }
    }

    pub fn status_end_GimmickTornado(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickTornado(self).into()
        }
    }

    pub fn uniq_process_GimmickTornado_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickTornado_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickTornado_init_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickTornado_init_status(self)
        }
    }

    pub fn uniq_process_GimmickTornado_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickTornado_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickTornado_exit_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickTornado_exit_status(self)
        }
    }

    pub fn FighterStatusUniqProcessGimmickTornado_shoot(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickTornado_shoot(self)
        }
    }

    pub fn FighterStatusUniqProcessGimmickTornado_move(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickTornado_move(self)
        }
    }

    pub fn FighterStatusUniqProcessGimmickTornado_move_for_hyrule_tornado(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickTornado_move_for_hyrule_tornado(self)
        }
    }

    pub fn FighterStatusUniqProcessGimmickTornado_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickTornado_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickTornado_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickTornado_exec_stop(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickTornado_notify_event_gimmick(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickTornado_notify_event_gimmick(self, arg1.into()).into()
        }
    }

    pub fn status_pre_GimmickFishCapture(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickFishCapture(self).into()
        }
    }

    pub fn status_GimmickFishCapture(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickFishCapture(self).into()
        }
    }

    pub fn status_GimmickFishCapture_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickFishCapture_Main(self).into()
        }
    }

    pub fn status_end_GimmickFishCapture(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickFishCapture(self).into()
        }
    }

    pub fn uniq_process_GimmickFishCapture_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickFishCapture_init_status(self).into()
        }
    }

    pub fn uniq_process_GimmickFishCapture_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickFishCapture_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickFishCapture_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickFishCapture_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickFishCapture_update_posture(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickFishCapture_update_posture(self)
        }
    }

    pub fn FighterStatusUniqProcessGimmickFishCapture_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickFishCapture_exec_stop(self).into()
        }
    }

    pub fn status_pre_GimmickDrum(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickDrum(self).into()
        }
    }

    pub fn status_GimmickDrum(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickDrum(self).into()
        }
    }

    pub fn status_GimmickDrum_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickDrum_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickDrum_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickDrum_exec_fix_pos(self)
        }
    }

    pub fn status_end_GimmickDrum(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickDrum(self).into()
        }
    }

    pub fn uniq_process_GimmickDrum_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickDrum_init_status(self).into()
        }
    }

    pub fn uniq_process_GimmickDrum_exit_status(&mut self) {
        unsafe {
            uniq_process_GimmickDrum_exit_status(self)
        }
    }

    pub fn FighterStatusUniqProcessGimmickDrum_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickDrum_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickDrum_notify_event_gimmick(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickDrum_notify_event_gimmick(self, arg1.into()).into()
        }
    }

    pub fn status_pre_GimmickOdinSlashed(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickOdinSlashed(self).into()
        }
    }

    pub fn status_GimmickOdinSlashed(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickOdinSlashed(self).into()
        }
    }

    pub fn status_GimmickOdinSlashed_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickOdinSlashed_Main(self).into()
        }
    }

    pub fn status_end_GimmickOdinSlashed(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickOdinSlashed(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickOdinSlashed_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickOdinSlashed_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickOdinSlashed_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickOdinSlashed_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickOdinSlashed_notify_event_gimmick(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickOdinSlashed_notify_event_gimmick(self, arg1.into()).into()
        }
    }

    pub fn status_pre_DollyStageDead(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DollyStageDead(self).into()
        }
    }

    pub fn status_DollyStageDead(&mut self) -> lib::L2CValue {
        unsafe {
            status_DollyStageDead(self).into()
        }
    }

    pub fn status_DollyStageDead_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DollyStageDead_Main(self).into()
        }
    }

    pub fn exec_stop_DollyStageDead(&mut self) -> lib::L2CValue {
        unsafe {
            exec_stop_DollyStageDead(self).into()
        }
    }

    pub fn status_end_DollyStageDead(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DollyStageDead(self).into()
        }
    }

    pub fn status_pre_GimmickJumpBoard(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickJumpBoard(self).into()
        }
    }

    pub fn status_GimmickJumpBoard(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickJumpBoard(self).into()
        }
    }

    pub fn status_GimmickJumpBoard_Common(&mut self) {
        unsafe {
            status_GimmickJumpBoard_Common(self)
        }
    }

    pub fn status_GimmickJumpBoard_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickJumpBoard_Main(self).into()
        }
    }

    pub fn status_end_GimmickJumpBoard(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickJumpBoard(self).into()
        }
    }

    pub fn uniq_process_GimmickJumpBoard_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickJumpBoard_init_status(self).into()
        }
    }

    pub fn uniq_process_GimmickJumpBoard_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickJumpBoard_exec_status(self).into()
        }
    }

    pub fn uniq_process_GimmickJumpBoard_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickJumpBoard_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickJumpBoard_set_pos_and_safe_pos(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickJumpBoard_set_pos_and_safe_pos(self, arg1.into())
        }
    }

    pub fn status_pre_GimmickJumpBoardJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GimmickJumpBoardJump(self).into()
        }
    }

    pub fn status_GimmickJumpBoardJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickJumpBoardJump(self).into()
        }
    }

    pub fn status_JumpBoardJump_sub(&mut self) {
        unsafe {
            status_JumpBoardJump_sub(self)
        }
    }

    pub fn status_GimmickJumpBoardJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GimmickJumpBoardJump_Main(self).into()
        }
    }

    pub fn uniq_process_GimmickJumpBoardJump_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_GimmickJumpBoardJump_exec_status(self).into()
        }
    }

    pub fn status_end_GimmickJumpBoardJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GimmickJumpBoardJump(self).into()
        }
    }

    pub fn status_pre_DamageFlyReflectJumpBoard(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DamageFlyReflectJumpBoard(self).into()
        }
    }

    pub fn status_DamageFlyReflectJumpBoard(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFlyReflectJumpBoard(self).into()
        }
    }

    pub fn status_DamageFlyReflectJumpBoard_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DamageFlyReflectJumpBoard_Main(self).into()
        }
    }

    pub fn status_end_DamageFlyReflectJumpBoard(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DamageFlyReflectJumpBoard(self).into()
        }
    }

    pub fn sub_damage_fly_reflect_jump_board_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_damage_fly_reflect_jump_board_uniq_process_init(self).into()
        }
    }

    pub fn FighterStatusGuard__set_shield_scale(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusGuard__set_shield_scale(self, arg1.into()).into()
        }
    }

    pub fn status_pre_GuardOn(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GuardOn(self).into()
        }
    }

    pub fn sub_guard_cont_pre(&mut self) {
        unsafe {
            sub_guard_cont_pre(self)
        }
    }

    pub fn sub_status_guard_on_common(&mut self) {
        unsafe {
            sub_status_guard_on_common(self)
        }
    }

    pub fn sub_ftStatusUniqProcessGuardFunc_updateShield(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessGuardFunc_updateShield(self, arg1.into())
        }
    }

    pub fn sub_guard_on_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_guard_on_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_GuardOn(&mut self) -> lib::L2CValue {
        unsafe {
            status_GuardOn(self).into()
        }
    }

    pub fn status_GuardOn_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GuardOn_Main(self).into()
        }
    }

    pub fn sub_status_guard_on_main_air_common(&mut self) -> lib::L2CValue {
        unsafe {
            sub_status_guard_on_main_air_common(self).into()
        }
    }

    pub fn sub_guard_cont(&mut self) -> lib::L2CValue {
        unsafe {
            sub_guard_cont(self).into()
        }
    }

    pub fn status_guard_main_common(&mut self) -> lib::L2CValue {
        unsafe {
            status_guard_main_common(self).into()
        }
    }

    pub fn check_guard_hold(&mut self) -> lib::L2CValue {
        unsafe {
            check_guard_hold(self).into()
        }
    }

    pub fn check_guard_attack_special_hi(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            check_guard_attack_special_hi(self, arg1.into()).into()
        }
    }

    pub fn sub_status_end_guard_on_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_status_end_guard_on_common(self, arg1.into())
        }
    }

    pub fn status_end_GuardOn(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GuardOn(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardOn_initStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuardOn_initStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardOn_initStatus_common(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessGuardOn_initStatus_common(self)
        }
    }

    pub fn sub_ftStatusUniqProcessGuardOn_execStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuardOn_execStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardOn_execStatus_common(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessGuardOn_execStatus_common(self)
        }
    }

    pub fn sub_ftStatusUniqProcessGuardOn_exitStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuardOn_exitStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardOn_exitStatus_common(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessGuardOn_exitStatus_common(self)
        }
    }

    pub fn sub_ftStatusUniqProcessGuardOn_execStop(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuardOn_execStop(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardOn_execStop_Inner(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessGuardOn_execStop_Inner(self, arg1.into())
        }
    }

    pub fn status_pre_Guard(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Guard(self).into()
        }
    }

    pub fn sub_status_guard_common(&mut self) {
        unsafe {
            sub_status_guard_common(self)
        }
    }

    pub fn sub_guard_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_guard_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_Guard(&mut self) -> lib::L2CValue {
        unsafe {
            status_Guard(self).into()
        }
    }

    pub fn status_Guard_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Guard_Main(self).into()
        }
    }

    pub fn status_guard_main_common_air(&mut self) -> lib::L2CValue {
        unsafe {
            status_guard_main_common_air(self).into()
        }
    }

    pub fn status_end_Guard(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Guard(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuard_initStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuard_initStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuard_initStatus_common(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessGuard_initStatus_common(self)
        }
    }

    pub fn sub_ftStatusUniqProcessGuard_execStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuard_execStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuard_execStatus_common(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessGuard_execStatus_common(self)
        }
    }

    pub fn sub_ftStatusUniqProcessGuard_execStop(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuard_execStop(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuard_execStop_common(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessGuard_execStop_common(self)
        }
    }

    pub fn sub_ftStatusUniqProcessGuard_exitStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuard_exitStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuard_exitStatus_common(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessGuard_exitStatus_common(self)
        }
    }

    pub fn status_pre_GuardOff(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GuardOff(self).into()
        }
    }

    pub fn status_GuardOff_Common(&mut self) -> lib::L2CValue {
        unsafe {
            status_GuardOff_Common(self).into()
        }
    }

    pub fn sub_guard_off_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_guard_off_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_GuardOff(&mut self) -> lib::L2CValue {
        unsafe {
            status_GuardOff(self).into()
        }
    }

    pub fn status_GuardOff_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GuardOff_Main(self).into()
        }
    }

    pub fn status_GuardOff_Main_common(&mut self) -> lib::L2CValue {
        unsafe {
            status_GuardOff_Main_common(self).into()
        }
    }

    pub fn sub_status_guard_off_main_common_cancel(&mut self) -> lib::L2CValue {
        unsafe {
            sub_status_guard_off_main_common_cancel(self).into()
        }
    }

    pub fn sub_status_guard_off_main_common_air(&mut self) -> lib::L2CValue {
        unsafe {
            sub_status_guard_off_main_common_air(self).into()
        }
    }

    pub fn sub_status_guard_off_main_common_control(&mut self) -> lib::L2CValue {
        unsafe {
            sub_status_guard_off_main_common_control(self).into()
        }
    }

    pub fn status_end_GuardOff(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GuardOff(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardOff_initStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuardOff_initStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardOff_execStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuardOff_execStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardOff_execStop(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuardOff_execStop(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardOff_exitStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuardOff_exitStatus(self).into()
        }
    }

    pub fn status_pre_GuardDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GuardDamage(self).into()
        }
    }

    pub fn status_GuardDamage_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_GuardDamage_common(self, arg1.into())
        }
    }

    pub fn sub_GuardDamageUniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_GuardDamageUniq(self, arg1.into()).into()
        }
    }

    pub fn status_GuardDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_GuardDamage(self).into()
        }
    }

    pub fn status_GuardDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GuardDamage_Main(self).into()
        }
    }

    pub fn status_guard_damage_main_common_air(&mut self) -> lib::L2CValue {
        unsafe {
            status_guard_damage_main_common_air(self).into()
        }
    }

    pub fn status_guard_damage_main_common(&mut self) -> lib::L2CValue {
        unsafe {
            status_guard_damage_main_common(self).into()
        }
    }

    pub fn status_end_GuardDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GuardDamage(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardDamage_initStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuardDamage_initStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardDamage_initStatus_Inner(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessGuardDamage_initStatus_Inner(self)
        }
    }

    pub fn sub_ftStatusUniqProcessGuardDamage_execStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuardDamage_execStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardDamage_execStatus_common(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessGuardDamage_execStatus_common(self)
        }
    }

    pub fn sub_ftStatusUniqProcessGuardDamage_execStop(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuardDamage_execStop(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardDamage_execStop_common(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessGuardDamage_execStop_common(self)
        }
    }

    pub fn sub_ftStatusUniqProcessGuardDamage_exitStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessGuardDamage_exitStatus(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessGuardDamage_exitStatus_common(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessGuardDamage_exitStatus_common(self)
        }
    }

    pub fn FighterStatusUniqProcessGuardDamage_leave_stop(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGuardDamage_leave_stop(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn status_pre_ShieldBreakFly(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ShieldBreakFly(self).into()
        }
    }

    pub fn sub_status_shield_break_fly_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_status_shield_break_fly_common(self, arg1.into())
        }
    }

    pub fn status_ShieldBreakFly(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShieldBreakFly(self).into()
        }
    }

    pub fn status_ShieldBreakFly_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShieldBreakFly_Main(self).into()
        }
    }

    pub fn status_end_ShieldBreakFly(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ShieldBreakFly(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessShieldBreakFly_initStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessShieldBreakFly_initStatus(self).into()
        }
    }

    pub fn FighterStatusUniqProcessShieldBreak_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessShieldBreak_exec_status(self).into()
        }
    }

    pub fn status_pre_ShieldBreakFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ShieldBreakFall(self).into()
        }
    }

    pub fn status_ShieldBreakFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShieldBreakFall(self).into()
        }
    }

    pub fn status_ShieldBreakFall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShieldBreakFall_Main(self).into()
        }
    }

    pub fn status_end_ShieldBreakFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ShieldBreakFall(self).into()
        }
    }

    pub fn status_pre_ShieldBreakDown(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ShieldBreakDown(self).into()
        }
    }

    pub fn status_ShieldBreakDown(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShieldBreakDown(self).into()
        }
    }

    pub fn status_ShieldBreakDown_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShieldBreakDown_Main(self).into()
        }
    }

    pub fn status_end_ShieldBreakDown(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ShieldBreakDown(self).into()
        }
    }

    pub fn status_pre_FuraFuraStand(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FuraFuraStand(self).into()
        }
    }

    pub fn status_FuraFuraStand(&mut self) -> lib::L2CValue {
        unsafe {
            status_FuraFuraStand(self).into()
        }
    }

    pub fn sub_furafura_stand_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_furafura_stand_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_FuraFuraStand_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FuraFuraStand_Main(self).into()
        }
    }

    pub fn status_end_FuraFuraStand(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FuraFuraStand(self).into()
        }
    }

    pub fn status_pre_FuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FuraFura(self).into()
        }
    }

    pub fn status_FuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_FuraFura(self).into()
        }
    }

    pub fn status_FuraFura_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FuraFura_Main(self).into()
        }
    }

    pub fn status_end_FuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FuraFura(self).into()
        }
    }

    pub fn status_pre_FuraFuraEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FuraFuraEnd(self).into()
        }
    }

    pub fn status_FuraFuraEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_FuraFuraEnd(self).into()
        }
    }

    pub fn status_FuraFuraEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FuraFuraEnd_Main(self).into()
        }
    }

    pub fn status_end_FuraFuraEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FuraFuraEnd(self).into()
        }
    }

    pub fn status_pre_ItemLightPickup(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemLightPickup(self).into()
        }
    }

    pub fn status_ItemLightPickup(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemLightPickup(self).into()
        }
    }

    pub fn status_ItemLightPickup_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemLightPickup_Main(self).into()
        }
    }

    pub fn status_end_ItemLightPickup(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemLightPickup(self).into()
        }
    }

    pub fn sub_pre_ItemThrow(&mut self) -> lib::L2CValue {
        unsafe {
            sub_pre_ItemThrow(self).into()
        }
    }

    pub fn status_pre_ItemThrow(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemThrow(self).into()
        }
    }

    pub fn ItemThrowLightMotionDecision(&mut self) {
        unsafe {
            ItemThrowLightMotionDecision(self)
        }
    }

    pub fn ItemThrowLightMotionDecisionAir(&mut self) {
        unsafe {
            ItemThrowLightMotionDecisionAir(self)
        }
    }

    pub fn status_ItemThrowCommon(&mut self) {
        unsafe {
            status_ItemThrowCommon(self)
        }
    }

    pub fn status_ItemThrow_Loop(&mut self) {
        unsafe {
            status_ItemThrow_Loop(self)
        }
    }

    pub fn status_ItemThrow(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemThrow(self).into()
        }
    }

    pub fn status_ItemThrow_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemThrow_Main(self).into()
        }
    }

    pub fn sub_item_throw_uniq_process_fix_pos(&mut self) {
        unsafe {
            sub_item_throw_uniq_process_fix_pos(self)
        }
    }

    pub fn status_end_ItemThrow(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemThrow(self).into()
        }
    }

    pub fn status_pre_ItemThrowDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemThrowDash(self).into()
        }
    }

    pub fn status_ItemThrowDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemThrowDash(self).into()
        }
    }

    pub fn ItemThrowDashUniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            ItemThrowDashUniq(self, arg1.into()).into()
        }
    }

    pub fn status_ItemThrowDash_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemThrowDash_Main(self).into()
        }
    }

    pub fn status_end_ItemThrowDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemThrowDash(self).into()
        }
    }

    pub fn status_pre_ItemHeavyPickup(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemHeavyPickup(self).into()
        }
    }

    pub fn status_ItemHeavyPickup(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemHeavyPickup(self).into()
        }
    }

    pub fn status_ItemHeavyPickup_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemHeavyPickup_Main(self).into()
        }
    }

    pub fn status_end_ItemHeavyPickup(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemHeavyPickup(self).into()
        }
    }

    pub fn ItemLiftCommon(&mut self) -> lib::L2CValue {
        unsafe {
            ItemLiftCommon(self).into()
        }
    }

    pub fn status_pre_ItemLiftTurn(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemLiftTurn(self).into()
        }
    }

    pub fn status_ItemLiftTurn(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemLiftTurn(self).into()
        }
    }

    pub fn status_ItemLiftTurn_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemLiftTurn_Main(self).into()
        }
    }

    pub fn status_end_ItemLiftTurn(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemLiftTurn(self).into()
        }
    }

    pub fn status_pre_ItemLiftWalk(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemLiftWalk(self).into()
        }
    }

    pub fn FL_sub_get_item_walk_start_frame(&mut self) -> lib::L2CValue {
        unsafe {
            FL_sub_get_item_walk_start_frame(self).into()
        }
    }

    pub fn status_ItemLiftWalk(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemLiftWalk(self).into()
        }
    }

    pub fn status_ItemLiftWalk_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemLiftWalk_Main(self).into()
        }
    }

    pub fn status_end_ItemLiftWalk(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemLiftWalk(self).into()
        }
    }

    pub fn status_pre_ItemLiftWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemLiftWait(self).into()
        }
    }

    pub fn status_ItemLiftWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemLiftWait(self).into()
        }
    }

    pub fn status_ItemLiftWait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemLiftWait_Main(self).into()
        }
    }

    pub fn status_end_ItemLiftWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemLiftWait(self).into()
        }
    }

    pub fn ItemThrowHeavyCommon(&mut self) {
        unsafe {
            ItemThrowHeavyCommon(self)
        }
    }

    pub fn status_pre_ItemThrowHeavy(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemThrowHeavy(self).into()
        }
    }

    pub fn status_ItemThrowHeavy(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemThrowHeavy(self).into()
        }
    }

    pub fn status_ItemThrowHeavy_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemThrowHeavy_Main(self).into()
        }
    }

    pub fn status_end_ItemThrowHeavy(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemThrowHeavy(self).into()
        }
    }

    pub fn item_lift_walk_set_motion_rate(&mut self) {
        unsafe {
            item_lift_walk_set_motion_rate(self)
        }
    }

    pub fn init_move_speed(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            init_move_speed(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into()).into()
        }
    }

    pub fn set_speed_ratio(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            set_speed_ratio(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn sub_uniq_process_item_lift_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_item_lift_init(self).into()
        }
    }

    pub fn sub_uniq_process_item_lift_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_item_lift_exec(self).into()
        }
    }

    pub fn calc_walk_speed(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue, arg8: lib::L2CValue, arg9: lib::L2CValue) {
        unsafe {
            calc_walk_speed(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into(), arg8.into(), arg9.into())
        }
    }

    pub fn sub_uniq_process_item_lift_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_item_lift_exit(self).into()
        }
    }

    pub fn status_pre_ItemAssistHoist(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemAssistHoist(self).into()
        }
    }

    pub fn status_ItemAssistHoist(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemAssistHoist(self).into()
        }
    }

    pub fn status_ItemAssistHoist_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemAssistHoist_Main(self).into()
        }
    }

    pub fn status_end_ItemAssistHoist(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemAssistHoist(self).into()
        }
    }

    pub fn status_pre_WarpStar(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_WarpStar(self).into()
        }
    }

    pub fn status_WarpStar(&mut self) -> lib::L2CValue {
        unsafe {
            status_WarpStar(self).into()
        }
    }

    pub fn status_WarpStar_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_WarpStar_Main(self).into()
        }
    }

    pub fn status_end_WarpStar(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_WarpStar(self).into()
        }
    }

    pub fn status_pre_WarpStarJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_WarpStarJump(self).into()
        }
    }

    pub fn status_WarpStarJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_WarpStarJump(self).into()
        }
    }

    pub fn status_WarpStarJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_WarpStarJump_Main(self).into()
        }
    }

    pub fn status_end_WarpStarJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_WarpStarJump(self).into()
        }
    }

    pub fn status_pre_DragoonRide(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DragoonRide(self).into()
        }
    }

    pub fn status_DragoonRide(&mut self) -> lib::L2CValue {
        unsafe {
            status_DragoonRide(self).into()
        }
    }

    pub fn status_DragoonRide_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DragoonRide_Main(self).into()
        }
    }

    pub fn status_DragoonRide_Main_Term(&mut self) -> lib::L2CValue {
        unsafe {
            status_DragoonRide_Main_Term(self).into()
        }
    }

    pub fn status_end_DragoonRide(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DragoonRide(self).into()
        }
    }

    pub fn FighterStatusUniqProcessDragoon_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessDragoon_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessDragoon_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessDragoon_exit_status(self).into()
        }
    }

    pub fn status_pre_Killer(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Killer(self).into()
        }
    }

    pub fn status_Killer(&mut self) -> lib::L2CValue {
        unsafe {
            status_Killer(self).into()
        }
    }

    pub fn status_KillerMain(&mut self) -> lib::L2CValue {
        unsafe {
            status_KillerMain(self).into()
        }
    }

    pub fn status_end_Killer(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Killer(self).into()
        }
    }

    pub fn FighterStatusUniqProcessKiller_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessKiller_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessKiller_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessKiller_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessKiller_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessKiller_exec_stop(self).into()
        }
    }

    pub fn FighterStatusUniqProcessKiller_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessKiller_exit_status(self).into()
        }
    }

    pub fn status_pre_KillerJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_KillerJump(self).into()
        }
    }

    pub fn status_KillerJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_KillerJump(self).into()
        }
    }

    pub fn status_KillerJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_KillerJump_Main(self).into()
        }
    }

    pub fn status_end_KillerJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_KillerJump(self).into()
        }
    }

    pub fn uniq_process_KillerJump_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_KillerJump_init_status(self).into()
        }
    }

    pub fn sub_ItemRocketbeltHop_set_kinetic(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            sub_ItemRocketbeltHop_set_kinetic(self, arg1.into(), arg2.into())
        }
    }

    pub fn uniq_process_KillerJump_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_KillerJump_exec_status(self).into()
        }
    }

    pub fn uniq_process_KillerJump_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_KillerJump_exec_stop(self).into()
        }
    }

    pub fn uniq_process_KillerJump_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_KillerJump_exit_status(self).into()
        }
    }

    pub fn hammer_jump_chk(&mut self) {
        unsafe {
            hammer_jump_chk(self)
        }
    }

    pub fn sub_pass_chk(&mut self) {
        unsafe {
            sub_pass_chk(self)
        }
    }

    pub fn sub_set_pass(&mut self) {
        unsafe {
            sub_set_pass(self)
        }
    }

    pub fn hammer_ground_uniq_chk(&mut self) -> lib::L2CValue {
        unsafe {
            hammer_ground_uniq_chk(self).into()
        }
    }

    pub fn hammer_set_motion(&mut self) {
        unsafe {
            hammer_set_motion(self)
        }
    }

    pub fn sub_hammer_exit(&mut self) {
        unsafe {
            sub_hammer_exit(self)
        }
    }

    pub fn sub_hammer_turn_exit(&mut self) {
        unsafe {
            sub_hammer_turn_exit(self)
        }
    }

    pub fn FighterStatusUniqProcessHammer_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessHammer_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_HammerWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_HammerWait(self).into()
        }
    }

    pub fn status_HammerWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerWait(self).into()
        }
    }

    pub fn status_HammerWait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerWait_Main(self).into()
        }
    }

    pub fn sub_uniq_process_hammer_exec_fix_pos_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_hammer_exec_fix_pos_main(self).into()
        }
    }

    pub fn sub_set_hammer_bgm(&mut self) {
        unsafe {
            sub_set_hammer_bgm(self)
        }
    }

    pub fn sub_uniq_process_hammer_exec_fix_pos_counter(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_hammer_exec_fix_pos_counter(self).into()
        }
    }

    pub fn status_end_HammerWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_HammerWait(self).into()
        }
    }

    pub fn status_pre_HammerWalk(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_HammerWalk(self).into()
        }
    }

    pub fn status_HammerWalk(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerWalk(self).into()
        }
    }

    pub fn status_HammerWalk_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerWalk_Main(self).into()
        }
    }

    pub fn status_end_HammerWalk(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_HammerWalk(self).into()
        }
    }

    pub fn status_pre_HammerTurn(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_HammerTurn(self).into()
        }
    }

    pub fn status_HammerTurn(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerTurn(self).into()
        }
    }

    pub fn status_HammerTurn_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerTurn_Main(self).into()
        }
    }

    pub fn hammer_jump_squat_uniq_chk(&mut self) -> lib::L2CValue {
        unsafe {
            hammer_jump_squat_uniq_chk(self).into()
        }
    }

    pub fn status_end_HammerTurn(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_HammerTurn(self).into()
        }
    }

    pub fn status_pre_HammerJumpSquat(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_HammerJumpSquat(self).into()
        }
    }

    pub fn status_HammerJumpSquat(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerJumpSquat(self).into()
        }
    }

    pub fn status_HammerJumpSquat_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerJumpSquat_Main(self).into()
        }
    }

    pub fn status_end_HammerJumpSquat(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_HammerJumpSquat(self).into()
        }
    }

    pub fn status_pre_HammerJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_HammerJump(self).into()
        }
    }

    pub fn status_HammerJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerJump(self).into()
        }
    }

    pub fn status_HammerJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerJump_Main(self).into()
        }
    }

    pub fn status_end_HammerJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_HammerJump(self).into()
        }
    }

    pub fn status_pre_HammerFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_HammerFall(self).into()
        }
    }

    pub fn status_HammerFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerFall(self).into()
        }
    }

    pub fn status_HammerFall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerFall_Main(self).into()
        }
    }

    pub fn status_end_HammerFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_HammerFall(self).into()
        }
    }

    pub fn status_pre_HammerLanding(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_HammerLanding(self).into()
        }
    }

    pub fn status_HammerLanding(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerLanding(self).into()
        }
    }

    pub fn status_HammerLanding_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_HammerLanding_Main(self).into()
        }
    }

    pub fn hammer_landing_uniq_chk(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            hammer_landing_uniq_chk(self, arg1.into()).into()
        }
    }

    pub fn status_end_HammerLanding(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_HammerLanding(self).into()
        }
    }

    pub fn sub_uniq_process_hammer_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_hammer_init_status(self).into()
        }
    }

    pub fn sub_uniq_process_hammer_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_hammer_exec_status(self).into()
        }
    }

    pub fn sub_uniq_process_hammer_exec_fix_pos(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_hammer_exec_fix_pos(self).into()
        }
    }

    pub fn sub_uniq_process_hammer_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_hammer_exit_status(self).into()
        }
    }

    pub fn sub_uniq_process_hammer_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_hammer_check_damage(self).into()
        }
    }

    pub fn sub_big_small_common_mtrans(&mut self) {
        unsafe {
            sub_big_small_common_mtrans(self)
        }
    }

    pub fn sub_BigSmallCommon(&mut self) {
        unsafe {
            sub_BigSmallCommon(self)
        }
    }

    pub fn sub_BigSmallCommon_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_BigSmallCommon_Main(self).into()
        }
    }

    pub fn sub_BigSmallExitCommon(&mut self) {
        unsafe {
            sub_BigSmallExitCommon(self)
        }
    }

    pub fn status_pre_Big(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Big(self).into()
        }
    }

    pub fn status_Big(&mut self) -> lib::L2CValue {
        unsafe {
            status_Big(self).into()
        }
    }

    pub fn status_Big_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Big_Main(self).into()
        }
    }

    pub fn status_end_Big(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Big(self).into()
        }
    }

    pub fn status_pre_Small(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Small(self).into()
        }
    }

    pub fn status_Small(&mut self) -> lib::L2CValue {
        unsafe {
            status_Small(self).into()
        }
    }

    pub fn status_Small_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Small_Main(self).into()
        }
    }

    pub fn status_end_Small(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Small(self).into()
        }
    }

    pub fn FighterStatusUniqProcessKinoko_init_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessKinoko_init_status(self)
        }
    }

    pub fn get_kinoko_scale_table_size(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_kinoko_scale_table_size(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessKinoko_exec_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessKinoko_exec_status(self)
        }
    }

    pub fn get_kinoko_scale(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_kinoko_scale(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessKinoko_exit_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessKinoko_exit_status(self)
        }
    }

    pub fn status_pre_ItemGrassPull(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemGrassPull(self).into()
        }
    }

    pub fn status_ItemGrassPull(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemGrassPull(self).into()
        }
    }

    pub fn status_ItemGrassPull_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemGrassPull_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemGrassPull_exec_fix_pos_counter(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemGrassPull_exec_fix_pos_counter(self)
        }
    }

    pub fn FighterStatusUniqProcessItemGrassPull_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemGrassPull_exec_fix_pos(self)
        }
    }

    pub fn local_func__fighter_status_item_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_3(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemGrassPull_get_item(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemGrassPull_get_item(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessItemGrassPull_fall_item(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessItemGrassPull_fall_item(self, arg1.into(), arg2.into(), arg3.into(), arg4.into())
        }
    }

    pub fn local_func__fighter_status_item_6(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_6(self).into()
        }
    }

    pub fn status_end_ItemGrassPull(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemGrassPull(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemGrassPull_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemGrassPull_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemGrassPull_update_grasspickup_kinetic_speed(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemGrassPull_update_grasspickup_kinetic_speed(self)
        }
    }

    pub fn local_func__fighter_status_item_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_4(self).into()
        }
    }

    pub fn local_func__fighter_status_item_5(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_5(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemGrassPull_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemGrassPull_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemGrassPull_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemGrassPull_exit_status(self).into()
        }
    }

    pub fn status_pre_CaptureItem(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureItem(self).into()
        }
    }

    pub fn status_CaptureItem(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureItem(self).into()
        }
    }

    pub fn status_CaptureItem_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureItem_Main(self).into()
        }
    }

    pub fn status_end_CaptureItem(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureItem(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureItem_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureItem_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CaptureBeetle(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureBeetle(self).into()
        }
    }

    pub fn status_CaptureBeetle(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureBeetle(self).into()
        }
    }

    pub fn status_CaptureBeetle_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureBeetle_Main(self).into()
        }
    }

    pub fn status_end_CaptureBeetle(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureBeetle(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureBeetle_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureBeetle_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CaptureBossgalaga(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureBossgalaga(self).into()
        }
    }

    pub fn status_CaptureBossgalaga(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureBossgalaga(self).into()
        }
    }

    pub fn status_CaptureBossgalaga_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureBossgalaga_Main(self).into()
        }
    }

    pub fn status_end_CaptureBossgalaga(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureBossgalaga(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureBossgalaga_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureBossgalaga_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CaptureBeitcrane(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureBeitcrane(self).into()
        }
    }

    pub fn status_CaptureBeitcrane(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureBeitcrane(self).into()
        }
    }

    pub fn status_CaptureBeitcrane_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureBeitcrane_Main(self).into()
        }
    }

    pub fn status_end_CaptureBeitcrane(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureBeitcrane(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureBeitcrane_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureBeitcrane_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CaptureKawasaki(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureKawasaki(self).into()
        }
    }

    pub fn status_CaptureKawasaki(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureKawasaki(self).into()
        }
    }

    pub fn status_CaptureKawasaki_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureKawasaki_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureKawasaki_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureKawasaki_exec_status(self).into()
        }
    }

    pub fn status_end_CaptureKawasaki(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureKawasaki(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureKawasaki_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureKawasaki_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CaptureDriver(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureDriver(self).into()
        }
    }

    pub fn status_CaptureDriver(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureDriver(self).into()
        }
    }

    pub fn status_CaptureDriver_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureDriver_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureDriver_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureDriver_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureDriver_exec_fix_pos_slow(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureDriver_exec_fix_pos_slow(self).into()
        }
    }

    pub fn status_end_CaptureDriver(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureDriver(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureDriver_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureDriver_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_CaptureMimikkyu(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptureMimikkyu(self).into()
        }
    }

    pub fn status_CaptureMimikkyu(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureMimikkyu(self).into()
        }
    }

    pub fn status_CaptureMimikkyu_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptureMimikkyu_Main(self).into()
        }
    }

    pub fn status_end_CaptureMimikkyu(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptureMimikkyu(self).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureMimikkyu_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureMimikkyu_check_damage(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessCaptureClaptrap_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessCaptureClaptrap_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_KaseyWarp(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_KaseyWarp(self).into()
        }
    }

    pub fn status_KaseyWarp(&mut self) -> lib::L2CValue {
        unsafe {
            status_KaseyWarp(self).into()
        }
    }

    pub fn status_KaseyWarp_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_KaseyWarp_Main(self).into()
        }
    }

    pub fn status_end_KaseyWarp(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_KaseyWarp(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemRocketbeltHover_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemRocketbeltHover_init_status(self).into()
        }
    }

    pub fn local_func__fighter_status_item_7(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_7(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_fire_start(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_fire_start(self)
        }
    }

    pub fn FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_smoke_start(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_smoke_start(self)
        }
    }

    pub fn local_func__fighter_status_item_8(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_8(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemRocketbeltHover_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemRocketbeltHover_exec_status(self).into()
        }
    }

    pub fn local_func__fighter_status_item_9(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_9(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemRocketbeltHover_button_push_check(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemRocketbeltHover_button_push_check(self)
        }
    }

    pub fn FighterStatusUniqProcessItemRocketbeltHover_control_burner(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessItemRocketbeltHover_control_burner(self, arg1.into())
        }
    }

    pub fn local_func__fighter_status_item_11(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_11(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_smoke_end(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_smoke_end(self)
        }
    }

    pub fn FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_fire_end(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemRocketbeltHover_check_rocketbelt_fire_end(self)
        }
    }

    pub fn FighterStatusUniqProcessItemRocketbeltHover_control_effect(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemRocketbeltHover_control_effect(self)
        }
    }

    pub fn FighterStatusUniqProcessItemRocketbeltHover_control_rumble(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemRocketbeltHover_control_rumble(self)
        }
    }

    pub fn FighterStatusUniqProcessItemRocketbeltHover_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemRocketbeltHover_exec_fix_pos(self)
        }
    }

    pub fn local_func__fighter_status_item_10(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_10(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemRocketbeltHover_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemRocketbeltHover_exit_status(self).into()
        }
    }

    pub fn status_pre_ItemRocketbeltHover(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemRocketbeltHover(self).into()
        }
    }

    pub fn status_ItemRocketbeltHover(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemRocketbeltHover(self).into()
        }
    }

    pub fn status_ItemRocketbeltHover_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemRocketbeltHover_Main(self).into()
        }
    }

    pub fn status_end_ItemRocketbeltHover(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemRocketbeltHover(self).into()
        }
    }

    pub fn status_pre_ItemRocketbeltHoverKeep(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemRocketbeltHoverKeep(self).into()
        }
    }

    pub fn status_ItemRocketbeltHoverKeep(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemRocketbeltHoverKeep(self).into()
        }
    }

    pub fn status_ItemRocketbeltHoverKeep_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemRocketbeltHoverKeep_Main(self).into()
        }
    }

    pub fn status_end_ItemRocketbeltHoverKeep(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemRocketbeltHoverKeep(self).into()
        }
    }

    pub fn status_pre_ItemRocketbeltHop(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemRocketbeltHop(self).into()
        }
    }

    pub fn status_ItemRocketbeltHop(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemRocketbeltHop(self).into()
        }
    }

    pub fn status_ItemRocketbeltHop_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemRocketbeltHop_Main(self).into()
        }
    }

    pub fn status_end_ItemRocketbeltHop(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemRocketbeltHop(self).into()
        }
    }

    pub fn uniq_process_ItemRocketbeltHop_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_ItemRocketbeltHop_init_status(self).into()
        }
    }

    pub fn status_pre_ItemSpecialflagHoist(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemSpecialflagHoist(self).into()
        }
    }

    pub fn status_ItemSpecialflagHoist(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSpecialflagHoist(self).into()
        }
    }

    pub fn sub_item_specialflag_hoist_uniq(&mut self) -> lib::L2CValue {
        unsafe {
            sub_item_specialflag_hoist_uniq(self).into()
        }
    }

    pub fn status_ItemSpecialflagHoist_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSpecialflagHoist_Main(self).into()
        }
    }

    pub fn status_end_ItemSpecialflagHoist(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemSpecialflagHoist(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_isShootGroundStatus(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessShoot_isShootGroundStatus(self, arg1.into()).into()
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_isShootAirStatus(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessShoot_isShootAirStatus(self, arg1.into()).into()
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_initShoot_Common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessShoot_initShoot_Common(self, arg1.into(), arg2.into())
        }
    }

    pub fn init_walk_speed(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue) {
        unsafe {
            init_walk_speed(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into())
        }
    }

    pub fn init_walk_motion(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) {
        unsafe {
            init_walk_motion(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into())
        }
    }

    pub fn get_walk_motion_kind(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_walk_motion_kind(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn FL_get_walk_motion_rate(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FL_get_walk_motion_rate(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into()).into()
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_execShoot_Common(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessShoot_execShoot_Common(self)
        }
    }

    pub fn set_walk_motion(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue) {
        unsafe {
            set_walk_motion(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into())
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_isShootStatus(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessShoot_isShootStatus(self, arg1.into()).into()
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_exitShoot_Common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessShoot_exitShoot_Common(self, arg1.into())
        }
    }

    pub fn sub_ItemShootWait_Common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ItemShootWait_Common(self, arg1.into())
        }
    }

    pub fn sub_ItemShootWait_Common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWait_Common_Main(self).into()
        }
    }

    pub fn sub_ItemShootWalkF_Common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ItemShootWalkF_Common(self, arg1.into())
        }
    }

    pub fn sub_ItemShootWalkF_Common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkF_Common_Main(self).into()
        }
    }

    pub fn sub_ItemShootWalkB_Common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ItemShootWalkB_Common(self, arg1.into())
        }
    }

    pub fn sub_ItemShootWalkB_Common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkB_Common_Main(self).into()
        }
    }

    pub fn sub_ItemShootWalkFBrake_Common(&mut self) {
        unsafe {
            sub_ItemShootWalkFBrake_Common(self)
        }
    }

    pub fn sub_ItemShootWalkFBrake_Common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkFBrake_Common_Main(self).into()
        }
    }

    pub fn sub_ItemShootWalkBBrake_Common(&mut self) {
        unsafe {
            sub_ItemShootWalkBBrake_Common(self)
        }
    }

    pub fn sub_ItemShootWalkBBrake_Common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkBBrake_Common_Main(self).into()
        }
    }

    pub fn sub_ItemShootDashF_Common(&mut self) {
        unsafe {
            sub_ItemShootDashF_Common(self)
        }
    }

    pub fn sub_ItemShootDashF_Common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootDashF_Common_Main(self).into()
        }
    }

    pub fn sub_ItemShootDashB_Common(&mut self) {
        unsafe {
            sub_ItemShootDashB_Common(self)
        }
    }

    pub fn sub_ItemShootDashB_Common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootDashB_Common_Main(self).into()
        }
    }

    pub fn sub_ItemShootAir_Common(&mut self) {
        unsafe {
            sub_ItemShootAir_Common(self)
        }
    }

    pub fn sub_ItemShootAir_Common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootAir_Common_Main(self).into()
        }
    }

    pub fn sub_item_shoot_jump_squat_uniq_check(&mut self) -> lib::L2CValue {
        unsafe {
            sub_item_shoot_jump_squat_uniq_check(self).into()
        }
    }

    pub fn sub_jump_squat_uniq_check_sub(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_jump_squat_uniq_check_sub(self, arg1.into())
        }
    }

    pub fn sub_ItemShootJumpSquat_Common(&mut self) {
        unsafe {
            sub_ItemShootJumpSquat_Common(self)
        }
    }

    pub fn sub_ItemShootJumpSquat_Common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootJumpSquat_Common_Main(self).into()
        }
    }

    pub fn sub_ItemShootLanding_Common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ItemShootLanding_Common(self, arg1.into())
        }
    }

    pub fn sub_ItemShootLanding_Common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootLanding_Common_Main(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPos_Common(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPos_Common(self)
        }
    }

    pub fn sub_ItemShootJump_enable_aerial(&mut self) {
        unsafe {
            sub_ItemShootJump_enable_aerial(self)
        }
    }

    pub fn sub_ItemShootJumpAerial_Common(&mut self) {
        unsafe {
            sub_ItemShootJumpAerial_Common(self)
        }
    }

    pub fn sub_ItemShootJumpCommon_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootJumpCommon_Main(self).into()
        }
    }

    pub fn status_ItemShootAir_before(&mut self) {
        unsafe {
            status_ItemShootAir_before(self)
        }
    }

    pub fn get_jump_aerial_motion_common_New(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_jump_aerial_motion_common_New(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into()).into()
        }
    }

    pub fn sub_ItemShootCommon_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootCommon_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootJump_enable_aerial_New(&mut self) {
        unsafe {
            sub_ItemShootJump_enable_aerial_New(self)
        }
    }

    pub fn sub_ItemShootJumpCommon_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootJumpCommon_Main_New(self).into()
        }
    }

    pub fn status_pre_ItemShootWait_common_New(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootWait_common_New(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn status_pre_ItemShootWait_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootWait_New(self).into()
        }
    }

    pub fn FT_UTIL_IS_ITEM_SHOOT_STATUS_NEW(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FT_UTIL_IS_ITEM_SHOOT_STATUS_NEW(self, arg1.into()).into()
        }
    }

    pub fn status_ItemShootWait_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootWait_New(self).into()
        }
    }

    pub fn sub_ItemShootWait_LGun_New(&mut self) {
        unsafe {
            sub_ItemShootWait_LGun_New(self)
        }
    }

    pub fn sub_ItemShootWait_FFlower_New(&mut self) {
        unsafe {
            sub_ItemShootWait_FFlower_New(self)
        }
    }

    pub fn sub_ItemShootWait_SScope_New(&mut self) {
        unsafe {
            sub_ItemShootWait_SScope_New(self)
        }
    }

    pub fn sub_ItemShootWait_GenesisSet_New(&mut self) {
        unsafe {
            sub_ItemShootWait_GenesisSet_New(self)
        }
    }

    pub fn sub_ItemShootWait_MagicPot_New(&mut self) {
        unsafe {
            sub_ItemShootWait_MagicPot_New(self)
        }
    }

    pub fn status_ItemShootWait_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootWait_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWait_LGun_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWait_LGun_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWait_FFlower_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWait_FFlower_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWait_SScope_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWait_SScope_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWait_GenesisSet_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWait_GenesisSet_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWait_MagicPot_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWait_MagicPot_Main_New(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPosCounter_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPosCounter_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPos_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPos_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPos_Common_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPos_Common_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPosCounter_SScope_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPosCounter_SScope_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPosCounter_FFlower_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPosCounter_FFlower_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPosCounter_MagicPot_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPosCounter_MagicPot_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_isMotionEnd_MagicPot_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_isMotionEnd_MagicPot_New(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_toHold_MagicPot_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_toHold_MagicPot_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_toEnd_MagicPot_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_toEnd_MagicPot_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_toEnd_FFlower_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_toEnd_FFlower_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_isShootAirStatus_New(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessShoot_isShootAirStatus_New(self, arg1.into()).into()
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_isMotionEnd_SScope_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_isMotionEnd_SScope_New(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_toRapid_SScope_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_toRapid_SScope_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_toHold_SScope_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_toHold_SScope_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_toEnd_SScope_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_toEnd_SScope_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_loopMotion_SScope_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_loopMotion_SScope_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_toHoldFire_SScope_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_toHoldFire_SScope_New(self)
        }
    }

    pub fn sub_ItemShootWait_Common_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWait_Common_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWait_Common_New(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ItemShootWait_Common_New(self, arg1.into())
        }
    }

    pub fn status_end_ItemShootWait_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemShootWait_New(self).into()
        }
    }

    pub fn status_pre_ItemShootWalkF_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootWalkF_New(self).into()
        }
    }

    pub fn status_ItemShootWalkF_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootWalkF_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkF_LGun_New(&mut self) {
        unsafe {
            sub_ItemShootWalkF_LGun_New(self)
        }
    }

    pub fn sub_ItemShootWalkF_FFlower_New(&mut self) {
        unsafe {
            sub_ItemShootWalkF_FFlower_New(self)
        }
    }

    pub fn sub_ItemShootWalkF_SScope_New(&mut self) {
        unsafe {
            sub_ItemShootWalkF_SScope_New(self)
        }
    }

    pub fn sub_ItemShootWalkF_GenesisSet_New(&mut self) {
        unsafe {
            sub_ItemShootWalkF_GenesisSet_New(self)
        }
    }

    pub fn sub_ItemShootWalkF_MagicPot_New(&mut self) {
        unsafe {
            sub_ItemShootWalkF_MagicPot_New(self)
        }
    }

    pub fn status_ItemShootWalkF_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootWalkF_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkF_LGun_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkF_LGun_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkF_FFlower_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkF_FFlower_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkF_SScope_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkF_SScope_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkF_GenesisSet_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkF_GenesisSet_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkF_MagicPot_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkF_MagicPot_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkF_Common_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkF_Common_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkF_Common_New(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ItemShootWalkF_Common_New(self, arg1.into())
        }
    }

    pub fn status_end_ItemShootWalkF_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemShootWalkF_New(self).into()
        }
    }

    pub fn status_pre_ItemShootWalkFBrake_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootWalkFBrake_New(self).into()
        }
    }

    pub fn status_ItemShootWalkFBrake_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootWalkFBrake_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkFBrake_Common_New(&mut self) {
        unsafe {
            sub_ItemShootWalkFBrake_Common_New(self)
        }
    }

    pub fn status_ItemShootWalkFBrake_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootWalkFBrake_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkFBrake_Common_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkFBrake_Common_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkBrakeF_LGun_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeF_LGun_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeF_FFlower_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeF_FFlower_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeF_SScope_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeF_SScope_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeF_GenesisSet_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeF_GenesisSet_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeF_MagicPot_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeF_MagicPot_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeF_LGun_Main_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeF_LGun_Main_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeF_FFlower_Main_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeF_FFlower_Main_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeF_SScope_Main_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeF_SScope_Main_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeF_GenesisSet_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkBrakeF_GenesisSet_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkBrakeF_MagicPot_Main_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeF_MagicPot_Main_New(self)
        }
    }

    pub fn status_end_ItemShootWalkFBrake_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemShootWalkFBrake_New(self).into()
        }
    }

    pub fn status_pre_ItemShootWalkB_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootWalkB_New(self).into()
        }
    }

    pub fn status_ItemShootWalkB_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootWalkB_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkB_LGun_New(&mut self) {
        unsafe {
            sub_ItemShootWalkB_LGun_New(self)
        }
    }

    pub fn sub_ItemShootWalkB_FFlower_New(&mut self) {
        unsafe {
            sub_ItemShootWalkB_FFlower_New(self)
        }
    }

    pub fn sub_ItemShootWalkB_SScope_New(&mut self) {
        unsafe {
            sub_ItemShootWalkB_SScope_New(self)
        }
    }

    pub fn sub_ItemShootWalkB_GenesisSet_New(&mut self) {
        unsafe {
            sub_ItemShootWalkB_GenesisSet_New(self)
        }
    }

    pub fn sub_ItemShootWalkB_MagicPot_New(&mut self) {
        unsafe {
            sub_ItemShootWalkB_MagicPot_New(self)
        }
    }

    pub fn status_ItemShootWalkB_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootWalkB_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkB_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkB_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkB_LGun_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkB_LGun_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkB_FFlower_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkB_FFlower_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkB_SScope_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkB_SScope_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkB_GenesisSet_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkB_GenesisSet_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkB_MagicPot_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkB_MagicPot_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkB_Common_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkB_Common_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkB_Common_New(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ItemShootWalkB_Common_New(self, arg1.into())
        }
    }

    pub fn status_end_ItemShootWalkB_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemShootWalkB_New(self).into()
        }
    }

    pub fn status_pre_ItemShootWalkBBrake_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootWalkBBrake_New(self).into()
        }
    }

    pub fn status_ItemShootWalkBBrake_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootWalkBBrake_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkBBrake_Common_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBBrake_Common_New(self)
        }
    }

    pub fn status_ItemShootWalkBBrake_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootWalkBBrake_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkBBrake_Common_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkBBrake_Common_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootWalkBrakeB_LGun_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeB_LGun_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeB_FFlower_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeB_FFlower_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeB_SScope_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeB_SScope_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeB_GenesisSet_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeB_GenesisSet_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeB_MagicPot_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeB_MagicPot_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeB_LGun_Main_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeB_LGun_Main_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeB_FFlower_Main_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeB_FFlower_Main_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeB_SScope_Main_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeB_SScope_Main_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeB_MagicPot_Main_New(&mut self) {
        unsafe {
            sub_ItemShootWalkBrakeB_MagicPot_Main_New(self)
        }
    }

    pub fn status_end_ItemShootWalkBBrake_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemShootWalkBBrake_New(self).into()
        }
    }

    pub fn status_pre_ItemShootAir_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootAir_New(self).into()
        }
    }

    pub fn status_ItemShootAir_before_New(&mut self) {
        unsafe {
            status_ItemShootAir_before_New(self)
        }
    }

    pub fn status_ItemShootAir_after_New(&mut self) {
        unsafe {
            status_ItemShootAir_after_New(self)
        }
    }

    pub fn sub_ItemShootAir_SScope_New(&mut self) {
        unsafe {
            sub_ItemShootAir_SScope_New(self)
        }
    }

    pub fn sub_ItemShootAir_MagicPot_New(&mut self) {
        unsafe {
            sub_ItemShootAir_MagicPot_New(self)
        }
    }

    pub fn sub_ItemShootAir_LGun_New(&mut self) {
        unsafe {
            sub_ItemShootAir_LGun_New(self)
        }
    }

    pub fn sub_ItemShootAir_FFlower_New(&mut self) {
        unsafe {
            sub_ItemShootAir_FFlower_New(self)
        }
    }

    pub fn sub_ItemShootAir_GenesisSet_New(&mut self) {
        unsafe {
            sub_ItemShootAir_GenesisSet_New(self)
        }
    }

    pub fn status_ItemShootAirCommon_New(&mut self) {
        unsafe {
            status_ItemShootAirCommon_New(self)
        }
    }

    pub fn status_ItemShootAir_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootAir_New(self).into()
        }
    }

    pub fn status_ItemShootAir_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootAir_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootAir_LGun_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootAir_LGun_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootAir_FFlower_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootAir_FFlower_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootAir_SScope_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootAir_SScope_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootAir_GenesisSet_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootAir_GenesisSet_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootAir_MagicPot_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootAir_MagicPot_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootAir_Common_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootAir_Common_Main_New(self).into()
        }
    }

    pub fn status_end_ItemShootAir_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemShootAir_New(self).into()
        }
    }

    pub fn status_pre_ItemShootJumpSquat_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootJumpSquat_New(self).into()
        }
    }

    pub fn status_ItemShootJumpSquat_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootJumpSquat_New(self).into()
        }
    }

    pub fn sub_ItemShootJumpSquat_Common_New(&mut self) {
        unsafe {
            sub_ItemShootJumpSquat_Common_New(self)
        }
    }

    pub fn status_ItemShootJumpSquat_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootJumpSquat_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootJumpSquat_Common_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootJumpSquat_Common_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootJumpSquat_LGun_New(&mut self) {
        unsafe {
            sub_ItemShootJumpSquat_LGun_New(self)
        }
    }

    pub fn sub_ItemShootJumpSquat_FFlower_New(&mut self) {
        unsafe {
            sub_ItemShootJumpSquat_FFlower_New(self)
        }
    }

    pub fn sub_ItemShootJumpSquat_SScope_New(&mut self) {
        unsafe {
            sub_ItemShootJumpSquat_SScope_New(self)
        }
    }

    pub fn sub_ItemShootJumpSquat_GenesisSet_New(&mut self) {
        unsafe {
            sub_ItemShootJumpSquat_GenesisSet_New(self)
        }
    }

    pub fn sub_ItemShootJumpSquat_MagicPot_New(&mut self) {
        unsafe {
            sub_ItemShootJumpSquat_MagicPot_New(self)
        }
    }

    pub fn sub_item_shoot_jump_squat_uniq_check_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_item_shoot_jump_squat_uniq_check_New(self).into()
        }
    }

    pub fn sub_ItemShootJumpSquat_LGun_Main_New(&mut self) {
        unsafe {
            sub_ItemShootJumpSquat_LGun_Main_New(self)
        }
    }

    pub fn sub_ItemShootJumpSquat_FFlower_Main_New(&mut self) {
        unsafe {
            sub_ItemShootJumpSquat_FFlower_Main_New(self)
        }
    }

    pub fn sub_ItemShootJumpSquat_SScope_Main_New(&mut self) {
        unsafe {
            sub_ItemShootJumpSquat_SScope_Main_New(self)
        }
    }

    pub fn sub_ItemShootJumpSquat_GenesisSet_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootJumpSquat_GenesisSet_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootJumpSquat_MagicPot_Main_New(&mut self) {
        unsafe {
            sub_ItemShootJumpSquat_MagicPot_Main_New(self)
        }
    }

    pub fn status_end_ItemShootJumpSquat_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemShootJumpSquat_New(self).into()
        }
    }

    pub fn status_pre_ItemShootJump_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootJump_New(self).into()
        }
    }

    pub fn status_ItemShootJump_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootJump_New(self).into()
        }
    }

    pub fn sub_ItemShootJump_SScope_New(&mut self) {
        unsafe {
            sub_ItemShootJump_SScope_New(self)
        }
    }

    pub fn sub_ItemShootJump_GenesisSet_New(&mut self) {
        unsafe {
            sub_ItemShootJump_GenesisSet_New(self)
        }
    }

    pub fn sub_ItemShootJump_MagicPot_New(&mut self) {
        unsafe {
            sub_ItemShootJump_MagicPot_New(self)
        }
    }

    pub fn status_ItemShootJump_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootJump_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootJump_LGun_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootJump_LGun_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootJump_FFlower_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootJump_FFlower_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootJump_SScope_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootJump_SScope_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootJump_GenesisSet_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootJump_GenesisSet_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootJump_MagicPot_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootJump_MagicPot_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootJump_LGun_New(&mut self) {
        unsafe {
            sub_ItemShootJump_LGun_New(self)
        }
    }

    pub fn sub_ItemShootJump_FFlower_New(&mut self) {
        unsafe {
            sub_ItemShootJump_FFlower_New(self)
        }
    }

    pub fn status_end_ItemShootJump_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemShootJump_New(self).into()
        }
    }

    pub fn status_pre_ItemShootJumpAerial_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootJumpAerial_New(self).into()
        }
    }

    pub fn status_ItemShootJumpAerial_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootJumpAerial_New(self).into()
        }
    }

    pub fn status_ItemShootJumpAerial_NewSub(&mut self) {
        unsafe {
            status_ItemShootJumpAerial_NewSub(self)
        }
    }

    pub fn status_ItemShootJumpAerial_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootJumpAerial_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootJumpAerial_Common_New(&mut self) {
        unsafe {
            sub_ItemShootJumpAerial_Common_New(self)
        }
    }

    pub fn sub_ItemShootJumpAerial_SScope_New(&mut self) {
        unsafe {
            sub_ItemShootJumpAerial_SScope_New(self)
        }
    }

    pub fn sub_ItemShootJumpAerial_GenesisSet_New(&mut self) {
        unsafe {
            sub_ItemShootJumpAerial_GenesisSet_New(self)
        }
    }

    pub fn sub_ItemShootJumpAerial_MagicPot_New(&mut self) {
        unsafe {
            sub_ItemShootJumpAerial_MagicPot_New(self)
        }
    }

    pub fn sub_ItemShootJumpAerial_FFlower_New(&mut self) {
        unsafe {
            sub_ItemShootJumpAerial_FFlower_New(self)
        }
    }

    pub fn status_end_ItemShootJumpAerial_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemShootJumpAerial_New(self).into()
        }
    }

    pub fn status_pre_ItemShootFly_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootFly_New(self).into()
        }
    }

    pub fn status_ItemShootFly_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootFly_New(self).into()
        }
    }

    pub fn status_ItemShootFly_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootFly_Main_New(self).into()
        }
    }

    pub fn status_end_ItemShootFly_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemShootFly_New(self).into()
        }
    }

    pub fn status_pre_ItemShootLanding_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootLanding_New(self).into()
        }
    }

    pub fn status_ItemShootLanding_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootLanding_New(self).into()
        }
    }

    pub fn sub_ItemShootLanding_Common_New(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ItemShootLanding_Common_New(self, arg1.into())
        }
    }

    pub fn status_ItemShootLanding_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootLanding_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootLanding_Common_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootLanding_Common_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootLanding_GenesisSet_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootLanding_GenesisSet_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootLanding_LGun_New(&mut self) {
        unsafe {
            sub_ItemShootLanding_LGun_New(self)
        }
    }

    pub fn sub_ItemShootLanding_FFlower_New(&mut self) {
        unsafe {
            sub_ItemShootLanding_FFlower_New(self)
        }
    }

    pub fn sub_ItemShootLanding_SScope_New(&mut self) {
        unsafe {
            sub_ItemShootLanding_SScope_New(self)
        }
    }

    pub fn sub_ItemShootLanding_GenesisSet_New(&mut self) {
        unsafe {
            sub_ItemShootLanding_GenesisSet_New(self)
        }
    }

    pub fn sub_ItemShootLanding_MagicPot_New(&mut self) {
        unsafe {
            sub_ItemShootLanding_MagicPot_New(self)
        }
    }

    pub fn sub_ItemShootLanding_LGun_Main_New(&mut self) {
        unsafe {
            sub_ItemShootLanding_LGun_Main_New(self)
        }
    }

    pub fn sub_ItemShootLanding_FFlower_Main_New(&mut self) {
        unsafe {
            sub_ItemShootLanding_FFlower_Main_New(self)
        }
    }

    pub fn sub_ItemShootLanding_SScope_Main_New(&mut self) {
        unsafe {
            sub_ItemShootLanding_SScope_Main_New(self)
        }
    }

    pub fn sub_ItemShootLanding_MagicPot_Main_New(&mut self) {
        unsafe {
            sub_ItemShootLanding_MagicPot_Main_New(self)
        }
    }

    pub fn status_end_ItemShootLanding_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemShootLanding_New(self).into()
        }
    }

    pub fn sub_ItemShootDashF_Common_New(&mut self) {
        unsafe {
            sub_ItemShootDashF_Common_New(self)
        }
    }

    pub fn sub_ItemShootDashF_Common_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootDashF_Common_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootDashB_Common_New(&mut self) {
        unsafe {
            sub_ItemShootDashB_Common_New(self)
        }
    }

    pub fn sub_ItemShootDashB_Common_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootDashB_Common_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootAir_Common_New(&mut self) {
        unsafe {
            sub_ItemShootAir_Common_New(self)
        }
    }

    pub fn sub_ItemShootWalkBrakeB_GenesisSet_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootWalkBrakeB_GenesisSet_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootTurn_GenesisSet_New(&mut self) {
        unsafe {
            sub_ItemShootTurn_GenesisSet_New(self)
        }
    }

    pub fn sub_ItemShootTurn_Common_New(&mut self) {
        unsafe {
            sub_ItemShootTurn_Common_New(self)
        }
    }

    pub fn sub_ItemShootTurn_GenesisSet_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootTurn_GenesisSet_Main_New(self).into()
        }
    }

    pub fn sub_ItemShootTurn_Common_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ItemShootTurn_Common_Main_New(self).into()
        }
    }

    pub fn status_pre_GenesisGet_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GenesisGet_New(self).into()
        }
    }

    pub fn status_GenesisGet_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_GenesisGet_New(self).into()
        }
    }

    pub fn sub_genesis_get_mtrans_New(&mut self) {
        unsafe {
            sub_genesis_get_mtrans_New(self)
        }
    }

    pub fn status_GenesisGet_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_GenesisGet_Main_New(self).into()
        }
    }

    pub fn sub_genesis_get_uniq_New(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_genesis_get_uniq_New(self, arg1.into()).into()
        }
    }

    pub fn status_end_GenesisGet_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GenesisGet_New(self).into()
        }
    }

    pub fn status_pre_GenesisShootStart_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GenesisShootStart_New(self).into()
        }
    }

    pub fn status_GenesisShootStart_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_GenesisShootStart_New(self).into()
        }
    }

    pub fn status_GenesisShootStart_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_GenesisShootStart_Main_New(self).into()
        }
    }

    pub fn sub_genesis_shoot_mtrans_New(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_genesis_shoot_mtrans_New(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn sub_GenesisShootUniq_New(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_GenesisShootUniq_New(self, arg1.into()).into()
        }
    }

    pub fn status_end_GenesisShootStart_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GenesisShootStart_New(self).into()
        }
    }

    pub fn status_pre_GenesisShoot_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GenesisShoot_New(self).into()
        }
    }

    pub fn status_GenesisShoot_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_GenesisShoot_New(self).into()
        }
    }

    pub fn status_GenesisShoot_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_GenesisShoot_Main_New(self).into()
        }
    }

    pub fn status_end_GenesisShoot_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GenesisShoot_New(self).into()
        }
    }

    pub fn status_pre_GenesisShootEnd_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GenesisShootEnd_New(self).into()
        }
    }

    pub fn status_GenesisShootEnd_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_GenesisShootEnd_New(self).into()
        }
    }

    pub fn status_GenesisShootEnd_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_GenesisShootEnd_Main_New(self).into()
        }
    }

    pub fn status_end_GenesisShootEnd_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GenesisShootEnd_New(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_isShootStatus_New(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessShoot_isShootStatus_New(self, arg1.into()).into()
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_isShootGroundStatus_New(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessShoot_isShootGroundStatus_New(self, arg1.into()).into()
        }
    }

    pub fn item_shoot_walk_set_motion_rate_New(&mut self, arg1: lib::L2CValue) {
        unsafe {
            item_shoot_walk_set_motion_rate_New(self, arg1.into())
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_initShoot_Common_New(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessShoot_initShoot_Common_New(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_initShoot_CommonAirUpper_New(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessShoot_initShoot_CommonAirUpper_New(self, arg1.into(), arg2.into())
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_execShoot_Common_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessShoot_execShoot_Common_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessShoot_exitShoot_Common_New(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessShoot_exitShoot_Common_New(self, arg1.into())
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_initStatus_common_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_initStatus_common_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_initStatus_LGun_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_initStatus_LGun_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_initStatus_SScope_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_initStatus_SScope_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_initStatus_FFlower_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_initStatus_FFlower_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_initStatus_GenesisSet_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_initStatus_GenesisSet_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_initStatus_Drill_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_initStatus_Drill_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_initStatus_MagicPot_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_initStatus_MagicPot_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_initStatus_SteelDiver_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_initStatus_SteelDiver_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_initStatus_Staff_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_initStatus_Staff_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_initStatus_Revengeshooter_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_initStatus_Revengeshooter_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_detachHoldAnim_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_detachHoldAnim_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_setup_status_kind_table_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_setup_status_kind_table_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_initStatus_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_initStatus_New(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_attachHoldAnim_New(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_attachHoldAnim_New(self, arg1.into())
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execStatus_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execStatus_New(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execStatus_SScope_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execStatus_SScope_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execStatus_FFlower_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execStatus_FFlower_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execStatus_MagicPot_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execStatus_MagicPot_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execStatus_Staff_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execStatus_Staff_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execStatus_Revengeshooter_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execStatus_Revengeshooter_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_toHold_FFlower_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_toHold_FFlower_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execStatus_LGun_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execStatus_LGun_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execStatus_GenesisSet_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execStatus_GenesisSet_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPosCounter_LGun_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPosCounter_LGun_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPosCounter_GenesisSet_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPosCounter_GenesisSet_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPos_LGun_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPos_LGun_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPos_SScope_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPos_SScope_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPos_FFlower_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPos_FFlower_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPos_GenesisSet_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPos_GenesisSet_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_execFixPos_MagicPot_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_execFixPos_MagicPot_New(self)
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_exitStatus_New(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_exitStatus_New(self).into()
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_exit_LGun_New(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_exit_LGun_New(self, arg1.into())
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_exit_SScope_New(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_exit_SScope_New(self, arg1.into())
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_exit_FFlower_New(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_exit_FFlower_New(self, arg1.into())
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_exit_MagicPot_New(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_exit_MagicPot_New(self, arg1.into())
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_exit_GenesisSet_New(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_exit_GenesisSet_New(self, arg1.into())
        }
    }

    pub fn sub_ftStatusUniqProcessItemShoot_loopMotion_MagicPot_New(&mut self) {
        unsafe {
            sub_ftStatusUniqProcessItemShoot_loopMotion_MagicPot_New(self)
        }
    }

    pub fn FighterStatusUniqProcessItemShootWalkF_calc_param_New(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemShootWalkF_calc_param_New(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemShootWalkB_calc_param_New(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemShootWalkB_calc_param_New(self).into()
        }
    }

    pub fn status_pre_ItemShootTurn_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemShootTurn_New(self).into()
        }
    }

    pub fn sub_status_ItemShootTurn_New(&mut self) {
        unsafe {
            sub_status_ItemShootTurn_New(self)
        }
    }

    pub fn status_ItemShootTurn_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootTurn_New(self).into()
        }
    }

    pub fn status_ItemShootTurn_Main_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemShootTurn_Main_New(self).into()
        }
    }

    pub fn status_end_ItemShootTurn_New(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemShootTurn_New(self).into()
        }
    }

    pub fn status_pre_ItemStarring(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemStarring(self).into()
        }
    }

    pub fn status_ItemStarring(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemStarring(self).into()
        }
    }

    pub fn local_func__fighter_status_item_starring_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_starring_1(self).into()
        }
    }

    pub fn status_ItemStarring_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemStarring_Main(self).into()
        }
    }

    pub fn local_func__fighter_status_item_starring_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_starring_2(self).into()
        }
    }

    pub fn local_func__fighter_status_item_starring_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_starring_3(self).into()
        }
    }

    pub fn local_func__fighter_status_item_starring_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_starring_4(self).into()
        }
    }

    pub fn status_end_ItemStarring(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemStarring(self).into()
        }
    }

    pub fn status_pre_ItemStarringShoot(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemStarringShoot(self).into()
        }
    }

    pub fn status_ItemStarringShoot(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemStarringShoot(self).into()
        }
    }

    pub fn status_ItemStarringShoot_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemStarringShoot_Main(self).into()
        }
    }

    pub fn status_end_ItemStarringShoot(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemStarringShoot(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemStarringShoot_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemStarringShoot_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemStarringShoot_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemStarringShoot_exec_status(self).into()
        }
    }

    pub fn local_func__fighter_status_item_starring_5(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_starring_5(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemStarringShoot_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemStarringShoot_exec_stop(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemStarringShoot_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemStarringShoot_exit_status(self).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_11(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue, arg8: lib::L2CValue, arg9: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_11(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into(), arg8.into(), arg9.into()).into()
        }
    }

    pub fn get_swing_log_attack_kind(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_swing_log_attack_kind(self, arg1.into()).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_12(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_12(self).into()
        }
    }

    pub fn local_func__fighter_status_item_swing_13(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_13(self).into()
        }
    }

    pub fn status_pre_ItemSwing(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemSwing(self).into()
        }
    }

    pub fn item_swing_motion(&mut self) -> lib::L2CValue {
        unsafe {
            item_swing_motion(self).into()
        }
    }

    pub fn item_swing_motion_rate(&mut self) -> lib::L2CValue {
        unsafe {
            item_swing_motion_rate(self).into()
        }
    }

    pub fn sub_item_swing_uniq_check(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_item_swing_uniq_check(self, arg1.into()).into()
        }
    }

    pub fn status_ItemSwing(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSwing(self).into()
        }
    }

    pub fn status_ItemSwing_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSwing_Main(self).into()
        }
    }

    pub fn ItemSwingCommon(&mut self) -> lib::L2CValue {
        unsafe {
            ItemSwingCommon(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemSwing_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemSwing_exec_fix_pos(self)
        }
    }

    pub fn FighterStatusUniqProcessItemSwing_update_attack_num_by_firebar_ball_num(&mut self) {
        unsafe {
            FighterStatusUniqProcessItemSwing_update_attack_num_by_firebar_ball_num(self)
        }
    }

    pub fn local_func__fighter_status_item_swing_14(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_item_swing_14(self).into()
        }
    }

    pub fn status_end_ItemSwing(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemSwing(self).into()
        }
    }

    pub fn status_pre_ItemSwingS3(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemSwingS3(self).into()
        }
    }

    pub fn item_swing3_motion(&mut self) -> lib::L2CValue {
        unsafe {
            item_swing3_motion(self).into()
        }
    }

    pub fn status_ItemSwingS3(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSwingS3(self).into()
        }
    }

    pub fn status_ItemSwingS3_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSwingS3_Main(self).into()
        }
    }

    pub fn status_end_ItemSwingS3(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemSwingS3(self).into()
        }
    }

    pub fn status_pre_ItemSwingS4Start(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemSwingS4Start(self).into()
        }
    }

    pub fn item_swing_motion4(&mut self) -> lib::L2CValue {
        unsafe {
            item_swing_motion4(self).into()
        }
    }

    pub fn status_ItemSwingS4Start(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSwingS4Start(self).into()
        }
    }

    pub fn status_ItemSwingS4Start_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSwingS4Start_Main(self).into()
        }
    }

    pub fn sub_item_swing_smash_start_uniq(&mut self) -> lib::L2CValue {
        unsafe {
            sub_item_swing_smash_start_uniq(self).into()
        }
    }

    pub fn status_end_ItemSwingS4Start(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemSwingS4Start(self).into()
        }
    }

    pub fn status_pre_ItemSwingS4Hold(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemSwingS4Hold(self).into()
        }
    }

    pub fn status_pre_ItemSwingS4Hold_Common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_ItemSwingS4Hold_Common(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn status_ItemSwingS4Hold(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSwingS4Hold(self).into()
        }
    }

    pub fn sub_item_swing_smash_hold_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_item_swing_smash_hold_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_ItemSwingS4Hold_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSwingS4Hold_Main(self).into()
        }
    }

    pub fn status_end_ItemSwingS4Hold(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemSwingS4Hold(self).into()
        }
    }

    pub fn status_pre_ItemSwingS4(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemSwingS4(self).into()
        }
    }

    pub fn status_ItemSwingS4(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSwingS4(self).into()
        }
    }

    pub fn ItemSwingS4Uniq(&mut self) -> lib::L2CValue {
        unsafe {
            ItemSwingS4Uniq(self).into()
        }
    }

    pub fn status_ItemSwingS4_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSwingS4_Main(self).into()
        }
    }

    pub fn status_end_ItemSwingS4(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemSwingS4(self).into()
        }
    }

    pub fn status_pre_ItemSwingDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemSwingDash(self).into()
        }
    }

    pub fn item_swing_dash_motion(&mut self) -> lib::L2CValue {
        unsafe {
            item_swing_dash_motion(self).into()
        }
    }

    pub fn status_ItemSwingDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSwingDash(self).into()
        }
    }

    pub fn status_ItemSwingDash_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemSwingDash_Main(self).into()
        }
    }

    pub fn status_end_ItemSwingDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemSwingDash(self).into()
        }
    }

    pub fn sub_uniq_process_item_swing_init(&mut self) {
        unsafe {
            sub_uniq_process_item_swing_init(self)
        }
    }

    pub fn sub_uniq_process_item_swing_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_item_swing_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemSwing_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemSwing_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemSwing_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemSwing_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessItemSwing_check_attack(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessItemSwing_check_attack(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn status_pre_JumpSquat(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_JumpSquat(self).into()
        }
    }

    pub fn status_pre_JumpSquat_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_JumpSquat_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into()).into()
        }
    }

    pub fn status_pre_Jump_sub(&mut self) {
        unsafe {
            status_pre_Jump_sub(self)
        }
    }

    pub fn status_pre_Jump_sub_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) {
        unsafe {
            status_pre_Jump_sub_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into())
        }
    }

    pub fn status_pre_Jump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Jump(self).into()
        }
    }

    pub fn status_pre_JumpAerial_sub(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_JumpAerial_sub(self).into()
        }
    }

    pub fn status_pre_JumpAerial_sub_param(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_JumpAerial_sub_param(self, arg1.into()).into()
        }
    }

    pub fn status_pre_JumpAerial(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_JumpAerial(self).into()
        }
    }

    pub fn status_pre_JumpAerial_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_JumpAerial_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into()).into()
        }
    }

    pub fn status_pre_Fly(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Fly(self).into()
        }
    }

    pub fn status_pre_Fall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Fall(self).into()
        }
    }

    pub fn status_pre_Fall_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Fall_main(self).into()
        }
    }

    pub fn status_pre_Fall_main_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_Fall_main_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into()).into()
        }
    }

    pub fn status_pre_FallAerial(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FallAerial(self).into()
        }
    }

    pub fn status_pre_FallAerial_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FallAerial_main(self).into()
        }
    }

    pub fn status_pre_FallAerial_main_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_FallAerial_main_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into()).into()
        }
    }

    pub fn status_pre_Landing(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Landing(self).into()
        }
    }

    pub fn sub_pre_landing(&mut self) -> lib::L2CValue {
        unsafe {
            sub_pre_landing(self).into()
        }
    }

    pub fn status_pre_Landing_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_Landing_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into()).into()
        }
    }

    pub fn status_pre_LandingLight(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_LandingLight(self).into()
        }
    }

    pub fn status_pre_LandingLight_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_LandingLight_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into()).into()
        }
    }

    pub fn status_pre_LandingAttackAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_LandingAttackAir(self).into()
        }
    }

    pub fn status_pre_LandingDamageLight(&mut self) {
        unsafe {
            status_pre_LandingDamageLight(self)
        }
    }

    pub fn status_pre_GlideStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GlideStart(self).into()
        }
    }

    pub fn status_pre_Glide(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Glide(self).into()
        }
    }

    pub fn status_pre_GlideLanding(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GlideLanding(self).into()
        }
    }

    pub fn status_pre_GlideAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GlideAttack(self).into()
        }
    }

    pub fn status_pre_GlideEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GlideEnd(self).into()
        }
    }

    pub fn status_JumpSquat_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_JumpSquat_common(self, arg1.into())
        }
    }

    pub fn sub_status_JumpSquat_check_stick_lr_update(&mut self) -> lib::L2CValue {
        unsafe {
            sub_status_JumpSquat_check_stick_lr_update(self).into()
        }
    }

    pub fn status_JumpSquat(&mut self) -> lib::L2CValue {
        unsafe {
            status_JumpSquat(self).into()
        }
    }

    pub fn status_JumpSquat_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_JumpSquat_Main(self).into()
        }
    }

    pub fn uniq_process_JumpSquat_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_JumpSquat_exec_status(self).into()
        }
    }

    pub fn uniq_process_JumpSquat_exec_status_param(&mut self, arg1: lib::L2CValue) {
        unsafe {
            uniq_process_JumpSquat_exec_status_param(self, arg1.into())
        }
    }

    pub fn sub_jump_squat_uniq_check_sub_mini_attack(&mut self) {
        unsafe {
            sub_jump_squat_uniq_check_sub_mini_attack(self)
        }
    }

    pub fn sub_jump_squat_uniq_check(&mut self) {
        unsafe {
            sub_jump_squat_uniq_check(self)
        }
    }

    pub fn sub_jump_aerial_item_rocketbelt(&mut self) {
        unsafe {
            sub_jump_aerial_item_rocketbelt(self)
        }
    }

    pub fn status_JumpAerial(&mut self) -> lib::L2CValue {
        unsafe {
            status_JumpAerial(self).into()
        }
    }

    pub fn status_Fly(&mut self) -> lib::L2CValue {
        unsafe {
            status_Fly(self).into()
        }
    }

    pub fn status_FlySub(&mut self) {
        unsafe {
            status_FlySub(self)
        }
    }

    pub fn status_Fly_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Fly_Main(self).into()
        }
    }

    pub fn sub_getFlyMotion(&mut self) -> lib::L2CValue {
        unsafe {
            sub_getFlyMotion(self).into()
        }
    }

    pub fn sub_fly_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_fly_uniq(self, arg1.into()).into()
        }
    }

    pub fn local_func__fighter_status_jump_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_jump_1(self).into()
        }
    }

    pub fn status_Fall(&mut self) -> lib::L2CValue {
        unsafe {
            status_Fall(self).into()
        }
    }

    pub fn status_FallSub(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_FallSub(self, arg1.into())
        }
    }

    pub fn status_Fall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Fall_Main(self).into()
        }
    }

    pub fn status_FallSub_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            status_FallSub_param(self, arg1.into(), arg2.into())
        }
    }

    pub fn status_FallAerial(&mut self) -> lib::L2CValue {
        unsafe {
            status_FallAerial(self).into()
        }
    }

    pub fn status_FallAerialSub_param(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_FallAerialSub_param(self, arg1.into())
        }
    }

    pub fn status_FallAerial_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FallAerial_Main(self).into()
        }
    }

    pub fn status_FallAerialSub(&mut self) {
        unsafe {
            status_FallAerialSub(self)
        }
    }

    pub fn status_Landing(&mut self) -> lib::L2CValue {
        unsafe {
            status_Landing(self).into()
        }
    }

    pub fn sub_landing_start_check_damage_face(&mut self) {
        unsafe {
            sub_landing_start_check_damage_face(self)
        }
    }

    pub fn status_LandingLight(&mut self) -> lib::L2CValue {
        unsafe {
            status_LandingLight(self).into()
        }
    }

    pub fn status_LandingLightSub_param(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_LandingLightSub_param(self, arg1.into())
        }
    }

    pub fn status_LandingLight_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_LandingLight_Main(self).into()
        }
    }

    pub fn status_LandingLightSub(&mut self) {
        unsafe {
            status_LandingLightSub(self)
        }
    }

    pub fn sub_landing_uniq_check_attack_air(&mut self) -> lib::L2CValue {
        unsafe {
            sub_landing_uniq_check_attack_air(self).into()
        }
    }

    pub fn status_LandingAttackAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_LandingAttackAir(self).into()
        }
    }

    pub fn status_LandingAttackAirSub(&mut self) {
        unsafe {
            status_LandingAttackAirSub(self)
        }
    }

    pub fn status_LandingAttackAir_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_LandingAttackAir_Main(self).into()
        }
    }

    pub fn status_LandingDamageLight(&mut self) -> lib::L2CValue {
        unsafe {
            status_LandingDamageLight(self).into()
        }
    }

    pub fn status_GlideStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_GlideStart(self).into()
        }
    }

    pub fn status_GlideStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GlideStart_Main(self).into()
        }
    }

    pub fn status_Glide(&mut self) -> lib::L2CValue {
        unsafe {
            status_Glide(self).into()
        }
    }

    pub fn status_Glide_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Glide_Main(self).into()
        }
    }

    pub fn status_GlideLanding(&mut self) -> lib::L2CValue {
        unsafe {
            status_GlideLanding(self).into()
        }
    }

    pub fn status_GlideLanding_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GlideLanding_Main(self).into()
        }
    }

    pub fn status_GlideAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_GlideAttack(self).into()
        }
    }

    pub fn status_GlideAttack_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GlideAttack_Main(self).into()
        }
    }

    pub fn status_GlideEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_GlideEnd(self).into()
        }
    }

    pub fn status_GlideEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GlideEnd_Main(self).into()
        }
    }

    pub fn status_end_JumpSquat(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_JumpSquat(self).into()
        }
    }

    pub fn status_end_Jump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Jump(self).into()
        }
    }

    pub fn status_end_JumpAerial(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_JumpAerial(self).into()
        }
    }

    pub fn status_end_JumpAerial_param(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_end_JumpAerial_param(self, arg1.into())
        }
    }

    pub fn status_end_Fly(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Fly(self).into()
        }
    }

    pub fn status_end_Fall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Fall(self).into()
        }
    }

    pub fn status_end_FallAerial(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FallAerial(self).into()
        }
    }

    pub fn status_end_Landing(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Landing(self).into()
        }
    }

    pub fn status_end_LandingLight(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_LandingLight(self).into()
        }
    }

    pub fn status_end_LandingAttackAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_LandingAttackAir(self).into()
        }
    }

    pub fn status_end_LandingDamageLight(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_LandingDamageLight(self).into()
        }
    }

    pub fn status_end_GlideStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GlideStart(self).into()
        }
    }

    pub fn status_end_Glide(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Glide(self).into()
        }
    }

    pub fn status_end_GlideLanding(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GlideLanding(self).into()
        }
    }

    pub fn status_end_GlideAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GlideAttack(self).into()
        }
    }

    pub fn status_end_GlideEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GlideEnd(self).into()
        }
    }

    pub fn sub_glide_check(&mut self) -> lib::L2CValue {
        unsafe {
            sub_glide_check(self).into()
        }
    }

    pub fn status_pre_ItemScrewJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemScrewJump(self).into()
        }
    }

    pub fn status_ItemScrewJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemScrewJump(self).into()
        }
    }

    pub fn status_ItemScrewJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemScrewJump_Main(self).into()
        }
    }

    pub fn status_end_ItemScrewJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemScrewJump(self).into()
        }
    }

    pub fn status_pre_ItemScrewJumpAerial(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemScrewJumpAerial(self).into()
        }
    }

    pub fn status_ItemScrewJumpAerial(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemScrewJumpAerial(self).into()
        }
    }

    pub fn status_ItemScrewJumpAerialSub(&mut self) {
        unsafe {
            status_ItemScrewJumpAerialSub(self)
        }
    }

    pub fn status_ItemScrewJumpAerial_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemScrewJumpAerial_Main(self).into()
        }
    }

    pub fn status_end_ItemScrewJumpAerial(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemScrewJumpAerial(self).into()
        }
    }

    pub fn status_pre_ItemScrewFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ItemScrewFall(self).into()
        }
    }

    pub fn status_ItemScrewFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemScrewFall(self).into()
        }
    }

    pub fn status_ItemScrewFall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ItemScrewFall_Main(self).into()
        }
    }

    pub fn status_end_ItemScrewFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ItemScrewFall(self).into()
        }
    }

    pub fn sub_fall_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fall_uniq_process_init(self).into()
        }
    }

    pub fn sub_fall_uniq_process_init_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_fall_uniq_process_init_param(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn sub_fall_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fall_uniq_process_exec(self).into()
        }
    }

    pub fn sub_fall_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fall_uniq_process_exit(self).into()
        }
    }

    pub fn sub_fall_uniq_process_exit_param(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_fall_uniq_process_exit_param(self, arg1.into())
        }
    }

    pub fn sub_jump_squat_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_jump_squat_uniq_process_init(self).into()
        }
    }

    pub fn sub_jump_squat_uniq_process_init_param(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_jump_squat_uniq_process_init_param(self, arg1.into())
        }
    }

    pub fn sub_fly_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_fly_uniq_process_init(self).into()
        }
    }

    pub fn sub_calc_landing_motion_rate(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_calc_landing_motion_rate(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn sub_landing_uniq_process_init_main(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_landing_uniq_process_init_main(self, arg1.into())
        }
    }

    pub fn sub_landing_uniq_process_init_main_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) {
        unsafe {
            sub_landing_uniq_process_init_main_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into())
        }
    }

    pub fn sub_landing_attack_air_init(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_landing_attack_air_init(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn sub_landing_fall_special_init(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_landing_fall_special_init(self, arg1.into())
        }
    }

    pub fn sub_get_landing_motion_rate(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_get_landing_motion_rate(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn sub_landing_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_landing_uniq_process_init(self).into()
        }
    }

    pub fn sub_landing_attack_air_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_landing_attack_air_common(self, arg1.into())
        }
    }

    pub fn sub_landing_uniq_process_exec_main(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_landing_uniq_process_exec_main(self, arg1.into())
        }
    }

    pub fn sub_landing_uniq_process_exec_main_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_landing_uniq_process_exec_main_param(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn sub_landing_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_landing_uniq_process_exec(self).into()
        }
    }

    pub fn sub_landing_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_landing_uniq_process_exit(self).into()
        }
    }

    pub fn status_pre_FinalJumpEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalJumpEnd(self).into()
        }
    }

    pub fn status_FinalJumpEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalJumpEnd(self).into()
        }
    }

    pub fn sub_FinalJumpEndUniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_FinalJumpEndUniq(self, arg1.into()).into()
        }
    }

    pub fn status_FinalJumpEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalJumpEnd_Main(self).into()
        }
    }

    pub fn status_end_FinalJumpEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FinalJumpEnd(self).into()
        }
    }

    pub fn status_pre_KamuiPierce(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_KamuiPierce(self).into()
        }
    }

    pub fn status_KamuiPierce(&mut self) -> lib::L2CValue {
        unsafe {
            status_KamuiPierce(self).into()
        }
    }

    pub fn status_KamuiPierce_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_KamuiPierce_Main(self).into()
        }
    }

    pub fn status_end_KamuiPierce(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_KamuiPierce(self).into()
        }
    }

    pub fn status_pre_KoopaDived(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_KoopaDived(self).into()
        }
    }

    pub fn status_KoopaDived(&mut self) -> lib::L2CValue {
        unsafe {
            status_KoopaDived(self).into()
        }
    }

    pub fn status_end_KoopaDived(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_KoopaDived(self).into()
        }
    }

    pub fn sub_uniq_process_KoopaDived_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_KoopaDived_init(self).into()
        }
    }

    pub fn sub_uniq_process_KoopaDived_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_KoopaDived_exec(self).into()
        }
    }

    pub fn sub_uniq_process_KoopaDived_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_KoopaDived_exit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessKoopaDived_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessKoopaDived_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_LadderCatch(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_LadderCatch(self).into()
        }
    }

    pub fn status_LadderCatch(&mut self) -> lib::L2CValue {
        unsafe {
            status_LadderCatch(self).into()
        }
    }

    pub fn sub_status_LadderCatch_mtrans(&mut self) {
        unsafe {
            sub_status_LadderCatch_mtrans(self)
        }
    }

    pub fn status_LadderCatch_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_LadderCatch_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_exec_fix_pos(self)
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_catch(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_catch(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_catch_bottom(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_catch_bottom(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_fix_pos_ladder(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_fix_pos_ladder(self, arg1.into(), arg2.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_end(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_end(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_fix_pos_ladder_param(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn status_end_LadderCatch(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_LadderCatch(self).into()
        }
    }

    pub fn status_pre_LadderCatchBottom(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_LadderCatchBottom(self).into()
        }
    }

    pub fn status_LadderCatchBottom(&mut self) -> lib::L2CValue {
        unsafe {
            status_LadderCatchBottom(self).into()
        }
    }

    pub fn sub_LadderCatchBottom_common(&mut self) {
        unsafe {
            sub_LadderCatchBottom_common(self)
        }
    }

    pub fn status_LadderCatchBottom_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_LadderCatchBottom_Main(self).into()
        }
    }

    pub fn status_end_LadderCatchBottom(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_LadderCatchBottom(self).into()
        }
    }

    pub fn status_pre_Ladder(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Ladder(self).into()
        }
    }

    pub fn status_Ladder(&mut self) -> lib::L2CValue {
        unsafe {
            status_Ladder(self).into()
        }
    }

    pub fn sub_LadderUniqChk(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_LadderUniqChk(self, arg1.into()).into()
        }
    }

    pub fn status_Ladder_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Ladder_Main(self).into()
        }
    }

    pub fn status_end_Ladder(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Ladder(self).into()
        }
    }

    pub fn status_pre_LadderAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_LadderAttack(self).into()
        }
    }

    pub fn status_LadderAttack_common(&mut self) {
        unsafe {
            status_LadderAttack_common(self)
        }
    }

    pub fn status_LadderAttack_common_param(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_LadderAttack_common_param(self, arg1.into())
        }
    }

    pub fn status_LadderAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_LadderAttack(self).into()
        }
    }

    pub fn status_LadderAttack_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_LadderAttack_Main(self).into()
        }
    }

    pub fn status_end_LadderAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_LadderAttack(self).into()
        }
    }

    pub fn status_pre_LadderEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_LadderEnd(self).into()
        }
    }

    pub fn status_LadderEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_LadderEnd(self).into()
        }
    }

    pub fn status_LadderEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_LadderEnd_Main(self).into()
        }
    }

    pub fn status_end_LadderEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_LadderEnd(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_correct_x(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_correct_x(self, arg1.into(), arg2.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_correct_y(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_correct_y(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_adjust_ladder_movement(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_adjust_ladder_movement(self, arg1.into(), arg2.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_init_ladder_catch_bottom(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_init_ladder_catch_bottom(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_init_ladder(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_init_ladder(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_init_ladder_attack(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_init_ladder_attack(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_init_ladder_end(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_init_ladder_end(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_exec_ladder_catch(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_exec_ladder_catch(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_exec_ladder_catch_bottom(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_exec_ladder_catch_bottom(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_exec_ladder(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_exec_ladder(self, arg1.into(), arg2.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_exec_ladder_end(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_exec_ladder_end(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_exec_ladder_flip_rotation(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_exec_ladder_flip_rotation(self)
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_exec_stop_ladder(&mut self) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_exec_stop_ladder(self)
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_exit_ladder(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_exit_ladder(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_exit_ladder_attack(&mut self, arg1: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_exit_ladder_attack(self, arg1.into())
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadderAttack_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickLadderAttack_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_exec_stop(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGimmickLadder_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGimmickLadder_exit_status(self).into()
        }
    }

    pub fn status_pre_MewtwoThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_MewtwoThrown(self).into()
        }
    }

    pub fn status_MewtwoThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_MewtwoThrown(self).into()
        }
    }

    pub fn local_func__fighter_status_mewtwo_thrown_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_status_mewtwo_thrown_1(self).into()
        }
    }

    pub fn status_end_MewtwoThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MewtwoThrown(self).into()
        }
    }

    pub fn FighterStatusUniqProcessMewtwoThrown_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessMewtwoThrown_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessMewtwoThrown_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessMewtwoThrown_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_MiifighterCounterThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_MiifighterCounterThrown(self).into()
        }
    }

    pub fn status_MiifighterCounterThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_MiifighterCounterThrown(self).into()
        }
    }

    pub fn status_MiifighterCounterThrown_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_MiifighterCounterThrown_Main(self).into()
        }
    }

    pub fn status_end_MiifighterCounterThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MiifighterCounterThrown(self).into()
        }
    }

    pub fn FighterStatusUniqProcessMiifighterCounterThrown_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessMiifighterCounterThrown_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_MiifighterSuplexThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_MiifighterSuplexThrown(self).into()
        }
    }

    pub fn status_MiifighterSuplexThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_MiifighterSuplexThrown(self).into()
        }
    }

    pub fn status_MiifighterSuplexThrown_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_MiifighterSuplexThrown_Main(self).into()
        }
    }

    pub fn update_suplex_offset(&mut self) {
        unsafe {
            update_suplex_offset(self)
        }
    }

    pub fn status_end_MiifighterSuplexThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MiifighterSuplexThrown(self).into()
        }
    }

    pub fn FighterStatusUniqProcessMiifighterSuplexThrown_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessMiifighterSuplexThrown_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_MiifighterSuplexAirCaptured(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_MiifighterSuplexAirCaptured(self).into()
        }
    }

    pub fn status_MiifighterSuplexAirCaptured(&mut self) -> lib::L2CValue {
        unsafe {
            status_MiifighterSuplexAirCaptured(self).into()
        }
    }

    pub fn status_MiifighterSuplexAirCaptured_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_MiifighterSuplexAirCaptured_Main(self).into()
        }
    }

    pub fn status_end_MiifighterSuplexAirCaptured(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MiifighterSuplexAirCaptured(self).into()
        }
    }

    pub fn FighterStatusUniqProcessMiifighterSuplexAirCaptured_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessMiifighterSuplexAirCaptured_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_MiifighterSuplexAirFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_MiifighterSuplexAirFall(self).into()
        }
    }

    pub fn status_MiifighterSuplexAirFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_MiifighterSuplexAirFall(self).into()
        }
    }

    pub fn status_MiifighterSuplexAirFall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_MiifighterSuplexAirFall_Main(self).into()
        }
    }

    pub fn status_end_MiifighterSuplexAirFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MiifighterSuplexAirFall(self).into()
        }
    }

    pub fn FighterStatusUniqProcessMiifighterSuplexAirFall_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessMiifighterSuplexAirFall_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_MiifighterSuplexAirLanding(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_MiifighterSuplexAirLanding(self).into()
        }
    }

    pub fn status_MiifighterSuplexAirLanding(&mut self) -> lib::L2CValue {
        unsafe {
            status_MiifighterSuplexAirLanding(self).into()
        }
    }

    pub fn status_MiifighterSuplexAirLanding_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_MiifighterSuplexAirLanding_Main(self).into()
        }
    }

    pub fn status_end_MiifighterSuplexAirLanding(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MiifighterSuplexAirLanding(self).into()
        }
    }

    pub fn FighterStatusUniqProcessMiifighterSuplexAirLanding_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessMiifighterSuplexAirLanding_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_MiiswordsmanCounterDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_MiiswordsmanCounterDamage(self).into()
        }
    }

    pub fn status_MiiswordsmanCounterDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_MiiswordsmanCounterDamage(self).into()
        }
    }

    pub fn status_end_MiiswordsmanCounterDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MiiswordsmanCounterDamage(self).into()
        }
    }

    pub fn status_pre_Ottotto(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Ottotto(self).into()
        }
    }

    pub fn sub_ottotto_common_pre(&mut self) {
        unsafe {
            sub_ottotto_common_pre(self)
        }
    }

    pub fn sub_ottotto_common(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ottotto_common(self).into()
        }
    }

    pub fn status_Ottotto(&mut self) -> lib::L2CValue {
        unsafe {
            status_Ottotto(self).into()
        }
    }

    pub fn status_Ottotto_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Ottotto_Main(self).into()
        }
    }

    pub fn status_OttottoWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_OttottoWait(self).into()
        }
    }

    pub fn status_Ottotto_Wait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Ottotto_Wait_Main(self).into()
        }
    }

    pub fn sub_ottotto_uniq_process_exec_fix_pos(&mut self) {
        unsafe {
            sub_ottotto_uniq_process_exec_fix_pos(self)
        }
    }

    pub fn sub_ottotto_uniq_process_main(&mut self) {
        unsafe {
            sub_ottotto_uniq_process_main(self)
        }
    }

    pub fn status_end_Ottotto(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Ottotto(self).into()
        }
    }

    pub fn sub_ottotto_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ottotto_uniq_process_init(self).into()
        }
    }

    pub fn sub_ottotto_uniq_process_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ottotto_uniq_process_exec_status(self).into()
        }
    }

    pub fn sub_ottotto_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ottotto_uniq_process_exit(self).into()
        }
    }

    pub fn status_pre_Passive(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Passive(self).into()
        }
    }

    pub fn status_Passive(&mut self) -> lib::L2CValue {
        unsafe {
            status_Passive(self).into()
        }
    }

    pub fn status_Passive_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Passive_Main(self).into()
        }
    }

    pub fn status_end_Passive(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Passive(self).into()
        }
    }

    pub fn status_pre_PassiveFB(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_PassiveFB(self).into()
        }
    }

    pub fn status_PassiveFB(&mut self) -> lib::L2CValue {
        unsafe {
            status_PassiveFB(self).into()
        }
    }

    pub fn status_PassiveFB_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_PassiveFB_Main(self).into()
        }
    }

    pub fn status_end_PassiveFB(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_PassiveFB(self).into()
        }
    }

    pub fn status_pre_PassiveWall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_PassiveWall(self).into()
        }
    }

    pub fn status_PassiveWall(&mut self) -> lib::L2CValue {
        unsafe {
            status_PassiveWall(self).into()
        }
    }

    pub fn PassiveWallUniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            PassiveWallUniq(self, arg1.into()).into()
        }
    }

    pub fn status_PassiveWall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_PassiveWall_Main(self).into()
        }
    }

    pub fn status_end_PassiveWall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_PassiveWall(self).into()
        }
    }

    pub fn status_pre_PassiveWallJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_PassiveWallJump(self).into()
        }
    }

    pub fn status_PassiveWallJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_PassiveWallJump(self).into()
        }
    }

    pub fn PassiveWallJumpUniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            PassiveWallJumpUniq(self, arg1.into()).into()
        }
    }

    pub fn status_PassiveWallJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_PassiveWallJump_Main(self).into()
        }
    }

    pub fn status_end_PassiveWallJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_PassiveWallJump(self).into()
        }
    }

    pub fn status_pre_PassiveCeil(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_PassiveCeil(self).into()
        }
    }

    pub fn status_PassiveCeil(&mut self) -> lib::L2CValue {
        unsafe {
            status_PassiveCeil(self).into()
        }
    }

    pub fn status_PassiveCeil_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_PassiveCeil_Main(self).into()
        }
    }

    pub fn status_end_PassiveCeil(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_PassiveCeil(self).into()
        }
    }

    pub fn FL_get_wall_lr(&mut self) -> lib::L2CValue {
        unsafe {
            FL_get_wall_lr(self).into()
        }
    }

    pub fn set_passive_effect(&mut self, arg1: lib::L2CValue) {
        unsafe {
            set_passive_effect(self, arg1.into())
        }
    }

    pub fn sub_uniq_process_Passive_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_Passive_init(self).into()
        }
    }

    pub fn sub_uniq_process_Passive_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_Passive_exec_stop(self).into()
        }
    }

    pub fn sub_uniq_process_Passive_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_Passive_exec_status(self).into()
        }
    }

    pub fn sub_uniq_process_Passive_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_Passive_exit_status(self).into()
        }
    }

    pub fn status_pre_Psychobreak(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Psychobreak(self).into()
        }
    }

    pub fn status_Psychobreak(&mut self) -> lib::L2CValue {
        unsafe {
            status_Psychobreak(self).into()
        }
    }

    pub fn status_Psychobreak_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Psychobreak_Main(self).into()
        }
    }

    pub fn status_end_Psychobreak(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Psychobreak(self).into()
        }
    }

    pub fn FighterStatusUniqProcessPsychobreak_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessPsychobreak_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_Rebirth(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Rebirth(self).into()
        }
    }

    pub fn status_Rebirth(&mut self) -> lib::L2CValue {
        unsafe {
            status_Rebirth(self).into()
        }
    }

    pub fn sub_rebirth_common_pre(&mut self) {
        unsafe {
            sub_rebirth_common_pre(self)
        }
    }

    pub fn status_Rebirth_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Rebirth_Main(self).into()
        }
    }

    pub fn sub_rebirth_common(&mut self) -> lib::L2CValue {
        unsafe {
            sub_rebirth_common(self).into()
        }
    }

    pub fn sub_wait_motion(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_wait_motion(self, arg1.into())
        }
    }

    pub fn sub_rebirth_uniq_process_exec_fix_pos(&mut self) {
        unsafe {
            sub_rebirth_uniq_process_exec_fix_pos(self)
        }
    }

    pub fn sub_update_effect(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_update_effect(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn sub_remove_effect(&mut self) {
        unsafe {
            sub_remove_effect(self)
        }
    }

    pub fn sub_attach_effect(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_attach_effect(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn sub_rebirth_uniq_check(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_rebirth_uniq_check(self, arg1.into()).into()
        }
    }

    pub fn sub_rebirth_uniq_process_main(&mut self) {
        unsafe {
            sub_rebirth_uniq_process_main(self)
        }
    }

    pub fn status_end_Rebirth_common(&mut self) {
        unsafe {
            status_end_Rebirth_common(self)
        }
    }

    pub fn status_end_Rebirth(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Rebirth(self).into()
        }
    }

    pub fn sub_rebirth_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_rebirth_uniq_process_init(self).into()
        }
    }

    pub fn sub_rebirth_uniq_process_init_core(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_rebirth_uniq_process_init_core(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn FighterStatusUniqProcessRebirth_init_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessRebirth_init_status(self)
        }
    }

    pub fn FighterStatusUniqProcessRebirth_check_discretion_final(&mut self) {
        unsafe {
            FighterStatusUniqProcessRebirth_check_discretion_final(self)
        }
    }

    pub fn FighterStatusUniqProcessRebirth_is_operation_cpu(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRebirth_is_operation_cpu(self).into()
        }
    }

    pub fn FighterStatusUniqProcessRebirth_is_item_appearance_off(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRebirth_is_item_appearance_off(self).into()
        }
    }

    pub fn FighterStatusUniqProcessRebirth_is_entry_count(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRebirth_is_entry_count(self).into()
        }
    }

    pub fn FighterStatusUniqProcessRebirth_is_bottom(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRebirth_is_bottom(self).into()
        }
    }

    pub fn sub_check_discretion_final(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_check_discretion_final(self, arg1.into())
        }
    }

    pub fn sub_end_move(&mut self) {
        unsafe {
            sub_end_move(self)
        }
    }

    pub fn sub_rebirth_uniq_process_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_rebirth_uniq_process_exec_status(self).into()
        }
    }

    pub fn sub_rebirth_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_rebirth_uniq_process_exit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessRebirth_is_final_used(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRebirth_is_final_used(self).into()
        }
    }

    pub fn FighterStatusUniqProcessRebirth_is_last_dead_suicide(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRebirth_is_last_dead_suicide(self, arg1.into()).into()
        }
    }

    pub fn status_pre_Win(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Win(self).into()
        }
    }

    pub fn status_Win(&mut self) -> lib::L2CValue {
        unsafe {
            status_Win(self).into()
        }
    }

    pub fn status_Win_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Win_Main(self).into()
        }
    }

    pub fn status_end_Win(&mut self) {
        unsafe {
            status_end_Win(self)
        }
    }

    pub fn status_pre_Lose(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Lose(self).into()
        }
    }

    pub fn status_Lose(&mut self) -> lib::L2CValue {
        unsafe {
            status_Lose(self).into()
        }
    }

    pub fn status_Lose_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Lose_Main(self).into()
        }
    }

    pub fn status_pre_Roulette(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Roulette(self).into()
        }
    }

    pub fn sub_roulette_set_step(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_roulette_set_step(self, arg1.into())
        }
    }

    pub fn sub_get_wait_motion(&mut self) -> lib::L2CValue {
        unsafe {
            sub_get_wait_motion(self).into()
        }
    }

    pub fn sub_Roulette(&mut self) {
        unsafe {
            sub_Roulette(self)
        }
    }

    pub fn status_Roulette(&mut self) -> lib::L2CValue {
        unsafe {
            status_Roulette(self).into()
        }
    }

    pub fn status_Roulette_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Roulette_Main(self).into()
        }
    }

    pub fn set_roulette_shoot_motion(&mut self) {
        unsafe {
            set_roulette_shoot_motion(self)
        }
    }

    pub fn status_end_Roulette(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Roulette(self).into()
        }
    }

    pub fn status_pre_RouletteFuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_RouletteFuraFura(self).into()
        }
    }

    pub fn status_RouletteFuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_RouletteFuraFura(self).into()
        }
    }

    pub fn status_RouletteFuraFura_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_RouletteFuraFura_Main(self).into()
        }
    }

    pub fn status_end_RouletteFuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_RouletteFuraFura(self).into()
        }
    }

    pub fn FighterStatusRouletteFuraFura_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusRouletteFuraFura_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_SavingDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SavingDamage(self).into()
        }
    }

    pub fn status_SavingDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_SavingDamage(self).into()
        }
    }

    pub fn status_SavingDamage_Main(&mut self) {
        unsafe {
            status_SavingDamage_Main(self)
        }
    }

    pub fn status_end_SavingDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SavingDamage(self).into()
        }
    }

    pub fn sub_saving_damage_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_saving_damage_uniq_process_main(self).into()
        }
    }

    pub fn status_pre_ShoulderedDonkeyStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ShoulderedDonkeyStart(self).into()
        }
    }

    pub fn status_ShoulderedDonkeyStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShoulderedDonkeyStart(self).into()
        }
    }

    pub fn status_ShoulderedDonkeyStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShoulderedDonkeyStart_Main(self).into()
        }
    }

    pub fn uniq_process_ShoulderedDonkey_main(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_ShoulderedDonkey_main(self).into()
        }
    }

    pub fn uniq_process_ShoulderedDonkey_exec_fix_pos_counter(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_ShoulderedDonkey_exec_fix_pos_counter(self).into()
        }
    }

    pub fn status_end_ShoulderedDonkeyStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ShoulderedDonkeyStart(self).into()
        }
    }

    pub fn status_pre_ShoulderedDonkey(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ShoulderedDonkey(self).into()
        }
    }

    pub fn sub_shouldered_donkey_uniq(&mut self) -> lib::L2CValue {
        unsafe {
            sub_shouldered_donkey_uniq(self).into()
        }
    }

    pub fn status_ShoulderedDonkey(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShoulderedDonkey(self).into()
        }
    }

    pub fn status_ShoulderedDonkey_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShoulderedDonkey_Main(self).into()
        }
    }

    pub fn status_end_ShoulderedDonkey(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ShoulderedDonkey(self).into()
        }
    }

    pub fn status_pre_ShoulderedDonkeyThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ShoulderedDonkeyThrown(self).into()
        }
    }

    pub fn status_ShoulderedDonkeyThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShoulderedDonkeyThrown(self).into()
        }
    }

    pub fn uniq_process_ShoulderedDonkeyThrown_main(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_ShoulderedDonkeyThrown_main(self).into()
        }
    }

    pub fn status_ShoulderedDonkeyThrown_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShoulderedDonkeyThrown_Main(self).into()
        }
    }

    pub fn status_end_ShoulderedDonkeyThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ShoulderedDonkeyThrown(self).into()
        }
    }

    pub fn uniq_process_ShoulderedDonkey_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_ShoulderedDonkey_init_status(self).into()
        }
    }

    pub fn uniq_process_ShoulderedDonkey_exec_fix_pos(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_ShoulderedDonkey_exec_fix_pos(self).into()
        }
    }

    pub fn uniq_process_ShoulderedDonkey_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_ShoulderedDonkey_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessShoulderedDonkey_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessShoulderedDonkey_check_damage(self, arg1.into()).into()
        }
    }

    pub fn uniq_process_ShoulderedDonkeyThrown_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_ShoulderedDonkeyThrown_init_status(self).into()
        }
    }

    pub fn uniq_process_ShoulderedDonkeyThrown_exec_fix_pos_counter(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_ShoulderedDonkeyThrown_exec_fix_pos_counter(self).into()
        }
    }

    pub fn uniq_process_ShoulderedDonkeyThrown_exec_fix_pos(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_ShoulderedDonkeyThrown_exec_fix_pos(self).into()
        }
    }

    pub fn uniq_process_ShoulderedDonkeyThrown_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            uniq_process_ShoulderedDonkeyThrown_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessShoulderedDonkeyThrown_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessShoulderedDonkeyThrown_check_damage(self, arg1.into()).into()
        }
    }

    pub fn sub_SlipCommon(&mut self) {
        unsafe {
            sub_SlipCommon(self)
        }
    }

    pub fn sub_SlipCommon_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_SlipCommon_Main(self).into()
        }
    }

    pub fn sub_SlipDownCommon(&mut self) {
        unsafe {
            sub_SlipDownCommon(self)
        }
    }

    pub fn sub_SlipDownCommon_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_SlipDownCommon_Main(self).into()
        }
    }

    pub fn sub_SlipStandCommon(&mut self) {
        unsafe {
            sub_SlipStandCommon(self)
        }
    }

    pub fn sub_SlipStandCommon_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_SlipStandCommon_Main(self).into()
        }
    }

    pub fn status_pre_Slip(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Slip(self).into()
        }
    }

    pub fn status_Slip(&mut self) -> lib::L2CValue {
        unsafe {
            status_Slip(self).into()
        }
    }

    pub fn status_Slip_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Slip_Main(self).into()
        }
    }

    pub fn status_end_Slip(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Slip(self).into()
        }
    }

    pub fn status_pre_SlipDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SlipDamage(self).into()
        }
    }

    pub fn status_SlipDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipDamage(self).into()
        }
    }

    pub fn status_SlipDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipDamage_Main(self).into()
        }
    }

    pub fn status_SlipWait_Common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipWait_Common_Main(self).into()
        }
    }

    pub fn status_end_SlipDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SlipDamage(self).into()
        }
    }

    pub fn status_pre_SlipWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SlipWait(self).into()
        }
    }

    pub fn status_SlipWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipWait(self).into()
        }
    }

    pub fn sub_SlipWaitUniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_SlipWaitUniq(self, arg1.into()).into()
        }
    }

    pub fn status_SlipWait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipWait_Main(self).into()
        }
    }

    pub fn status_end_SlipWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SlipWait(self).into()
        }
    }

    pub fn status_pre_SlipStand(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SlipStand(self).into()
        }
    }

    pub fn status_SlipStand(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipStand(self).into()
        }
    }

    pub fn status_SlipStand_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipStand_Main(self).into()
        }
    }

    pub fn status_end_SlipStand(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SlipStand(self).into()
        }
    }

    pub fn status_pre_SlipStandAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SlipStandAttack(self).into()
        }
    }

    pub fn status_SlipStandAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipStandAttack(self).into()
        }
    }

    pub fn status_SlipStandAttack_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipStandAttack_Main(self).into()
        }
    }

    pub fn status_end_SlipStandAttack(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SlipStandAttack(self).into()
        }
    }

    pub fn status_pre_SlipStandF(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SlipStandF(self).into()
        }
    }

    pub fn status_SlipStandF(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipStandF(self).into()
        }
    }

    pub fn status_SlipStandF_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipStandF_Main(self).into()
        }
    }

    pub fn status_end_SlipStandF(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SlipStandF(self).into()
        }
    }

    pub fn status_pre_SlipStandB(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SlipStandB(self).into()
        }
    }

    pub fn status_SlipStandB(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipStandB(self).into()
        }
    }

    pub fn status_SlipStandB_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SlipStandB_Main(self).into()
        }
    }

    pub fn status_end_SlipStandB(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SlipStandB(self).into()
        }
    }

    pub fn status_pre_Squat(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Squat(self).into()
        }
    }

    pub fn status_pre_Squat_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_Squat_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into()).into()
        }
    }

    pub fn status_pre_SquatWait_common_check_interrupt(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SquatWait_common_check_interrupt(self).into()
        }
    }

    pub fn status_pre_SquatWait_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_SquatWait_common(self, arg1.into()).into()
        }
    }

    pub fn status_pre_SquatWait_common_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_SquatWait_common_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into()).into()
        }
    }

    pub fn status_pre_SquatWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SquatWait(self).into()
        }
    }

    pub fn status_pre_SquatF(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SquatF(self).into()
        }
    }

    pub fn status_pre_SquatB(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SquatB(self).into()
        }
    }

    pub fn status_pre_SquatRv(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SquatRv(self).into()
        }
    }

    pub fn status_pre_SquatRv_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_SquatRv_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into()).into()
        }
    }

    pub fn status_Squat(&mut self) -> lib::L2CValue {
        unsafe {
            status_Squat(self).into()
        }
    }

    pub fn status_Squat_sub(&mut self) {
        unsafe {
            status_Squat_sub(self)
        }
    }

    pub fn status_Squat_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Squat_Main(self).into()
        }
    }

    pub fn sub_squat_common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_squat_common_Main(self).into()
        }
    }

    pub fn status_Squat_sub_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            status_Squat_sub_param(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn sub_squat_common_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            sub_squat_common_param(self, arg1.into(), arg2.into())
        }
    }

    pub fn status_SquatWait_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_SquatWait_common(self, arg1.into())
        }
    }

    pub fn status_SquatWait_common_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            status_SquatWait_common_param(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn sub_squat_common(&mut self) {
        unsafe {
            sub_squat_common(self)
        }
    }

    pub fn sub_squat_wait_mtrans_loop_param(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_squat_wait_mtrans_loop_param(self, arg1.into())
        }
    }

    pub fn sub_squat_check_front_pre(&mut self) {
        unsafe {
            sub_squat_check_front_pre(self)
        }
    }

    pub fn sub_squat_check_back_pre(&mut self) {
        unsafe {
            sub_squat_check_back_pre(self)
        }
    }

    pub fn status_SquatWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_SquatWait(self).into()
        }
    }

    pub fn status_SquatWait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SquatWait_Main(self).into()
        }
    }

    pub fn sub_squat_check_front(&mut self) -> lib::L2CValue {
        unsafe {
            sub_squat_check_front(self).into()
        }
    }

    pub fn sub_squat_check_back(&mut self) -> lib::L2CValue {
        unsafe {
            sub_squat_check_back(self).into()
        }
    }

    pub fn sub_squat_wait_mtrans_conditions(&mut self) -> lib::L2CValue {
        unsafe {
            sub_squat_wait_mtrans_conditions(self).into()
        }
    }

    pub fn sub_squat_wait_mtrans_loop(&mut self) {
        unsafe {
            sub_squat_wait_mtrans_loop(self)
        }
    }

    pub fn status_SquatF(&mut self) -> lib::L2CValue {
        unsafe {
            status_SquatF(self).into()
        }
    }

    pub fn status_SquatF_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SquatF_Main(self).into()
        }
    }

    pub fn status_SquatB(&mut self) -> lib::L2CValue {
        unsafe {
            status_SquatB(self).into()
        }
    }

    pub fn status_SquatB_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SquatB_Main(self).into()
        }
    }

    pub fn status_SquatRv_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            status_SquatRv_param(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn SquatRvUniq(&mut self) -> lib::L2CValue {
        unsafe {
            SquatRvUniq(self).into()
        }
    }

    pub fn status_SquatRv(&mut self) -> lib::L2CValue {
        unsafe {
            status_SquatRv(self).into()
        }
    }

    pub fn status_SquatRv_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SquatRv_Main(self).into()
        }
    }

    pub fn status_end_Squat(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Squat(self).into()
        }
    }

    pub fn status_end_SquatWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SquatWait(self).into()
        }
    }

    pub fn status_end_SquatF(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SquatF(self).into()
        }
    }

    pub fn status_end_SquatB(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SquatB(self).into()
        }
    }

    pub fn status_end_SquatRv(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SquatRv(self).into()
        }
    }

    pub fn sub_squat_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_squat_uniq_process_init(self).into()
        }
    }

    pub fn sub_squat_uniq_process_init_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            sub_squat_uniq_process_init_param(self, arg1.into(), arg2.into())
        }
    }

    pub fn sub_squat_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_squat_uniq_process_main(self).into()
        }
    }

    pub fn sub_squat_uniq_process_main_param(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_squat_uniq_process_main_param(self, arg1.into())
        }
    }

    pub fn sub_squat_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_squat_uniq_process_exit(self).into()
        }
    }

    pub fn sub_squat_walk_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_squat_walk_uniq_process_init(self).into()
        }
    }

    pub fn FL_get_squat_walk_max_speed(&mut self) -> lib::L2CValue {
        unsafe {
            FL_get_squat_walk_max_speed(self).into()
        }
    }

    pub fn get_motion_kind(&mut self) -> lib::L2CValue {
        unsafe {
            get_motion_kind(self).into()
        }
    }

    pub fn sub_squat_walk_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_squat_walk_uniq_process_main(self).into()
        }
    }

    pub fn FL_get_stick_x_rate(&mut self) -> lib::L2CValue {
        unsafe {
            FL_get_stick_x_rate(self).into()
        }
    }

    pub fn set_squat_walk_motion_rate(&mut self) {
        unsafe {
            set_squat_walk_motion_rate(self)
        }
    }

    pub fn sub_squat_walk_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_squat_walk_uniq_process_exit(self).into()
        }
    }

    pub fn sub_squat_walk_uniq_process_calc_param(&mut self) -> lib::L2CValue {
        unsafe {
            sub_squat_walk_uniq_process_calc_param(self).into()
        }
    }

    pub fn status_pre_StabbedDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_StabbedDamage(self).into()
        }
    }

    pub fn status_StabbedDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_StabbedDamage(self).into()
        }
    }

    pub fn status_end_StabbedDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_StabbedDamage(self).into()
        }
    }

    pub fn sub_stabbed_damage_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_stabbed_damage_uniq_process_init(self).into()
        }
    }

    pub fn sub_stabbed_damage_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_stabbed_damage_uniq_process_main(self).into()
        }
    }

    pub fn status_pre_StabbedRidley(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_StabbedRidley(self).into()
        }
    }

    pub fn status_StabbedRidley(&mut self) -> lib::L2CValue {
        unsafe {
            status_StabbedRidley(self).into()
        }
    }

    pub fn status_StabbedRidley_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_StabbedRidley_Main(self).into()
        }
    }

    pub fn status_end_StabbedRidley(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_StabbedRidley(self).into()
        }
    }

    pub fn sub_uniq_process_StabbedRidley_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_StabbedRidley_init(self).into()
        }
    }

    pub fn sub_uniq_process_StabbedRidley_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_StabbedRidley_exec(self).into()
        }
    }

    pub fn sub_uniq_process_StabbedRidley_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_StabbedRidley_exit(self).into()
        }
    }

    pub fn FighterStatusUniqProcessStabbedRidley_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessStabbedRidley_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_StopWall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_StopWall(self).into()
        }
    }

    pub fn status_StopWall(&mut self) -> lib::L2CValue {
        unsafe {
            status_StopWall(self).into()
        }
    }

    pub fn status_StopWall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_StopWall_Main(self).into()
        }
    }

    pub fn status_end_StopWall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_StopWall(self).into()
        }
    }

    pub fn sub_ground_check_stop_wall(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ground_check_stop_wall(self).into()
        }
    }

    pub fn status_pre_StopCeil(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_StopCeil(self).into()
        }
    }

    pub fn status_StopCeil(&mut self) -> lib::L2CValue {
        unsafe {
            status_StopCeil(self).into()
        }
    }

    pub fn sub_StopCeil_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_StopCeil_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_StopCeil_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_StopCeil_Main(self).into()
        }
    }

    pub fn status_end_StopCeil(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_StopCeil(self).into()
        }
    }

    pub fn super_jump_punch_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            super_jump_punch_uniq(self, arg1.into()).into()
        }
    }

    pub fn super_jump_punch(&mut self, arg1: lib::L2CValue) {
        unsafe {
            super_jump_punch(self, arg1.into())
        }
    }

    pub fn super_jump_punch_main(&mut self) {
        unsafe {
            super_jump_punch_main(self)
        }
    }

    pub fn super_jump_punch_reset_common_condition(&mut self) -> lib::L2CValue {
        unsafe {
            super_jump_punch_reset_common_condition(self).into()
        }
    }

    pub fn super_jump_punch_end(&mut self, arg1: lib::L2CValue) {
        unsafe {
            super_jump_punch_end(self, arg1.into())
        }
    }

    pub fn status_pre_Swallowed(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Swallowed(self).into()
        }
    }

    pub fn status_Swallowed(&mut self) -> lib::L2CValue {
        unsafe {
            status_Swallowed(self).into()
        }
    }

    pub fn status_Swallowed_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Swallowed_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowed_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessSwallowed_exec_fix_pos(self)
        }
    }

    pub fn status_end_Swallowed(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Swallowed(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowed_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowed_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowed_getOffset(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowed_getOffset(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowed_getOffsetKind(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowed_getOffsetKind(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowed_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowed_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowed_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowed_exec_stop(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowed_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowed_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowed_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowed_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_SwallowedDrink(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwallowedDrink(self).into()
        }
    }

    pub fn status_SwallowedDrink(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwallowedDrink(self).into()
        }
    }

    pub fn status_SwallowedDrink_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwallowedDrink_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedDrink_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessSwallowedDrink_exec_fix_pos(self)
        }
    }

    pub fn status_end_SwallowedDrink(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwallowedDrink(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedDrink_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedDrink_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedDrink_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedDrink_exit_status(self).into()
        }
    }

    pub fn status_pre_SwallowedCapture(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwallowedCapture(self).into()
        }
    }

    pub fn status_SwallowedCapture(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwallowedCapture(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedCapture_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessSwallowedCapture_exec_fix_pos(self)
        }
    }

    pub fn status_SwallowedCapture_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwallowedCapture_Main(self).into()
        }
    }

    pub fn status_end_SwallowedCapture(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwallowedCapture(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedCapture_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedCapture_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedCapture_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedCapture_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedCapture_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedCapture_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedCapture_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedCapture_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_SwallowedThrownStar(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwallowedThrownStar(self).into()
        }
    }

    pub fn status_SwallowedThrownStar(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwallowedThrownStar(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedThrownStar_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessSwallowedThrownStar_exec_fix_pos(self)
        }
    }

    pub fn status_SwallowedThrownStar_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwallowedThrownStar_Main(self).into()
        }
    }

    pub fn status_end_SwallowedThrownStar(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwallowedThrownStar(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedThrownStar_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedThrownStar_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedThrownStar_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedThrownStar_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedThrownStar_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedThrownStar_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedThrownStar_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedThrownStar_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_SwallowedThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwallowedThrown(self).into()
        }
    }

    pub fn status_SwallowedThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwallowedThrown(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedThrown_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessSwallowedThrown_exec_fix_pos(self)
        }
    }

    pub fn status_SwallowedThrown_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwallowedThrown_Main(self).into()
        }
    }

    pub fn status_end_SwallowedThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwallowedThrown(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedThrown_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedThrown_init_status(self).into()
        }
    }

    pub fn status_pre_SwallowedCancel(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwallowedCancel(self).into()
        }
    }

    pub fn status_SwallowedCancel(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwallowedCancel(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedCancel_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessSwallowedCancel_exec_fix_pos(self)
        }
    }

    pub fn status_SwallowedCancel_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwallowedCancel_Main(self).into()
        }
    }

    pub fn status_end_SwallowedCancel(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwallowedCancel(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedCancel_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedCancel_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwallowedCancel_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwallowedCancel_exit_status(self).into()
        }
    }

    pub fn status_pre_SwimDive(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwimDive(self).into()
        }
    }

    pub fn status_SwimDive(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimDive(self).into()
        }
    }

    pub fn sub_SwimDropItemChk(&mut self) {
        unsafe {
            sub_SwimDropItemChk(self)
        }
    }

    pub fn status_SwimDive_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimDive_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwim_exec_fix_pos_counter(&mut self) {
        unsafe {
            FighterStatusUniqProcessSwim_exec_fix_pos_counter(self)
        }
    }

    pub fn FighterStatusUniqProcessSwim_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessSwim_exec_fix_pos(self)
        }
    }

    pub fn FighterStatusUniqProcessSwim_get_water_surface_offset_pos(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwim_get_water_surface_offset_pos(self, arg1.into()).into()
        }
    }

    pub fn status_end_SwimDive(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwimDive(self).into()
        }
    }

    pub fn status_pre_SwimRise(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwimRise(self).into()
        }
    }

    pub fn status_SwimRise(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimRise(self).into()
        }
    }

    pub fn status_SwimRise_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimRise_Main(self).into()
        }
    }

    pub fn status_end_SwimRise(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwimRise(self).into()
        }
    }

    pub fn status_pre_SwimUp(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwimUp(self).into()
        }
    }

    pub fn status_SwimUp(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimUp(self).into()
        }
    }

    pub fn status_SwimUp_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimUp_Main(self).into()
        }
    }

    pub fn status_end_SwimUp(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwimUp(self).into()
        }
    }

    pub fn status_pre_SwimWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwimWait(self).into()
        }
    }

    pub fn status_SwimWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimWait(self).into()
        }
    }

    pub fn sub_pre_SwimDrownChk(&mut self) {
        unsafe {
            sub_pre_SwimDrownChk(self)
        }
    }

    pub fn status_SwimWait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimWait_Main(self).into()
        }
    }

    pub fn sub_SwimJumpChk(&mut self) -> lib::L2CValue {
        unsafe {
            sub_SwimJumpChk(self).into()
        }
    }

    pub fn sub_SwimDrownChk(&mut self) -> lib::L2CValue {
        unsafe {
            sub_SwimDrownChk(self).into()
        }
    }

    pub fn status_end_SwimWait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwimWait(self).into()
        }
    }

    pub fn status_pre_Swim(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Swim(self).into()
        }
    }

    pub fn status_Swim(&mut self) -> lib::L2CValue {
        unsafe {
            status_Swim(self).into()
        }
    }

    pub fn status_Swim_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Swim_Main(self).into()
        }
    }

    pub fn status_end_Swim(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Swim(self).into()
        }
    }

    pub fn status_pre_SwimEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwimEnd(self).into()
        }
    }

    pub fn status_SwimEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimEnd(self).into()
        }
    }

    pub fn status_SwimEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimEnd_Main(self).into()
        }
    }

    pub fn status_end_SwimEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwimEnd(self).into()
        }
    }

    pub fn status_pre_SwimTurn(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwimTurn(self).into()
        }
    }

    pub fn status_SwimTurn(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimTurn(self).into()
        }
    }

    pub fn status_SwimTurn_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimTurn_Main(self).into()
        }
    }

    pub fn status_end_SwimTurn(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwimTurn(self).into()
        }
    }

    pub fn status_pre_SwimJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwimJump(self).into()
        }
    }

    pub fn status_SwimJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimJump(self).into()
        }
    }

    pub fn status_SwimJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimJump_Main(self).into()
        }
    }

    pub fn sub_SwimJumpUniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_SwimJumpUniq(self, arg1.into()).into()
        }
    }

    pub fn status_end_SwimJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwimJump(self).into()
        }
    }

    pub fn status_pre_SwimDrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwimDrown(self).into()
        }
    }

    pub fn status_SwimDrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimDrown(self).into()
        }
    }

    pub fn status_SwimDrown_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimDrown_Main(self).into()
        }
    }

    pub fn sub_SwimDrownUniq(&mut self) -> lib::L2CValue {
        unsafe {
            sub_SwimDrownUniq(self).into()
        }
    }

    pub fn status_end_SwimDrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwimDrown(self).into()
        }
    }

    pub fn status_pre_SwimDrownOut(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwimDrownOut(self).into()
        }
    }

    pub fn status_SwimDrownOut(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimDrownOut(self).into()
        }
    }

    pub fn status_SwimDrownOut_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwimDrownOut_Main(self).into()
        }
    }

    pub fn status_end_SwimDrownOut(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwimDrownOut(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwim_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwim_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwim_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwim_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwim_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwim_exit_status(self).into()
        }
    }

    pub fn status_pre_SwingGaogaenCatched(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwingGaogaenCatched(self).into()
        }
    }

    pub fn status_SwingGaogaenCatched(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenCatched(self).into()
        }
    }

    pub fn set_swing_special_state_SwingGaogaen(&mut self) {
        unsafe {
            set_swing_special_state_SwingGaogaen(self)
        }
    }

    pub fn status_SwingGaogaenCatched_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenCatched_Main(self).into()
        }
    }

    pub fn status_end_SwingGaogaenCatched(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwingGaogaenCatched(self).into()
        }
    }

    pub fn FighterStatusCapture_set_invalid_capture_SwingGaogaen(&mut self) {
        unsafe {
            FighterStatusCapture_set_invalid_capture_SwingGaogaen(self)
        }
    }

    pub fn status_pre_SwingGaogaenThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwingGaogaenThrown(self).into()
        }
    }

    pub fn status_SwingGaogaenThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenThrown(self).into()
        }
    }

    pub fn sub_get_rope_run_motion(&mut self) -> lib::L2CValue {
        unsafe {
            sub_get_rope_run_motion(self).into()
        }
    }

    pub fn status_SwingGaogaenThrown_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenThrown_Main(self).into()
        }
    }

    pub fn status_end_SwingGaogaenThrown(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwingGaogaenThrown(self).into()
        }
    }

    pub fn status_pre_SwingGaogaenAttachRope(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwingGaogaenAttachRope(self).into()
        }
    }

    pub fn status_SwingGaogaenAttachRope(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenAttachRope(self).into()
        }
    }

    pub fn status_SwingGaogaenAttachRope_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenAttachRope_Main(self).into()
        }
    }

    pub fn status_end_SwingGaogaenAttachRope(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwingGaogaenAttachRope(self).into()
        }
    }

    pub fn status_pre_SwingGaogaenReturn(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwingGaogaenReturn(self).into()
        }
    }

    pub fn status_SwingGaogaenReturn(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenReturn(self).into()
        }
    }

    pub fn status_SwingGaogaenReturn_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenReturn_Main(self).into()
        }
    }

    pub fn status_end_SwingGaogaenReturn(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwingGaogaenReturn(self).into()
        }
    }

    pub fn status_pre_SwingGaogaenLariat(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwingGaogaenLariat(self).into()
        }
    }

    pub fn status_SwingGaogaenLariat(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenLariat(self).into()
        }
    }

    pub fn status_SwingGaogaenLariat_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenLariat_Main(self).into()
        }
    }

    pub fn status_end_SwingGaogaenLariat(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwingGaogaenLariat(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwingGaogaenLariat_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwingGaogaenLariat_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_SwingGaogaenShoulder(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwingGaogaenShoulder(self).into()
        }
    }

    pub fn status_SwingGaogaenShoulder(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenShoulder(self).into()
        }
    }

    pub fn status_SwingGaogaenShoulder_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenShoulder_Main(self).into()
        }
    }

    pub fn status_end_SwingGaogaenShoulder(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwingGaogaenShoulder(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwingGaogaenShoulder_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwingGaogaenShoulder_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_SwingGaogaenFailure(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SwingGaogaenFailure(self).into()
        }
    }

    pub fn status_SwingGaogaenFailure(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenFailure(self).into()
        }
    }

    pub fn status_SwingGaogaenFailure_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SwingGaogaenFailure_Main(self).into()
        }
    }

    pub fn status_end_SwingGaogaenFailure(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SwingGaogaenFailure(self).into()
        }
    }

    pub fn FighterStatusUniqProcessSwingGaogaenFailure_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessSwingGaogaenFailure_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_ThrowKirby(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ThrowKirby(self).into()
        }
    }

    pub fn status_ThrowKirby(&mut self) -> lib::L2CValue {
        unsafe {
            status_ThrowKirby(self).into()
        }
    }

    pub fn status_ThrowKirby_Uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_ThrowKirby_Uniq(self, arg1.into()).into()
        }
    }

    pub fn status_ThrowKirby_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ThrowKirby_Main(self).into()
        }
    }

    pub fn sub_status_uniq_process_ThrowKirby_execFixPos(&mut self) {
        unsafe {
            sub_status_uniq_process_ThrowKirby_execFixPos(self)
        }
    }

    pub fn status_end_ThrowKirby(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ThrowKirby(self).into()
        }
    }

    pub fn sub_status_uniq_process_ThrowKirby_initStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_status_uniq_process_ThrowKirby_initStatus(self).into()
        }
    }

    pub fn sub_status_uniq_process_ThrowKirby_exitStatus(&mut self) -> lib::L2CValue {
        unsafe {
            sub_status_uniq_process_ThrowKirby_exitStatus(self).into()
        }
    }

    pub fn status_pre_TreadDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TreadDamage(self).into()
        }
    }

    pub fn status_TreadDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_TreadDamage(self).into()
        }
    }

    pub fn status_TreadDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TreadDamage_Main(self).into()
        }
    }

    pub fn status_end_TreadDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TreadDamage(self).into()
        }
    }

    pub fn sub_tread_damage_unique_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_tread_damage_unique_process_exit(self).into()
        }
    }

    pub fn status_pre_TreadDamageAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TreadDamageAir(self).into()
        }
    }

    pub fn status_TreadDamageAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_TreadDamageAir(self).into()
        }
    }

    pub fn status_TreadDamageAir_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TreadDamageAir_Main(self).into()
        }
    }

    pub fn status_end_TreadDamageAir(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TreadDamageAir(self).into()
        }
    }

    pub fn sub_tread_damage_air_unique_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_tread_damage_air_unique_process_exit(self).into()
        }
    }

    pub fn status_pre_TreadDamageRv(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TreadDamageRv(self).into()
        }
    }

    pub fn status_TreadDamageRv(&mut self) -> lib::L2CValue {
        unsafe {
            status_TreadDamageRv(self).into()
        }
    }

    pub fn status_TreadDamageRv_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TreadDamageRv_Main(self).into()
        }
    }

    pub fn status_end_TreadDamageRv(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TreadDamageRv(self).into()
        }
    }

    pub fn status_pre_TreadFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TreadFall(self).into()
        }
    }

    pub fn sub_tread_fall_uniq_check(&mut self) -> lib::L2CValue {
        unsafe {
            sub_tread_fall_uniq_check(self).into()
        }
    }

    pub fn sub_TreadFallPassive(&mut self) {
        unsafe {
            sub_TreadFallPassive(self)
        }
    }

    pub fn status_TreadFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_TreadFall(self).into()
        }
    }

    pub fn status_TreadFall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TreadFall_Main(self).into()
        }
    }

    pub fn status_end_TreadFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TreadFall(self).into()
        }
    }

    pub fn sub_tread_fall_unique_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_tread_fall_unique_process_init(self).into()
        }
    }

    pub fn sub_tread_fall_unique_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_tread_fall_unique_process_exec(self).into()
        }
    }

    pub fn status_pre_TreadJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TreadJump(self).into()
        }
    }

    pub fn sub_tread_jump_uniq_check(&mut self) -> lib::L2CValue {
        unsafe {
            sub_tread_jump_uniq_check(self).into()
        }
    }

    pub fn status_TreadJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_TreadJump(self).into()
        }
    }

    pub fn sub_tread_jump_unique_process_init_inner(&mut self) {
        unsafe {
            sub_tread_jump_unique_process_init_inner(self)
        }
    }

    pub fn status_TreadJump_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TreadJump_Main(self).into()
        }
    }

    pub fn status_end_TreadJump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TreadJump(self).into()
        }
    }

    pub fn sub_tread_jump_unique_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_tread_jump_unique_process_init(self).into()
        }
    }

    pub fn sub_tread_jump_unique_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_tread_jump_unique_process_exit(self).into()
        }
    }

    pub fn status_pre_Wait_check_interrupt(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Wait_check_interrupt(self).into()
        }
    }

    pub fn status_pre_Wait_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Wait_main(self).into()
        }
    }

    pub fn status_pre_Wait_main_param(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_Wait_main_param(self, arg1.into()).into()
        }
    }

    pub fn status_pre_Wait(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Wait(self).into()
        }
    }

    pub fn status_Wait(&mut self) -> lib::L2CValue {
        unsafe {
            status_Wait(self).into()
        }
    }

    pub fn status_Wait_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Wait_Main(self).into()
        }
    }

    pub fn sub_wait_common_Main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_wait_common_Main(self).into()
        }
    }

    pub fn sub_ground_check_ottotto(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ground_check_ottotto(self).into()
        }
    }

    pub fn status_end_Wait(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Wait(self).into()
        }
    }

    pub fn status_pre_Pass(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Pass(self).into()
        }
    }

    pub fn status_Pass_common(&mut self) {
        unsafe {
            status_Pass_common(self)
        }
    }

    pub fn status_Pass(&mut self) -> lib::L2CValue {
        unsafe {
            status_Pass(self).into()
        }
    }

    pub fn status_Pass_Main_sub(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_Pass_Main_sub(self, arg1.into()).into()
        }
    }

    pub fn end_pass_ground(&mut self) -> lib::L2CValue {
        unsafe {
            end_pass_ground(self).into()
        }
    }

    pub fn status_Pass_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Pass_Main(self).into()
        }
    }

    pub fn status_end_Pass(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Pass(self).into()
        }
    }

    pub fn sub_uniq_process_Pass_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_Pass_init(self).into()
        }
    }

    pub fn sub_uniq_process_Pass_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_Pass_exec_status(self).into()
        }
    }

    pub fn status_pre_Appeal_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_pre_Appeal_common(self, arg1.into())
        }
    }

    pub fn status_pre_Appeal(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Appeal(self).into()
        }
    }

    pub fn status_Appeal_Common(&mut self) {
        unsafe {
            status_Appeal_Common(self)
        }
    }

    pub fn status_Appeal_common_uniq(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_Appeal_common_uniq(self, arg1.into())
        }
    }

    pub fn status_Appeal(&mut self) -> lib::L2CValue {
        unsafe {
            status_Appeal(self).into()
        }
    }

    pub fn status_Appeal_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Appeal_Main(self).into()
        }
    }

    pub fn status_end_Appeal(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Appeal(self).into()
        }
    }

    pub fn status_pre_SmashAppeal(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SmashAppeal(self).into()
        }
    }

    pub fn status_SmashAppeal(&mut self) -> lib::L2CValue {
        unsafe {
            status_SmashAppeal(self).into()
        }
    }

    pub fn sub_ground_check_ottotto_motion_end(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ground_check_ottotto_motion_end(self).into()
        }
    }

    pub fn status_pre_MetamonOut(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_MetamonOut(self).into()
        }
    }

    pub fn status_MetamonOut(&mut self) -> lib::L2CValue {
        unsafe {
            status_MetamonOut(self).into()
        }
    }

    pub fn status_MetamonOut_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_MetamonOut_Main(self).into()
        }
    }

    pub fn status_end_MetamonOut(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MetamonOut(self).into()
        }
    }

    pub fn status_pre_SuicideBomb(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SuicideBomb(self).into()
        }
    }

    pub fn status_SuicideBomb(&mut self) -> lib::L2CValue {
        unsafe {
            status_SuicideBomb(self).into()
        }
    }

    pub fn status_SuicideBomb_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SuicideBomb_Main(self).into()
        }
    }

    pub fn status_end_SuicideBomb(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SuicideBomb(self).into()
        }
    }

    pub fn check_turn_attack_s4_pad_rev(&mut self) {
        unsafe {
            check_turn_attack_s4_pad_rev(self)
        }
    }

    pub fn status_pre_Walk(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Walk(self).into()
        }
    }

    pub fn sub_pre_Walk(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) {
        unsafe {
            sub_pre_Walk(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into())
        }
    }

    pub fn sub_pre_Walk_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue, arg8: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_pre_Walk_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into(), arg8.into()).into()
        }
    }

    pub fn status_pre_WalkBrake(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_WalkBrake(self).into()
        }
    }

    pub fn status_pre_WalkBrake_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_WalkBrake_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into()).into()
        }
    }

    pub fn status_pre_DashCommon(&mut self) {
        unsafe {
            status_pre_DashCommon(self)
        }
    }

    pub fn status_pre_Dash(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Dash(self).into()
        }
    }

    pub fn status_pre_Run(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Run(self).into()
        }
    }

    pub fn status_pre_RunBrake(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_RunBrake(self).into()
        }
    }

    pub fn status_pre_TurnCommon(&mut self) {
        unsafe {
            status_pre_TurnCommon(self)
        }
    }

    pub fn status_TurnCommon(&mut self) -> lib::L2CValue {
        unsafe {
            status_TurnCommon(self).into()
        }
    }

    pub fn status_pre_Turn_Common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            status_pre_Turn_Common(self, arg1.into())
        }
    }

    pub fn status_pre_Turn(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_Turn(self).into()
        }
    }

    pub fn status_pre_TurnDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TurnDash(self).into()
        }
    }

    pub fn status_pre_TurnRun(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TurnRun(self).into()
        }
    }

    pub fn status_pre_TurnRunBrake(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_TurnRunBrake(self).into()
        }
    }

    pub fn status_Walk_Common(&mut self) {
        unsafe {
            status_Walk_Common(self)
        }
    }

    pub fn status_Walk(&mut self) -> lib::L2CValue {
        unsafe {
            status_Walk(self).into()
        }
    }

    pub fn sub_walk_uniq_check(&mut self) -> lib::L2CValue {
        unsafe {
            sub_walk_uniq_check(self).into()
        }
    }

    pub fn status_Walk_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Walk_Main(self).into()
        }
    }

    pub fn status_Walk_Main_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_Walk_Main_common(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn reset_walk_speed_ratio(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue) {
        unsafe {
            reset_walk_speed_ratio(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into())
        }
    }

    pub fn FighterStatusUniqProcessWalk_calc_param(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessWalk_calc_param(self).into()
        }
    }

    pub fn status_WalkBrake_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) {
        unsafe {
            status_WalkBrake_common(self, arg1.into(), arg2.into(), arg3.into(), arg4.into())
        }
    }

    pub fn status_WalkBrake(&mut self) -> lib::L2CValue {
        unsafe {
            status_WalkBrake(self).into()
        }
    }

    pub fn status_WalkBrake_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_WalkBrake_Main(self).into()
        }
    }

    pub fn status_Dash(&mut self) -> lib::L2CValue {
        unsafe {
            status_Dash(self).into()
        }
    }

    pub fn status_Dash_Sub(&mut self) {
        unsafe {
            status_Dash_Sub(self)
        }
    }

    pub fn status_Dash_Main_common(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_Dash_Main_common(self, arg1.into()).into()
        }
    }

    pub fn status_DashCommon(&mut self) {
        unsafe {
            status_DashCommon(self)
        }
    }

    pub fn status_Dash_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Dash_Main(self).into()
        }
    }

    pub fn status_Run_Sub(&mut self) {
        unsafe {
            status_Run_Sub(self)
        }
    }

    pub fn status_Run(&mut self) -> lib::L2CValue {
        unsafe {
            status_Run(self).into()
        }
    }

    pub fn status_Run_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Run_Main(self).into()
        }
    }

    pub fn check_run_brake_l_frame(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            check_run_brake_l_frame(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessRun_calc_param(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRun_calc_param(self).into()
        }
    }

    pub fn sub_status_RunBrake(&mut self) {
        unsafe {
            sub_status_RunBrake(self)
        }
    }

    pub fn sub_run_brake_uniq_check(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_run_brake_uniq_check(self, arg1.into()).into()
        }
    }

    pub fn status_RunBrake(&mut self) -> lib::L2CValue {
        unsafe {
            status_RunBrake(self).into()
        }
    }

    pub fn status_RunBrake_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_RunBrake_Main(self).into()
        }
    }

    pub fn status_Turn(&mut self) -> lib::L2CValue {
        unsafe {
            status_Turn(self).into()
        }
    }

    pub fn status_Turn_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Turn_Main(self).into()
        }
    }

    pub fn status_TurnDash_Sub(&mut self) {
        unsafe {
            status_TurnDash_Sub(self)
        }
    }

    pub fn status_TurnDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_TurnDash(self).into()
        }
    }

    pub fn status_TurnDash_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TurnDash_Main(self).into()
        }
    }

    pub fn status_TurnRun(&mut self) -> lib::L2CValue {
        unsafe {
            status_TurnRun(self).into()
        }
    }

    pub fn status_TurnRun_Sub(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            status_TurnRun_Sub(self, arg1.into(), arg2.into())
        }
    }

    pub fn status_TurnRun_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TurnRun_Main(self).into()
        }
    }

    pub fn sub_turn_run_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_turn_run_uniq(self, arg1.into()).into()
        }
    }

    pub fn sub_turn_run_uniq_main(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            sub_turn_run_uniq_main(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn status_TurnRunBrake_Sub(&mut self) {
        unsafe {
            status_TurnRunBrake_Sub(self)
        }
    }

    pub fn status_TurnRunBrake(&mut self) -> lib::L2CValue {
        unsafe {
            status_TurnRunBrake(self).into()
        }
    }

    pub fn status_TurnRunBrake_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_TurnRunBrake_Main(self).into()
        }
    }

    pub fn status_end_Walk(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Walk(self).into()
        }
    }

    pub fn status_end_WalkBrake(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_WalkBrake(self).into()
        }
    }

    pub fn status_end_Dash(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Dash(self).into()
        }
    }

    pub fn status_end_Run(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Run(self).into()
        }
    }

    pub fn status_end_RunBrake(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_RunBrake(self).into()
        }
    }

    pub fn status_end_Turn(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_Turn(self).into()
        }
    }

    pub fn sub_exit_Turn(&mut self) {
        unsafe {
            sub_exit_Turn(self)
        }
    }

    pub fn status_end_TurnDash(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TurnDash(self).into()
        }
    }

    pub fn status_end_TurnRun(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TurnRun(self).into()
        }
    }

    pub fn status_end_TurnRunBrake(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_TurnRunBrake(self).into()
        }
    }

    pub fn sub_dash_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_dash_uniq_process_init(self).into()
        }
    }

    pub fn sub_dash_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_dash_uniq_process_main(self).into()
        }
    }

    pub fn sub_dash_uniq_process_main_internal(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_dash_uniq_process_main_internal(self, arg1.into())
        }
    }

    pub fn sub_dash_uniq_process_exit_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_dash_uniq_process_exit_common(self, arg1.into())
        }
    }

    pub fn sub_dash_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_dash_uniq_process_exit(self).into()
        }
    }

    pub fn run_set_motion_rate(&mut self) {
        unsafe {
            run_set_motion_rate(self)
        }
    }

    pub fn sub_run_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_run_uniq_process_init(self).into()
        }
    }

    pub fn sub_run_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_run_uniq_process_main(self).into()
        }
    }

    pub fn calc_jostle_speed_attenuation_rate(&mut self) {
        unsafe {
            calc_jostle_speed_attenuation_rate(self)
        }
    }

    pub fn sub_run_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_run_uniq_process_exit(self).into()
        }
    }

    pub fn sub_run_uniq_process_on_change_lr(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_run_uniq_process_on_change_lr(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn sub_run_brake_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_run_brake_uniq_process_init(self).into()
        }
    }

    pub fn sub_run_brake_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_run_brake_uniq_process_main(self).into()
        }
    }

    pub fn sub_run_brake_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_run_brake_uniq_process_exit(self).into()
        }
    }

    pub fn sub_turn_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_turn_uniq_process_init(self).into()
        }
    }

    pub fn sub_turn_dash_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_turn_dash_uniq_process_init(self).into()
        }
    }

    pub fn sub_turn_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_turn_uniq_process_main(self).into()
        }
    }

    pub fn sub_turn_dash_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_turn_dash_uniq_process_main(self).into()
        }
    }

    pub fn sub_turn_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_turn_uniq_process_exit(self).into()
        }
    }

    pub fn sub_turn_dash_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_turn_dash_uniq_process_exit(self).into()
        }
    }

    pub fn sub_turn_run_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_turn_run_uniq_process_init(self).into()
        }
    }

    pub fn sub_turn_run_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_turn_run_uniq_process_main(self).into()
        }
    }

    pub fn sub_turn_run_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_turn_run_uniq_process_exit(self).into()
        }
    }

    pub fn init_walk_motion_middle_fast_blend(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) {
        unsafe {
            init_walk_motion_middle_fast_blend(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into())
        }
    }

    pub fn get_walk_motion_kind_middle_fast_blend(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_walk_motion_kind_middle_fast_blend(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn FL_get_walk_motion_rate_middle_fast_blend(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FL_get_walk_motion_rate_middle_fast_blend(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into()).into()
        }
    }

    pub fn sub_walk_uniq_process_init_common_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) {
        unsafe {
            sub_walk_uniq_process_init_common_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into())
        }
    }

    pub fn sub_walk_uniq_process_init_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) {
        unsafe {
            sub_walk_uniq_process_init_common(self, arg1.into(), arg2.into(), arg3.into(), arg4.into())
        }
    }

    pub fn sub_walk_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_walk_uniq_process_init(self).into()
        }
    }

    pub fn sub_walk_uniq_process_main_common_param(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue) {
        unsafe {
            sub_walk_uniq_process_main_common_param(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into())
        }
    }

    pub fn set_walk_motion_middle_fast_blend(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) {
        unsafe {
            set_walk_motion_middle_fast_blend(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into())
        }
    }

    pub fn sub_walk_fix_pos_counter_exec(&mut self) {
        unsafe {
            sub_walk_fix_pos_counter_exec(self)
        }
    }

    pub fn sub_walk_fix_pos_exec(&mut self) {
        unsafe {
            sub_walk_fix_pos_exec(self)
        }
    }

    pub fn sub_walk_uniq_process_main_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) {
        unsafe {
            sub_walk_uniq_process_main_common(self, arg1.into(), arg2.into(), arg3.into(), arg4.into())
        }
    }

    pub fn sub_walk_uniq_process_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_walk_uniq_process_main(self).into()
        }
    }

    pub fn sub_walk_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_walk_uniq_process_exit(self).into()
        }
    }

    pub fn sub_walk_uniq_process_on_change_lr(&mut self) -> lib::L2CValue {
        unsafe {
            sub_walk_uniq_process_on_change_lr(self).into()
        }
    }

    pub fn sub_walk_brake_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_walk_brake_uniq_process_init(self).into()
        }
    }

    pub fn sub_walk_brake_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_walk_brake_uniq_process_exec(self).into()
        }
    }

    pub fn sub_walk_brake_uniq_process_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_walk_brake_uniq_process_exit(self).into()
        }
    }

    pub fn status_pre_WalkBack(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_WalkBack(self).into()
        }
    }

    pub fn status_WalkBack_common(&mut self) {
        unsafe {
            status_WalkBack_common(self)
        }
    }

    pub fn status_end_WalkBack(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_WalkBack(self).into()
        }
    }

    pub fn sub_walk_back_uniq_process_exit(&mut self) {
        unsafe {
            sub_walk_back_uniq_process_exit(self)
        }
    }

    pub fn sub_walk_back_uniq_process_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_walk_back_uniq_process_init(self).into()
        }
    }

    pub fn sub_walk_back_uniq_process_exec(&mut self) -> lib::L2CValue {
        unsafe {
            sub_walk_back_uniq_process_exec(self).into()
        }
    }

    pub fn FighterStatusUniqProcessWalkBack_calc_param(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessWalkBack_calc_param(self).into()
        }
    }

    pub fn status_WalkBrakeBack(&mut self) -> lib::L2CValue {
        unsafe {
            status_WalkBrakeBack(self).into()
        }
    }

    pub fn status_end_WalkBrakeBack(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_WalkBrakeBack(self).into()
        }
    }

    pub fn status_pre_wall_jump(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_wall_jump(self).into()
        }
    }

    pub fn status_wall_jump(&mut self) -> lib::L2CValue {
        unsafe {
            status_wall_jump(self).into()
        }
    }

    pub fn sub_wall_jump_uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_wall_jump_uniq(self, arg1.into()).into()
        }
    }

    pub fn status_wall_jump_main(&mut self) -> lib::L2CValue {
        unsafe {
            status_wall_jump_main(self).into()
        }
    }

    pub fn status_end_wall_jump(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_wall_jump(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTarget_get_final_actor_module_accessor(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTarget_get_final_actor_module_accessor(self).into()
        }
    }

    pub fn status_pre_BayonettaFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BayonettaFinalTargetStart(self).into()
        }
    }

    pub fn status_BayonettaFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_BayonettaFinalTargetStart(self).into()
        }
    }

    pub fn status_BayonettaFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BayonettaFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_BayonettaFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BayonettaFinalTargetStart(self).into()
        }
    }

    pub fn sub_BayonettaFinalTargetStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_BayonettaFinalTargetStart_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetStart_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetStart_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_BayonettaFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BayonettaFinalTargetDamage(self).into()
        }
    }

    pub fn status_BayonettaFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_BayonettaFinalTargetDamage(self).into()
        }
    }

    pub fn status_BayonettaFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BayonettaFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_BayonettaFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BayonettaFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage_exit_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage_exit_status(self)
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_core(&mut self) {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_core(self)
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage_get_gomorrah_joint_global_position(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage_get_gomorrah_joint_global_position(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage_set_damage_reaction(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage_set_damage_reaction(self, arg1.into(), arg2.into())
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamageget_damage_motion_rand(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamageget_damage_motion_rand(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_stop(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_stop(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_fix_camera(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage_exec_fix_camera(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage_check_damage(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage_get_damage_motion_rand(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage_get_damage_motion_rand(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage_set_init_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage_set_init_pos(self)
        }
    }

    pub fn status_pre_BayonettaFinalTargetStart2(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BayonettaFinalTargetStart2(self).into()
        }
    }

    pub fn status_BayonettaFinalTargetStart2(&mut self) -> lib::L2CValue {
        unsafe {
            status_BayonettaFinalTargetStart2(self).into()
        }
    }

    pub fn status_BayonettaFinalTargetStart2_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BayonettaFinalTargetStart2_Main(self).into()
        }
    }

    pub fn status_end_BayonettaFinalTargetStart2(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BayonettaFinalTargetStart2(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetStart2_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetStart2_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_BayonettaFinalTargetDamage2(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BayonettaFinalTargetDamage2(self).into()
        }
    }

    pub fn status_BayonettaFinalTargetDamage2(&mut self) -> lib::L2CValue {
        unsafe {
            status_BayonettaFinalTargetDamage2(self).into()
        }
    }

    pub fn status_BayonettaFinalTargetDamage2_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BayonettaFinalTargetDamage2_Main(self).into()
        }
    }

    pub fn status_end_BayonettaFinalTargetDamage2(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BayonettaFinalTargetDamage2(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage2_exit_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage2_exit_status(self)
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage2_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage2_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage2_set_init_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage2_set_init_pos(self)
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetDamage2_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetDamage2_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_BayonettaFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_BayonettaFinalTargetEnd(self).into()
        }
    }

    pub fn status_BayonettaFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_BayonettaFinalTargetEnd(self).into()
        }
    }

    pub fn status_BayonettaFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_BayonettaFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_BayonettaFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_BayonettaFinalTargetEnd(self).into()
        }
    }

    pub fn FighterStatusUniqProcessBayonettaFinalTargetEnd_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessBayonettaFinalTargetEnd_init_status(self).into()
        }
    }

    pub fn status_CaptainFinalStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptainFinalStart(self).into()
        }
    }

    pub fn status_CaptainFinalStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptainFinalStart_Main(self).into()
        }
    }

    pub fn status_end_CaptainFinalStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptainFinalStart(self).into()
        }
    }

    pub fn sub_CaptainFinalStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_CaptainFinalStart_init_status(self).into()
        }
    }

    pub fn status_pre_CaptainFinalFuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptainFinalFuraFura(self).into()
        }
    }

    pub fn status_CaptainFinalFuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptainFinalFuraFura(self).into()
        }
    }

    pub fn status_CaptainFinalFuraFura_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptainFinalFuraFura_Main(self).into()
        }
    }

    pub fn status_end_CaptainFinalFuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptainFinalFuraFura(self).into()
        }
    }

    pub fn status_pre_CaptainFinalClash(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptainFinalClash(self).into()
        }
    }

    pub fn status_CaptainFinalClash(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptainFinalClash(self).into()
        }
    }

    pub fn status_CaptainFinalClash_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptainFinalClash_Main(self).into()
        }
    }

    pub fn status_end_CaptainFinalClash(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptainFinalClash(self).into()
        }
    }

    pub fn status_pre_CaptainFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_CaptainFinalEnd(self).into()
        }
    }

    pub fn status_CaptainFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptainFinalEnd(self).into()
        }
    }

    pub fn status_CaptainFinalEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_CaptainFinalEnd_Main(self).into()
        }
    }

    pub fn status_end_CaptainFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_CaptainFinalEnd(self).into()
        }
    }

    pub fn status_pre_ChromFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ChromFinalTargetDamage(self).into()
        }
    }

    pub fn status_ChromFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_ChromFinalTargetDamage(self).into()
        }
    }

    pub fn status_ChromFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ChromFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_ChromFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ChromFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusChromFinalTargetDamage_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusChromFinalTargetDamage_check_damage(self).into()
        }
    }

    pub fn status_DededeFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_DededeFinalTargetStart(self).into()
        }
    }

    pub fn status_DededeFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DededeFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_DededeFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DededeFinalTargetStart(self).into()
        }
    }

    pub fn sub_DededeFinalTargetStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_DededeFinalTargetStart_init_status(self).into()
        }
    }

    pub fn status_pre_DededeFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DededeFinalTargetDamage(self).into()
        }
    }

    pub fn status_DededeFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_DededeFinalTargetDamage(self).into()
        }
    }

    pub fn status_DededeFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DededeFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_DededeFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DededeFinalTargetDamage(self).into()
        }
    }

    pub fn status_pre_DededeFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DededeFinalTargetEnd(self).into()
        }
    }

    pub fn status_DededeFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_DededeFinalTargetEnd(self).into()
        }
    }

    pub fn status_DededeFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DededeFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_DededeFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DededeFinalTargetEnd(self).into()
        }
    }

    pub fn status_DuckhuntFinalStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_DuckhuntFinalStart(self).into()
        }
    }

    pub fn status_DuckhuntFinalStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DuckhuntFinalStart_Main(self).into()
        }
    }

    pub fn status_end_DuckhuntFinalStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DuckhuntFinalStart(self).into()
        }
    }

    pub fn sub_DuckhuntFinalStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_DuckhuntFinalStart_init_status(self).into()
        }
    }

    pub fn status_pre_DuckhuntFinalFuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DuckhuntFinalFuraFura(self).into()
        }
    }

    pub fn status_DuckhuntFinalFuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_DuckhuntFinalFuraFura(self).into()
        }
    }

    pub fn status_DuckhuntFinalFuraFura_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DuckhuntFinalFuraFura_Main(self).into()
        }
    }

    pub fn status_end_DuckhuntFinalFuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DuckhuntFinalFuraFura(self).into()
        }
    }

    pub fn status_pre_DuckhuntFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_DuckhuntFinalEnd(self).into()
        }
    }

    pub fn status_DuckhuntFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_DuckhuntFinalEnd(self).into()
        }
    }

    pub fn status_DuckhuntFinalEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_DuckhuntFinalEnd_Main(self).into()
        }
    }

    pub fn status_end_DuckhuntFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_DuckhuntFinalEnd(self).into()
        }
    }

    pub fn status_FalcoFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_FalcoFinalTargetStart(self).into()
        }
    }

    pub fn status_FalcoFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FalcoFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_FalcoFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FalcoFinalTargetStart(self).into()
        }
    }

    pub fn sub_FalcoFinalTargetStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_FalcoFinalTargetStart_init_status(self).into()
        }
    }

    pub fn status_pre_FalcoFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FalcoFinalTargetDamage(self).into()
        }
    }

    pub fn status_FalcoFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_FalcoFinalTargetDamage(self).into()
        }
    }

    pub fn status_FalcoFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FalcoFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_FalcoFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FalcoFinalTargetDamage(self).into()
        }
    }

    pub fn status_pre_FalcoFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FalcoFinalTargetEnd(self).into()
        }
    }

    pub fn status_FalcoFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_FalcoFinalTargetEnd(self).into()
        }
    }

    pub fn status_FalcoFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FalcoFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_FalcoFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FalcoFinalTargetEnd(self).into()
        }
    }

    pub fn status_pre_FinalDamageFlyMain(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_FinalDamageFlyMain(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into()).into()
        }
    }

    pub fn status_pre_FinalDamageFly(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalDamageFly(self).into()
        }
    }

    pub fn status_pre_FinalDamageFlyCloud(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalDamageFlyCloud(self).into()
        }
    }

    pub fn status_pre_FinalDamageFlyLittlemac(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalDamageFlyLittlemac(self).into()
        }
    }

    pub fn status_FinalDamageFlyCommon(&mut self) {
        unsafe {
            status_FinalDamageFlyCommon(self)
        }
    }

    pub fn status_FinalDamageFly(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFly(self).into()
        }
    }

    pub fn status_FinalDamageFlyReflet(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFlyReflet(self).into()
        }
    }

    pub fn status_FinalDamageFly_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFly_Main(self).into()
        }
    }

    pub fn status_end_FinalDamageFly(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FinalDamageFly(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFly_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFly_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFly_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFly_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFly_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFly_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFly_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessFinalDamageFly_exec_fix_pos(self)
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFly_exec_fix_camera(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFly_exec_fix_camera(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFly_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFly_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_FinalDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalDamage(self).into()
        }
    }

    pub fn status_FinalDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamage(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamage_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessFinalDamage_exec_fix_pos(self)
        }
    }

    pub fn status_FinalDamageCloud(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageCloud(self).into()
        }
    }

    pub fn status_FinalDamageCloud_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageCloud_Main(self).into()
        }
    }

    pub fn status_FinalDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamage_Main(self).into()
        }
    }

    pub fn status_end_FinalDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FinalDamage(self).into()
        }
    }

    pub fn status_end_FinalDamageCloud(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FinalDamageCloud(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamage_exec_status(&mut self) {
        unsafe {
            FighterStatusUniqProcessFinalDamage_exec_status(self)
        }
    }

    pub fn FighterStatusUniqProcessFinalDamage_exec_fix_camera(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamage_exec_fix_camera(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamage_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamage_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_FinalDamageFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalDamageFall(self).into()
        }
    }

    pub fn status_FinalDamageFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFall(self).into()
        }
    }

    pub fn status_FinalDamageFallCommon(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFallCommon(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFall_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessFinalDamageFall_exec_fix_pos(self)
        }
    }

    pub fn status_FinalDamageFall_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFall_Main(self).into()
        }
    }

    pub fn status_end_FinalDamageFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FinalDamageFall(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFall_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFall_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFall_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFall_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFall_exec_fix_camera(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFall_exec_fix_camera(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFall_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFall_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_KirbyFinalDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_KirbyFinalDamage(self).into()
        }
    }

    pub fn FighterStatusUniqProcessKirbyFinalDamage_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessKirbyFinalDamage_init_status(self).into()
        }
    }

    pub fn status_FinalDamageFlyCloud(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFlyCloud(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyCloud_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyCloud_exec_fix_pos(self)
        }
    }

    pub fn status_FinalDamageFlyCloud_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFlyCloud_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageCloud_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageCloud_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyCloud_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyCloud_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyCloud_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyCloud_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyCloud_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyCloud_exit_status(self).into()
        }
    }

    pub fn status_pre_FinalDamageFallCloud(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalDamageFallCloud(self).into()
        }
    }

    pub fn status_FinalDamageFallCloud(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFallCloud(self).into()
        }
    }

    pub fn status_Final2DamageFallCloud_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_Final2DamageFallCloud_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyIke_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyIke_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyIke_init_status_main(&mut self) {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyIke_init_status_main(self)
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyIke_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyIke_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageIke_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageIke_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageIke_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageIke_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyKoopa_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyKoopa_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyKoopa_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyKoopa_exec_status(self).into()
        }
    }

    pub fn status_FinalDamageFlyLittlemac(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFlyLittlemac(self).into()
        }
    }

    pub fn status_FinalDamageFlyLittlemac_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFlyLittlemac_Main(self).into()
        }
    }

    pub fn status_FinalDamageLittlemac(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageLittlemac(self).into()
        }
    }

    pub fn status_end_FinalDamageLittlemac(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FinalDamageLittlemac(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyLittlemac_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyLittlemac_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyLittlemac_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyLittlemac_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyLittlemac_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyLittlemac_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageLittlemac_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageLittlemac_exit_status(self).into()
        }
    }

    pub fn status_pre_FinalDamageFlyPikachu(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalDamageFlyPikachu(self).into()
        }
    }

    pub fn status_FinalDamageFlyPikachu(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFlyPikachu(self).into()
        }
    }

    pub fn is_finish_FinalDamageFlyPikachu(&mut self) -> lib::L2CValue {
        unsafe {
            is_finish_FinalDamageFlyPikachu(self).into()
        }
    }

    pub fn status_FinalDamageFlyPikachu_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFlyPikachu_Main(self).into()
        }
    }

    pub fn status_end_FinalDamageFlyPikachu(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FinalDamageFlyPikachu(self).into()
        }
    }

    pub fn status_pre_FinalDamageRyu(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalDamageRyu(self).into()
        }
    }

    pub fn status_FinalDamageRyu(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageRyu(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageRyu_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessFinalDamageRyu_exec_fix_pos(self)
        }
    }

    pub fn status_FinalDamageRyu_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageRyu_Main(self).into()
        }
    }

    pub fn status_pre_FinalDamageFlyRyu(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalDamageFlyRyu(self).into()
        }
    }

    pub fn status_FinalDamageFlyRyu(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFlyRyu(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyRyu_exec_fix_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyRyu_exec_fix_pos(self)
        }
    }

    pub fn status_FinalDamageFlyRyu_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFlyRyu_Main(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageRyu_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageRyu_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageRyu_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageRyu_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageRyu_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageRyu_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageRyu_exec_fix_camera(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageRyu_exec_fix_camera(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageRyu_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageRyu_check_damage(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyRyu_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyRyu_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyRyu_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyRyu_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyRyu_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyRyu_check_damage(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyZoroark_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyZoroark_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyZoroark_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyZoroark_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageZoroark_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageZoroark_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageZoroark_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageZoroark_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageZoroark_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageZoroark_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageZoroark_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageZoroark_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_FinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalTargetStart(self).into()
        }
    }

    pub fn status_pre_FinalTargetStartShulk(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalTargetStartShulk(self).into()
        }
    }

    pub fn status_pre_FinalVisualAttackOther(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FinalVisualAttackOther(self).into()
        }
    }

    pub fn status_pre_FinalVisualAttackOther_param(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_pre_FinalVisualAttackOther_param(self, arg1.into()).into()
        }
    }

    pub fn status_FinalVisualAttackOther(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalVisualAttackOther(self).into()
        }
    }

    pub fn sub_captain_finalother_uniq(&mut self) -> lib::L2CValue {
        unsafe {
            sub_captain_finalother_uniq(self).into()
        }
    }

    pub fn status_FinalVisualAttackOther_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalVisualAttackOther_Main(self).into()
        }
    }

    pub fn status_end_FinalVisualAttackOther(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FinalVisualAttackOther(self).into()
        }
    }

    pub fn FinalVisualAttackOther_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FinalVisualAttackOther_check_damage(self).into()
        }
    }

    pub fn status_FoxFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_FoxFinalTargetStart(self).into()
        }
    }

    pub fn status_FoxFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FoxFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_FoxFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FoxFinalTargetStart(self).into()
        }
    }

    pub fn sub_FoxFinalTargetStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_FoxFinalTargetStart_init_status(self).into()
        }
    }

    pub fn status_pre_FoxFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FoxFinalTargetDamage(self).into()
        }
    }

    pub fn status_FoxFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_FoxFinalTargetDamage(self).into()
        }
    }

    pub fn status_FoxFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FoxFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_FoxFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FoxFinalTargetDamage(self).into()
        }
    }

    pub fn status_pre_FoxFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_FoxFinalTargetEnd(self).into()
        }
    }

    pub fn status_FoxFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_FoxFinalTargetEnd(self).into()
        }
    }

    pub fn status_FoxFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FoxFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_FoxFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FoxFinalTargetEnd(self).into()
        }
    }

    pub fn status_GaogaenFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_GaogaenFinalTargetStart(self).into()
        }
    }

    pub fn status_GaogaenFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GaogaenFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_GaogaenFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GaogaenFinalTargetStart(self).into()
        }
    }

    pub fn sub_GaogaenFinalTargetStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_GaogaenFinalTargetStart_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessGaogaenFinalTargetStart_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessGaogaenFinalTargetStart_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_GaogaenFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GaogaenFinalTargetDamage(self).into()
        }
    }

    pub fn status_GaogaenFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_GaogaenFinalTargetDamage(self).into()
        }
    }

    pub fn update_thrown_offset(&mut self) {
        unsafe {
            update_thrown_offset(self)
        }
    }

    pub fn status_GaogaenFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GaogaenFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_GaogaenFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GaogaenFinalTargetDamage(self).into()
        }
    }

    pub fn status_pre_GaogaenFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GaogaenFinalTargetEnd(self).into()
        }
    }

    pub fn status_GaogaenFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_GaogaenFinalTargetEnd(self).into()
        }
    }

    pub fn status_GaogaenFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_GaogaenFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_GaogaenFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_GaogaenFinalTargetEnd(self).into()
        }
    }

    pub fn status_pre_GekkougaFinalDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GekkougaFinalDamage(self).into()
        }
    }

    pub fn status_FinalDamageFlyGekkouga(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFlyGekkouga(self).into()
        }
    }

    pub fn status_FinalDamageFlyGekkouga_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageFlyGekkouga_Main(self).into()
        }
    }

    pub fn status_end_FinalDamageFlyGekkouga(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_FinalDamageFlyGekkouga(self).into()
        }
    }

    pub fn status_pre_GekkougaFinalDamageFall(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_GekkougaFinalDamageFall(self).into()
        }
    }

    pub fn status_KamuiFinalStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_KamuiFinalStart(self).into()
        }
    }

    pub fn status_KamuiFinalStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_KamuiFinalStart_Main(self).into()
        }
    }

    pub fn status_end_KamuiFinalStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_KamuiFinalStart(self).into()
        }
    }

    pub fn sub_KamuiFinalTargetStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_KamuiFinalTargetStart_init_status(self).into()
        }
    }

    pub fn status_pre_KamuiFinalDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_KamuiFinalDamage(self).into()
        }
    }

    pub fn status_KamuiFinalDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_KamuiFinalDamage(self).into()
        }
    }

    pub fn status_KamuiFinalDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_KamuiFinalDamage_Main(self).into()
        }
    }

    pub fn status_end_KamuiFinalDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_KamuiFinalDamage(self).into()
        }
    }

    pub fn status_pre_KamuiFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_KamuiFinalEnd(self).into()
        }
    }

    pub fn status_KamuiFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_KamuiFinalEnd(self).into()
        }
    }

    pub fn status_KamuiFinalEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_KamuiFinalEnd_Main(self).into()
        }
    }

    pub fn status_end_KamuiFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_KamuiFinalEnd(self).into()
        }
    }

    pub fn status_KroolFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_KroolFinalTargetStart(self).into()
        }
    }

    pub fn status_KroolFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_KroolFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_KroolFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_KroolFinalTargetStart(self).into()
        }
    }

    pub fn sub_KroolFinalTargetStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_KroolFinalTargetStart_init_status(self).into()
        }
    }

    pub fn status_pre_KroolFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_KroolFinalTargetDamage(self).into()
        }
    }

    pub fn status_KroolFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_KroolFinalTargetDamage(self).into()
        }
    }

    pub fn status_KroolFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_KroolFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_KroolFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_KroolFinalTargetDamage(self).into()
        }
    }

    pub fn status_pre_KroolFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_KroolFinalTargetEnd(self).into()
        }
    }

    pub fn status_KroolFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_KroolFinalTargetEnd(self).into()
        }
    }

    pub fn status_KroolFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_KroolFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_KroolFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_KroolFinalTargetEnd(self).into()
        }
    }

    pub fn status_pre_LuigiFinalVacuum(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_LuigiFinalVacuum(self).into()
        }
    }

    pub fn status_LuigiFinalVacuum(&mut self) -> lib::L2CValue {
        unsafe {
            status_LuigiFinalVacuum(self).into()
        }
    }

    pub fn status_LuigiFinalVacuum_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_LuigiFinalVacuum_Main(self).into()
        }
    }

    pub fn status_end_LuigiFinalVacuum(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_LuigiFinalVacuum(self).into()
        }
    }

    pub fn status_pre_LuigiFinalShoot(&mut self) {
        unsafe {
            status_pre_LuigiFinalShoot(self)
        }
    }

    pub fn status_LuigiFinalShoot(&mut self) -> lib::L2CValue {
        unsafe {
            status_LuigiFinalShoot(self).into()
        }
    }

    pub fn status_LuigiFinalShoot_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_LuigiFinalShoot_Main(self).into()
        }
    }

    pub fn status_LuigiFinalShoot_exec_fix_pos(&mut self) {
        unsafe {
            status_LuigiFinalShoot_exec_fix_pos(self)
        }
    }

    pub fn status_end_LuigiFinalShoot(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_LuigiFinalShoot(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyMetaknight_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyMetaknight_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFlyMetaknight_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFlyMetaknight_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageMetaknight_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageMetaknight_init_status(self).into()
        }
    }

    pub fn status_FinalDamageMetaknight(&mut self) -> lib::L2CValue {
        unsafe {
            status_FinalDamageMetaknight(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageMetaknight_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageMetaknight_exit_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageMetaknight_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageMetaknight_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFallMetaknight_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFallMetaknight_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessFinalDamageFallMetaknight_exit_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessFinalDamageFallMetaknight_exit_status(self).into()
        }
    }

    pub fn status_pre_MurabitoFinalCapture(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_MurabitoFinalCapture(self).into()
        }
    }

    pub fn status_MurabitoFinalCapture(&mut self) -> lib::L2CValue {
        unsafe {
            status_MurabitoFinalCapture(self).into()
        }
    }

    pub fn status_MurabitoFinalCapture_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_MurabitoFinalCapture_Main(self).into()
        }
    }

    pub fn status_end_MurabitoFinalCapture(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_MurabitoFinalCapture(self).into()
        }
    }

    pub fn status_pre_PacmanFinalEaten(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_PacmanFinalEaten(self).into()
        }
    }

    pub fn status_PacmanFinalEaten(&mut self) -> lib::L2CValue {
        unsafe {
            status_PacmanFinalEaten(self).into()
        }
    }

    pub fn status_PacmanFinalEaten_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_PacmanFinalEaten_Main(self).into()
        }
    }

    pub fn status_end_PacmanFinalEaten(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_PacmanFinalEaten(self).into()
        }
    }

    pub fn status_pre_PalutenaFinalBlackhole(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_PalutenaFinalBlackhole(self).into()
        }
    }

    pub fn status_PalutenaFinalBlackhole(&mut self) -> lib::L2CValue {
        unsafe {
            status_PalutenaFinalBlackhole(self).into()
        }
    }

    pub fn status_PalutenaFinalBlackhole_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_PalutenaFinalBlackhole_Main(self).into()
        }
    }

    pub fn status_end_PalutenaFinalBlackhole(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_PalutenaFinalBlackhole(self).into()
        }
    }

    pub fn status_pre_RefletFinalDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_RefletFinalDamage(self).into()
        }
    }

    pub fn status_RidleyFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_RidleyFinalTargetStart(self).into()
        }
    }

    pub fn status_RidleyFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_RidleyFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_RidleyFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_RidleyFinalTargetStart(self).into()
        }
    }

    pub fn status_pre_RidleyFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_RidleyFinalTargetDamage(self).into()
        }
    }

    pub fn status_RidleyFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_RidleyFinalTargetDamage(self).into()
        }
    }

    pub fn status_RidleyFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_RidleyFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_RidleyFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_RidleyFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusRidleyFinalTargetDamage_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusRidleyFinalTargetDamage_check_damage(self).into()
        }
    }

    pub fn status_pre_RidleyFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_RidleyFinalTargetEnd(self).into()
        }
    }

    pub fn status_RidleyFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_RidleyFinalTargetEnd(self).into()
        }
    }

    pub fn status_RidleyFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_RidleyFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_RidleyFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_RidleyFinalTargetEnd(self).into()
        }
    }

    pub fn FighterStatusUniqProcessRockmanFinalTarget_get_final_actor_module_accessor(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRockmanFinalTarget_get_final_actor_module_accessor(self).into()
        }
    }

    pub fn status_pre_RockmanFinalTargetScene01(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_RockmanFinalTargetScene01(self).into()
        }
    }

    pub fn status_RockmanFinalTargetScene01(&mut self) -> lib::L2CValue {
        unsafe {
            status_RockmanFinalTargetScene01(self).into()
        }
    }

    pub fn status_RockmanFinalTargetScene01_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_RockmanFinalTargetScene01_Main(self).into()
        }
    }

    pub fn status_end_RockmanFinalTargetScene01(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_RockmanFinalTargetScene01(self).into()
        }
    }

    pub fn FighterStatusUniqProcessRockmanFinalTargetScene01_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRockmanFinalTargetScene01_init_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessRockmanFinalTargetScene01_set_pos(&mut self) {
        unsafe {
            FighterStatusUniqProcessRockmanFinalTargetScene01_set_pos(self)
        }
    }

    pub fn status_pre_RockmanFinalTargetScene02(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_RockmanFinalTargetScene02(self).into()
        }
    }

    pub fn status_RockmanFinalTargetScene02(&mut self) -> lib::L2CValue {
        unsafe {
            status_RockmanFinalTargetScene02(self).into()
        }
    }

    pub fn status_RockmanFinalTargetScene02_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_RockmanFinalTargetScene02_Main(self).into()
        }
    }

    pub fn status_end_RockmanFinalTargetScene02(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_RockmanFinalTargetScene02(self).into()
        }
    }

    pub fn FighterStatusUniqProcessRockmanFinalTargetScene02_exec_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRockmanFinalTargetScene02_exec_status(self).into()
        }
    }

    pub fn FighterStatusUniqProcessRockmanFinalTargetScene02_set_damage_reaction(&mut self) {
        unsafe {
            FighterStatusUniqProcessRockmanFinalTargetScene02_set_damage_reaction(self)
        }
    }

    pub fn FighterStatusUniqProcessRockmanFinalTargetScene02_get_damage_motion_rand(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRockmanFinalTargetScene02_get_damage_motion_rand(self, arg1.into()).into()
        }
    }

    pub fn FighterStatusUniqProcessRockmanFinalTargetScene02_check_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRockmanFinalTargetScene02_check_damage(self, arg1.into()).into()
        }
    }

    pub fn status_pre_RockmanFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_RockmanFinalTargetEnd(self).into()
        }
    }

    pub fn status_RockmanFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_RockmanFinalTargetEnd(self).into()
        }
    }

    pub fn status_RockmanFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_RockmanFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_RockmanFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_RockmanFinalTargetEnd(self).into()
        }
    }

    pub fn FighterStatusUniqProcessRockmanFinalTargetEnd_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusUniqProcessRockmanFinalTargetEnd_init_status(self).into()
        }
    }

    pub fn status_pre_SheikFinalDamage(&mut self) {
        unsafe {
            status_pre_SheikFinalDamage(self)
        }
    }

    pub fn status_SheikFinalDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_SheikFinalDamage(self).into()
        }
    }

    pub fn status_SheikFinalDamage_Uniq(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            status_SheikFinalDamage_Uniq(self, arg1.into()).into()
        }
    }

    pub fn status_SheikFinalDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SheikFinalDamage_Main(self).into()
        }
    }

    pub fn status_end_SheikFinalDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SheikFinalDamage(self).into()
        }
    }

    pub fn status_ShulkFinalStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShulkFinalStart(self).into()
        }
    }

    pub fn status_ShulkFinalStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShulkFinalStart_Main(self).into()
        }
    }

    pub fn status_end_ShulkFinalStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ShulkFinalStart(self).into()
        }
    }

    pub fn sub_ShulkFinalStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_ShulkFinalStart_init_status(self).into()
        }
    }

    pub fn status_pre_ShulkFinalFuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ShulkFinalFuraFura(self).into()
        }
    }

    pub fn status_ShulkFinalFuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShulkFinalFuraFura(self).into()
        }
    }

    pub fn status_ShulkFinalFuraFura_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShulkFinalFuraFura_Main(self).into()
        }
    }

    pub fn status_end_ShulkFinalFuraFura(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ShulkFinalFuraFura(self).into()
        }
    }

    pub fn status_pre_ShulkFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_ShulkFinalEnd(self).into()
        }
    }

    pub fn status_ShulkFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShulkFinalEnd(self).into()
        }
    }

    pub fn status_ShulkFinalEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_ShulkFinalEnd_Main(self).into()
        }
    }

    pub fn status_end_ShulkFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_ShulkFinalEnd(self).into()
        }
    }

    pub fn status_SimonFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_SimonFinalTargetStart(self).into()
        }
    }

    pub fn status_SimonFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SimonFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_SimonFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SimonFinalTargetStart(self).into()
        }
    }

    pub fn status_pre_SimonFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SimonFinalTargetDamage(self).into()
        }
    }

    pub fn status_SimonFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_SimonFinalTargetDamage(self).into()
        }
    }

    pub fn status_SimonFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SimonFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_SimonFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SimonFinalTargetDamage(self).into()
        }
    }

    pub fn status_pre_SimonFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_SimonFinalTargetEnd(self).into()
        }
    }

    pub fn status_SimonFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_SimonFinalTargetEnd(self).into()
        }
    }

    pub fn status_SimonFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_SimonFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_SimonFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_SimonFinalTargetEnd(self).into()
        }
    }

    pub fn sub_uniq_process_pre_link_final(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_pre_link_final(self).into()
        }
    }

    pub fn sub_uniq_process_link_final_init(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_link_final_init(self).into()
        }
    }

    pub fn sub_uniq_process_link_final_main(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_link_final_main(self).into()
        }
    }

    pub fn sub_uniq_process_link_final_exec_fix_pos(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_link_final_exec_fix_pos(self).into()
        }
    }

    pub fn sub_check_cancel(&mut self) {
        unsafe {
            sub_check_cancel(self)
        }
    }

    pub fn sub_uniq_process_link_final_exec_fix_pos_counter(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_link_final_exec_fix_pos_counter(self).into()
        }
    }

    pub fn sub_uniq_process_link_final_exit(&mut self) -> lib::L2CValue {
        unsafe {
            sub_uniq_process_link_final_exit(self).into()
        }
    }

    pub fn sub_get_random_damage_motion(&mut self) -> lib::L2CValue {
        unsafe {
            sub_get_random_damage_motion(self).into()
        }
    }

    pub fn sub_is_link_final_captured(&mut self) -> lib::L2CValue {
        unsafe {
            sub_is_link_final_captured(self).into()
        }
    }

    pub fn status_pre_LinkFinalArrowHit(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_LinkFinalArrowHit(self).into()
        }
    }

    pub fn status_LinkFinalArrowHit(&mut self) -> lib::L2CValue {
        unsafe {
            status_LinkFinalArrowHit(self).into()
        }
    }

    pub fn status_LinkFinalArrowHit_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_LinkFinalArrowHit_Main(self).into()
        }
    }

    pub fn status_end_LinkFinalArrowHit(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_LinkFinalArrowHit(self).into()
        }
    }

    pub fn status_WarioFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_WarioFinalTargetStart(self).into()
        }
    }

    pub fn status_WarioFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_WarioFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_WarioFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_WarioFinalTargetStart(self).into()
        }
    }

    pub fn sub_WarioFinalTargetStart_init_status(&mut self) -> lib::L2CValue {
        unsafe {
            sub_WarioFinalTargetStart_init_status(self).into()
        }
    }

    pub fn status_pre_WarioFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_WarioFinalTargetDamage(self).into()
        }
    }

    pub fn status_WarioFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_WarioFinalTargetDamage(self).into()
        }
    }

    pub fn status_WarioFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_WarioFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_WarioFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_WarioFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusWarioFinalTargetDamage_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusWarioFinalTargetDamage_check_damage(self).into()
        }
    }

    pub fn status_pre_WarioFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_WarioFinalTargetEnd(self).into()
        }
    }

    pub fn status_WarioFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_WarioFinalTargetEnd(self).into()
        }
    }

    pub fn status_WarioFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_WarioFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_WarioFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_WarioFinalTargetEnd(self).into()
        }
    }

    pub fn status_YoshiFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_YoshiFinalTargetStart(self).into()
        }
    }

    pub fn status_YoshiFinalTargetStart_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_YoshiFinalTargetStart_Main(self).into()
        }
    }

    pub fn status_end_YoshiFinalTargetStart(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_YoshiFinalTargetStart(self).into()
        }
    }

    pub fn status_pre_YoshiFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_YoshiFinalTargetDamage(self).into()
        }
    }

    pub fn status_YoshiFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_YoshiFinalTargetDamage(self).into()
        }
    }

    pub fn status_YoshiFinalTargetDamage_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_YoshiFinalTargetDamage_Main(self).into()
        }
    }

    pub fn status_end_YoshiFinalTargetDamage(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_YoshiFinalTargetDamage(self).into()
        }
    }

    pub fn FighterStatusYoshiFinalTargetDamage_check_damage(&mut self) -> lib::L2CValue {
        unsafe {
            FighterStatusYoshiFinalTargetDamage_check_damage(self).into()
        }
    }

    pub fn status_pre_YoshiFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_pre_YoshiFinalTargetEnd(self).into()
        }
    }

    pub fn status_YoshiFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_YoshiFinalTargetEnd(self).into()
        }
    }

    pub fn status_YoshiFinalTargetEnd_Main(&mut self) -> lib::L2CValue {
        unsafe {
            status_YoshiFinalTargetEnd_Main(self).into()
        }
    }

    pub fn status_end_YoshiFinalTargetEnd(&mut self) -> lib::L2CValue {
        unsafe {
            status_end_YoshiFinalTargetEnd(self).into()
        }
    }

    pub fn sub_fighter_common_settings(&mut self) {
        unsafe {
            sub_fighter_common_settings(self)
        }
    }

    pub fn global_fighter_init(&mut self) {
        unsafe {
            global_fighter_init(self)
        }
    }

    pub fn local_func__fighter_global_variable_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_global_variable_1(self).into()
        }
    }

    pub fn local_func__fighter_global_variable_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__fighter_global_variable_2(self).into()
        }
    }

    pub fn sub_set_fighter_common_table(&mut self) {
        unsafe {
            sub_set_fighter_common_table(self)
        }
    }

    pub fn sub_set_status_line_msc_common_table(&mut self) {
        unsafe {
            sub_set_status_line_msc_common_table(self)
        }
    }

    pub fn sub_set_status_pre_msc_common_table(&mut self) {
        unsafe {
            sub_set_status_pre_msc_common_table(self)
        }
    }

    pub fn sub_set_status_main_msc_common_table(&mut self) {
        unsafe {
            sub_set_status_main_msc_common_table(self)
        }
    }

    pub fn sub_set_status_end_msc_common_table(&mut self) {
        unsafe {
            sub_set_status_end_msc_common_table(self)
        }
    }

    pub fn sub_set_init_status_msc_common_table(&mut self) {
        unsafe {
            sub_set_init_status_msc_common_table(self)
        }
    }

    pub fn sub_set_exec_status_msc_common_table(&mut self) {
        unsafe {
            sub_set_exec_status_msc_common_table(self)
        }
    }

    pub fn sub_set_exec_stop_msc_common_table(&mut self) {
        unsafe {
            sub_set_exec_stop_msc_common_table(self)
        }
    }

    pub fn sub_set_exec_status_post_msc_common_table(&mut self) {
        unsafe {
            sub_set_exec_status_post_msc_common_table(self)
        }
    }

    pub fn sub_set_exit_status_msc_common_table(&mut self) {
        unsafe {
            sub_set_exit_status_msc_common_table(self)
        }
    }

    pub fn sub_set_map_correction_msc_common_table(&mut self) {
        unsafe {
            sub_set_map_correction_msc_common_table(self)
        }
    }

    pub fn sub_set_fix_pos_slow_msc_common_table(&mut self) {
        unsafe {
            sub_set_fix_pos_slow_msc_common_table(self)
        }
    }

    pub fn sub_set_fix_camera_msc_common_table(&mut self) {
        unsafe {
            sub_set_fix_camera_msc_common_table(self)
        }
    }

    pub fn sub_set_check_damage_common_table(&mut self) {
        unsafe {
            sub_set_check_damage_common_table(self)
        }
    }

    pub fn sub_set_check_attack_common_table(&mut self) {
        unsafe {
            sub_set_check_attack_common_table(self)
        }
    }

    pub fn sub_set_on_change_lr_common_table(&mut self) {
        unsafe {
            sub_set_on_change_lr_common_table(self)
        }
    }

    pub fn sub_set_leave_stop_common_table(&mut self) {
        unsafe {
            sub_set_leave_stop_common_table(self)
        }
    }

    pub fn sub_set_notify_event_gimmick_common_table(&mut self) {
        unsafe {
            sub_set_notify_event_gimmick_common_table(self)
        }
    }

    pub fn sub_set_status_uniq_process_common_table(&mut self) {
        unsafe {
            sub_set_status_uniq_process_common_table(self)
        }
    }

    pub fn sub_set_uniq_process_final_damage_common_table(&mut self) {
        unsafe {
            sub_set_uniq_process_final_damage_common_table(self)
        }
    }

    pub fn sub_set_calc_param_common_table(&mut self) {
        unsafe {
            sub_set_calc_param_common_table(self)
        }
    }

    pub fn sub_set_item_shoot(&mut self) {
        unsafe {
            sub_set_item_shoot(self)
        }
    }

    pub fn sub_set_func_common_table(&mut self) {
        unsafe {
            sub_set_func_common_table(self)
        }
    }

    pub fn sys_line_waza_customize_init(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_waza_customize_init(self).into()
        }
    }

    pub fn sys_line_waza_customize_control(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_waza_customize_control(self).into()
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CFighterCommon {
    pub fn assert() {
        assert_eq!(size_of!(L2CFighterCommon), 0x228);
        assert_eq!(offset_of!(L2CFighterCommon, fighter_base),                 0x0);
        assert_eq!(offset_of!(L2CFighterCommon, attack_info),                  0x118);
        assert_eq!(offset_of!(L2CFighterCommon, damage_info1),                 0x128);
        assert_eq!(offset_of!(L2CFighterCommon, damage_info2),                 0x138);
        assert_eq!(offset_of!(L2CFighterCommon, guard_info),                   0x148);
        assert_eq!(offset_of!(L2CFighterCommon, item_info1),                   0x158);
        assert_eq!(offset_of!(L2CFighterCommon, item_info2),                   0x168);
        assert_eq!(offset_of!(L2CFighterCommon, ladder_info1),                 0x178);
        assert_eq!(offset_of!(L2CFighterCommon, ladder_info2),                 0x188);
        assert_eq!(offset_of!(L2CFighterCommon, swallowed_info1),              0x198);
        assert_eq!(offset_of!(L2CFighterCommon, swallowed_info2),              0x1A8);
        assert_eq!(offset_of!(L2CFighterCommon, swallowed_info3),              0x1B8);
        assert_eq!(offset_of!(L2CFighterCommon, swim_info1),                   0x1C8);
        assert_eq!(offset_of!(L2CFighterCommon, swim_info2),                   0x1D8);
        assert_eq!(offset_of!(L2CFighterCommon, bayonetta_final_target_info1), 0x1E8);
        assert_eq!(offset_of!(L2CFighterCommon, bayonetta_final_target_info2), 0x1F8);
        assert_eq!(offset_of!(L2CFighterCommon, final_damage_info),            0x208);
        assert_eq!(offset_of!(L2CFighterCommon, rockman_final_target_info),    0x218);
    }
}