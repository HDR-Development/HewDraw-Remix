use super::*;

// FIGHTER_STATUS_KIND_ATTACK

unsafe extern "C" fn attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Attack()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK, attack_pre);
}
