use super::*;
 
utils::import_noreturn!(common::opff::fighter_common_opff);

#[utils::macros::opff(FIGHTER_KIND_DAISY )]
pub unsafe fn daisy_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
}