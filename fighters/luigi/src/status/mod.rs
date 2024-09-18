use super::*;
use globals::*;
// status script import

mod special_hi;
mod special_n;
mod special_s;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.battle_object, vars::luigi::instance::SPECIAL_S_MISFIRE_STORED);
    VarModule::set_float(fighter.battle_object, vars::luigi::instance::SPECIAL_S_MISFIRE_DAMAGE_MUL, 1.0);
    VarModule::set_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_SMOKE_EFFECT_HANDLE, -1);
    VarModule::set_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_PULSE_EFFECT_HANDLE, -1);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_hi::install(agent);
    special_n::install(agent);
    special_s::install(agent);
}
