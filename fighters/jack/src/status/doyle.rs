use super::*;

#[status_script(agent = "jack_doyle", status = WEAPON_JACK_DOYLE_STATUS_KIND_ENTRY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn entry_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = LinkModule::get_parent_id(weapon.module_accessor, *LINK_NO_CONSTRAINT, true) as u32;
    let owner = sv_battle_object::module_accessor(owner_id);
    let rebel_gauge = WorkModule::get_float(owner, 0x4D);
    WorkModule::mul_float(weapon.module_accessor, rebel_gauge / 100.0, 0x6);
    original!(weapon)
}

pub fn install() {
    smashline::install_status_scripts!(
        entry_main
    );
}