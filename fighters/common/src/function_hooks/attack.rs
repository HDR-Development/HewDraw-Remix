use super::*;
use utils::ext::*;
use std::arch::asm;


#[skyline::hook(offset = 0x3dc160)]
unsafe fn attack_module_set_attack(module: u64, id: i32, group: i32, data: &mut smash2::app::AttackData) {
    let boma = *(module as *mut *mut BattleObjectModuleAccessor).add(1);

    // if a hitbox does not intentionally trip 100% of time, remove random trip chance
    if data.slip < 1.0 {
        // -1.0 trip chance prevents any tripping whatsoever
        data.slip = -1.0;
    }

    // Reduce strength of getup attacks
    if (*boma).is_status(*FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK) {
        data.power = 6.0;  // damage
        data.vector = 361;  // angle
        data.r_eff = 50;  // KBG
        data.r_add = 80;  // BKB
        data.sub_shield = 0;  // shield damage modifier
        data.lr_check = smash2::app::AttackLRCheck::Pos; // always allow reverse hit
    }
    if (*boma).is_status(*FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK) {
        data.power = 5.0;
        data.vector = 361;
        data.r_eff = 50;
        data.r_add = 80;
        data.sub_shield = 0;
        data.lr_check = smash2::app::AttackLRCheck::Pos;
    }
    if (*boma).is_status(*FIGHTER_STATUS_KIND_CLIFF_ATTACK) {
        data.power = 8.0;
        data.vector = 361;
        data.r_eff = 50;
        data.r_add = 70;
        data.sub_shield = 0;
    }
    
    call_original!(module, id, group, data)
}

pub fn install() {
    skyline::install_hooks!(attack_module_set_attack);
}