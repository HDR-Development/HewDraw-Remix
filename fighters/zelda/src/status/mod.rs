use super::*;
use globals::*;
// status script import

mod special_hi;
mod special_lw;
mod special_s;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::zelda::instance::SPECIAL_LW_COOLDOWN_EFFECT_HANDLE, -1); //phantom
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_hi::install(agent);
    special_lw::install(agent);
    special_s::install(agent);
}
