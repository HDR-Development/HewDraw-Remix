
use super::*;


#[acmd_script( agent = "gamewatch", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::set_int(boma, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_TROPICALFISH, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
        ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_air_n"), false, 0.0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        }
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.5, 235, 100, 30, 0, 4.0, 0.0, 19.4, 3.8, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.5, 235, 100, 30, 0, 4.0, 0.0, 18.6, -2.2, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 367, 100, 35, 0, 4.0, 0.0, 13.0, -1.0, Some(0.0), Some(13.0), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.5, 105, 100, 45, 0, 4.0, 0.0, 9.5, -1.0, Some(0.0), Some(9.5), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.5, 225, 100, 30, 0, 4.5, 0.0, 20.799999, 10.8, Some(0.0), Some(21.0), Some(9.6), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.5, 215, 100, 30, 0, 4.5, 0.0, 17.9, -8.8, Some(0.0), Some(18.299999), Some(-7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 367, 100, 35, 0, 4.25, 0.0, 13.0, -1.0, Some(0.0), Some(13.0), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.5, 105, 100, 25, 0, 4.25, 0.0, 9.5, -1.0, Some(0.0), Some(9.5), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.5, 160, 100, 30, 0, 4.75, 0.0, 12.4, 13.3, Some(0.0), Some(13.6), Some(13.1), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.5, 155, 100, 30, 0, 4.75, 0.0, 11.0, -9.9, Some(0.0), Some(12.0), Some(-9.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 367, 100, 35, 0, 4.75, 0.0, 13.0, -1.0, Some(0.0), Some(13.0), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.5, 120, 100, 20, 0, 4.75, 0.0, 9.5, -1.0, Some(0.0), Some(9.5), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 145, 0, 35, 5.0, 0.0, 2.9, 13.5, Some(0.0), Some(1.5), Some(13.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 145, 0, 35, 5.0, 0.0, 2.4, -10.2, Some(0.0), Some(1.0), Some(-10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 361, 145, 0, 35, 8.25, 0.0, 11.0, 1.8, Some(0.0), Some(12.2), Some(3.4), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.000);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "gamewatch", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB, false, 0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB, false, 0);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB, false, 0);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB, false, 0);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {  }frame(lua_state, 10.0);
    if is_excute(fighter) {
        let box_position = Vector3f{ x:-2.0, y: 9.0, z: 0.0 };
        ModelModule::set_joint_translate(boma, Hash40::new("havel"), &box_position, false, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 361, 100, 0, 14, 5.7, 0.0, 8.5, 13.0, Some(0.0), Some(7.5), Some(7.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        //ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 361, 100, 0, 14, 5.0, 0.0, 9.0, 10.0, Some(0.0), Some(7.0), Some(7.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 10, 5.7, 0.0, 8.5, 13.0, Some(0.0), Some(7.5), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        //ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 90, 0, 10, 5.0, 0.0, 9.0, 10.0, Some(0.0), Some(7.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ArticleModule::remove_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "gamewatch", script = "game_landingairf" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_landing_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "gamewatch", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::set_int(boma, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_TURTLE, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
        ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_air_b"), false, 1.0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    for _ in 0..3 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 80, 0, 35, 4.2, 0.0, 3.5, -14.3, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 367, 80, 0, 35, 4.5, 0.0, 1.8, -8.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 68, 65, 0, 60, 6.0, 0.0, 3.5, -14.3, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 68, 65, 0, 60, 4.5, 0.0, 1.8, -8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "gamewatch", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.500);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    // frame(lua_state, 8.0);
    // if is_excute(fighter) {
    //     VarModule::off_flag(fighter.battle_object, vars::common::IS_HEAVY_ATTACK);
    // }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BREATH, false, 0);
        ArticleModule::shoot(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BREATH, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        //if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
        //    VarModule::on_flag(fighter.battle_object, vars::common::IS_HEAVY_ATTACK);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 367, 65, 0, 45, 7.0, 2.0, 25.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 367, 65, 0, 45, 6.0, 0.0, 15.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 367, 65, 0, 45, 4.0, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 94, 55, 0, 25, 7.0, 2.0, 25.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            ATTACK(fighter, 4, 0, Hash40::new("top"), 4.0, 94, 55, 0, 25, 6.0, 0.0, 15.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            ATTACK(fighter, 5, 0, Hash40::new("top"), 4.0, 94, 55, 0, 25, 4.0, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        // }
        // else {
        //     ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 86, 50, 0, 52, 7.0, 2.0, 25.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        //     ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 86, 50, 0, 52, 6.0, 0.0, 15.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        //     ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 86, 50, 0, 52, 4.0, 0.0, 9.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        // }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        //if VarModule::is_flag(fighter.battle_object, vars::common::IS_HEAVY_ATTACK) {
        ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BREATH, false, 0);
        ArticleModule::shoot(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BREATH, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 78, 82, 0, 80, 7.5, 2.0, 25.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 78, 82, 0, 80, 6.5, 0.0, 15.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 78, 82, 0, 80, 4.5, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        FT_MOTION_RATE(fighter, 1.250);
        // }
        // if !VarModule::is_flag(fighter.battle_object, vars::common::IS_HEAVY_ATTACK) {
        //     FT_MOTION_RATE(fighter, 0.400);
        // }
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "gamewatch", script = "sound_attackairhi" , category = ACMD_SOUND , low_priority)]
unsafe fn gamewatch_attack_air_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_wave08_mi"));
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        //if VarModule::is_flag(fighter.battle_object, vars::common::IS_HEAVY_ATTACK) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_wave08_mi"));
        //}
    }
}

#[acmd_script( agent = "gamewatch_breath", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_breath_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
}

#[acmd_script( agent = "gamewatch", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn gamewatch_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 270, 106, 0, 20, 5.5, 0.0, -6.5, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 270, 106, 0, 20, 4.0, 0.0, -2.5, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 108, 0, 22, 5.5, 0.0, -6.5, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 60, 108, 0, 22, 5.5, 0.0, -2.5, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 60, 100, 0, 20, 4.0, 0.0, -8.0, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 60, 100, 0, 20, 3.0, 0.0, -2.0, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        gamewatch_attack_air_n_game,
        gamewatch_attack_air_f_game,
        gamewatch_landing_air_f_game,
        gamewatch_attack_air_b_game,
        gamewatch_attack_air_hi_game,
        gamewatch_attack_air_hi_sound,
        gamewatch_breath_attack_air_hi_game,
        gamewatch_attack_air_lw_game,
    );
}

