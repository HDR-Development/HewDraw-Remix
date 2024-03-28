use super::*;

// FIGHTER_STATUS_KIND_APPEAL

// stub out mask removal at the end of taunt
unsafe extern "C" fn appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    1.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_APPEAL, appeal_end);
}