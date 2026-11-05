use super::*;

// Ported from common's disabled `elight_bunshin` hook. The slot is assumed from the old `_008` name
// and the offsets predate 13.0.5 (0x33f5cb0 no longer resolves to a function), so this module
// stays disabled until they are re-verified.

#[skyline::from_offset(0x33f5cb0)]
unsafe extern "C" fn elight_bunshin_008_original(vtable: u64, weapon: &mut app::Weapon, param_3: u64);

// WeaponVtableSlot::WeaponGenerate3
unsafe extern "C" fn elight_bunshin_008(vtable: u64, weapon: &mut app::Weapon, param_3: u64) {
    elight_bunshin_008_original(vtable, weapon, param_3);
    let module = (weapon.battle_object.module_accessor as *mut u64).add(0x58 / 0x8);
    let vtable = *module as *const u64;
    let set_shape_kind : fn(*mut u64, u32) = std::mem::transmute(*((*vtable + 0x368) as *const u64));
    set_shape_kind(module, 2);
    let set_capsule : fn(*mut u64, f32, f32, f32, u64, bool) = std::mem::transmute(*((*vtable + 0xc8) as *const u64));
    set_capsule(module, 5.0, -5.0, 5.0, hash40("rot"), false);
}

pub fn install() {
    unsafe {
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u64;
        let _ = skyline::patching::Patch::in_text(0x5189840 + (*WEAPON_KIND_ELIGHT_BUNSHIN as usize * 0x1d * 0x8)).data(text.add(0x33b8850 / 0x8));
    }

    replace_weapon_vtable(
        *WEAPON_KIND_ELIGHT_BUNSHIN,
        WeaponVtableSlot::WeaponGenerate3,
        elight_bunshin_008 as *const () as _
    );
}
