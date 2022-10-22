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
    change_motion_pos_shift_check(boma);
    original!()(boma, motion_hash, arg3, arg4, arg5, arg6, arg7)
}

#[skyline::hook(replace=MotionModule::change_motion_inherit_frame_keep_rate)]
unsafe fn change_motion_inherit_frame_keep_rate_hook(boma: &mut BattleObjectModuleAccessor, motion_hash: smash::phx::Hash40, arg3: f32, arg4: f32, arg5: f32) -> u64 {
    change_motion_pos_shift_check(boma);
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
    // Corrects your vertical positioning on landing:
    // Our aerial ECB shift code currently runs a frame "late"
    // which causes characters to appear stuck halfway into the ground on the first frame they land
    // so we need to shift your character's position up to the proper height for that single frame
    if boma.is_fighter() {
        if VarModule::get_float(boma.object(), vars::common::instance::ECB_Y_OFFSETS) != 0.0
        && !(*boma).is_status_one_of(&[
            *FIGHTER_STATUS_KIND_ENTRY,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_THROWN])
        && boma.is_situation(*SITUATION_KIND_GROUND)
        && boma.is_prev_situation(*SITUATION_KIND_AIR)
        {
            let mut fighter_pos = Vector3f {
                x: PostureModule::pos_x(boma),
                y: PostureModule::pos_y(boma),
                z: PostureModule::pos_z(boma)
            };
            fighter_pos.y += VarModule::get_float(boma.object(), vars::common::instance::ECB_Y_OFFSETS);
            PostureModule::set_pos(boma, &fighter_pos);
            VarModule::set_float(boma.object(), vars::common::instance::ECB_Y_OFFSETS, 0.0);
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
