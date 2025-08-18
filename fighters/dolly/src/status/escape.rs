use super::*;

// FIGHTER_STATUS_KIND_LANDING

pub unsafe extern "C" fn escape_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Escape()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ESCAPE, escape_main);
}