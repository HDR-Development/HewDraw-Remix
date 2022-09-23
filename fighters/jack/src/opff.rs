// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn wings_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, true);
        }
    }
}

// Joker Arsene Grappling Hook
unsafe fn arsene_grappling_hook(boma: &mut BattleObjectModuleAccessor, situation_kind: i32, motion_kind: u64) {
    if motion_kind == hash40("special_hi_start") {//&& situation_kind == *SITUATION_KIND_GROUND {
        MotionModule::change_motion_kind(boma, smash::phx::Hash40::new("special_hi"));
    }
}

// Joker Aerial Grappling Hook stall
unsafe fn aerial_grappling_hook_stall(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("special_air_hi_throw") {
        if frame < 36.0 {
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        if frame >= 36.0 {
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
}

// Lengthen knife
unsafe fn knife_length(boma: &mut BattleObjectModuleAccessor) {
	let long_sword_scale = Vector3f{x: 1.01, y: 1.1, z: 1.01};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("knife"), &long_sword_scale);
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    wings_cancel(boma, status_kind);
    arsene_grappling_hook(boma, situation_kind, motion_kind);
    aerial_grappling_hook_stall(boma, motion_kind, frame);
	knife_length(boma);
}

#[utils::macros::opff(FIGHTER_KIND_JACK )]
pub fn jack_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		jack_frame(fighter)
    }
}

pub unsafe fn jack_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}