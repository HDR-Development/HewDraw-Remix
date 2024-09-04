use super::*;
use globals::*;
// status script import

mod special_hi;
mod special_n;
mod special_s;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::common::instance::SPECIAL_STALL_USED) && fighter.is_situation(*SITUATION_KIND_GROUND) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::SPECIAL_STALL_USED);
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