use super::*;

// FIGHTER_STATUS_KIND_ATTACK_AIR

unsafe extern "C" fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_AttackAir()
} // remove vanilla rob up b -> attack mechanics

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_pre);
}
