use super::*;
use utils::ext::*;
use std::arch::asm;


#[skyline::hook(offset = 0x3dc160)]
unsafe fn attack_module_set_attack(module: u64, id: i32, group: i32, data: &mut smash2::app::AttackData) {
    // if a hitbox does not intentionally trip 100% of time, remove random trip chance
    if data.slip < 1.0 {
        // -1.0 trip chance prevents any tripping whatsoever
        data.slip = -1.0;
    }
    call_original!(module, id, group, data)
}

pub fn install() {
    skyline::install_hooks!(attack_module_set_attack);
}