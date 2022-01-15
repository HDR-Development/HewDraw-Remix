use super::*;

utils::import_noreturn!(common::opff::fighter_common_opff);

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
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END
    ])
    && fighter.motion_frame() < 5.0
    && fighter.is_stick_backward()
    {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        if VarModule::is_flag(fighter.battle_object, vars::common::B_REVERSED) {
            return;
        }
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(-1.0, 1.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        VarModule::on_flag(fighter.battle_object, vars::common::B_REVERSED);
    }
}

#[utils::macros::opff(FIGHTER_KIND_CLOUD)]
pub unsafe fn cloud_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);

    dspecial_cancels(fighter);
    limit_charge_start_b_rev(fighter);
}