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
    change_motion_pos_shift_check(boma);
    if boma.is_fighter() {
        // Starts landing animations on frame 2
        // This is a purely aesthetic change, makes for snappier landings
        if [Hash40::new("landing_air_n"),
            Hash40::new("landing_air_f"),
            Hash40::new("landing_air_b"),
            Hash40::new("landing_air_hi"),
            Hash40::new("landing_air_lw"),
            Hash40::new("landing_fall_special")].contains(&motion_hash)
        {
            start_frame = 1.0;
        }
        else if motion_hash == Hash40::new("landing_heavy") {
            start_frame = 3.0;
        }

        // Allows a frame-perfect edge canceled waveland to still generate landing smoke GFX
        if VarModule::is_flag(boma.object(), vars::common::instance::FLUSH_EFFECT_ACMD)
        && MotionModule::motion_kind(boma) == hash40("landing_heavy") {
            MotionAnimcmdModule::flush(boma, false);
        }
    }
    original!()(boma, motion_hash, start_frame, arg4, arg5, arg6, arg7, arg8)
}

#[skyline::hook(replace=MotionModule::change_motion_inherit_frame)]
unsafe fn change_motion_inherit_frame_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40, arg3: f32, arg4: f32, arg5: f32, arg6: bool, arg7: bool) -> u64 {
    change_motion_pos_shift_check(boma);
    if boma.is_fighter()
    && (VarModule::is_flag(boma.object(), vars::common::instance::FLUSH_EFFECT_ACMD)
        || StatusModule::is_changing(boma))
    {
        MotionAnimcmdModule::flush(boma, false);
    }
    original!()(boma, motion_hash, arg3, arg4, arg5, arg6, arg7)
}

#[skyline::hook(replace=MotionModule::change_motion_inherit_frame_keep_rate)]
unsafe fn change_motion_inherit_frame_keep_rate_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40, arg3: f32, arg4: f32, arg5: f32) -> u64 {
    change_motion_pos_shift_check(boma);
    if boma.is_fighter()
    && (VarModule::is_flag(boma.object(), vars::common::instance::FLUSH_EFFECT_ACMD)
        || StatusModule::is_changing(boma))
    {
        MotionAnimcmdModule::flush(boma, false);
    }
    original!()(boma, motion_hash, arg3, arg4, arg5)
}

#[skyline::hook(replace=MotionModule::change_motion_force_inherit_frame)]
unsafe fn change_motion_force_inherit_frame_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40, arg3: f32, arg4: f32, arg5: f32) -> u64 {
    change_motion_pos_shift_check(boma);
    original!()(boma, motion_hash, arg3, arg4, arg5)
}

#[skyline::hook(replace=MotionModule::change_motion_kind)]
unsafe fn change_motion_kind_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40) -> u64 {
    change_motion_pos_shift_check(boma);
    original!()(boma, motion_hash)
}

unsafe fn change_motion_pos_shift_check(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_fighter() {
        if boma.is_prev_situation(*SITUATION_KIND_AIR)
        && boma.is_situation(*SITUATION_KIND_GROUND)
        {
            if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_MOTION {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
            }
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        change_motion_hook,
        change_motion_inherit_frame_hook,
        change_motion_inherit_frame_keep_rate_hook,
        change_motion_force_inherit_frame_hook,
        change_motion_kind_hook,
    );
}
