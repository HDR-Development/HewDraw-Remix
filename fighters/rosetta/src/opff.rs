use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn launch_star_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_JUMP {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_END, false);
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    launch_star_cancel(boma, status_kind);
}

#[utils::opff(FIGHTER_KIND_ROSETTA )]
pub fn rosetta_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		rosetta_frame(fighter)
    }
}

pub unsafe fn rosetta_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}