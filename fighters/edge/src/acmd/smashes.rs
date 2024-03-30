use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) { 
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 0.875);
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 22.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 8.0, 3.0);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE_RANGE(agent, 25.0, 25.5, 1.0);
    frame(lua_state, 25.5);
    FT_MOTION_RATE_RANGE(agent, 25.5, 26.0, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("swordl1"), 20.0, 361, 78, 0, 57, 3.5, 8.0, -1.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("swordl1"), 20.0, 361, 78, 0, 57, 3.5, 15.0, -1.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("swordl1"), 10.5, 361, 85, 0, 57, 3.0, 1.0, -1.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("swordl1"), 15.0, 361, 88, 0, 57, 3.0, 22.0, -1.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE(agent, 1.0);
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 3.0);
    }
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 13, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4.0, 0.0, -0.7, Hash40::new("swordl2"), 29.2, 0.0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 3);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);

        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 10.0, 0, 0.8, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 17.0, 0, 0.8, 0, 0, 0, 1.3, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 10.0, 0, 0.8, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 17.0, 0, 0.8, 0, 0, 0, 1.3, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 10, 0, 4, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_sword_flare"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword"), Hash40::new("top"), -1, 13.2, 1.5, 0, -50, 24, 0.95, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        EFFECT_FOLLOW(agent, Hash40::new("edge_slash_light"), Hash40::new("top"), -1, 13.2,1.5, 0, 40, 24, 1.05, true);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("edge_attack_dash"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("edge_attack_dash_aura"), -1);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_attack_dash"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("edge_attack_dash_aura"), false, false);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.6);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 25.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("swordl1"), 15.5, 92, 79, 0, 67, 4.5, 15.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("swordl1"), 15.5, 92, 79, 0, 67, 4.5, 9.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("swordl1"), 11.5, 92, 81, 0, 67, 3.5, 21.0, 0.0, 0.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("swordl1"), 10.5, 92, 81, 0, 62, 3.5, 4.5, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //ATTACK(agent, 3, 0, Hash40::new("swordl1"), 10.5, 92, 81, 0, 62, 4.5, 5.2, 0.0, 3.0, Some(4.5), Some(-2.0), Some(9.5), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //ATTACK(agent, 4, 0, Hash40::new("swordl1"), 15.5, 92, 79, 0, 67, 6.5, 13.8, 0.0, 3.5, Some(12.5), Some(-2.5), Some(11.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //ATTACK(agent, 5, 0, Hash40::new("swordl1"), 11.5, 92, 81, 0, 67, 4.5, 22.0, 0.0, 3.0, Some(19.0), Some(-3.0), Some(13.5), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 3, false);
        AttackModule::clear(boma, 4, false);
        AttackModule::clear(boma, 5, false);
        ATTACK(agent, 0, 0, Hash40::new("swordl1"), 17.0, 97, 79, 0, 67, 4.5, 15.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("swordl1"), 17.0, 97, 79, 0, 67, 4.5, 9.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("swordl1"), 13.0, 97, 81, 0, 67, 4.5, 21.0, 0.0, 0.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("swordl1"), 12.0, 97, 81, 0, 62, 4.5, 4.5, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("swordl1"), 17.0, 83, 79, 0, 67, 4.5, 15.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("swordl1"), 17.0, 83, 79, 0, 67, 4.5, 9.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("swordl1"), 13.0, 83, 81, 0, 67, 4.5, 21.0, 0.0, 0.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("swordl1"), 12.0, 83, 81, 0, 62, 4.5, 4.5, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    // frame(lua_state, 30.0);
    // if is_excute(agent) {
        // ATTACK(agent, 3, 0, Hash40::new("top"), 17.0, 97, 79, 0, 67, 6.5, 0.0, 34.0, 5.0, Some(0.0), Some(34.0), Some(-5.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // ATTACK(agent, 4, 0, Hash40::new("top"), 13.0, 97, 81, 0, 67, 4.5, 0.0, 40.5, 8.5, Some(0.0), Some(40.5), Some(-8.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    // }
    // frame(lua_state, 31.0);
    // if is_excute(agent) {
        // AttackModule::clear(boma, 3, false);
        // AttackModule::clear(boma, 4, false);
    // }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 23, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 10.0, 0, 0.8, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 1.0);
        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 17.0, 0, 0.8, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 1.0);
        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 10.0, 0, 0.8, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 1.0);
        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 17.0, 0, 0.8, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 1.0);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4.0, 0.0, -0.6, Hash40::new("swordl2"), 29.0, 0.0, 1.4, true, Hash40::new("null"), Hash40::new("swordl2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.2);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_slash_light"), Hash40::new("top"), -1.5, 17.5, -2.5, 2.5, 120, 90, 1, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword"), Hash40::new("top"), -1.5, 17.5, -2.5, 2.5, 85, 90, 0.92, true);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_sword_flare"), false, false);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 5);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("edge_attack_dash"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("edge_attack_dash_aura"), -1);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_attack_dash"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("edge_attack_dash_aura"), false, false);
    }
    frame(lua_state, 54.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -4.5, 0, 2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 10.0);
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if is_excute(agent) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 20);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 8.0, 3.0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        FighterSpecializer_Edge::attack_lw4_ray_check(boma);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_ATTACK_LW4_WORK_FLAG_IS_HIT_FLOOR) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
            ATTACK(agent, 0, 0, Hash40::new("top"), 16.5, 63, 68, 0, 85, 4.6, 0.0, 2.5, 23.5, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"), 13.5, 63, 68, 0, 85, 8.8, 0.0, 6.5, 23.5, Some(0.0), Some(8.5), Some(23.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATTACK(agent, 2, 0, Hash40::new("swordl1"), 10.5, 48, 68, 0, 80, 1.5, -2.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("swordl1"), 11.5, 48, 68, 0, 80, 1.5, 10.0, 0.0, 0.0, Some(19.0), Some(0.0), Some(0.8), 0.7, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 4, 0, Hash40::new("swordl1"), 13.0, 48, 68, 0, 80, 1.5, 20.0, 0.0, 0.8, Some(24.0), Some(0.0), Some(1.5), 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        else {
                ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
                ATTACK(agent, 0, 0, Hash40::new("swordl1"), 10.5, 48, 63, 0, 75, 1.7, -2.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 1, 0, Hash40::new("swordl1"), 11.5, 48, 63, 0, 75, 1.7, 10.0, 0.0, 0.0, Some(19.0), Some(0.0), Some(0.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 2, 0, Hash40::new("swordl1"), 13.0, 315, 63, 0, 75, 1.7, 20.0, 0.0, 0.8, Some(24.0), Some(0.0), Some(1.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 3, 0, Hash40::new("swordl1"), 13.0, 48, 63, 0, 75, 1.7, 20.0, 0.0, 0.8, Some(24.0), Some(0.0), Some(1.5), 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 23.0);
    if !WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_ATTACK_LW4_WORK_FLAG_IS_HIT_FLOOR) {
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 24.0);
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_ATTACK_LW4_WORK_FLAG_IS_HIT_FLOOR) {
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 3.0);
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, 7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 22, 0, 0.8, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 22, 0, 0.8, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_smashlw_speedline"), Hash40::new("swordl2"), 0, 0, 0, 0, -90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_smashlw_swordflare"), Hash40::new("swordl2"), 0, 0, 0, 0, -90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_ATTACK_LW4_WORK_FLAG_IS_HIT_FLOOR) {
        frame(lua_state, 21.0);
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("edge_attack_dash"), false, false);
            EFFECT_OFF_KIND(agent, Hash40::new("edge_attack_dash_aura"), false, false);
            EFFECT_OFF_KIND(agent, Hash40::new("edge_sword_flash2"), false, false);
            EFFECT_OFF_KIND(agent, Hash40::new("edge_attack_dash2"), false, false);
            
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 23.5, 0, 0, 0, 180, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT(agent, Hash40::new("edge_smashlw_impact"), Hash40::new("top"), 0, 0, 23.5, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 0.9);
            FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 24.5, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 0.6);
        }
        frame(lua_state, 22.0);
        if is_excute(agent) {
            EFFECT_DETACH_KIND(agent, Hash40::new("edge_smashlw_speedline"), -1);
        }
        frame(lua_state, 42.0);
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("edge_sword_light3"), false, true);
        }
        frame(lua_state, 49.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("edge_smashlw_smoke"), Hash40::new("top"), 0, 0, 27, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    } else {
        frame(lua_state, 21.0);
        if is_excute(agent) {
            EFFECT_FLW_POS(agent, Hash40::new("edge_sword_flash2"), Hash40::new("swordl2"), 22, 0, 0.8, 0, 0, 0, 1.2, true);
            EFFECT(agent, Hash40::new("edge_attack_dash2"), Hash40::new("swordl2"), 22.0, 0, 0.8, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.0);
        }
        frame(lua_state, 22.0);
        if is_excute(agent) {
            EFFECT_DETACH_KIND(agent, Hash40::new("edge_smashlw_speedline"), -1);
            EFFECT_OFF_KIND(agent, Hash40::new("edge_smashlw_swordflare"), false, true);
        }
        frame(lua_state, 34.0);
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("edge_sword_light3"), false, true);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4);
    agent.acmd("effect_attacks4", effect_attacks4);

    agent.acmd("game_attackhi4", game_attackhi4);
    agent.acmd("effect_attackhi4", effect_attackhi4);

    agent.acmd("game_attacklw4", game_attacklw4);
    agent.acmd("effect_attacklw4", effect_attacklw4);
}
