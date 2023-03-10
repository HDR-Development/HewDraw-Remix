
use super::*;


#[acmd_script( agent = "gamewatch", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_CHAIR, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
        ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_s3"), false, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 35, 99, 0, 35, 4.0, 0.0, 4.5, 15.0, Some(0.0), Some(6.5), Some(15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 35, 103, 0, 35, 3.5, 0.0, 4.0, 7.0, Some(0.0), Some(4.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 35, 135, 0, 35, 3.3, 0.0, 4.5, 15.0, Some(0.0), Some(6.5), Some(15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 35, 135, 0, 35, 2.8, 0.0, 4.0, 7.0, Some(0.0), Some(4.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ArticleModule::remove(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "gamewatch", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_attack_hi3_l_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.650);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.400);
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 75, 76, 0, 60, 4.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 75, 76, 0, 60, 4.5, 0.0, 14.0, 8.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 9.5, 75, 76, 0, 60, 3.7, 0.0, 9.1, 6.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        AttackModule::clear(boma, 1, false);
        AttackModule::clear(boma, 2, false);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.000);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 75, 76, 0, 60, 4.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 75, 76, 0, 60, 4.5, 0.0, 14.0, -8.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 9.5, 75, 76, 0, 60, 3.7, 0.0, 9.1, -6.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 20.5);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        VisibilityModule::set_default_int64(boma, Hash40::new("lhand").hash as i64);
    }
    
}

#[acmd_script( agent = "gamewatch", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::set_int(boma, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_MANHOLE, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
        ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_lw3"), false, 1.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 85, 116, 0, 36, 3.5, 0.0, 4.2, 6.7, Some(0.0), Some(2.1), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ArticleModule::remove(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        gamewatch_attack_s3_s_game,
        //gamewatch_attack_hi3_game,
        gamewatch_attack_hi3_l_game,
        gamewatch_attack_lw3_game,
    );
}

