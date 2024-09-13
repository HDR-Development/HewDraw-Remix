use super::*;
use globals::*;
// status script import

mod special_hi;
mod special_n;
mod special_s;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_ENABLE_FREEFALL);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_hi::install(agent);
    special_n::install(agent);
    special_s::install(agent);
}