use super::*;

unsafe extern "C" fn game_attack_air_n(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::set_int(boma, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_TROPICALFISH, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
        ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_air_n"), false, 0.0);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        }
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.5, 235, 100, 35, 0, 4.0, 0.0, 19.4, 4.4, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.5, 235, 100, 35, 0, 4.0, 0.0, 18.6, -2.2, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 367, 22, 0, 26, 4.0, 0.0, 13.0, -1.0, Some(0.0), Some(13.0), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.5, 367, 22, 0, 26, 4.0, 0.0, 9.5, -1.0, Some(0.0), Some(9.5), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.5, 225, 100, 35, 0, 4.5, 0.0, 20.799999, 10.8, Some(0.0), Some(21.0), Some(9.6), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.5, 215, 100, 35, 0, 4.5, 0.0, 17.9, -8.8, Some(0.0), Some(18.299999), Some(-7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 367, 22, 0, 26, 4.25, 0.0, 13.0, -1.0, Some(0.0), Some(13.0), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.5, 367, 22, 0, 26, 4.25, 0.0, 9.5, -1.0, Some(0.0), Some(9.5), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.5, 160, 100, 35, 0, 4.75, 0.0, 12.4, 13.3, Some(0.0), Some(13.6), Some(13.1), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.5, 155, 100, 35, 0, 4.75, 0.0, 11.0, -9.9, Some(0.0), Some(12.0), Some(-9.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 367, 22, 0, 26, 4.75, 0.0, 13.0, -1.0, Some(0.0), Some(13.0), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.5, 367, 22, 0, 26, 4.75, 0.0, 9.5, -1.0, Some(0.0), Some(9.5), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 50, 125, 0, 50, 2.0, 0.0, 11.0, 3.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 50, 125, 0, 50, 8.5, 0.0, 9.0, 1.8, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 4.2, 40, 190, 0, 35, 5.0, 0.0, 2.9, 12.8, Some(0.0), Some(1.5), Some(12.8), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("top"), 4.2, 40, 190, 0, 35, 5.0, 0.0, 2.9, -9.0, Some(0.0), Some(1.5), Some(-9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 35.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

unsafe extern "C" fn game_attack_air_f(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::set_int(boma, 0, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
        ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_air_f"), false, -1.0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 50, 96, 0, 14, 6.0, 0.0, 9.5, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 50, 96, 0, 14, 5.2, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 361, 96, 0, 14, 6.0, 0.0, 8.5, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 361, 96, 0, 14, 5.2, 0.0, 8.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 10, 6.0, 0.0, 8.5, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 90, 0, 10, 5.2, 0.0, 8.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ArticleModule::remove_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

unsafe extern "C" fn game_landing_air_f(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

unsafe extern "C" fn expression_attack_air_f(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);

    }
}

unsafe extern "C" fn game_attack_air_b(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::set_int(boma, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_TURTLE, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
        ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_air_b"), false, 1.0);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if !ArticleModule::is_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON) {
            ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, 0);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    for _ in 0..3 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 40, 0, 35, 4.2, 0.0, 3.5, -14.3, None, None, None, 0.5, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 367, 40, 0, 35, 4.5, 0.0, 1.8, -8.0, None, None, None, 0.5, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        wait(lua_state, 3.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 70, 0, 75, 6.0, 0.0, 3.5, -14.3, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 60, 70, 0, 75, 4.5, 0.0, 1.8, -8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

unsafe extern "C" fn game_landing_air_b(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("landing_air_b"), false, 1.0);
    }
    
}

unsafe extern "C" fn game_attack_air_hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 9.0, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BREATH, false, 0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 82, 70, 0, 55, 6.0, 2.0, 20.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 82, 70, 0, 55, 5.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE_RANGE(agent, 14.0, 42.0, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ArticleModule::remove_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BREATH, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

unsafe extern "C" fn sound_attack_air_hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_gamewatch_wave08_mi"));
    }
}

unsafe extern "C" fn expression_attack_air_hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        VisibilityModule::set_int64(boma, hash40("head") as i64, hash40("head_sparky") as i64);
        VisibilityModule::set_int64(boma, hash40("hand") as i64, hash40("hand_hold_lr") as i64);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_attack_air_lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        /* Ground-only */
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 270, 86, 0, 20, 4.5, 0.0, -7.5, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 270, 86, 0, 20, 4.0, 0.0, -2.5, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        /* Air-only */
        ATTACK(agent, 2, 0, Hash40::new("top"), 15.0, 270, 48, 0, 20, 4.5, 0.0, -7.5, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("top"), 15.0, 270, 48, 0, 20, 4.0, 0.0, -2.5, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 60, 108, 0, 22, 4.5, 0.0, -7.5, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 60, 108, 0, 22, 4.0, 0.0, -2.5, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 60, 100, 0, 20, 4.5, 0.0, -7.5, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 60, 100, 0, 20, 4.0, 0.0, -2.0, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

unsafe extern "C" fn game_landing_air_lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

}

unsafe extern "C" fn expression_landing_air_lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, Hash40::new("head").hash as i64, Hash40::new("head_close").hash as i64);
        VisibilityModule::set_int64(boma, Hash40::new("hand").hash as i64, Hash40::new("hand_hold_lr").hash as i64);
        VisibilityModule::set_int64(boma, Hash40::new("lhand").hash as i64, Hash40::new("lhand_key").hash as i64);
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *DAMAGE_NO_REACTION_MODE_NORMAL, *WEAPON_INKLING_ROLLER_INSTANCE_WORK_ID_FLOAT_B);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lands"), 0, false, 0 as u32);
    }
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attack_air_n);
    agent.acmd("game_attackairf", game_attack_air_f);
    agent.acmd("game_landingairf", game_landing_air_f);
    agent.acmd("expression_attackairf", expression_attack_air_f);
    agent.acmd("game_attackairb", game_attack_air_b);
    agent.acmd("game_landingairb", game_landing_air_b);
    agent.acmd("game_attackairhi", game_attack_air_hi);
    agent.acmd("sound_attackairhi", sound_attack_air_hi);
    agent.acmd("expression_attackairhi", expression_attack_air_hi);
    agent.acmd("game_attackairlw", game_attack_air_lw);
    agent.acmd("game_landingairlw", game_landing_air_lw);
    agent.acmd("expression_landingairlw", expression_landing_air_lw);
}
