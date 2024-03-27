use super::*;
use globals::*;
// status script import

mod attacks3;
mod special_n;
mod special_s;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::duckhunt::instance::GUNMAN_TIMER, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attacks3::install(agent);
    special_n::install(agent);
    special_s::install(agent);
}
