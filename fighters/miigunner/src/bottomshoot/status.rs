use super::*;

unsafe extern "C" fn shoot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ModelModule::set_scale(weapon.module_accessor, 0.7);
    return smashline::original_status(Main, weapon, *WEAPON_MIIGUNNER_BOTTOMSHOOT_STATUS_KIND_SHOOT)(weapon);
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_MIIGUNNER_BOTTOMSHOOT_STATUS_KIND_SHOOT, shoot_main);
}