use super::*;

// FIGHTER_STATUS_KIND_ATTACK_S4

// override vanilla shield visibility on forward smash
unsafe extern "C" fn attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS4()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, attack_s4_main);
}
