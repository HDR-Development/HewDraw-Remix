use super::*;
use globals::*;
// status script import

mod special_n;
mod special_s;

unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::shulk::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_LANDING])
    {
        VarModule::off_flag(fighter.battle_object, vars::shulk::instance::DISABLE_SPECIAL_S);
    }

    0.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
    VarModule::set_int(fighter.object(), vars::shulk::instance::SPECIAL_S_STEP, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_n::install(agent);
    special_s::install(agent);
}
