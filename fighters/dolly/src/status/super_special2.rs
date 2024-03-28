use super::*;

// FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2

pub unsafe extern "C" fn pre_superspecial2(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Only use meter if you didn't cancel directly from a different supper
    if !VarModule::is_flag(fighter.battle_object, vars::dolly::instance::SUPER_CANCEL) {
        MeterModule::drain(fighter.battle_object, 4);
    }
    smashline::original_status(Pre, fighter, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, pre_superspecial2);
}