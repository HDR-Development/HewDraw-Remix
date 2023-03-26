
use super::*;

#[acmd_script( agent = "edge", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn sephiroth_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.8);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) { 
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 0.875);
    frame(lua_state, 21.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 8.0, 3.0);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0/(25.5-25.0));
    }
    frame(lua_state, 25.5);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0/(26.0-25.5));
        ATTACK(fighter, 0, 0, Hash40::new("swordl1"), 20.0, 361, 78, 0, 57, 3.5, 8.0, -1.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("swordl1"), 20.0, 361, 78, 0, 57, 3.5, 15.0, -1.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordl1"), 10.5, 361, 85, 0, 57, 3.0, 1.0, -1.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordl1"), 15.0, 361, 88, 0, 57, 3.0, 22.0, -1.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 3.0);
    }
    
}

#[acmd_script( agent = "edge", script = "effect_attacks4" , category = ACMD_EFFECT , low_priority)]
unsafe fn sephiroth_attack_s4_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 13, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4.0, 0.0, -0.7, Hash40::new("swordl2"), 29.2, 0.0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 3);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);

        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 10.0, 0, 0.8, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 17.0, 0, 0.8, 0, 0, 0, 1.3, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 10.0, 0, 0.8, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 17.0, 0, 0.8, 0, 0, 0, 1.3, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 10, 0, 4, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_sword_flare"), false, true);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword"), Hash40::new("top"), -1, 13.2, 1.5, 0, -50, 24, 0.95, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_slash_light"), Hash40::new("top"), -1, 13.2,1.5, 0, 40, 24, 1.05, true);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_attack_dash"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_attack_dash_aura"), -1);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_attack_dash"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_attack_dash_aura"), false, false);
    }
    
}

#[acmd_script( agent = "edge", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn sephiroth_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.6);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {  }frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 25.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("swordl1"), 15.5, 92, 79, 0, 67, 4.5, 15.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("swordl1"), 15.5, 92, 79, 0, 67, 4.5, 9.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordl1"), 11.5, 92, 81, 0, 67, 3.5, 21.0, 0.0, 0.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordl1"), 10.5, 92, 81, 0, 62, 3.5, 4.5, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //ATTACK(fighter, 3, 0, Hash40::new("swordl1"), 10.5, 92, 81, 0, 62, 4.5, 5.2, 0.0, 3.0, Some(4.5), Some(-2.0), Some(9.5), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //ATTACK(fighter, 4, 0, Hash40::new("swordl1"), 15.5, 92, 79, 0, 67, 6.5, 13.8, 0.0, 3.5, Some(12.5), Some(-2.5), Some(11.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //ATTACK(fighter, 5, 0, Hash40::new("swordl1"), 11.5, 92, 81, 0, 67, 4.5, 22.0, 0.0, 3.0, Some(19.0), Some(-3.0), Some(13.5), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 3, false);
        AttackModule::clear(boma, 4, false);
        AttackModule::clear(boma, 5, false);
        ATTACK(fighter, 0, 0, Hash40::new("swordl1"), 17.0, 97, 79, 0, 67, 4.5, 15.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("swordl1"), 17.0, 97, 79, 0, 67, 4.5, 9.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordl1"), 13.0, 97, 81, 0, 67, 4.5, 21.0, 0.0, 0.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordl1"), 12.0, 97, 81, 0, 62, 4.5, 4.5, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("swordl1"), 17.0, 83, 79, 0, 67, 4.5, 15.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("swordl1"), 17.0, 83, 79, 0, 67, 4.5, 9.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordl1"), 13.0, 83, 81, 0, 67, 4.5, 21.0, 0.0, 0.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordl1"), 12.0, 83, 81, 0, 62, 4.5, 4.5, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    // frame(lua_state, 30.0);
    // if is_excute(fighter) {
        // ATTACK(fighter, 3, 0, Hash40::new("top"), 17.0, 97, 79, 0, 67, 6.5, 0.0, 34.0, 5.0, Some(0.0), Some(34.0), Some(-5.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // ATTACK(fighter, 4, 0, Hash40::new("top"), 13.0, 97, 81, 0, 67, 4.5, 0.0, 40.5, 8.5, Some(0.0), Some(40.5), Some(-8.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    // }
    // frame(lua_state, 31.0);
    // if is_excute(fighter) {
        // AttackModule::clear(boma, 3, false);
        // AttackModule::clear(boma, 4, false);
    // }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "edge", script = "effect_attackhi4" , category = ACMD_EFFECT , low_priority)]
unsafe fn sephiroth_attack_hi4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 23, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 10.0, 0, 0.8, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 17.0, 0, 0.8, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 10.0, 0, 0.8, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 17.0, 0, 0.8, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4.0, 0.0, -0.6, Hash40::new("swordl2"), 29.0, 0.0, 1.4, true, Hash40::new("null"), Hash40::new("swordl2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_slash_light"), Hash40::new("top"), -1.5, 17.5, -2.5, 2.5, 120, 90, 1, true);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword"), Hash40::new("top"), -1.5, 17.5, -2.5, 2.5, 85, 90, 0.92, true);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_sword_flare"), false, false);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 5);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_attack_dash"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_attack_dash_aura"), -1);
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_attack_dash"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_attack_dash_aura"), false, false);
    }
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -4.5, 0, 2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        sephiroth_attack_s4_s_game,
        sephiroth_attack_s4_s_effect,
        sephiroth_attack_hi4_game,
        sephiroth_attack_hi4_effect,
		//sephiroth_attack_lw4_game,
    );
}

