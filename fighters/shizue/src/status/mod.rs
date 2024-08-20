use super::*;
use globals::*;
// status script import

mod special_hi;
mod special_s;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.object(), vars::shizue::instance::LLOID_ASYNC);
    VarModule::set_int(fighter.object(), vars::shizue::instance::LLOID_TIMER, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_hi::install(agent);
    special_s::install(agent);
}
