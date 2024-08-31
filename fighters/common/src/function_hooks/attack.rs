use super::*;
use smash_rs::app::CollisionSoundAttr;
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
            data.r_add = 70;
            data.sub_shield = 0;
            data.lr_check = smash_rs::app::AttackLRCheck::Pos;
        }
        if (*boma).is_status(*FIGHTER_STATUS_KIND_CATCH_ATTACK) {
            if !VarModule::is_flag((*boma).object(), vars::common::status::PUMMEL_OVERRIDE_GLOBAL_STATS) {
                data.stop_frame = 3.5;  // hitlag mul
            }
        }
    }

    match utils::game_modes::get_custom_mode() {
        Some(modes) => {
            if modes.contains(&CustomMode::ElementMode) {
                let rand = sv_math::rand(hash40("fighter"), 20);
                match rand { 
                    0 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_aura");          data.sound_attr = CollisionSoundAttr::Fire; },
                    1 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_bury");          data.sound_attr = CollisionSoundAttr::Heavy; },
                    2 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_bind_extra");    data.sound_attr = CollisionSoundAttr::Elec; },
                    3 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_cutup");         data.sound_attr = CollisionSoundAttr::CutUp; },
                    4 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_coin");          data.sound_attr = CollisionSoundAttr::Coin; },
                    5 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_curse_poison");  data.sound_attr = CollisionSoundAttr::Fire; },
                    6 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_elec");          data.sound_attr = CollisionSoundAttr::Elec; },
                    7 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_flower");        data.sound_attr = CollisionSoundAttr::Kick; },
                    8 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_ice");           data.sound_attr = CollisionSoundAttr::Freeze; },
                    9 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_magic");         data.sound_attr = CollisionSoundAttr::Magic; },
                    10 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_normal");        data.sound_attr = CollisionSoundAttr::Punch; },
                    11 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_paralyze");      data.sound_attr = CollisionSoundAttr::Elec; },
                    12 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_purple");        data.sound_attr = CollisionSoundAttr::Fire; },
                    13 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_sleep");         data.sound_attr = CollisionSoundAttr::Magic; },
                    14 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_slip");          data.sound_attr = CollisionSoundAttr::Slap; },
                    15 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_sting");         data.sound_attr = CollisionSoundAttr::CutUp; },
                    16 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_turn");          data.sound_attr = CollisionSoundAttr::Harisen; },
                    _ => {} // (slightly larger) chance for the attack to not be randomized
                }

                let ret = call_original!(module, id, group, data);
                if rand == 5 {
                    AttackModule::set_poison_param(boma, 0, 361, 45, 1.0, false);
                }
                return ret;
            }
        },
        _ => {}
    }

    call_original!(module, id, group, data);
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
    let min = 1.0;
    let max = 2.0;
    let power = 1.4;
    let kb_start = 150.0;
    let kb_end = 250.0;

    let ratio = ((kb - kb_start) / (kb_end - kb_start));
    if ratio <= 0.0 {
        return min;
    }
    if ratio >= 1.0 {
        return max;
    }

    let scalar = max - min;
    let hitlag_mul = ratio.powf(power) * scalar + min;
    return hitlag_mul.clamp(min, max);
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

// Forces parry hitlag to be a constant value
#[skyline::hook(offset = 0x641d84, inline)]
unsafe fn set_parry_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
    let parry_hitlag = *ctx.registers[28].w.as_ref();
    *ctx.registers[26].x.as_mut() = parry_hitlag as u64;
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

//Runs on general hits, used for Jigglypuff's Disarming Voice item removal
#[skyline::hook(offset=0x67a7b0)]
unsafe fn notify_log_event_collision_hit(fighter_manager: u64, attacker_object_id: u32, defender_object_id: u32, move_type: u64, arg5: u64, move_type_again: u64) -> u64 {
	let attacker_boma = &mut *smash::app::sv_battle_object::module_accessor(attacker_object_id);
	let defender_boma = &mut *smash::app::sv_battle_object::module_accessor(defender_object_id);
	let attacker_status_kind = StatusModule::status_kind(attacker_boma);
    if attacker_status_kind == articles::purin::DISARMING_VOICE {
        ItemModule::drop_item(defender_boma, 0.0, 0.0, 0);
    }
	original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

pub fn install() {
    skyline::patching::Patch::in_text(0x641d84).nop();
    skyline::install_hooks!(
        attack_module_set_attack,
        get_damage_frame_mul,
        get_hitstop_frame_add,
        get_hitstop_mul,
        post_calc_reaction,
        set_weapon_hitlag,
        set_fighter_hitlag,
        handle_on_attack_event,
        set_parry_hitlag,
        x03df93c,
        notify_log_event_collision_hit
    );
}