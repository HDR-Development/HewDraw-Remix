
use super::*;



unsafe extern "C" fn shizue_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 11.5, 361, 78, 0, 56, 4.5, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 11.5, 361, 78, 0, 56, 2.5, -5.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 11.5, 361, 78, 0, 56, 2.5, 5.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 11.5, 361, 78, 0, 56, 2.0, 0.0, 4.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 4, 0, Hash40::new("armr"), 11.5, 361, 78, 0, 56, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        ArticleModule::remove(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}


unsafe extern "C" fn shizue_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.625);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 4.0);
        ArticleModule::generate_article(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_WEEDS, true, 0);
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
        ArticleModule::remove(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_WEEDS, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 92, 0, 45, 5.0, 0.0, 3.0, 3.0, Some(0.0), Some(3.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.5, 275, 52, 0, 77, 3.0, 0.0, 3.0, 13.0, Some(0.0), Some(3.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.8, 3.0);
    }
    
}


unsafe extern "C" fn shizue_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM, true, 0);
        ArticleModule::change_motion(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM, Hash40::new("attack_hi3"), true, 0.0);
    }
    FT_MOTION_RATE(fighter, 0.833);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("havel"), 8.0, 82, 86, 0, 62, 3.5, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("havel"), 8.0, 82, 86, 0, 62, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    FT_MOTION_RATE(fighter, 1);
    wait(lua_state, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    FT_MOTION_RATE(fighter, 0.842);
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        ArticleModule::remove(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}




pub fn install() {
    smashline::Agent::new("shizue")
        .acmd("game_attacks3", shizue_attack_s3_s_game)
        .acmd("game_attacklw3", shizue_attack_lw3_game)
        .acmd("game_attackhi3", shizue_attack_hi3_game)
        .install();
}
