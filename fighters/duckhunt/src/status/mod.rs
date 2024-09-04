use super::*;
use globals::*;
// status script import

mod attacks3;
mod special_hi;
mod special_n;
mod special_s;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.object(), vars::duckhunt::instance::SPECIAL_HI2_ENABLE);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
    VarModule::off_flag(fighter.object(), vars::duckhunt::instance::SPECIAL_HI2_ENABLE);
    VarModule::set_int(fighter.battle_object, vars::duckhunt::instance::GUNMAN_TIMER, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attacks3::install(agent);
    special_hi::install(agent);
    special_n::install(agent);
    special_s::install(agent);
}