// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn luigi_missle_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END]) {
        // allows ledgegrab during Luigi Missile
        fighter.sub_transition_group_check_air_cliff();
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    luigi_missle_ledgegrab(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_LUIGI )]
pub fn luigi_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		luigi_frame(fighter);
    }
}

pub unsafe fn luigi_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}