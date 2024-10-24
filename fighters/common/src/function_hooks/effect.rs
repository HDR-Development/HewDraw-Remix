use super::*;
use globals::*;

const SHOCKWAVE_FX: [u64 ; 3] = [hash40("sys_crown"), hash40("sys_crown_collision"), hash40("sys_nopassive")];
const SMOKE_FX: [u64 ; 19] = [hash40("sys_atk_smoke"),
                            hash40("sys_atk_smoke2"),
                            hash40("sys_bound_smoke"),
                            hash40("sys_dash_smoke"),
                            hash40("sys_dash_smoke2"),
                            hash40("sys_down_smoke"),
                            hash40("sys_h_smoke_a"),
                            hash40("sys_h_smoke_b"),
                            hash40("sys_landing_smoke"),
                            hash40("sys_landing_smoke_s"),
                            hash40("sys_run_smoke"),
                            hash40("sys_shield_smoke"),
                            hash40("sys_sliding_smoke"),
                            hash40("sys_turn_smoke"),
                            hash40("sys_v_smoke_a"),
                            hash40("sys_v_smoke_b"),
                            hash40("sys_action_smoke_h"),
                            hash40("sys_action_smoke_v"),
                            hash40("null")];

unsafe extern "C" fn is_tech_lockout(boma: &mut BattleObjectModuleAccessor) -> bool {
    if !boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR]) {
        return false;
    }

    if StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON {
        return true;
    }

    let passive_trigger_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("passive_trigger_frame"));
    let no_rapid_frame_value = WorkModule::get_param_int(boma, hash40("common"), hash40("no_rapid_frame_value"));
    let guard_trigger_count = ControlModule::get_trigger_count(boma, *CONTROL_PAD_BUTTON_GUARD as u8) & 0xff;
    let prev_guard_trigger_count = ControlModule::get_trigger_count_prev(boma, *CONTROL_PAD_BUTTON_GUARD as u8) & 0xff;
    
    if guard_trigger_count < passive_trigger_frame {
        return prev_guard_trigger_count < no_rapid_frame_value;
    }
    
    false
}

#[skyline::hook(replace=smash::app::sv_animcmd::FOOT_EFFECT)]
unsafe fn FOOT_EFFECT_hook(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 16] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..16 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    l2c_agent.clear_lua_stack();

    for i in 0..16 {
        // Index of effect name
        if i == 8 {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * 0.7);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    VarModule::on_flag(boma.object(), vars::common::instance::ACMD_EFFECT);
    original!()(lua_state);
    VarModule::off_flag(boma.object(), vars::common::instance::ACMD_EFFECT);
}

#[skyline::hook(replace=smash::app::sv_animcmd::FOOT_EFFECT_FLIP)]
unsafe fn FOOT_EFFECT_FLIP_hook(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 18] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..18 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    l2c_agent.clear_lua_stack();

    for i in 0..18 {
        // Index of effect name
        if i == 9 {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * 0.7);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    VarModule::on_flag(boma.object(), vars::common::instance::ACMD_EFFECT);
    original!()(lua_state);
    VarModule::off_flag(boma.object(), vars::common::instance::ACMD_EFFECT);
}

#[skyline::hook(replace=smash::app::sv_animcmd::LANDING_EFFECT)]
unsafe fn LANDING_EFFECT_hook(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 16] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..16 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    l2c_agent.clear_lua_stack();

    for i in 0..16 {
        // Index of effect name
        if i == 8 {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * 0.7);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    VarModule::on_flag(boma.object(), vars::common::instance::ACMD_EFFECT);
    original!()(lua_state);
    VarModule::off_flag(boma.object(), vars::common::instance::ACMD_EFFECT);
}

#[skyline::hook(replace=smash::app::sv_animcmd::LANDING_EFFECT_FLIP)]
unsafe fn LANDING_EFFECT_FLIP_hook(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 18] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..18 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    l2c_agent.clear_lua_stack();

    for i in 0..18 {
        // Index of effect name
        if i == 9 {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * 0.7);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    VarModule::on_flag(boma.object(), vars::common::instance::ACMD_EFFECT);
    original!()(lua_state);
    VarModule::off_flag(boma.object(), vars::common::instance::ACMD_EFFECT);
}

#[skyline::hook(replace=smash::app::sv_animcmd::DOWN_EFFECT)]
unsafe fn DOWN_EFFECT_hook(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 16] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..16 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    l2c_agent.clear_lua_stack();

    for i in 0..16 {
        // Index of effect name
        if i == 8 {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * 0.7);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    VarModule::on_flag(boma.object(), vars::common::instance::ACMD_EFFECT);
    original!()(lua_state);
    VarModule::off_flag(boma.object(), vars::common::instance::ACMD_EFFECT);
}

#[skyline::hook(replace=smash::app::sv_animcmd::EFFECT_GLOBAL_BACK_GROUND_CUT_IN_CENTER_POS)]
unsafe fn CUT_IN_CENTER_hook(lua_state: u64) {
    let mut agent: L2CAgent = L2CAgent::new(lua_state);
    let mut params: [L2CValue ; 16] = [
        L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), 
        L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), 
        L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), 
        L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()
    ];
    for i in 0..16 { params[i as usize] = agent.pop_lua_stack(i + 1) };

    agent.clear_lua_stack();
    for i in 0..16 {
        if i == 0 { // effect name index
            let mut effect_name = params[i as usize].get_hash();
            let mut hash = effect_name.hash;
            if effect_name.hash == hash40("sys_bg_finishhit") {
                // determines which effect will play based on the team color of the last attack
                // println!("effect being set to team color {}", LAST_ATTACK_TEAM_COLOR);
                hash = match LAST_ATTACK_TEAM_COLOR {
                    0 => hash40("sys_bg_finishhit_r"),
                    1 => hash40("sys_bg_finishhit_b"),
                    2 => hash40("sys_bg_finishhit_y"),
                    3 => hash40("sys_bg_finishhit_g"),
                    4 => hash40("sys_bg_finishhit_o"),
                    5 => hash40("sys_bg_finishhit_c"),
                    6 => hash40("sys_bg_finishhit_m"),
                    7 => hash40("sys_bg_finishhit_p"),
                    _ => hash40("sys_bg_finishhit")
                };
            }
            agent.push_lua_stack(&mut L2CValue::new_hash(hash));
        } else {
            agent.push_lua_stack(&mut params[i as usize]);
        }
    }

    original!()(lua_state);
}

// EffectModule::req
#[skyline::hook(offset = 0x44de70)]
unsafe fn req_hook(effect_module: u64, effHash: smash::phx::Hash40, pos: *mut Vector3f, rot: *mut Vector3f, size: f32, arg6: u32, arg7: i32, arg8: bool, arg9: i32) -> u64 {
    let boma = *(effect_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let mut eff_size = size;
    let mut new_eff_hash = effHash;
    if SHOCKWAVE_FX.contains(&effHash.hash) {
        let mut effect_size_mul = if effHash.hash == hash40("sys_nopassive") {
            0.5
        } else {
            0.7
        };

        if is_tech_lockout(&mut *boma) {
            effect_size_mul = 0.5;
            new_eff_hash = Hash40::new("sys_nopassive");
        }

        eff_size = size * effect_size_mul;
    }

    let handle = call_original!(effect_module, new_eff_hash, pos, rot, eff_size, arg6, arg7, arg8, arg9);

    if SMOKE_FX.contains(&effHash.hash) {
        EffectModule::set_alpha(boma, handle as u32, 0.5);
    }

    handle
}

// EffectModule::req_on_joint
#[skyline::hook(offset = 0x44e200)]
unsafe fn req_on_joint_hook(effect_module: u64, effHash: smash::phx::Hash40, boneHash: smash::phx::Hash40, pos: *mut Vector3f, rot: *mut Vector3f, size: f32, arg7: *mut Vector3f, arg8: *mut Vector3f, arg9: bool, arg10: u32, arg11: i32, arg12: i32) -> u64 {
    let boma = *(effect_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let mut eff_size = size;
    let mut new_eff_hash = effHash;
    if SHOCKWAVE_FX.contains(&effHash.hash) {
        let mut effect_size_mul = if effHash.hash == hash40("sys_nopassive") {
            0.5
        } else {
            0.7
        };
        
        if is_tech_lockout(&mut *boma) {
            effect_size_mul = 0.5;
            new_eff_hash = Hash40::new("sys_nopassive");
        }

        eff_size = size * effect_size_mul;
    }

    if SMOKE_FX.contains(&effHash.hash)
    && !VarModule::is_flag((*boma).object(), vars::common::instance::ACMD_EFFECT) {
        eff_size = size * 0.7;
    }

    let handle = call_original!(effect_module, new_eff_hash, boneHash, pos, rot, eff_size, arg7, arg8, arg9, arg10, arg11, arg12);

    if SMOKE_FX.contains(&effHash.hash) {
        EffectModule::set_alpha(boma, handle as u32, 0.5);
    }

    handle
}

// EffectModule::req_follow
#[skyline::hook(offset = 0x44f880)]
unsafe fn req_follow(effect_module: u64, effHash: smash::phx::Hash40, boneHash: smash::phx::Hash40, pos: *mut Vector3f, rot: *mut Vector3f, size: f32, arg7: bool, arg8: u32, arg9: i32, arg10: i32, arg11: i32, arg12: i32, arg13: bool, arg14: bool) -> u64 {
    let boma = *(effect_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let mut eff_size = size;
    let mut new_eff_hash = effHash;

    // Shrink knockback smoke effect by 25%
    let mut is_kb_smoke = false;
    if effHash.hash == hash40("sys_flyroll_smoke") as u64 {  // hash for kb smoke
        eff_size = size * 0.7;
        is_kb_smoke = true;
    }

    if SHOCKWAVE_FX.contains(&effHash.hash) {
        let mut effect_size_mul = if effHash.hash == hash40("sys_nopassive") {
            0.5
        } else {
            0.7
        };
        
        if is_tech_lockout(&mut *boma) {
            effect_size_mul = 0.5;
            new_eff_hash = Hash40::new("sys_nopassive");
        }

        eff_size = size * effect_size_mul;
    }

    let handle = original!()(effect_module, new_eff_hash, boneHash, pos, rot, eff_size, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14);

    // Knockback smoke opacity scaling
    if is_kb_smoke {
        let fighter = util::get_fighter_common_from_accessor(&mut *boma);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let speed = app::sv_kinetic_energy::get_speed_length(fighter.lua_state_agent);

        let min_alpha = 0.0;
        let max_alpha = 1.0;

        let alpha = (min_alpha + ((speed - 3.5) / 2.0)).clamp(min_alpha, max_alpha);
        EffectModule::set_alpha(boma, handle as u32, alpha);
    }

    if SMOKE_FX.contains(&effHash.hash) {
        EffectModule::set_alpha(boma, handle as u32, 0.5);
    }

    handle
}

#[skyline::hook(replace=EffectModule::preset_lifetime_rate_partial)]
unsafe fn preset_lifetime_rate_partial_hook(boma: &mut BattleObjectModuleAccessor, rate: f32) -> u64 {
    let mut rate = rate.clone();
    // Halve the lifetime of knockback smoke
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR])
    {
        rate = 0.35;
    }
    original!()(boma, rate)
}

#[skyline::hook(replace=EffectModule::get_dead_effect_scale)]
unsafe fn get_dead_effect_scale_hook(boma: &mut BattleObjectModuleAccessor, arg1: &Vector3f, arg2: f32, arg3: bool) -> f32 {
    // Shrink KO gfx by 25%
    original!()(boma, arg1, arg2, arg3) * 0.75
}

#[skyline::hook(replace=smash::app::sv_module_access::effect)]
unsafe fn module_access_effect_hook(lua_state: u64) {
    let mut agent: L2CAgent = L2CAgent::new(lua_state);
    let mut params: [L2CValue ; 17] = [
        L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), 
        L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), 
        L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), 
        L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(),
        L2CValue::new_void()
    ];
    for i in 0..17 { params[i as usize] = agent.pop_lua_stack(i + 1) };

    agent.clear_lua_stack();
    for i in 0..17 {
        if i == 1 { // effect hash index
            let mut effect_name = params[i as usize].get_hash();
            let mut hash = effect_name.hash;
            if effect_name.hash == hash40("sys_hit_dead") {
                // determines which effect will play based on the team color of the last attack
                // println!("effect being set to team color {}", LAST_ATTACK_TEAM_COLOR);
                hash = match LAST_ATTACK_TEAM_COLOR {
                    0 => hash40("sys_hit_dead_r"),
                    1 => hash40("sys_hit_dead_b"),
                    2 => hash40("sys_hit_dead_y"),
                    3 => hash40("sys_hit_dead_g"),
                    4 => hash40("sys_hit_dead_o"),
                    5 => hash40("sys_hit_dead_c"),
                    6 => hash40("sys_hit_dead_m"),
                    7 => hash40("sys_hit_dead_p"),
                    _ => hash40("sys_hit_dead")
                };
            }
            agent.push_lua_stack(&mut L2CValue::new_hash(hash));
        } else {
            agent.push_lua_stack(&mut params[i as usize]);
        }
    }

    original!()(lua_state);
}

pub fn install() {
    skyline::install_hooks!(
        FOOT_EFFECT_hook,
        FOOT_EFFECT_FLIP_hook,
        LANDING_EFFECT_hook,
        LANDING_EFFECT_FLIP_hook,
        DOWN_EFFECT_hook,
        CUT_IN_CENTER_hook,
        req_hook,
        req_on_joint_hook,
        req_follow,
        preset_lifetime_rate_partial_hook,
        get_dead_effect_scale_hook,
        module_access_effect_hook
    );
}
