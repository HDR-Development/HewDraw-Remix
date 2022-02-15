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
    
    if boma.is_fighter() {
        let fighter_kind = boma.kind();
        if (fighter_kind == *FIGHTER_KIND_CLOUD && [hash40("special_hi2"), hash40("special_hi2_fall")].contains(&motion_hash.hash))
        || (fighter_kind == *FIGHTER_KIND_KOOPA && [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_hash.hash))
        || (fighter_kind == *FIGHTER_KIND_YOSHI && [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_hash.hash)) {
            // Don't clear hitboxes
        } else {
            AttackModule::clear_all(boma);
        }
    }
    original!()(boma, motion_hash, arg3, arg4, arg5, arg6, arg7, arg8)
}

#[skyline::hook(replace=MotionModule::change_motion_inherit_frame)]
unsafe fn change_motion_inherit_frame_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40, arg3: f32, arg4: f32, arg5: f32, arg6: bool, arg7: bool) -> u64 {
    clear_all(boma);
    original!()(boma, motion_hash, arg3, arg4, arg5, arg6, arg7)
}

#[skyline::hook(replace=MotionModule::change_motion_inherit_frame_keep_rate)]
unsafe fn change_motion_inherit_frame_keep_rate_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40, arg3: f32, arg4: f32, arg5: f32) -> u64 {
    clear_all(boma);
    original!()(boma, motion_hash, arg3, arg4, arg5)
}

#[skyline::hook(replace=MotionModule::change_motion_force_inherit_frame)]
unsafe fn change_motion_force_inherit_frame_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40, arg3: f32, arg4: f32, arg5: f32) -> u64 {
    clear_all(boma);
    original!()(boma, motion_hash, arg3, arg4, arg5)
}

#[skyline::hook(replace=MotionModule::change_motion_kind)]
unsafe fn change_motion_kind_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40) -> u64 {
    clear_all(boma);
    original!()(boma, motion_hash)
}

unsafe fn clear_all(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_fighter() {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    skyline::install_hooks!(
        //change_motion_hook,
        //change_motion_inherit_frame_hook,
        //change_motion_inherit_frame_keep_rate_hook,
        //change_motion_force_inherit_frame_hook,
        //change_motion_kind_hook,
    );
}
