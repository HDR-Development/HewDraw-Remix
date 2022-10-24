
use super::*;


#[acmd_script( agent = "szerosuit", script = "game_attacks3hi" , category = ACMD_GAME , low_priority)]
unsafe fn szerosuit_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 9.0, 35, 100, 0, 18, 3.8, 7.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 9.0, 35, 100, 0, 18, 4.2, 1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 9.0, 35, 100, 0, 18, 3.8, -5.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("legl"), 9.0, 35, 100, 0, 18, 3.0, 1.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "szerosuit", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn szerosuit_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 9.0, 35, 100, 0, 18, 3.8, 7.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 9.0, 35, 100, 0, 18, 4.2, 1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 9.0, 35, 100, 0, 18, 3.8, -5.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("legl"), 9.0, 35, 100, 0, 18, 3.0, 1.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "szerosuit", script = "game_attacks3lw" , category = ACMD_GAME , low_priority)]
unsafe fn szerosuit_attack_s3_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 12.0, 35, 100, 0, 18, 3.8, 7.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 12.0, 35, 100, 0, 18, 4.2, 1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 12.0, 35, 100, 0, 18, 3.8, -5.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("legl"), 12.0, 35, 100, 0, 18, 3.0, 1.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "szerosuit", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn szerosuit_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 4.0/(3.0-1.0));
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 5.0, 128, 30, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 5.0, 128, 30, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 5.0, 136, 40, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("footr"), 5.0, 136, 40, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 5.0, 90, 40, 0, 45, 3.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 90, 100, 0, 50, 6.0, 0.0, 20.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 90, 100, 0, 50, 5.0, 0.0, 14.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "szerosuit", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn szerosuit_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.625);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 8.0, 80, 90, 0, 44, 4.5, 5.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("legl"), 6.0, 65, 90, 0, 44, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneel"), 7.0, 72, 90, 0, 44, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        szerosuit_attack_s3_hi_game,
        szerosuit_attack_s3_s_game,
        szerosuit_attack_s3_lw_game,
        szerosuit_attack_hi3_game,
        szerosuit_attack_lw3_game,
    );
}

