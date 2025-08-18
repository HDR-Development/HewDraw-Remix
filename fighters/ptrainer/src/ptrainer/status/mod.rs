use super::*;
use globals::*;
// status script import

mod special_lw;

unsafe extern "C" fn on_start(weapon: &mut L2CWeaponCommon) {
    VarModule::set_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_lw::install(agent);
}