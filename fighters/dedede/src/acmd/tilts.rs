
use super::*;

#[acmd_script( agent = "dedede", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 2.5, 15, 100, 60, 0, 2.5, -8.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 2.5, 25, 100, 50, 0, 2.5, 1.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 367, 100, 30, 0, 6.0, 0.0, 7.0, 26.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 100, 100, 30, 0, 4.0, 0.0, 4.0, 24.0, Some(0.0), Some(4.0), Some(29.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 2.5, 250, 100, 26, 0, 4.0, 0.0, 10.5, 24.0, Some(0.0), Some(10.5), Some(29.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 3.0, 361, 180, 0, 25, 3.5, -8.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 3.0, 361, 180, 0, 25, 3.5, 1.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 3.0, 361, 180, 0, 25, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 18.0/(43.0-27.0));
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    
}

#[acmd_script( agent = "dedede", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 7.0, 6.0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("head"), 12.0, 96, 100, 0, 45, 6.0, -1.5, 1.0, 1.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("head"), 12.0, 96, 100, 0, 45, 6.0, 4.5, 0.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 6.0);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("head"), 12.0, 84, 100, 0, 45, 6.0, -1.5, 1.0, 1.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("head"), 12.0, 84, 100, 0, 45, 6.0, 4.5, 0.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "dedede", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {

        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 30, 5.0, 0.0, 4.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 100, 0, 30, 5.0, 0.0, 4.0, 17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {        
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 100, 0, 30, 4.0, 0.0, 4.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 361, 100, 0, 30, 4.0, 0.0, 4.0, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "dedede", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn dedede_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    /* 
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, 0, 0, 0, -90, 1.2, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind_s"), Hash40::new("top"), 0, 8, 0, 0, 0, -90, 1.4, false, 0.15);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    */
}

pub fn install() {
    install_acmd_scripts!(
        dedede_attack_s3_s_game,
        dedede_attack_hi3_game,
        dedede_attack_lw3_game,
        dedede_attack_lw3_effect,
    );
}

