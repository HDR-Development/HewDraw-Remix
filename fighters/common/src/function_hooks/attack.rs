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

    if utils::game_modes::check_custom_mode(CustomMode::RandomAngleMode)
    && data.vector >= 0
    && data.vector <= 361 {
        // replace sakurai angle, so that we can randomize it
        if (data.vector == 361) {
            data.vector = 45;
        }

        // seed inputs
        let input_angle = data.vector as u32;
        let battle_object_id = (*boma).battle_object_id;
        let status_kind = (*boma).status() as u32;

        // seed generation
        let angle_seed = {
            // magic bullshit go
            let mut h = input_angle;
            h ^= battle_object_id.wrapping_add(0x9e3779b9).wrapping_add(h << 6).wrapping_add(h >> 2);
            h ^= status_kind.wrapping_add(0x9e3779b9).wrapping_add(h << 6).wrapping_add(h >> 2);
            h ^= MATCH_SCOPED_RANDOM_U32.wrapping_add(0x9e3779b9).wrapping_add(h << 6).wrapping_add(h >> 2);
            h
        };

        // psuedorandom angle offset from -90 to 90
        // when applied, results in a random angle in the same hemisphere as the original
        // and the result persists for the same attack of the same fighter throught a match
        let offset = ((angle_seed).wrapping_mul(2654435761) % 181) as i32 - 90;
        data.vector = (data.vector + offset).rem_euclid(361);

        let spike_kb_mul = 1.5;
        let was_spike_before = input_angle < 30 || input_angle > 150;
        let is_spike_after = data.vector < 30 || data.vector > 150;
        // reduce the knockback of moves that have become spikes but were not previously
        if !was_spike_before && is_spike_after {
            data.r_eff = (data.r_eff as f32 / spike_kb_mul).round() as i32;
            data.r_add = (data.r_add as f32 / spike_kb_mul).round() as i32;
        }
        // increase the knockback of moves that are no longer spikes but were previously
        else if was_spike_before && !is_spike_after {
            data.r_eff = (data.r_eff as f32 * spike_kb_mul).round() as i32;
            data.r_add = (data.r_add as f32 * spike_kb_mul).round() as i32;
        }
    }

    if utils::game_modes::check_custom_mode(CustomMode::ElementMode) {
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
    let damage_frame_mul: f32 = if utils::game_modes::check_custom_mode(CustomMode::Smash64Mode) {
        0.533
    }
    else {
        0.42 // TODO: find a way to parameterize this or otherwise notify that it's hardcoded
    };
    ctx.registers_f[0].set_s(damage_frame_mul)
}

unsafe extern "C" fn calc_hitstop_frame_mul(kb: f32) -> f32 {
    let min = 0.45;
    let max = 0.65;
    let power = 1.4;
    let kb_start = 100.0;
    let kb_end = 200.0;

    let ratio = ((kb - kb_start) / (kb_end - kb_start));
    let hitlag_mul = util::nlerp(min, max, power, ratio);
    return hitlag_mul;
}

unsafe extern "C" fn calc_hitstop_frame_add(kb: f32) -> f32 {
    let min = 4.0;
    let max = 6.0;
    let power = 1.4;
    let kb_start = 100.0;
    let kb_end = 200.0;

    let ratio = ((kb - kb_start) / (kb_end - kb_start));
    let hitlag_mul = util::nlerp(min, max, power, ratio);
    return hitlag_mul;
}

// This runs within the internal function used to calculate hitlag
// Used for both attacker hitlag and receiver hitlag (articles/items included)
#[skyline::hook(offset = 0x406bf4, inline)]
unsafe fn get_hitstop_params(ctx: &mut skyline::hooks::InlineCtx) {
    let attack_data = (ctx.registers[21].x() as *mut smash_rs::app::AttackData);
    let attr: smashline::Hash40 = std::mem::transmute((*attack_data).attr);
    let hitstop_frame_mul: f32 = if IS_KB_CALC_EARLY
    && ![Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_saving")].contains(&attr) {
        calc_hitstop_frame_mul(KB)
    }
    else {
        0.45 // TODO: find a way to parameterize this or otherwise notify that it's hardcoded
    };
    // Set hitstop_frame_mul
    ctx.registers_f[13].set_s(hitstop_frame_mul);

    let hitstop_frame_add: f32 = if utils::game_modes::check_custom_mode(CustomMode::Smash64Mode) {
        5.0
    }
    else {
        if IS_KB_CALC_EARLY
        && ![Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_saving")].contains(&attr) {
            calc_hitstop_frame_add(KB)
        }
        else {
            4.0 // TODO: find a way to parameterize this or otherwise notify that it's hardcoded
        }
    };
    // Set hitstop_frame_add
    ctx.registers_f[0].set_s(hitstop_frame_add)
}

// This runs within the internal function used to calculate hitlag
// for electric moves
// Used for both attacker hitlag and receiver hitlag (articles/items included)
#[skyline::hook(offset = 0x406b6c, inline)]
unsafe fn get_hitstop_params_elec(ctx: &mut skyline::hooks::InlineCtx) {
    let hitstop_frame_mul: f32 = if IS_KB_CALC_EARLY {
        calc_hitstop_frame_mul(KB)
    }
    else {
        0.45 // TODO: find a way to parameterize this or otherwise notify that it's hardcoded
    };
    // Set hitstop_frame_mul
    ctx.registers_f[13].set_s(hitstop_frame_mul);

    let hitstop_frame_add: f32 = if utils::game_modes::check_custom_mode(CustomMode::Smash64Mode) {
        5.0
    }
    else {
        if IS_KB_CALC_EARLY {
            calc_hitstop_frame_add(KB)
        }
        else {
            4.0 // TODO: find a way to parameterize this or otherwise notify that it's hardcoded
        }
    };
    // Set hitstop_frame_add
    ctx.registers_f[14].set_s(hitstop_frame_add)
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

// This runs directly after knockback is calculated
#[skyline::hook(offset = 0x402f04, inline)]
unsafe fn post_calc_reaction(ctx: &mut skyline::hooks::InlineCtx) {
    let damage_module = ctx.registers[19].x();
    let receiver_boma = &mut **((damage_module + 0x8) as *mut *mut smash::app::BattleObjectModuleAccessor);
    let mut kb = ctx.registers_f[0].s();

    // Handles application of knockback multiplier on grounded spikes
    if receiver_boma.is_fighter() {
        let attack_data = (ctx.registers[22].x() as *mut smash_rs::app::AttackData);
        let angle = (*attack_data).vector;
        let meteor_vector_min = WorkModule::get_param_int(receiver_boma, hash40("battle_object"), hash40("meteor_vector_min"));
        let meteor_vector_max = WorkModule::get_param_int(receiver_boma, hash40("battle_object"), hash40("meteor_vector_max"));
        let grounded_spike_knockback_mul = ParamModule::get_float(receiver_boma.object(), ParamType::Common, "grounded_spike_knockback_mul");

        if receiver_boma.is_situation(*SITUATION_KIND_GROUND)
        && angle >= meteor_vector_min
        && angle <= meteor_vector_max {
            kb *= grounded_spike_knockback_mul;
        }
    }

    let attacker_id = ctx.registers[27].w();
    let attacker_boma = &mut *(sv_battle_object::module_accessor(attacker_id));

    // Handles hitlag scaling for attacker
    if attacker_boma.is_fighter()
    && receiver_boma.is_fighter()
    && kb != 0.0 {
        let attacker_fighter = get_fighter_common_from_accessor(attacker_boma);
        let attacker_object = sv_system::battle_object(attacker_fighter.lua_state_agent);
        let attacker_fighta : *mut Fighter = std::mem::transmute(attacker_object);

        IS_KB_CALC_EARLY = true;
        KB = kb;
        
        let attack_data = (ctx.registers[22].x() as *mut smash_rs::app::AttackData);
        let power = AttackModule::get_power(attacker_boma, 0, false, 1.0, false);
        let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x4067a0);
        let calc_hitlag: extern "C" fn(f32, f32, f32, u8, &mut smash_rs::app::AttackData, u64, u32, u32) -> i32 = std::mem::transmute(func_addr);
        let hitlag = calc_hitlag(power, 1.0, 1.0, 0, &mut *attack_data, 0, 1, 0);

        // Set hitlag for attacker
        *(((attacker_fighta as u64) + 0xf70c) as *mut i32) = hitlag;

        ctx.registers_f[0].set_s(kb)
    }

    ctx.registers_f[0].set_s(kb)
}

// This runs immediately after an attacker's hitlag is calculated
// #[skyline::hook(offset = 0x406fdc, inline)]
// unsafe fn handle_on_attack_event(ctx: &mut skyline::hooks::InlineCtx) {
//     if IS_KB_CALC_EARLY {
//         let boma = &mut *(ctx.registers[23].x() as *mut BattleObjectModuleAccessor);
//         let hitlag = ctx.registers[0].w();
//         let kb = KB;
//         let max_hitlag = WorkModule::get_param_float(boma, hash40("battle_object"), hash40("hitstop_frame_max"));
//         let attack_data = (ctx.registers[24].x() as *mut smash_rs::app::AttackData);
//         let attr: smashline::Hash40 = std::mem::transmute((*attack_data).attr);

//         if ![Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_saving")].contains(&attr) {
//             // Set hitlag for attacker
//             ctx.registers[0].set_w((hitlag as f32 * calc_hitstop_frame_mul(boma, kb)).round().min(max_hitlag) as u32);
//         }
//     }
// }

// This runs immediately before hitlag is set for attacking articles
// Handles hitlag scaling for articles
#[skyline::hook(offset = 0x33a9924, inline)]
unsafe fn set_weapon_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
    let receiver_boma = &mut *(ctx.registers[24].x() as *mut BattleObjectModuleAccessor);
    let kb = DamageModule::reaction(receiver_boma, 0);
    
    if receiver_boma.is_fighter()
    && kb != 0.0 {
        // These get used by get_hitstop_params
        IS_KB_CALC_EARLY = true;
        KB = kb;
    }
}

// This runs immediately after hitlag is set for the receiver
#[skyline::hook(offset = 0x404658, inline)]
unsafe fn set_receiver_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
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
    let defender_battle_object_id = *(ctx.registers[22].x() as *const u32).add(0x24 / 4);
    let defender_battle_object = utils::util::get_battle_object_from_id(defender_battle_object_id);
    let defender_boma = (&mut *(*defender_battle_object).module_accessor);

    if defender_boma.is_status(*FIGHTER_STATUS_KIND_GUARD_OFF)
    && VarModule::is_flag(defender_battle_object, vars::common::instance::IS_PARRY_FOR_GUARD_OFF)
    && defender_boma.get_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME) > 0 {
        ctx.registers[8].set_w(ctx.registers[8].w() | *COLLISION_KIND_MASK_PARRY as u32);
        let attack_module = ctx.registers[19].x();
        let attacker_boma = &mut *(*(attack_module as *mut *mut BattleObjectModuleAccessor).add(1));

        if attacker_boma.is_fighter() {
            // clear ledge and respawn iframes
            VarModule::set_int(attacker_boma.object(), vars::common::instance::CLIFF_XLU_FRAME, 0);
            HitModule::set_xlu_frame_global(attacker_boma, 0, 0);
            HitModule::set_invincible_frame_global(attacker_boma, 0, false, 0);  // sub_rebirth_uniq_process_exit
        }

        // invuln on parry success for RoA mode
        if utils::game_modes::check_custom_mode(game_modes::CustomMode::RivalsOfAetherMode) {
            HitModule::set_xlu_frame_global(defender_boma, 60, 0);
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

    if app::smashball::is_training_mode() {
        if ControlModule::check_button_on(attacker_boma, *CONTROL_PAD_BUTTON_APPEAL_HI)
        || ControlModule::check_button_on(attacker_boma, *CONTROL_PAD_BUTTON_APPEAL_LW)
        || ControlModule::check_button_on(attacker_boma, *CONTROL_PAD_BUTTON_APPEAL_S_L)
        || ControlModule::check_button_on(attacker_boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            if VarModule::has_var_module(receiver_boma.object()) {
                let fighter = utils::util::get_fighter_common_from_accessor(attacker_boma);
                let prev = VarModule::is_flag(receiver_boma.object(), vars::common::instance::ENABLE_FRAME_DATA_DEBUG);
                println!("prev: {}", prev);
                if prev == true {
                    println!("15 -> 18");
                    // 15 -> 18
                    fighter.clear_lua_stack();
                    lua_args!(fighter, Hash40::new("sys_hit_dead"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
                    smash::app::sv_animcmd::EFFECT_FOLLOW(fighter.lua_state_agent);
                    fighter.pop_lua_stack(1);
                }
                else {
                    println!("18 -> 15");
                    // 18 -> 15
                    fighter.clear_lua_stack();
                    lua_args!(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
                    smash::app::sv_animcmd::EFFECT_FOLLOW(fighter.lua_state_agent);
                    fighter.pop_lua_stack(1);
                }
                println!("setting to {}", !prev);
                VarModule::set_flag(receiver_boma.object(), vars::common::instance::ENABLE_FRAME_DATA_DEBUG, !prev);
            }
        }
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

        let mut spike_tumble_threshold = ParamModule::get_float((*boma).object(), ParamType::Common, "spike_tumble_threshold");
        // Ensures grounded spikes do not trigger tumble earlier
        // despite their knockback multiplier
        if (*boma).is_situation(*SITUATION_KIND_GROUND) {
            let grounded_spike_knockback_mul = ParamModule::get_float((*boma).object(), ParamType::Common, "grounded_spike_knockback_mul");
            spike_tumble_threshold *= grounded_spike_knockback_mul;
        }

        if kb >= spike_tumble_threshold {
            // Set damage level to 3 (tumble)
            ctx.registers[24].set_w(3);
        }
    
        ctx.registers_f[11].set_s(kb)
    }

    // Forces tumble for throws
    let fighter = util::get_fighter_common_from_accessor(&mut (*boma));
    if VarModule::is_flag((*boma).object(), vars::common::instance::FORCE_TUMBLE_NO_BOUNCE)
    || [ // THROWN statuses
        *FIGHTER_STATUS_KIND_BITTEN_WARIO_END,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON,
        *FIGHTER_STATUS_KIND_CLUNG_THROWN_BLANK_DIDDY,
        *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY,
        *FIGHTER_STATUS_KIND_DEMON_DIVED,
        *FIGHTER_STATUS_KIND_DRAGGED_RIDLEY,
        *FIGHTER_STATUS_KIND_MEWTWO_THROWN,
        *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN,
        *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN,
        *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN,
        *FIGHTER_STATUS_KIND_SWALLOWED_THROWN,
        *FIGHTER_STATUS_KIND_SWALLOWED_THROWN_STAR,
        // *FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE,
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_LARIAT,
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_SHOULDER,
        *FIGHTER_STATUS_KIND_THROWN,
    ].contains(&(fighter.global_table[STATUS_KIND].get_i32())) {
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

    // damage multiplier for pikmin
    let pikmin_mul = if attacker_boma.is_weapon() && attacker_boma.kind() == *WEAPON_KIND_PIKMIN_PIKMIN {
        let variation = WorkModule::get_int(attacker_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
        let param = format!("param_pikmin_particular.{}.damage_mul", variation);
        let battle_object = attacker_boma.get_owner_boma().object(); // olimar's battle object
        ParamModule::get_float(battle_object, ParamType::Agent, &param)
    } else {
        1.0
    };
    // dbg!(pikmin_mul);

    // damage multiplier for nana
    let nana_mul = if attacker_boma.is_fighter() && attacker_boma.kind() == *FIGHTER_KIND_NANA {
        0.75
    } else {
        1.0
    };

    // final multiplier
    return pattern_mul * aura_mul * pikmin_mul * nana_mul;
}

#[skyline::hook(offset=0x4067a0)]
unsafe fn calc_hitlag(damage: f32, hit_stop_mul: f32, some_mul: f32, some_char: u8, attack_data: &mut smash_rs::app::AttackData, param_6: u64, param_7: u32, param_8: u32) -> i32 {
    let ret = original!()(damage, hit_stop_mul, some_mul, some_char, attack_data, param_6, param_7, param_8);

    println!("calc_hitlag({}, {}, {}, {}, attack_data, {}, {}, {}) = {}", damage, hit_stop_mul, some_mul, some_char, param_6, param_7, param_8, ret);

    ret
}

pub fn install() {
    skyline::patching::Patch::in_text(0x641d84).nop();
    skyline::install_hooks!(
        attack_module_set_attack,
        get_damage_frame_mul,
        get_hitstop_params,
        get_hitstop_params_elec,
        get_hitstop_mul,
        post_calc_reaction,
        set_weapon_hitlag,
        set_receiver_hitlag,
        // handle_on_attack_event,
        set_parry_hitlag,
        x03df93c,
        notify_log_event_collision_hit,
        disable_attacker_parry_pushback,
        post_spike_check,
        // attack_module_set_power_hook_5th,
        attack_module_set_power_hook_pattern,
        hit_module__on_attack__set_damage,
        damage_module__unk__set_damage,
        //calc_hitlag
    );
}

