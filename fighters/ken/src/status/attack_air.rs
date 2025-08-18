use super::*;

// FIGHTER_STATUS_KIND_ATTACK_AIR

pub unsafe extern "C" fn attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_spin_wind"), false, false);
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_end);
}