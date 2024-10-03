use super::*;
use globals::*;
// status script import

mod appeal;
mod attack_s4;
mod catch;
mod special_s;
mod rebirth;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::snake::instance::SPECIAL_S_AMMO_COUNT, 0);
    VarModule::off_flag(fighter.battle_object, vars::snake::instance::SPECIAL_S_FORCE_RELOAD);
    VarModule::off_flag(fighter.battle_object, vars::snake::instance::SPECIAL_S_RELOAD_VULNERABLE);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    appeal::install(agent);
    attack_s4::install(agent);
    catch::install(agent);
    special_s::install(agent);
    rebirth::install(agent);
}