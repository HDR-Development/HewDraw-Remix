use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE_RANGE(agent, 7.0, 20.0, 7.0);
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 20.0, 22.0, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 361, 84, 0, 40, 4.2, 0.0, 4.8, 7.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_WATER);
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 361, 84, 0, 40, 3.6, 0.0, 4.4, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_WATER);
        ATTACK(agent, 2, 0, Hash40::new("top"), 15.0, 361, 84, 0, 40, 3.4, 0.0, 2.8, 20.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_WATER);
    }
    frame(lua_state, 20.6);
    if is_excute(agent) {
        AttackModule::set_offset(boma, 0, &Vector3f::new(0.0, 4.8, 7.8));
        AttackModule::set_offset(boma, 1, &Vector3f::new(0.0, 5.4, 14.0));
        AttackModule::set_offset(boma, 2, &Vector3f::new(0.0, 5.6, 20.0));
    }
    frame(lua_state, 21.2);
    if is_excute(agent) {
        AttackModule::set_offset(boma, 0, &Vector3f::new(0.0, 4.8, 7.8));
        AttackModule::set_offset(boma, 1, &Vector3f::new(0.0, 7.4, 14.0));
        AttackModule::set_offset(boma, 2, &Vector3f::new(0.0, 10.6, 20.0));
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE_RANGE(agent, 25.0, 60.0, 22.0);
    frame(lua_state, 60.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pzenigame_mouth_water"), Hash40::new("head"), -0.5, 3, 0, -80, 0, 0, 1, true);
        if agent.lr() < 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("pzenigame_water_smash"), Hash40::new("top"), 0, 0, -1, 13, 160, 10, 0.8, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("pzenigame_water_smash_r"), Hash40::new("top"), 0, 0, 1, 3, 160, -10, 0.8, true);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 21.2);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pzenigame_mouth_water"), false, false);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE_RANGE(agent, 7.0, 19.0, 6.0);
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 110, 92, 110, 0, 5.5, 0.0, 3.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 110, 92, 110, 0, 5.5, 0.0, 3.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 2, 0, Hash40::new("top"), 11.0, 95, 60, 0, 60, 5.5, 0.0, 5.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        ATTACK(agent, 3, 0, Hash40::new("top"), 11.0, 95, 60, 0, 60, 5.5, 0.0, 5.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 95, 60, 0, 60, 5.5, 0.0, 8.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 95, 60, 0, 60, 5.5, 0.0, 8.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 95, 60, 0, 60, 4.5, 0.0, 14.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        ATTACK(agent, 3, 0, Hash40::new("top"), 10.0, 95, 60, 0, 60, 4.5, 0.0, 14.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 95, 60, 0, 60, 4.5, 0.0, 18.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 95, 60, 0, 60, 4.5, 0.0, 18.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 95, 60, 0, 60, 3.5, 0.0, 20.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 95, 60, 0, 60, 3.5, 0.0, 20.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 95, 60, 0, 60, 3.5, 0.0, 22.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 95, 60, 0, 60, 3.5, 0.0, 22.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE_RANGE(agent, 30.0, 60.0, 18.0);
    frame(lua_state, 60.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_water_ripple"), Hash40::new("top"), 0, 0, 7, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.6);
        EFFECT(agent, Hash40::new("pzenigame_water_ripple"), Hash40::new("top"), 0, 0, -7, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pzenigame_atk_hi4"), Hash40::new("top"), 0, 0, 7, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        EFFECT_FOLLOW(agent, Hash40::new("pzenigame_atk_hi4"), Hash40::new("top"), 0, 0, -7, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 7.0, 3.0);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(agent, 8.0, 17.0, 2.0);
    frame(lua_state, 17.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 35, 83, 0, 50, 4.0, 0.0, 3.0,  6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_WATER);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 35, 83, 0, 50, 4.0, 0.0, 3.0, 10.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_WATER);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 35, 83, 0, 50, 3.8, 0.0, 3.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_WATER);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE_RANGE(agent, 21.0, 25.0, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 35, 83, 0, 50, 4.0, 0.0, 3.0,  -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_WATER);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 35, 83, 0, 50, 4.0, 0.0, 3.0, -10.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_WATER);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 35, 83, 0, 50, 3.8, 0.0, 3.0, -15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_WATER);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE_RANGE(agent, 30.0, 60.0, 23.0);
    frame(lua_state, 60.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 2, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    if agent.lr() < 0.0 {
        frame(lua_state, 17.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("pzenigame_mouth_water"), Hash40::new("head"), -0.5, 3, 0, -80, 0, 0, 0.7, true);
            EFFECT_FOLLOW(agent, Hash40::new("pzenigame_water_cutter"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, true);
            EFFECT_FOLLOW(agent, Hash40::new("pzenigame_water_smash"), Hash40::new("top"), 0, 0.5, 0, 10, 180, 0, 0.75, true);
        }
        frame(lua_state, 19.0);
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("pzenigame_water_smash"), false, false);
        }
        frame(lua_state, 20.0);
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_ALPHA(agent, 0.6);
        }
    }
    else {
        frame(lua_state, 17.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("pzenigame_mouth_water"), Hash40::new("head"), -0.5, 3, 0, -80, 0, 0, 0.7, true);
            EFFECT_FOLLOW(agent, Hash40::new("pzenigame_water_cutter_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, true);
            EFFECT_FOLLOW(agent, Hash40::new("pzenigame_water_smash_r"), Hash40::new("top"), 0, 0.5, 0, 10, 180, 0, 0.75, true);
        }
        frame(lua_state, 19.0);
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("pzenigame_water_smash_r"), false, false);
        }
        frame(lua_state, 20.0);
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_ALPHA(agent, 0.6);
        }
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pzenigame_mouth_water"), false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);
    agent.acmd("game_attacks4hi", game_attacks4, Priority::Low);
    agent.acmd("game_attacks4lw", game_attacks4, Priority::Low);
    agent.acmd("effect_attacks4", effect_attacks4, Priority::Low);
    agent.acmd("effect_attacks4hi", effect_attacks4, Priority::Low);
    agent.acmd("effect_attacks4lw", effect_attacks4, Priority::Low);
    
    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
    agent.acmd("effect_attackhi4", effect_attackhi4, Priority::Low);

    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);
    agent.acmd("effect_attacklw4", effect_attacklw4, Priority::Low);
}