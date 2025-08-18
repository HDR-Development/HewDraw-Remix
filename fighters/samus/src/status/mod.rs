use super::*;
use globals::*;
// status script import

mod landing_fall_special;
mod special_lw;
mod special_n;
mod special_s;
//mod squat;

unsafe extern "C" fn use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, vars::samus::instance::SPECIAL_LW_BOMB_LOCKOUT) > 0 {
        return false.into();
    }

    return true.into();
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(use_special_lw_callback as *const () as _));
    VarModule::set_int(fighter.battle_object, vars::samus::instance::SPECIAL_LW_BOMB_LOCKOUT, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    landing_fall_special::install(agent);
    special_lw::install(agent);
    special_n::install(agent);
    special_s::install(agent);
    //squat::install(agent);
}