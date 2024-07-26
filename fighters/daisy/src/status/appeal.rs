use super::*;

// FIGHTER_STATUS_KIND_APPEAL

unsafe extern "C" fn appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_steam3"), false, false);
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_APPEAL)(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_APPEAL, appeal_end);
}