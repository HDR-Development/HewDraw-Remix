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
        if !boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_DEMO,
            *FIGHTER_STATUS_KIND_ENTRY,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_THROWN])
        && boma.is_prev_situation(*SITUATION_KIND_AIR)
        && boma.is_situation(*SITUATION_KIND_GROUND)
        {
            let ecb_center = *GroundModule::get_rhombus(boma, true).add(2);
            let mut pos = *PostureModule::pos(boma);
            let mut out_pos = Vector2f::zero();
            let is_underneath_floor = GroundModule::line_segment_check(boma, &Vector2f::new(pos.x, ecb_center.y), &Vector2f::new(pos.x, pos.y), &Vector2f::zero(), &mut out_pos, true);
            if is_underneath_floor != 0 {
                pos.y = out_pos.y + 0.01;
                PostureModule::set_pos(boma, &pos);
                GroundModule::attach_ground(boma, false);
            }
        }

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
