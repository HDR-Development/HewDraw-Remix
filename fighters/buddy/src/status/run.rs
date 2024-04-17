use super::*;

// FIGHTER_STATUS_KIND_RUN

pub unsafe extern "C" fn run_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, false);
    fighter.status_end_Run();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_RUN, run_end);
}