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

// Shoving this here for now until collision.rs gets merged
// TODO: move to collision.rs
#[skyline::hook(offset = 0x6ca950)]
unsafe fn ground_module_update_hook(ground_module: u64) {
    call_original!(ground_module);
    let boma = *((ground_module + 0x20) as *mut *mut BattleObjectModuleAccessor);
    let line = *((ground_module + 0x28) as *mut *mut f32);
    let shift = VarModule::get_float((*boma).object(), vars::common::instance::ECB_Y_OFFSETS);
    if !(*boma).is_status(*FIGHTER_STATUS_KIND_ENTRY)
    && (*boma).is_situation(*SITUATION_KIND_AIR)
    && shift != 0.0 {
        *line.add(0x3D4 / 4) += shift;
    }
}

pub fn install() {
    skyline::install_hooks!(attack_module_set_attack, ground_module_update_hook);
}