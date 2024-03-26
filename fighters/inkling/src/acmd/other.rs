
use super::*;

unsafe extern "C" fn damageflyhi_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(agent.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_inkling_rnd_futtobi01"), Hash40::new("seq_inkling_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_inkling_rnd_futtobi01"), Hash40::new("seq_inkling_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflyroll_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(agent.module_accessor) {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_inkling_rnd_futtobi01"), Hash40::new("seq_inkling_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        PLAY_FLY_VOICE(agent, Hash40::new("seq_inkling_rnd_futtobi01"), Hash40::new("seq_inkling_rnd_futtobi02"));
    }
}

unsafe extern "C" fn dash_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_inkling_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn inkling_inkbullet_fly_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 45, 100, 6, 0, 3.0, 0.0, 0.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        AttackModule::enable_safe_pos(boma);
        AttackModule::set_ink_value(boma, 0, 0.0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.0, 0.0, 0.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        AttackModule::set_ink_value(boma, 0, 0.0);
    }
}

unsafe extern "C" fn inkling_roller_special_s_walk_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 4.0, 60, 80, 0, 60, 3.6, 0.5, 3.8, 0.0, Some(0.5), Some(3.8), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("neck"), 4.0, 60, 80, 0, 60, 3.6, 0.5, 3.8, 0.0, Some(0.5), Some(3.8), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(boma, 0, 60.0);
        AttackModule::set_ink_value(boma, 1, 60.0);
    }
}

unsafe extern "C" fn inkling_roller_special_air_s_walk_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 4.0, 60, 80, 0, 60, 3.6, 0.5, 3.8, 0.0, Some(0.5), Some(3.8), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("neck"), 4.0, 60, 80, 0, 60, 3.6, 0.5, 3.8, 0.0, Some(0.5), Some(3.8), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(boma, 0, 60.0);
        AttackModule::set_ink_value(boma, 1, 60.0);
    }
}

unsafe extern "C" fn inkling_roller_special_s_walk_no_ink_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 2.0, 50, 50, 0, 50, 3.0, 0.5, 3.8, 0.0, Some(0.5), Some(3.8), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("neck"), 2.0, 50, 50, 0, 50, 3.8, 0.5, 3.8, 0.0, Some(0.5), Some(3.8), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn inkling_roller_special_s_dash_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 8.0, 60, 90, 0, 25, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("neck"), 8.0, 80, 80, 0, 95, 3.0, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(boma, 0, 120.0);
        AttackModule::set_ink_value(boma, 1, 120.0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 8.0, 80, 80, 0, 95, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("neck"), 8.0, 80, 80, 0, 95, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(boma, 0, 100.0);
        AttackModule::set_ink_value(boma, 1, 100.0);
    }
}

unsafe extern "C" fn inkling_roller_special_air_s_dash_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 8.0, 75, 60, 0, 100, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("neck"), 8.0, 75, 60, 0, 100, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(boma, 0, 100.0);
        AttackModule::set_ink_value(boma, 1, 100.0);
    }
}

unsafe extern "C" fn inkling_roller_special_s_dash_no_ink_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 5.0, 50, 80, 0, 60, 3.0, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("neck"), 5.0, 50, 80, 0, 60, 3.0, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn inkling_roller_special_s_run_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 11.0, 60, 105, 0, 25, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("neck"), 11.0, 80, 80, 0, 90, 3.0, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(boma, 0, 100.0);
        AttackModule::set_ink_value(boma, 1, 100.0);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 11.0, 80, 80, 0, 90, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("neck"), 11.0, 80, 80, 0, 90, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(boma, 0, 100.0);
        AttackModule::set_ink_value(boma, 1, 100.0);
    }
}

unsafe extern "C" fn inkling_roller_special_air_s_run_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 11.0, 75, 80, 0, 90, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("neck"), 11.0, 75, 80, 0, 90, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(boma, 0, 100.0);
        AttackModule::set_ink_value(boma, 1, 100.0);
    }
}

unsafe extern "C" fn inkling_roller_special_s_run_no_ink_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 8.0, 50, 80, 0, 60, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("neck"), 8.0, 50, 80, 0, 60, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn inkling_splash_normal_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if !WorkModule::is_flag(owner_boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_AFTER_SPECIAL_HI) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 50, 30, 0, 60, 5.0, 0.0, 2.0, -0.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 50, 30, 0, 60, 5.0, 0.0, 2.0, -0.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(boma, 0, 50.0);
            AttackModule::set_ink_value(boma, 1, 50.0);
        }
        frame(lua_state, 3.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 50, 30, 0, 60, 5.0, 0.0, 2.0, -7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 50, 30, 0, 60, 5.0, 0.0, 2.0, 7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(boma, 0, 40.0);
            AttackModule::set_ink_value(boma, 1, 40.0);
        }
        frame(lua_state, 5.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
    }
}

unsafe extern "C" fn inkling_splashbomb_explode_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 47, 0, 60, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_BOMB);
        AttackModule::set_ink_value(boma, 0, 0.0);
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 12.0);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 14.0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn escape_air_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
        if is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, -1);
        }
    }
    let motion_kind = MotionModule::motion_kind(agent.module_accessor);
    let motion_frame = MotionModule::frame(agent.module_accessor);
    let motion_rate = MotionModule::rate(agent.module_accessor);
    ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(motion_kind), false, -1.0);
    if is_excute(agent) {
        ArticleModule::set_frame(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, motion_frame);
        ArticleModule::set_rate(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, motion_rate);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
        let status_kind = StatusModule::status_kind(agent.module_accessor);
        if status_kind != *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, true);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn escape_air_slide_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
        if is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, -1);
        }
    }
    let motion_kind = MotionModule::motion_kind(agent.module_accessor);
    let motion_frame = MotionModule::frame(agent.module_accessor);
    let motion_rate = MotionModule::rate(agent.module_accessor);
    ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(motion_kind), false, -1.0);
    if is_excute(agent) {
        ArticleModule::set_frame(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, motion_frame);
        ArticleModule::set_rate(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, motion_rate);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
        let status_kind = StatusModule::status_kind(agent.module_accessor);
        if status_kind != *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, true);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    smashline::Agent::new("inkling_splash")
        .acmd("game_normal", inkling_splash_normal_game)
        .install();
    smashline::Agent::new("inkling_splashbomb")
        .acmd("game_explode", inkling_splashbomb_explode_game)
        .install();
    smashline::Agent::new("inkling")
        .acmd("sound_damageflyhi", damageflyhi_sound)
        .acmd("sound_damageflylw", damageflyhi_sound)
        .acmd("sound_damageflyn", damageflyhi_sound)
        .acmd("sound_damageflyroll", damageflyroll_sound)
        .acmd("sound_damageflytop", damageflyhi_sound)
        .acmd("sound_dash", dash_sound)
        .acmd("game_escapeair", escape_air_game)
        .acmd("game_escapeairslide", escape_air_slide_game)
        .install();
    smashline::Agent::new("inkling_inkbullet")
        .acmd("game_fly", inkling_inkbullet_fly_game)
        .install();
    smashline::Agent::new("inkling_roller")
        .acmd("game_specialswalk", inkling_roller_special_s_walk_game)
        .acmd("game_specialairswalk", inkling_roller_special_air_s_walk_game)
        .acmd("game_specialswalknoink", inkling_roller_special_s_walk_no_ink_game)
        .acmd("game_specialsdash", inkling_roller_special_s_dash_game)
        .acmd("game_specialairsdash", inkling_roller_special_air_s_dash_game)
        .acmd("game_specialsdashnoink", inkling_roller_special_s_dash_no_ink_game)
        .acmd("game_specialsrun", inkling_roller_special_s_run_game)
        .acmd("game_specialairsrun", inkling_roller_special_air_s_run_game)
        .acmd("game_specialsrunnoink", inkling_roller_special_s_run_no_ink_game)
        .install();
}
