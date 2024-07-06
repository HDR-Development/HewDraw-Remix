use super::*;
use globals::*;

unsafe extern "C" fn wait_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let ret = smashline::original_status(Main, weapon, *WEAPON_PACMAN_TRAMPOLINE_STATUS_KIND_WAIT)(weapon);
    //AreaModule::clean(weapon.module_accessor);
    AreaModule::reset_area(weapon.module_accessor, *WEAPON_PACMAN_TRAMPOLINE_AREA_KIND_BODY);
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_PACMAN_TRAMPOLINE_STATUS_KIND_WAIT, wait_main);
}