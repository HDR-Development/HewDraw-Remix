use super::*;

unsafe extern "C" fn attack_lw3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("samusd_win3_aura"), false, true);
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_LW3)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_LW3, attack_lw3_end);
}