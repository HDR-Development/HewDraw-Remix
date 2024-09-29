use super::*;
use globals::*;
// status script import

mod special_hi;
mod special_s;
mod special_lw;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon){
    VarModule::set_int(fighter.battle_object, vars::dedede::instance::SPECIAL_S_RECATCH_COUNT, 0);
    VarModule::set_flag(fighter.battle_object, vars::dedede::instance::SPECIAL_LW_CONTINUE_SPIN, false);
    VarModule::set_int(fighter.battle_object, vars::dedede::instance::SPECIAL_LW_CHARGE_FRAME, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_hi::install(agent);
    special_s::install(agent);
    special_lw::install(agent);
}