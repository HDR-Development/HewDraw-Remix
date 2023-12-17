use super::*;

#[acmd_script( agent = "peach", script = "game_throwf", category = ACMD_GAME, low_priority )]
unsafe fn game_throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 16.0, 11.0);
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 40, 118, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    if IS_EXIST_ARTICLE(fighter, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO) {
        if is_excute(fighter) {
            ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, Hash40::new("throw_f"), false, -1.0);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 50, 0, 55, 4.5, 0.0, 7.5, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(fighter, 11, 5);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(fighter, 16.0, 36.0, 16.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "peach", script = "game_throwb", category = ACMD_GAME, low_priority )]
unsafe fn game_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 127, 84, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    if IS_EXIST_ARTICLE(fighter, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO) {
        if is_excute(fighter) {
            ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, Hash40::new("throw_b"), false, -1.0);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 70, 200, 0, 10, 6.2, 0.0, 7.2, -6.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(fighter, -10, 5);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE_RANGE(fighter, 35.0, 50.0, 8.0);   
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
    }
}

#[acmd_script( agent = "peach", script = "game_throwhi", category = ACMD_GAME, low_priority )]
unsafe fn game_throwhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 88, 65, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    if IS_EXIST_ARTICLE(fighter, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO) {
        if is_excute(fighter) {
            ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, Hash40::new("throw_hi"), false, -1.0);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 2.0, 80, 100, 20, 0, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(fighter, 0, 24);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_throwf,
        game_throwb,
        game_throwhi
    );
}