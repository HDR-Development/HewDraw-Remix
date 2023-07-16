// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


pub unsafe fn moveset(fighter: &mut L2CFighterCommon) {
}

#[utils::macros::opff(FIGHTER_KIND_RICHTER )]
pub fn richter_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		richter_frame(fighter)
    }
}

pub unsafe fn richter_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter);
    }
}