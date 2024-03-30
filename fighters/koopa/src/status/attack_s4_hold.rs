use super::*;

// FIGHTER_STATUS_KIND_ATTACK_S4_HOLD

unsafe extern "C" fn attack_s4_hold_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_explosion_sign"), false, false);
    return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD)(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attack_s4_hold_exit);
}