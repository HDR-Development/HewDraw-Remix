use super::*;
use globals::*;
// status script import

mod special_s_jump;
mod special_hi_escape;
mod special_hi_damage;
mod rebirth;
 
// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::koopajr::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

// Re-enables the ability to use sideB when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::koopajr::instance::DISABLE_SPECIAL_S);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
    
    // clear mechakoopa cooldown on reset
    VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
    VarModule::off_flag(fighter.battle_object, vars::koopajr::instance::MECHAKOOPA_IS_COOLDOWN);
    VarModule::off_flag(fighter.battle_object, vars::koopajr::instance::DISABLE_MECHAKOOPA);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_s_jump::install(agent);
    special_hi_escape::install(agent);
    special_hi_damage::install(agent);
    rebirth::install(agent);
}