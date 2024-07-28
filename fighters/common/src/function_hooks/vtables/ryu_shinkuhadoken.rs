use super::*;

unsafe extern "C" fn shinku_on_hit(vtable: u64, weapon: *mut app::Weapon, something: u32) -> u64 {
    *(weapon as *mut bool).add(0x90) = true;
    normal_weapon_hit_handler(vtable, weapon, something)
}

#[skyline::from_offset(0x33bdc10)]
unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut app::Weapon, something: u32) -> u64;

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x5215940).data(shinku_on_hit as u64);
}