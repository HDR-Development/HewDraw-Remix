use super::*;
 
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn blue_eggs_land_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.is_prev_situation(*SITUATION_KIND_AIR)
    {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING, false);
    }
}

// Banjo Grenade Airdodge Cancel
unsafe fn grenade_ac(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.is_status_one_of(&[*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_LW_SHOOT, *FIGHTER_STATUS_KIND_SPECIAL_LW])
    && fighter.motion_frame() > 15.0
    && fighter.is_cat_flag(Cat1::AirEscape)
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
    {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
    }
}

#[utils::macros::opff(FIGHTER_KIND_BUDDY)]
pub unsafe fn buddy_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);

    blue_eggs_land_cancels(fighter);
    grenade_ac(fighter);
}