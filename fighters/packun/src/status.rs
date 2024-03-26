use super::*;
use globals::*;

mod special_hi;
mod special_s;
mod attack_s4;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_s::install(agent);
    special_hi::install(agent);
    attack_s4::install(agent);
}