use super::*;

// WeaponVtables::common::weapon_on_attack_mask
#[skyline::from_offset(0x33bdf70)]
unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut app::Weapon, collision_mask: u32) -> u64;

// WeaponVtableSlot::WeaponOnAttackMask
unsafe extern "C" fn pzenigame_water_on_hit(vtable: u64, weapon: *mut app::Weapon, collision_mask: u32) -> u64 {
    let boma = (&mut *(weapon)).battle_object.boma();
    if !boma.is_status(*WEAPON_PZENIGAME_WATER_STATUS_KIND_CLASH) {
        if collision_mask as i32 & (*COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) != 0 {
            boma.change_status_req(*WEAPON_PZENIGAME_WATER_STATUS_KIND_CLASH, false);
            return 0;
        }
    }
    else {
        return 0;
    }

    return normal_weapon_hit_handler(vtable, weapon, collision_mask);
}

pub fn install() {
    replace_weapon_vtable(
        *WEAPON_KIND_PZENIGAME_WATER,
        WeaponVtableSlot::WeaponOnAttackMask,
        pzenigame_water_on_hit as *const () as _
    );
}
