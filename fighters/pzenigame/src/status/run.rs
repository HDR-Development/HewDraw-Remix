use super::*;

// FIGHTER_STATUS_KIND_RUN

pub unsafe extern "C" fn run_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Run();
    MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_RUN, run_end);
}
