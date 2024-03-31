use super::*;

pub unsafe extern "C" fn entry_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = LinkModule::get_parent_id(weapon.module_accessor, *LINK_NO_CONSTRAINT, true) as u32;
    let owner = sv_battle_object::module_accessor(owner_id);
    let rebel_gauge = WorkModule::get_float(owner, 0x4D);
    WorkModule::mul_float(weapon.module_accessor, rebel_gauge / 100.0, 0x6);
    smashline::original_status(Main, weapon, *WEAPON_JACK_DOYLE_STATUS_KIND_ENTRY)(weapon)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_JACK_DOYLE_STATUS_KIND_ENTRY, entry_main);
}
