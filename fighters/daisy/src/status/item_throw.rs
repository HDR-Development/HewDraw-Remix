use super::*;

// FIGHTER_STATUS_KIND_ITEM_THROW

unsafe extern "C" fn item_throw_end (fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_ITEM_THROW)(fighter)
}


pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ITEM_THROW, item_throw_end);
}