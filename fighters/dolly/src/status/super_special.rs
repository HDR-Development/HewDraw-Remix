use super::*;
use globals::*;

// FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL //

pub unsafe extern "C" fn pre_superspecial(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Only use meter if you didn't cancel directly from a different super
    if !VarModule::is_flag(fighter.battle_object, vars::dolly::instance::SUPER_CANCEL) {
        MeterModule::drain(fighter.battle_object, 4);
    }
    smashline::original_status(Pre, fighter, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, pre_superspecial);
}