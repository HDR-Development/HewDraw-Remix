use super::*;

// WeaponVtableSlot::Unk56
// Only allows the tornado to be pocketed while in its first status.
unsafe extern "C" fn tornadoshot_can_pocket(_vtable: u64, weapon: &mut app::Weapon) -> bool {
    let module_accessor = weapon.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    status == 0
}

pub fn install() {
    replace_weapon_vtable(
        *WEAPON_KIND_MIISWORDSMAN_TORNADOSHOT,
        WeaponVtableSlot::Unk56,
        tornadoshot_can_pocket as *const () as _
    );
}
