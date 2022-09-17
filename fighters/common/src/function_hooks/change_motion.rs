use super::*;
use globals::*;

//=================================================================
//== CHANGE MOTION HOOKS
//== Note: Hooking functions to clear all hitboxes upon animation changes
//=================================================================

//=================================================================
//== MotionModule::change_motion
//=================================================================
#[skyline::hook(replace=MotionModule::change_motion)]
unsafe fn change_motion_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40, arg3: f32, arg4: f32, arg5: bool, arg6: f32, arg7: bool, arg8: bool) -> u64 {
    let mut start_frame = arg3;
    change_motion_ecb_shift_check(boma);
    if boma.is_fighter() {
        // Starts heavy landing animation on frame 2
        // This is a purely aesthetic change, makes for snappier landings
        if motion_hash == Hash40::new("landing_heavy") {
            start_frame = 1.0;
        }
    }
    original!()(boma, motion_hash, start_frame, arg4, arg5, arg6, arg7, arg8)
}

#[skyline::hook(replace=MotionModule::change_motion_inherit_frame)]
unsafe fn change_motion_inherit_frame_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40, arg3: f32, arg4: f32, arg5: f32, arg6: bool, arg7: bool) -> u64 {
    change_motion_ecb_shift_check(boma);
    original!()(boma, motion_hash, arg3, arg4, arg5, arg6, arg7)
}

#[skyline::hook(replace=MotionModule::change_motion_inherit_frame_keep_rate)]
unsafe fn change_motion_inherit_frame_keep_rate_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40, arg3: f32, arg4: f32, arg5: f32) -> u64 {
    change_motion_ecb_shift_check(boma);
    original!()(boma, motion_hash, arg3, arg4, arg5)
}

#[skyline::hook(replace=MotionModule::change_motion_force_inherit_frame)]
unsafe fn change_motion_force_inherit_frame_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40, arg3: f32, arg4: f32, arg5: f32) -> u64 {
    change_motion_ecb_shift_check(boma);
    original!()(boma, motion_hash, arg3, arg4, arg5)
}

#[skyline::hook(replace=MotionModule::change_motion_kind)]
unsafe fn change_motion_kind_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40) -> u64 {
    change_motion_ecb_shift_check(boma);
    original!()(boma, motion_hash)
}

unsafe fn change_motion_ecb_shift_check(boma: &mut BattleObjectModuleAccessor) {
    // Same routine/justification as fake ECB shift in init_settings hook
    // Only difference is that our status hasn't changed during a change_motion call
    if boma.is_fighter() {
        if !boma.is_prev_status_one_of(&[
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_THROWN
        ])
        && !boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_ENTRY,
            *FIGHTER_STATUS_KIND_THROWN,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_BURY,
            *FIGHTER_STATUS_KIND_BURY_WAIT
        ]) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
        && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
        && VarModule::get_float(boma.object(), vars::common::instance::ECB_Y_OFFSETS) != 0.0 {
            boma.shift_ecb_on_landing();
        }
    }
}

#[skyline::hook(replace=EffectModule::preset_lifetime_rate_partial)]
unsafe fn preset_lifetime_rate_partial_hook(boma: &mut BattleObjectModuleAccessor, rate: f32) -> u64 {
    let mut rate = rate.clone();
    // Halve the lifetime of knockback smoke
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR])
    {
        rate *= 0.5;
    }
    original!()(boma, rate)
}


pub fn install() {
    skyline::install_hooks!(
        change_motion_hook,
        change_motion_inherit_frame_hook,
        change_motion_inherit_frame_keep_rate_hook,
        change_motion_force_inherit_frame_hook,
        change_motion_kind_hook,
        preset_lifetime_rate_partial_hook
    );
}
