use super::*;
 
// FIGHTER_MIISWORDSMAN_STATUS_KIND_FINAL_HOLD

unsafe extern "C" fn pre_final_hold(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::WAVE_SPECIAL_N);
    smashline::original_status(Pre, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_FINAL_HOLD)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_FINAL_HOLD, pre_final_hold);
}