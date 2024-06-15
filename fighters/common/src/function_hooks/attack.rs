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

unsafe extern "C" fn calc_hitlag_mul(boma: &mut BattleObjectModuleAccessor, kb: f32) -> f32 {
    let mul = (0.414 * std::f32::consts::E.powf(0.0063 * kb)).clamp(1.0, 2.0);
    return mul;
}

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
        let max_hitlag = WorkModule::get_param_float(boma, hash40("battle_object"), hash40("hitstop_frame_max"));
        let attack_data = (*ctx.registers[22].x.as_ref() as *mut smash_rs::app::AttackData);
        let attr: smashline::Hash40 = std::mem::transmute((*attack_data).attr);
        if ![Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_saving")].contains(&attr) {
            // Set hitlag for attacker
            *(((fighta as u64) + 0xf70c) as *mut i32) = (hitlag as f32 * calc_hitlag_mul(boma, kb)).round().min(max_hitlag) as i32;
        }
        asm!("fmov s0, w8", in("w8") kb)
    }
}

// This runs immediately after an attacker's hitlag is calculated
#[skyline::hook(offset = 0x406fdc, inline)]
unsafe fn handle_on_attack_event(ctx: &mut skyline::hooks::InlineCtx) {
    if IS_KB_CALC_EARLY {
        let boma = &mut *(*ctx.registers[23].x.as_ref() as *mut BattleObjectModuleAccessor);
        let hitlag = *ctx.registers[0].w.as_ref();
        let kb = KB;
        let max_hitlag = WorkModule::get_param_float(boma, hash40("battle_object"), hash40("hitstop_frame_max"));
        let attack_data = (*ctx.registers[24].x.as_ref() as *mut smash_rs::app::AttackData);
        let attr: smashline::Hash40 = std::mem::transmute((*attack_data).attr);
        if ![Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_saving")].contains(&attr) {
            // Set hitlag for attacker
            *ctx.registers[0].w.as_mut() = (hitlag as f32 * calc_hitlag_mul(boma, kb)).round().min(max_hitlag) as u32;
        }
    }
}

// This runs immediately before hitlag is set for attacking articles
#[skyline::hook(offset = 0x33a9d90, inline)]
unsafe fn set_weapon_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
    let opponent_boma = &mut *(*ctx.registers[24].x.as_ref() as *mut BattleObjectModuleAccessor);
    if !opponent_boma.is_item() {
        let hitlag = *ctx.registers[21].w.as_ref();
        let kb = DamageModule::reaction(opponent_boma, 0);
        IS_KB_CALC_EARLY = true;
        KB = kb;
        let max_hitlag = WorkModule::get_param_float(opponent_boma, hash40("battle_object"), hash40("hitstop_frame_max"));
        let attack_data = (*ctx.registers[20].x.as_ref() as *mut smash_rs::app::AttackData);
        let attr: smashline::Hash40 = std::mem::transmute((*attack_data).attr);
        if ![Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_saving")].contains(&attr) {
            // Set hitlag for attacking article
            *ctx.registers[21].w.as_mut() = (hitlag as f32 * calc_hitlag_mul(opponent_boma, kb)).round().min(max_hitlag) as u32;
        }
    }
}

// This runs immediately before hitlag is set for the defender
#[skyline::hook(offset = 0x404658, inline)]
unsafe fn set_fighter_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = &mut *(*ctx.registers[19].x.as_ref() as *mut BattleObjectModuleAccessor);
    if !boma.is_item() {
        let hitlag = *ctx.registers[0].w.as_ref();
        let kb = DamageModule::reaction(boma, 0);
        let mut max_hitlag = WorkModule::get_param_float(boma, hash40("battle_object"), hash40("hitstop_frame_max"));
        let attr = *((*ctx.registers[20].x.as_ref() + 0xb8) as *mut u64);
        
        if [hash40("collision_attr_elec"),].contains(&attr) {
            max_hitlag *= WorkModule::get_param_float(boma, hash40("battle_object"), hash40("hitstop_elec_mul"));
        }
        if ![hash40("collision_attr_paralyze"), hash40("collision_attr_saving")].contains(&attr) {
            // Set hitlag for defender
            *ctx.registers[0].w.as_mut() = (hitlag as f32 * calc_hitlag_mul(boma, kb)).round().min(max_hitlag) as u32;
        }
    }
    IS_KB_CALC_EARLY = false;
}

// this code sets hitlag on parry to a static amount
static mut IS_HITLAG_FOR_PARRY: bool = false;
// check if defender is parrying and set a flag for later
#[skyline::hook(offset = 0x0627880)]
unsafe fn x0627880(battle_object: *mut BattleObject, arg1: u64) {
    let collision_event = *(arg1 as *const *const u32).add(2);
    let opponent_battle_object_id = *collision_event.add(9);
    let opponent_battle_object = utils::util::get_battle_object_from_id(opponent_battle_object_id);
    let opponent_boma = (&mut *(*opponent_battle_object).module_accessor);
    IS_HITLAG_FOR_PARRY = 
        opponent_boma.is_status(*FIGHTER_STATUS_KIND_GUARD_OFF)
        && VarModule::is_flag(opponent_battle_object, vars::common::instance::IS_PARRY_FOR_GUARD_OFF)
        && opponent_boma.get_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME) > 0;
    call_original!(battle_object, arg1)
}
// if defender is parrying, set attacker hitlag to static amount
#[skyline::hook(offset = 0x0627cc0, inline)]
unsafe fn x0627cc0(ctx: &mut skyline::hooks::InlineCtx) {
    if IS_HITLAG_FOR_PARRY {
        IS_HITLAG_FOR_PARRY = false;
        *ctx.registers[8].x.as_mut() = 18;
    }
}
// set defender hitlag
#[skyline::hook(offset = 0x0641948, inline)]
unsafe fn x0641948(ctx: &mut skyline::hooks::InlineCtx) {
    let hitlag = 10;
    let battle_object = &mut *(*ctx.registers[19].x.as_ref() as *mut BattleObject);
    battle_object.set_float(hitlag as f32, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_HIT_STOP_FRAME);
    let fighter = *ctx.registers[19].x.as_ref();
    *(fighter as *mut i32).add(0xf740 / 4) = hitlag;
}
// set parry AttackModule inflict flag
#[skyline::hook(offset = 0x03df93c, inline)]
unsafe fn x03df93c(ctx: &mut skyline::hooks::InlineCtx) {
    let opponent_battle_object_id = *(*ctx.registers[22].x.as_ref() as *const u32).add(0x24 / 4);
    let opponent_battle_object = utils::util::get_battle_object_from_id(opponent_battle_object_id);
    let opponent_boma = (&mut *(*opponent_battle_object).module_accessor);
    if opponent_boma.is_status(*FIGHTER_STATUS_KIND_GUARD_OFF)
    && VarModule::is_flag(opponent_battle_object, vars::common::instance::IS_PARRY_FOR_GUARD_OFF)
    && opponent_boma.get_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME) > 0 {
        *ctx.registers[8].w.as_mut() = *ctx.registers[8].w.as_ref() | *COLLISION_KIND_MASK_PARRY as u32;
        if opponent_boma.is_fighter() {
            let kind = opponent_boma.kind();
            if kind == *FIGHTER_KIND_RYU || kind == *FIGHTER_KIND_KEN {
                opponent_boma.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
                opponent_boma.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
            }
            if kind == *FIGHTER_KIND_DOLLY {
                opponent_boma.off_flag(*FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
                opponent_boma.off_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
            }
        }
    }
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
        handle_on_attack_event,
        x0627880, x0627cc0, x0641948, x03df93c
    );
}