use super::*;
use utils::ext::*;
use std::arch::asm;
use utils::game_modes::CustomMode;


#[skyline::hook(offset = 0x3dc180)]
unsafe fn attack_module_set_attack(module: u64, id: i32, group: i32, data: &mut smash2::app::AttackData) {
    let boma = *(module as *mut *mut BattleObjectModuleAccessor).add(1);

    // if a hitbox does not intentionally trip 100% of time, remove random trip chance
    if data.slip < 1.0 {
        // -1.0 trip chance prevents any tripping whatsoever
        data.slip = -1.0;
    }

    if (*boma).is_fighter() {
        // Reduce strength of getup attacks
        if (*boma).is_status(*FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK) {
            data.power = 6.0;  // damage
            data.vector = 361;  // angle
            data.r_eff = 50;  // KBG
            data.r_add = 55;  // BKB
            data.sub_shield = 0;  // shield damage modifier
            data.lr_check = smash2::app::AttackLRCheck::Pos; // always allow reverse hit
        }
        if (*boma).is_status(*FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK) {
            data.power = 5.0;
            data.vector = 361;
            data.r_eff = 50;
            data.r_add = 55;
            data.sub_shield = 0;
            data.lr_check = smash2::app::AttackLRCheck::Pos;
        }
        if (*boma).is_status(*FIGHTER_STATUS_KIND_CLIFF_ATTACK) {
            data.power = 8.0;
            data.vector = 361;
            data.r_eff = 50;
            data.r_add = 45;
            data.sub_shield = 0;
            data.lr_check = smash2::app::AttackLRCheck::Pos;
        }
        if (*boma).is_status(*FIGHTER_STATUS_KIND_CATCH_ATTACK) {
            if !VarModule::is_flag((*boma).object(), vars::common::status::PUMMEL_OVERRIDE_GLOBAL_STATS) {
                data.stop_frame = 3.5;  // hitlag mul
            }
        }
    }
    
    call_original!(module, id, group, data)
}

#[skyline::hook(offset = 0x403c3c, inline)]
unsafe fn get_damage_frame_mul(ctx: &mut skyline::hooks::InlineCtx) {
    match utils::game_modes::get_custom_mode() {
        Some(modes) => {
            if modes.contains(&CustomMode::Smash64Mode) {
                let damage_frame_mul_n64: f32 = 0.533;
                asm!("fmov s0, w8", in("w8") damage_frame_mul_n64)
            }
        },
        _ => {}
    }
}

#[skyline::hook(offset = 0x406bf4, inline)]
unsafe fn get_hitstop_frame_add(ctx: &mut skyline::hooks::InlineCtx) {
    match utils::game_modes::get_custom_mode() {
        Some(modes) => {
            if modes.contains(&CustomMode::Smash64Mode) {
                let hitstop_frame_add_n64: f32 = 5.0;
                asm!("fmov s0, w8", in("w8") hitstop_frame_add_n64)
            }
        },
        _ => {}
    }
}

pub fn install() {
    skyline::install_hooks!(
        attack_module_set_attack,
        get_damage_frame_mul,
        get_hitstop_frame_add
    );
}