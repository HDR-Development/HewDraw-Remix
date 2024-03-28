use super::*;

/// cargo carry

unsafe extern "C" fn shoulder_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.boma(), Hash40::new("shoulder_landing"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(shoulder_landing_main_loop as *const () as _))
}

unsafe extern "C" fn shoulder_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if you are in the air, somehow during landing status, transition into fall
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_SHOULDER_FALL.into(), false.into());
        return 1.into();
    }

    // if its been long enough, transition into wait
    // HDR only: reduce the landing lag
    if fighter.motion_frame() > MotionModule::end_frame(fighter.boma()) / 4.0 {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT.into(), false.into());
            return 1.into();
        }
    }

    // no action was taken
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_LANDING, shoulder_landing_main);
}
