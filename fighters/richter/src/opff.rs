// opff import
utils::import_noreturn!(common::opff::{fighter_common_opff, check_b_reverse});
use super::*;
use globals::*;

// Richter Holy Water B-Reverse
unsafe fn holy_water_b_rev(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) {
        common::opff::check_b_reverse(fighter);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon) {
    holy_water_b_rev(fighter);
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