use super::*;
use globals::*;

utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn dspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_INT_CANCEL_STATUS);
    }
}

// Fixes bug where touching ground during upB puts you into special fall for 1f before landing
unsafe fn up_special_proper_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL);
    }
}

#[utils::macros::opff(FIGHTER_KIND_CLOUD)]
pub unsafe fn cloud_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);

    dspecial_cancels(fighter);
    up_special_proper_landing(fighter);
}