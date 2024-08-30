use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Once per airtime
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::koopajr::instance::DISABLE_SPECIAL_S);
    }
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s);
}