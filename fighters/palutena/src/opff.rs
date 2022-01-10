use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    teleports::palutena_teleport_cancel(boma, id, status_kind, situation_kind, cat[0]);
}

#[utils::opff(FIGHTER_KIND_PALUTENA )]
pub fn palutena_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		palutena_frame(fighter)
    }
}

pub unsafe fn palutena_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}