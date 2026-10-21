use super::*;

// WeaponVtables::common::weapon_on_attack_mask
#[skyline::from_offset(0x33bdf70)]
unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut app::Weapon, collision_mask: u32) -> u64;

// WeaponVtableSlot::WeaponGenerate3
unsafe extern "C" fn pfushigisou_seed_init(_vtable: u64, _weapon: *mut app::Weapon) -> u64 {
    // nothing
    return 0;
}

// WeaponVtableSlot::WeaponOnAttackMask
unsafe extern "C" fn pfushigisou_seed_on_hit(vtable: u64, weapon: *mut app::Weapon, collision_mask: u32) -> u64 {
    let boma = (&mut *(weapon)).battle_object.boma();
    if !boma.is_status_one_of(&[*WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH, *WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH_GROUND]) {
        if collision_mask as i32 & (*COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) != 0 {
            boma.change_status_req(*WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH, false);
            return 0;
        }
    }
    else {
        return 0;
    }

    return normal_weapon_hit_handler(vtable, weapon, collision_mask);
}

pub fn install() {
    // enables Bullet Seed to be reflected
    enable_weapon_module(*WEAPON_KIND_PFUSHIGISOU_SEED, WeaponModule::Reflect);

    replace_weapon_vtable(
        *WEAPON_KIND_PFUSHIGISOU_SEED,
        WeaponVtableSlot::WeaponGenerate3,
        pfushigisou_seed_init as *const () as _
    );
    replace_weapon_vtable(
        *WEAPON_KIND_PFUSHIGISOU_SEED,
        WeaponVtableSlot::WeaponOnAttackMask,
        pfushigisou_seed_on_hit as *const () as _
    );
}
