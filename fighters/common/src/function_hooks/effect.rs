use super::*;
use globals::*;

const SHOCKWAVE_FX: [u64 ; 3] = [hash40("sys_crown"), hash40("sys_crown_collision"), hash40("sys_nopassive")];
const SMOKE_FX: [u64 ; 16] = [hash40("sys_atk_smoke"),
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
                            hash40("sys_v_smoke_b")];

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

//=================================================================
//== sv_animcmd::EFFECT
//== Note: Lua stack is 1-indexed, and "pop" means "get"
//=================================================================
#[skyline::hook(replace=smash::app::sv_animcmd::EFFECT)]
unsafe fn EFFECT_hook(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 16] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..16 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    let mut effect_size_mul = 1.0;
    let mut reduce_alpha = false;

    l2c_agent.clear_lua_stack();

    for i in 0..16 {
        // Index of effect name
        if i == 0 {
            let effect_name = hitbox_params[i as usize].get_hash();
            if SHOCKWAVE_FX.contains(&effect_name.hash) {
                effect_size_mul = if effect_name.hash == hash40("sys_nopassive") {
                    0.5
                } else {
                    0.7
                };

                if is_tech_lockout(boma) {
                    effect_size_mul = 0.5;
                    hitbox_params[i as usize] = L2CValue::new_hash(hash40("sys_nopassive"));
                }
            }
            if SMOKE_FX.contains(&effect_name.hash) {
                reduce_alpha = true;
            }
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
        // Index of effect size
        else if i == 8 && effect_size_mul != 1.0 {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * effect_size_mul);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    original!()(lua_state);
    if reduce_alpha {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.5));
        sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
        l2c_agent.clear_lua_stack();
    }
}

#[skyline::hook(replace=smash::app::sv_animcmd::EFFECT_FOLLOW)]
unsafe fn EFFECT_FOLLOW_hook(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 10] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..10 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    let mut effect_size_mul = 1.0;
    let mut reduce_alpha = false;

    l2c_agent.clear_lua_stack();

    for i in 0..10 {
        // Index of effect name
        if i == 0 {
            let effect_name = hitbox_params[i as usize].get_hash();
            if SHOCKWAVE_FX.contains(&effect_name.hash) {
                effect_size_mul = if effect_name.hash == hash40("sys_nopassive") {
                    0.5
                } else {
                    0.7
                };

                if is_tech_lockout(boma) {
                    effect_size_mul = 0.5;
                    hitbox_params[i as usize] = L2CValue::new_hash(hash40("sys_nopassive"));
                }
            }
            if SMOKE_FX.contains(&effect_name.hash) {
                reduce_alpha = true;
            }
            //let mut aux: L2CValue = L2CValue::new_int(*ATTACK_LR_CHECK_POS as u64);
            //l2c_agent.push_lua_stack(&mut aux);
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        } 
        else if i == 8 && effect_size_mul != 1.0 {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * effect_size_mul);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    original!()(lua_state);
    if reduce_alpha {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.5));
        sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
        l2c_agent.clear_lua_stack();
    }
}

#[skyline::hook(replace=smash::app::sv_animcmd::EFFECT_FOLLOW_FLIP)]
unsafe fn EFFECT_FOLLOW_FLIP_hook(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 12] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..12 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    let mut effect_size_mul = 1.0;
    let mut reduce_alpha = false;

    l2c_agent.clear_lua_stack();

    for i in 0..12 {
        // Index of effect name
        if i == 0 {
            let effect_name = hitbox_params[i as usize].get_hash();
            if SHOCKWAVE_FX.contains(&effect_name.hash) {
                effect_size_mul = if effect_name.hash == hash40("sys_nopassive") {
                    0.5
                } else {
                    0.7
                };

                if is_tech_lockout(boma) {
                    effect_size_mul = 0.5;
                    hitbox_params[i as usize] = L2CValue::new_hash(hash40("sys_nopassive"));
                }
            }
            if SMOKE_FX.contains(&effect_name.hash) {
                reduce_alpha = true;
            }
            //let mut aux: L2CValue = L2CValue::new_int(*ATTACK_LR_CHECK_POS as u64);
            //l2c_agent.push_lua_stack(&mut aux);
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        } 
        else if i == 9 && effect_size_mul != 1.0 {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * effect_size_mul);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    original!()(lua_state);
    if reduce_alpha {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.5));
        sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
        l2c_agent.clear_lua_stack();
    }
}

#[skyline::hook(replace=smash::app::sv_animcmd::FOOT_EFFECT)]
unsafe fn FOOT_EFFECT_hook(lua_state: u64) {
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

    original!()(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.5));
    sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
    l2c_agent.clear_lua_stack();
}

#[skyline::hook(replace=smash::app::sv_animcmd::FOOT_EFFECT_FLIP)]
unsafe fn FOOT_EFFECT_FLIP_hook(lua_state: u64) {
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

    original!()(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.5));
    sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
    l2c_agent.clear_lua_stack();
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

    original!()(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.5));
    sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
    l2c_agent.clear_lua_stack();
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

    original!()(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.5));
    sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
    l2c_agent.clear_lua_stack();
}

#[skyline::hook(replace=smash::app::sv_animcmd::DOWN_EFFECT)]
unsafe fn DOWN_EFFECT_hook(lua_state: u64) {
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

    original!()(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.5));
    sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
    l2c_agent.clear_lua_stack();
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

#[skyline::hook(replace=EffectModule::req)]
unsafe fn req_hook(boma: &mut BattleObjectModuleAccessor, effHash: smash::phx::Hash40, pos: &Vector3f, rot: &Vector3f, size: f32, arg6: u32, arg7: i32, arg8: bool, arg9: i32) -> u64 {
    let mut eff_size = size;
    let mut new_eff_hash = effHash;
    if SHOCKWAVE_FX.contains(&effHash.hash) {
        let mut effect_size_mul = if effHash.hash == hash40("sys_nopassive") {
            0.5
        } else {
            0.7
        };

        if is_tech_lockout(boma) {
            effect_size_mul = 0.5;
            new_eff_hash = Hash40::new("sys_nopassive");
        }

        eff_size = size * effect_size_mul;
    }
    original!()(boma, new_eff_hash, pos, rot, eff_size, arg6, arg7, arg8, arg9)
}

#[skyline::hook(replace=EffectModule::req_on_joint)]
unsafe fn req_on_joint_hook(boma: &mut BattleObjectModuleAccessor, effHash: smash::phx::Hash40, boneHash: smash::phx::Hash40, pos: &Vector3f, rot: &Vector3f, size: f32, arg7: &Vector3f, arg8: &Vector3f, arg9: bool, arg10: u32, arg11: i32, arg12: i32) -> u64 {
    let mut eff_size = size;
    let mut new_eff_hash = effHash;
    if SHOCKWAVE_FX.contains(&effHash.hash) {
        let mut effect_size_mul = if effHash.hash == hash40("sys_nopassive") {
            0.5
        } else {
            0.7
        };
        
        if is_tech_lockout(boma) {
            effect_size_mul = 0.5;
            new_eff_hash = Hash40::new("sys_nopassive");
        }

        eff_size = size * effect_size_mul;
    }
    original!()(boma, new_eff_hash, boneHash, pos, rot, eff_size, arg7, arg8, arg9, arg10, arg11, arg12)
}

#[skyline::hook(replace=EffectModule::req_follow)]
unsafe fn req_follow(boma: &mut BattleObjectModuleAccessor, effHash: smash::phx::Hash40, boneHash: smash::phx::Hash40, pos: &Vector3f, rot: &Vector3f, size: f32, arg7: bool, arg8: u32, arg9: i32, arg10: i32, arg11: i32, arg12: i32, arg13: bool, arg14: bool) -> u64 {
    let mut eff_size = size;
    // Shrink knockback smoke effect by 25%
    let mut is_flyroll = false;
    if effHash.hash == hash40("sys_flyroll_smoke") as u64 {  // hash for kb smoke
        eff_size = size * 0.7;
        is_flyroll  = true;
    }

    let ret = original!()(boma, effHash, boneHash, pos, rot, eff_size, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14);
    if is_flyroll {
        let fighter = util::get_fighter_common_from_accessor(boma);
        let reaction_frame = fighter.get_float(*FIGHTER_STATUS_DAMAGE_WORK_FLOAT_REACTION_FRAME);

        let min_alpha = 0.25;
        let max_alpha = 1.0;

        let alpha = (min_alpha + ((reaction_frame - 51.0) / 30.0)).clamp(min_alpha, max_alpha);
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_num(alpha));
        sv_animcmd::LAST_EFFECT_SET_ALPHA(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }

    ret
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
        EFFECT_hook,
        EFFECT_FOLLOW_hook,
        EFFECT_FOLLOW_FLIP_hook,
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
