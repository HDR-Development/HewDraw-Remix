
use super::*;


#[acmd_script( agent = "murabito", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 8.0, 20, 30, 0, 70, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 8.0, 20, 30, 0, 70, 2.5, 0.0, 2.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 12.0, 25, 60, 0, 70, 4.5, 0.0, 9.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 12.0, 25, 60, 0, 70, 2.5, -5.0, 7.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 4, 0, Hash40::new("haver"), 12.0, 25, 60, 0, 70, 2.5, 5.0, 7.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.877);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        ArticleModule::remove(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "murabito", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        FT_MOTION_RATE(fighter, 0.850);
        ATTACK(fighter, 0, 0, Hash40::new("havel"), 5.0, 108, 100, 75, 0, 3.8, 0.0, -1.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("havel"), 5.0, 108, 100, 75, 0, 4.0, 0.0, 5.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("havel"), 5.0, 135, 10, 0, 35, 4.0, 0.0, 5.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 2.0, false);
    }
    wait(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        ATTACK(fighter, 0, 0, Hash40::new("havel"), 7.0, 80, 155, 0, 50, 5.0, 0.0, 6.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("havel"), 7.0, 80, 155, 0, 50, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
    }
    
}

#[acmd_script( agent = "murabito", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.746);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 4.0);
        ArticleModule::generate_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_WEEDS, true, 0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ArticleModule::remove(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_WEEDS, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 78, 70, 0, 60, 6.0, 0.0, 3.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 89, 70, 0, 60, 6.0, 0.0, 3.0, 12.0, Some(0.0), Some(3.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.8, 3.0);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        murabito_attack_s3_s_game,
        murabito_attack_hi3_game,
        murabito_attack_lw3_game,
    );
}

