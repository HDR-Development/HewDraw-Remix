use super::*;
use globals::*;

utils::import_noreturn!(common::opff::{fighter_common_opff, b_reverse});

unsafe fn dspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_INT_CANCEL_STATUS);
    }
}

// Cloud Limit Charge start and release B-Reverse
unsafe fn limit_charge_start_b_rev(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) {
        common::opff::b_reverse(fighter);
    }
}

#[utils::macros::opff(FIGHTER_KIND_CLOUD)]
pub unsafe fn cloud_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);

    dspecial_cancels(fighter);
    limit_charge_start_b_rev(fighter);
}