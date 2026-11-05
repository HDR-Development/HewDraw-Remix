use super::*;

// WeaponVtables::common::weapon_on_attack_mask
#[skyline::from_offset(0x33bdf70)]
unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut app::Weapon, something: u32) -> u64;

// WeaponVtableSlot::WeaponOnAttackMask
unsafe extern "C" fn shinku_on_hit(vtable: u64, weapon: *mut app::Weapon, something: u32) -> u64 {
    *(weapon as *mut bool).add(0x90) = true;
    normal_weapon_hit_handler(vtable, weapon, something)
}

pub fn install() {
    replace_weapon_vtable(
        *WEAPON_KIND_RYU_SHINKUHADOKEN,
        WeaponVtableSlot::WeaponOnAttackMask,
        shinku_on_hit as *const () as _
    );
}
