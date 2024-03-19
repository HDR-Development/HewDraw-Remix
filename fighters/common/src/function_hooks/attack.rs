use super::*;
use utils::ext::*;
use std::arch::asm;
use utils::game_modes::CustomMode;

#[skyline::hook(offset = 0x3dc180)]
unsafe fn attack_module_set_attack(module: u64, id: i32, group: i32, data: &mut smash_rs::app::AttackData) {
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
            data.lr_check = smash_rs::app::AttackLRCheck::Pos; // always allow reverse hit
        }
        if (*boma).is_status(*FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK) {
            data.power = 5.0;
            data.vector = 361;
            data.r_eff = 50;
            data.r_add = 55;
            data.sub_shield = 0;
            data.lr_check = smash_rs::app::AttackLRCheck::Pos;
        }
        if (*boma).is_status(*FIGHTER_STATUS_KIND_CLIFF_ATTACK) {
            data.power = 8.0;
            data.vector = 361;
            data.r_eff = 50;
            data.r_add = 45;
            data.sub_shield = 0;
            data.lr_check = smash_rs::app::AttackLRCheck::Pos;
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
            let damage_frame_mul: f32 = if modes.contains(&CustomMode::Smash64Mode) {
                0.533
            }
            else {
                0.42
            };
            asm!("fmov s0, w8", in("w8") damage_frame_mul)
        },
        _ => {}
    }
}

#[skyline::hook(offset = 0x406bf4, inline)]
unsafe fn get_hitstop_frame_add(ctx: &mut skyline::hooks::InlineCtx) {
    match utils::game_modes::get_custom_mode() {
        Some(modes) => {
            let hitstop_frame_add: f32 = if modes.contains(&CustomMode::Smash64Mode) {
                5.0
            }
            else {
                4.0
            };
            asm!("fmov s0, w8", in("w8") hitstop_frame_add)
        },
        _ => {}
    }
}

// Only applies 0.67 crouch cancel hitlag multiplier to defender
#[skyline::hook(offset = 0x46b648, inline)]
unsafe fn get_hitstop_mul(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[1].w.as_ref() == 0x2 {
        let hitstop_mul: f32 = 1.0;
        asm!("fmov s0, w8", in("w8") hitstop_mul)
    }
}

static mut IS_KB_CALC_EARLY: bool = false;
static mut KB: f32 = 0.0;

// This runs directly after knockback is calculated
#[skyline::hook(offset = 0x402f04, inline)]
unsafe fn post_calc_reaction(ctx: &mut skyline::hooks::InlineCtx) {
    let id = *ctx.registers[27].w.as_ref();
    let boma = &mut *(sv_battle_object::module_accessor(id));
    if boma.is_fighter() {
        let fighter = get_fighter_common_from_accessor(boma);
        let object = sv_system::battle_object(fighter.lua_state_agent);
        let fighta : *mut Fighter = std::mem::transmute(object);
    
        let mut kb: f32;
        asm!("fmov w8, s0", out("w8") kb);
        IS_KB_CALC_EARLY = true;
        KB = kb;
        let hitlag = *(((fighta as u64) + 0xf70c) as *mut i32);
        // Set hitlag for attacker
        *(((fighta as u64) + 0xf70c) as *mut i32) = (hitlag as f32 * (0.63 * std::f32::consts::E.powf(0.00462 * kb)).clamp(1.0, 2.0)).round() as i32;
        asm!("fmov s0, w8", in("w8") kb)
    }
}

// This runs immediately after an attacker's hitlag is calculated
#[skyline::hook(offset = 0x406fdc, inline)]
unsafe fn handle_on_attack_event(ctx: &mut skyline::hooks::InlineCtx) {
    if IS_KB_CALC_EARLY {
        let hitlag = *ctx.registers[0].w.as_ref();
        let kb = KB;
        // Set hitlag for attacker
        *ctx.registers[0].w.as_mut() = (hitlag as f32 * (0.63 * std::f32::consts::E.powf(0.00462 * kb)).clamp(1.0, 2.0)).round() as u32;
    }
}

// This runs immediately before hitlag is set for attacking articles
#[skyline::hook(offset = 0x33a9d90, inline)]
unsafe fn set_weapon_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
    let opponent_boma = &mut *(*ctx.registers[24].x.as_ref() as *mut BattleObjectModuleAccessor);

    let hitlag = *ctx.registers[21].w.as_ref();
    let kb = DamageModule::reaction(opponent_boma, 0);
    IS_KB_CALC_EARLY = true;
    KB = kb;
    // Set hitlag for attacking article
    *ctx.registers[21].w.as_mut() = (hitlag as f32 * (0.63 * std::f32::consts::E.powf(0.00462 * kb)).clamp(1.0, 2.0)).round() as u32;
}

// This runs immediately before hitlag is set for the defender
#[skyline::hook(offset = 0x404658, inline)]
unsafe fn set_fighter_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = &mut *(*ctx.registers[19].x.as_ref() as *mut BattleObjectModuleAccessor);

    let hitlag = *ctx.registers[0].w.as_ref();
    let kb = DamageModule::reaction(boma, 0);
    // Set hitlag for defender
    *ctx.registers[0].w.as_mut() = (hitlag as f32 * (0.63 * std::f32::consts::E.powf(0.00462 * kb)).clamp(1.0, 2.0)).round() as u32;
    IS_KB_CALC_EARLY = false;
}

pub fn install() {
    skyline::install_hooks!(
        attack_module_set_attack,
        get_damage_frame_mul,
        get_hitstop_frame_add,
        get_hitstop_mul,
        post_calc_reaction,
        set_weapon_hitlag,
        set_fighter_hitlag,
        handle_on_attack_event
    );
}