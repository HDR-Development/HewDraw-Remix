use super::*;

utils::import!(common::opff::fighter_common_opff);

unsafe fn nspecial_cancels(fighter: &mut L2CFighterCommon) {
    //PM-like neutral-b canceling
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_CANCEL)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    {
        WorkModule::set_int(fighter.module_accessor, *STATUS_KIND_NONE, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
}

// Hero dash cancel Frizz
unsafe fn dash_cancel_frizz(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_SHOOT)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.is_motion_one_of(&[Hash40::new("special_n1"), Hash40::new("special_n2")])
    && fighter.motion_frame() > 17.0 
    {
        if fighter.is_cat_flag(Cat1::Walk) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_DASH, false);
        } else if fighter.is_cat_flag(Cat1::Turn) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_TURN_DASH, false);
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_BRAVE )]
pub unsafe fn brave_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);

    nspecial_cancels(fighter);
    dash_cancel_frizz(fighter);

    // Extend sword length
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("sword1"), &Vector3f::new(1.125, 1.05, 1.045));
}