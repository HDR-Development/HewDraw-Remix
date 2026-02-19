use super::*;
use smash_rs::app::CollisionSoundAttr;
use utils::ext::*;
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

            // ID 0:
            // ground-only, cannot reverse hit
            data.target_situation = smash_rs::app::CollisionSituationMask::Ground;
            data.lr_check = smash_rs::app::AttackLRCheck::Forward;
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
                let rand = sv_math::rand(hash40("fighter"), 21);
                match rand { 
                    0 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_aura");          data.sound_attr = CollisionSoundAttr::Fire; },
                    1 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_bury");          data.sound_attr = CollisionSoundAttr::Heavy; },
                    2 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_bind_extra");    data.sound_attr = CollisionSoundAttr::Elec; },
                    3 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_cutup");         data.sound_attr = CollisionSoundAttr::CutUp; },
                    4 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_coin");          data.sound_attr = CollisionSoundAttr::Coin; },
                    5 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_normal_poison"); data.sound_attr = CollisionSoundAttr::Fire; },
                    6 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_elec");          data.sound_attr = CollisionSoundAttr::Elec; },
                    7 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_fire");          data.sound_attr = CollisionSoundAttr::Fire; },
                    8 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_flower");        data.sound_attr = CollisionSoundAttr::Kick; },
                    9 =>  { data.attr = smash_rs::phx::Hash40::new("collision_attr_ice");           data.sound_attr = CollisionSoundAttr::Freeze; },
                    10 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_magic");         data.sound_attr = CollisionSoundAttr::Magic; },
                    11 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_normal");        data.sound_attr = CollisionSoundAttr::Punch; },
                    12 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_paralyze");      data.sound_attr = CollisionSoundAttr::Elec; },
                    13 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_purple");        data.sound_attr = CollisionSoundAttr::Fire; },
                    14 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_sleep");         data.sound_attr = CollisionSoundAttr::Magic; },
                    15 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_slip");          data.sound_attr = CollisionSoundAttr::Slap; },
                    16 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_sting");         data.sound_attr = CollisionSoundAttr::CutUp; },
                    17 => { data.attr = smash_rs::phx::Hash40::new("collision_attr_turn");          data.sound_attr = CollisionSoundAttr::Harisen; },
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

    if (*boma).is_status(*FIGHTER_STATUS_KIND_CLIFF_ATTACK) {
        // ID 1:
        // air-only, can reverse hit
        data.target_situation = smash_rs::app::CollisionSituationMask::Air;
        data.lr_check = smash_rs::app::AttackLRCheck::Pos;

        call_original!(module, 1, group, data);
    }
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
            ctx.registers_f[0].set_s(damage_frame_mul)
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
            ctx.registers_f[0].set_s(hitstop_frame_add)
        },
        _ => {}
    }
}

// Only applies 0.67 crouch cancel hitlag multiplier to receiver
#[skyline::hook(offset = 0x46b648, inline)]
unsafe fn get_hitstop_mul(ctx: &mut skyline::hooks::InlineCtx) {
    if ctx.registers[1].w() == 0x2 {
        let hitstop_mul: f32 = 1.0;
        ctx.registers_f[0].set_s(hitstop_mul)
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
    let hitlag_mul = util::nlerp(min, max, power, ratio);
    return hitlag_mul;
}

// This runs directly after knockback is calculated
#[skyline::hook(offset = 0x402f04, inline)]
unsafe fn post_calc_reaction(ctx: &mut skyline::hooks::InlineCtx) {
    let damage_module = ctx.registers[19].x();
    let receiver_boma = &mut **((damage_module + 0x8) as *mut *mut smash::app::BattleObjectModuleAccessor);

    // Handles application of knockback multiplier on grounded spikes
    if receiver_boma.is_fighter() {
        let mut kb = ctx.registers_f[0].s();

        let attack_data = (ctx.registers[22].x() as *mut smash_rs::app::AttackData);
        let angle = (*attack_data).vector;
        let meteor_vector_min = WorkModule::get_param_int(receiver_boma, hash40("battle_object"), hash40("meteor_vector_min"));
        let meteor_vector_max = WorkModule::get_param_int(receiver_boma, hash40("battle_object"), hash40("meteor_vector_max"));
        let spike_tumble_threshold = ParamModule::get_float(receiver_boma.object(), ParamType::Common, "spike_tumble_threshold");
        let damage_frame_mul = WorkModule::get_param_float(receiver_boma, hash40("battle_object"), hash40("damage_frame_mul"));
        let grounded_spike_knockback_mul = ParamModule::get_float(receiver_boma.object(), ParamType::Common, "grounded_spike_knockback_mul");

        let spike_tumble_threshold_kb = spike_tumble_threshold / damage_frame_mul;

        if receiver_boma.is_situation(*SITUATION_KIND_GROUND)
        && angle >= meteor_vector_min
        && angle <= meteor_vector_max
        && kb >= spike_tumble_threshold_kb {
            kb *= grounded_spike_knockback_mul;
        }

        ctx.registers_f[0].set_s(kb)
    }

    let attacker_id = ctx.registers[27].w();
    let attacker_boma = &mut *(sv_battle_object::module_accessor(attacker_id));

    // Handles hitlag scaling for attacker
    if attacker_boma.is_fighter() {
        let attacker_fighter = get_fighter_common_from_accessor(attacker_boma);
        let attacker_object = sv_system::battle_object(attacker_fighter.lua_state_agent);
        let attacker_fighta : *mut Fighter = std::mem::transmute(attacker_object);
    
        let mut kb = ctx.registers_f[0].s();
        IS_KB_CALC_EARLY = true;
        KB = kb;
        let hitlag = *(((attacker_fighta as u64) + 0xf70c) as *mut i32);
        let max_hitlag = WorkModule::get_param_float(attacker_boma, hash40("battle_object"), hash40("hitstop_frame_max"));
        let attack_data = (ctx.registers[22].x() as *mut smash_rs::app::AttackData);
        let attr: smashline::Hash40 = std::mem::transmute((*attack_data).attr);

        if ![Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_saving")].contains(&attr) {
            // Set hitlag for attacker
            *(((attacker_fighta as u64) + 0xf70c) as *mut i32) = (hitlag as f32 * calc_hitlag_mul(attacker_boma, kb)).round().min(max_hitlag) as i32;
        }

        ctx.registers_f[0].set_s(kb)
    }
}

// This runs immediately after an attacker's hitlag is calculated
#[skyline::hook(offset = 0x406fdc, inline)]
unsafe fn handle_on_attack_event(ctx: &mut skyline::hooks::InlineCtx) {
    if IS_KB_CALC_EARLY {
        let boma = &mut *(ctx.registers[23].x() as *mut BattleObjectModuleAccessor);
        let hitlag = ctx.registers[0].w();
        let kb = KB;
        let max_hitlag = WorkModule::get_param_float(boma, hash40("battle_object"), hash40("hitstop_frame_max"));
        let attack_data = (ctx.registers[24].x() as *mut smash_rs::app::AttackData);
        let attr: smashline::Hash40 = std::mem::transmute((*attack_data).attr);

        if ![Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_saving")].contains(&attr) {
            // Set hitlag for attacker
            ctx.registers[0].set_w((hitlag as f32 * calc_hitlag_mul(boma, kb)).round().min(max_hitlag) as u32);
        }
    }
}

// This runs immediately before hitlag is set for attacking articles
#[skyline::hook(offset = 0x33a9b40, inline)]
unsafe fn set_weapon_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
    let opponent_boma = &mut *(ctx.registers[24].x() as *mut BattleObjectModuleAccessor);
    if !opponent_boma.is_item() {
        let hitlag = ctx.registers[21].w();
        let kb = DamageModule::reaction(opponent_boma, 0);
        IS_KB_CALC_EARLY = true;
        KB = kb;
        let max_hitlag = WorkModule::get_param_float(opponent_boma, hash40("battle_object"), hash40("hitstop_frame_max"));
        let attack_data = (ctx.registers[20].x() as *mut smash_rs::app::AttackData);
        let attr: smashline::Hash40 = std::mem::transmute((*attack_data).attr);

        if ![Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_saving")].contains(&attr) {
            // Set hitlag for attacking article
            ctx.registers[21].set_w((hitlag as f32 * calc_hitlag_mul(opponent_boma, kb)).round().min(max_hitlag) as u32);
        }
    }
}

// This runs immediately before hitlag is set for the receiver
#[skyline::hook(offset = 0x404658, inline)]
unsafe fn set_receiver_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = &mut *(ctx.registers[19].x() as *mut BattleObjectModuleAccessor);
    if !boma.is_item() {
        let hitlag = ctx.registers[0].w();
        let kb = DamageModule::reaction(boma, 0);
        let mut max_hitlag = WorkModule::get_param_float(boma, hash40("battle_object"), hash40("hitstop_frame_max"));
        let attr = *((ctx.registers[20].x() + 0xb8) as *mut u64);
        
        if [hash40("collision_attr_elec"),].contains(&attr) {
            max_hitlag *= WorkModule::get_param_float(boma, hash40("battle_object"), hash40("hitstop_elec_mul"));
        }

        if ![hash40("collision_attr_paralyze"), hash40("collision_attr_saving")].contains(&attr) {
            // Set hitlag for receiver
            ctx.registers[0].set_w((hitlag as f32 * calc_hitlag_mul(boma, kb)).round().min(max_hitlag) as u32);
        }
    }
    IS_KB_CALC_EARLY = false;
}

// Forces parry hitlag to be a constant value
#[skyline::hook(offset = 0x641d84, inline)]
unsafe fn set_parry_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
    let parry_hitlag = ctx.registers[28].w();
    ctx.registers[26].set_x(parry_hitlag as u64);
}

// set parry AttackModule inflict flag
#[skyline::hook(offset = 0x03df93c, inline)]
unsafe fn x03df93c(ctx: &mut skyline::hooks::InlineCtx) {
    let opponent_battle_object_id = *(ctx.registers[22].x() as *const u32).add(0x24 / 4);
    let opponent_battle_object = utils::util::get_battle_object_from_id(opponent_battle_object_id);
    let opponent_boma = (&mut *(*opponent_battle_object).module_accessor);

    if opponent_boma.is_status(*FIGHTER_STATUS_KIND_GUARD_OFF)
    && VarModule::is_flag(opponent_battle_object, vars::common::instance::IS_PARRY_FOR_GUARD_OFF)
    && opponent_boma.get_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME) > 0 {
        ctx.registers[8].set_w(ctx.registers[8].w() | *COLLISION_KIND_MASK_PARRY as u32);
        let attack_module = ctx.registers[19].x();
        let attacker_boma = &mut *(*(attack_module as *mut *mut BattleObjectModuleAccessor).add(1));

        if attacker_boma.is_fighter() {
            // clear ledge and respawn iframes
            VarModule::set_int(attacker_boma.object(), vars::common::instance::CLIFF_XLU_FRAME, 0);
            HitModule::set_xlu_frame_global(attacker_boma, 0, 0);
            HitModule::set_invincible_frame_global(attacker_boma, 0, false, 0);  // sub_rebirth_uniq_process_exit
        }
    }
}

// Runs on general hits, used for Jigglypuff's Disarming Voice item removal
#[skyline::hook(offset=0x67a7b0)]
unsafe fn notify_log_event_collision_hit(fighter_manager: u64, attacker_object_id: u32, receiver_object_id: u32, move_type: u64, arg5: u64, move_type_again: u64) -> u64 {
	let attacker_boma = &mut *smash::app::sv_battle_object::module_accessor(attacker_object_id);
	let receiver_boma = &mut *smash::app::sv_battle_object::module_accessor(receiver_object_id);

    if VarModule::has_var_module(attacker_boma.object())
    && VarModule::is_flag(attacker_boma.object(), vars::common::status::HIT_EFFECT_DROP_ITEM)
    && ItemModule::is_have_item(receiver_boma, 0) {
        ItemModule::drop_item(receiver_boma, 90.0, 0.0, 0);
    }

	original!()(fighter_manager, attacker_object_id, receiver_object_id, move_type, arg5, move_type_again)
}

// Disables pushback when your attack is parried
#[skyline::hook(offset = 0x62864c, inline)]
unsafe fn disable_attacker_parry_pushback(ctx: &mut skyline::hooks::InlineCtx) {
    let fighter = ctx.registers[19].x() as *mut Fighter;
    let object = (*fighter).battle_object;
    
    if AttackModule::is_infliction(object.module_accessor, *COLLISION_KIND_MASK_PARRY) {
        ctx.registers_f[0].set_s(0.0);
    }
}

// We can manipulate your damage level here
// e.g. force tumble
#[skyline::hook(offset = 0x403ce4, inline)]
unsafe fn post_spike_check(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = ctx.registers[19].x() as *mut smash::app::BattleObjectModuleAccessor;

    if !(*boma).is_fighter() {
        return;
    }

    // Lowers the tumble threshold for spikes
    let is_spike = ctx.registers[0].w() != 0;

    if is_spike {
        let mut kb = ctx.registers_f[11].s();

        let spike_tumble_threshold = ParamModule::get_float((*boma).object(), ParamType::Common, "spike_tumble_threshold");

        if kb >= spike_tumble_threshold {
            // Set damage level to 3 (tumble)
            ctx.registers[24].set_w(3);
        }
    
        ctx.registers_f[11].set_s(kb)
    }

    // Forces tumble for knockdown throws
    if VarModule::is_flag((*boma).object(), vars::common::instance::IS_KNOCKDOWN_THROW) {
        // Set damage level to 3 (tumble)
        ctx.registers[24].set_w(3);
    }
}

// #[skyline::hook(offset = 0x3dd658, inline)]
// unsafe extern "C" fn attack_module_set_power_hook_5th(ctx: &mut skyline::hooks::InlineCtx) {
//     let attack_module = ctx.registers[19].x() as *mut u64;
//     let mul = *(attack_module as *const f32).add(0x20c / 0x4);
//     // println!("mul: {}", mul);
//     let mul_5th = ctx.registers_f[1].s();
//     ctx.registers_f[1].set_s(mul * mul_5th);
// }

// removes the effect of staling by setting the multiplier to 1.0
#[skyline::hook(offset = 0x3dd688, inline)]
unsafe extern "C" fn attack_module_set_power_hook_pattern(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers_f[2].set_s(1.0);
}

// reimplements staling by directly manipulating the damage value
#[skyline::hook(offset = 0x46ba9c, inline)]
unsafe fn hit_module__on_attack__set_damage(ctx: &mut skyline::hooks::InlineCtx) {
    let attacker_boma = &mut *(*(ctx.registers[19].x() as *mut *mut BattleObjectModuleAccessor).add(1));

    let current_damage = ctx.registers_f[0].s();
    let mul = calc_non_knockback_damage_mul(attacker_boma);

    let final_damage = current_damage * mul;

    // NOTE: this also affects hitlag for the RECEIVER ONLY
    ctx.registers_f[0].set_s(final_damage);
}

// This runs sometime after HitModule::on_attack
// and before set_receiver_hitlag
//
// Here, damage is stored in some DamageModule struct
// which then gets passed to receiver hitlag calculations
// among other places
#[skyline::hook(offset = 0x402920, inline)]
unsafe fn damage_module__unk__set_damage(ctx: &mut skyline::hooks::InlineCtx) {
    let attacker_battle_object_id = *(ctx.registers[25].x() as *const u32).add(0x24 / 4);
    let attacker_battle_object = utils::util::get_battle_object_from_id(attacker_battle_object_id);
    let attacker_boma = (&mut *(*attacker_battle_object).module_accessor);

    let damage_with_damage_mul = ctx.registers_f[10].s();
    let mul = calc_non_knockback_damage_mul(attacker_boma);

    // Un-apply the stale move damage multiplier
    // to the damage value which is used to calculate receiver hitlag
    //
    // Prevents reduced damage on stale moves from affecting hitlag
    let raw_damage = damage_with_damage_mul / mul;

    ctx.registers_f[10].set_s(raw_damage);
}

unsafe fn calc_non_knockback_damage_mul(attacker_boma: &mut BattleObjectModuleAccessor) -> f32 {
    // stale attack multiplier
    let pattern_mul = AttackModule::get_attack_power_mul_pattern(attacker_boma);
    // dbg!(pattern_mul);

    // aura multiplier for Lucario's attacks
    let aura_mul = if attacker_boma.is_fighter() && attacker_boma.kind() == *FIGHTER_KIND_LUCARIO
    && !attacker_boma.is_status_one_of(&[
        // these statuses still use the old aurapower multiplier
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_APPEAR,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_SPLIT,
    ]) {
        WorkModule::get_float(attacker_boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER)
    } else {
        1.0
    };
    // dbg!(aura_mul);

    // final multiplier
    return pattern_mul * aura_mul;
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
        set_receiver_hitlag,
        handle_on_attack_event,
        set_parry_hitlag,
        x03df93c,
        notify_log_event_collision_hit,
        disable_attacker_parry_pushback,
        post_spike_check,
        // attack_module_set_power_hook_5th,
        attack_module_set_power_hook_pattern,
        hit_module__on_attack__set_damage,
        damage_module__unk__set_damage
    );
}

