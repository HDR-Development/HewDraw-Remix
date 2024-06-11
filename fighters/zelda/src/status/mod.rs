use super::*;
use globals::*;
// status script import
mod special_hi;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::zelda::instance::EFF_COOLDOWN_HANDLER, -1); //phantom
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    special_hi::install(agent);
}
