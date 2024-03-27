use super::*;

// FIGHTER_STATUS_KIND_ATTACK_AIR

pub unsafe extern "C" fn attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::djc::attack_air_main_status(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air);
}