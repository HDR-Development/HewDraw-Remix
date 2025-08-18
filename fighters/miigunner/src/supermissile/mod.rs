use super::*;

mod acmd;
mod status;

unsafe extern "C" fn on_start(weapon: &mut L2CWeaponCommon) {
    VarModule::off_flag(weapon.battle_object, vars::miigunner_supermissile::instance::ENABLE_PULSE);
    VarModule::off_flag(weapon.battle_object, vars::miigunner_supermissile::instance::PULSE_DETONATE);
}

pub fn install() {
    let agent = &mut Agent::new("miigunner_supermissile");

    agent.on_start(on_start);

    acmd::install(agent);
    status::install(agent);
    agent.install();
}