use super::*;

// FIGHTER_STATUS_KIND_LANDING

pub unsafe extern "C" fn landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HDR
    app::FighterSpecializer_Demon::update_opponent_lr_1on1(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING);
    // vanilla
    fighter.status_LandingSub();
    fighter.status_LandingStiffness();
    fighter.sub_landing_start_check_damage_face();
    // HDR
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr != 0.0 && PostureModule::lr(fighter.module_accessor) != lr {
        PostureModule::set_lr(fighter.module_accessor, lr);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(landing_main_loop as *const () as _))
}

unsafe extern "C" fn landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    // HDR
    if MotionModule::is_end(fighter.module_accessor)
    && lr != 0.0
    && PostureModule::lr(fighter.module_accessor) != lr
    {
        fighter.sub_wait_common();
    }
    // vanilla
    if !fighter.status_Landing_MainSub().get_bool() {
        fighter.sub_landing_cancel_check_damage_face();
        return 0.into();
    }
    1.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_LANDING, landing_main);
}