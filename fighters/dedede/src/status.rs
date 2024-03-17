use super::*;
use globals::*;

mod special_hi;
mod special_lw;

unsafe extern "C" fn on_start(agent: &mut L2CFighterCommon){
    VarModule::set_int(agent.battle_object, vars::dedede::instance::RECATCH_COUNTER, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_hi::install(agent);

    special_lw::install(agent);
}