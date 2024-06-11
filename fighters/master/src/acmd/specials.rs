use super::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.500);
        if ArticleModule::is_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW) {
            ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, false, 0);
        if !ArticleModule::is_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1) {
            ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, false, 0);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_TURN);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_CAN_SHOOT);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.75);
    }
}

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::shoot_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        FT_MOTION_RATE(agent, 0.8);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialsf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_F_REQUEST);
        FT_MOTION_RATE(agent, 3.0/(8.0-2.0));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        // Spear hitboxes
        ATTACK(agent, 0, 0, Hash40::new("haver"), 8.5, 45, 35, 0, 85, 3.0, 0.0, 8.5, -1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 8.5, 45, 35, 0, 85, 3.0, 0.0, 13.5, -1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 11.0, 45, 35, 0, 85, 3.5, 0.0, 19.5, -1.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 11.0, 45, 35, 0, 85, 3.5, 0.0, 25.0, -1.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        // Trail hitoxes
        ATTACK(agent, 4, 0, Hash40::new("haver"), 8.5, 45, 35, 0, 85, 3.0, 0.0, 8.5, -2.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 5, 0, Hash40::new("haver"), 8.5, 45, 35, 0, 85, 3.0, 0.0, 13.5, -2.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 6, 0, Hash40::new("haver"), 11.0, 45, 35, 0, 85, 3.5, 0.0, 19.5, -2.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 7, 0, Hash40::new("haver"), 11.0, 45, 35, 0, 85, 3.5, 0.0, 25.0, -2.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        //ATTACK(agent, 4, 0, Hash40::new("top"), 8.5, 45, 35, 0, 85, 5.0, 0.0, 5.0, 9.0, Some(0.0), Some(10.0), Some(9.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        //ATTACK(agent, 5, 0, Hash40::new("top"), 8.5, 45, 35, 0, 85, 5.0, 0.0, 5.0, 9.5, Some(0.0), Some(10.0), Some(9.5), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        //ATTACK(agent, 6, 0, Hash40::new("top"), 8.5, 45, 35, 0, 85, 5.0, 0.0, 5.0, 9.5, Some(0.0), Some(10.0), Some(9.5), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        // Spear hitboxes
        ATTACK(agent, 0, 0, Hash40::new("haver"), 8.5, 67, 40, 0, 75, 3.0, 0.0, 8.5, -1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 8.5, 67, 40, 0, 75, 3.0, 0.0, 13.5, -1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 11.0, 100, 40, 0, 100, 3.5, 0.0, 19.5, -1.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 11.0, 100, 40, 0, 100, 3.5, 0.0, 25.0, -1.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        // Trail hitboxes
        ATTACK(agent, 4, 0, Hash40::new("haver"), 8.5, 67, 40, 0, 75, 3.0, 0.0, 8.5, -5.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 5, 0, Hash40::new("haver"), 8.5, 67, 40, 0, 75, 3.0, 0.0, 13.5, -5.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 6, 0, Hash40::new("haver"), 11.0, 100, 40, 0, 100, 3.5, 0.0, 19.5, -6.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 7, 0, Hash40::new("haver"), 11.0, 100, 40, 0, 100, 3.5, 0.0, 27.0, -6.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        //ATTACK(agent, 4, 0, Hash40::new("top"), 8.5, 70, 40, 0, 65, 5.0, 0.0, 5.0, 11.0, Some(0.0), Some(10.0), Some(11.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        //ATTACK(agent, 5, 0, Hash40::new("top"), 11.0, 70, 40, 0, 75, 5.5, 0.0, 6.0, 22.0, Some(0.0), Some(11.0), Some(22.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        //ATTACK(agent, 6, 0, Hash40::new("top"), 11.0, 70, 40, 0, 75, 2.0, 0.0, 8.5, 28.0, Some(0.0), Some(14.0), Some(28.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        //ATTACK(agent, 4, 0, Hash40::new("top"), 8.5, 70, 40, 0, 65, 5.0, 0.0, 5.0, 11.0, Some(0.0), Some(14.0), Some(11.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        //ATTACK(agent, 5, 0, Hash40::new("top"), 11.0, 70, 40, 0, 75, 5.5, 0.0, 6.0, 22.0, Some(0.0), Some(21.0), Some(22.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        //ATTACK(agent, 6, 0, Hash40::new("top"), 11.0, 70, 40, 0, 75, 2.0, 0.0, 8.5, 28.0, Some(0.0), Some(18.5), Some(28.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            FT_MOTION_RATE(agent, 0.8);
        }
        else{
            FT_MOTION_RATE(agent, 1.1);
        }
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_specialsf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_spear_aura"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_spear_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("master_spear_scrape_ground"), Hash40::new("top"), -8, 0, 13, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("master_spear_scrape"), Hash40::new("top"), -16, 0, 4, 0, 18, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_spear_slash"), Hash40::new("top"), 2, 15, 8, 8, -5, 68, 0.95, true);
        EffectModule::set_disable_render_offset_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("master_spear_slash_particle_end"), Hash40::new("top"), 2, 15, 8, 8, -5, 68, 0.75, true);
        EffectModule::set_disable_render_offset_last(boma);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("master_spear_slash_particle"), Hash40::new("top"), 2, 14, 8, 8, -5, 68, 0.65, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("master_spear_aura_particle"), Hash40::new("haver"), -8, 6, 0, 0, 0, 0, 0.8, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_spear_scrape"), false, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_spear_slash_particle"), false, true);
    }
}

unsafe extern "C" fn game_specialsfdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_UP_REQUEST);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_DOWN_REQUEST);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        // Spear hitboxes
        ATTACK(agent, 0, 0, Hash40::new("haver"), 11.5, 45, 35, 0, 85, 3.0, 0.0, 8.5, -1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 11.5, 45, 35, 0, 85, 3.0, 0.0, 13.5, -1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 17.0, 45, 35, 0, 85, 3.5, 0.0, 19.5, -1.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 17.0, 45, 35, 0, 85, 3.5, 0.0, 25.0, -1.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        // Trail hitoxes
        ATTACK(agent, 4, 0, Hash40::new("haver"), 11.5, 45, 35, 0, 85, 3.0, 0.0, 8.5, -2.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 5, 0, Hash40::new("haver"), 11.5, 45, 35, 0, 85, 3.0, 0.0, 13.5, -2.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 6, 0, Hash40::new("haver"), 17.0, 45, 35, 0, 85, 3.5, 0.0, 19.5, -2.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 7, 0, Hash40::new("haver"), 17.0, 45, 35, 0, 85, 3.5, 0.0, 25.0, -2.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        // Spear hitboxes
        ATTACK(agent, 0, 0, Hash40::new("haver"), 11.5, 45, 35, 0, 85, 3.5, 0.0, 8.5, -1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 11.5, 45, 35, 0, 85, 3.5, 0.0, 13.5, -1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 17.0, 59, 48, 0, 115, 4.5, 0.0, 19.5, -1.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 17.0, 59, 48, 0, 115, 4.5, 0.0, 25.0, -1.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        // Trail hitoxes
        ATTACK(agent, 4, 0, Hash40::new("haver"), 11.5, 45, 35, 0, 85, 3.5, 0.0, 8.5, -5.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 5, 0, Hash40::new("haver"), 11.5, 45, 35, 0, 85, 3.5, 0.0, 13.5, -5.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 6, 0, Hash40::new("haver"), 17.0, 59, 48, 0, 115, 4.5, 0.0, 19.5, -6.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 7, 0, Hash40::new("haver"), 17.0, 59, 48, 0, 115, 4.5, 0.0, 26.0, -6.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialairsf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 10.0, 5.0, 5.0, 9.0);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_F_REQUEST);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 5.0, 5.0, 5.0, 5.0);
        if !VarModule::is_flag(agent.battle_object, vars::common::instance::SPECIAL_STALL_USED){
            let stick_y = ControlModule::get_stick_y(boma);
            VarModule::on_flag(agent.battle_object, vars::common::instance::SPECIAL_STALL_USED);
            if stick_y > 0.0{
                KineticModule::add_speed(boma, &Vector3f::new(0.0, 1.25 * stick_y, 0.0));
            }
        }
        
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 9.5, 65, 35, 0, 75, 4.5, 0.0, 7.0, -2.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 9.5, 65, 35, 0, 75, 4.5, 0.0, 11.5, -2.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 15.0, 85, 59, 0, 75, 4.5, 0.0, 19.5, -2.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 15.0, 85, 59, 0, 75, 4.5, 0.0, 25.5, -2.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn game_specialairsfdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 10.0, 5.0, 5.0, 9.0);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_UP_REQUEST);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_DOWN_REQUEST);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 5.0, 5.0, 5.0, 5.0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 9.5, 45, 35, 0, 85, 4.5, 0.0, 7.0, -2.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 9.5, 45, 35, 0, 85, 4.5, 0.0, 11.5, -2.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 15.0, 55, 62, 0, 86, 5.0, 0.0, 19.5, -2.5, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 15.0, 55, 62, 0, 86, 5.0, 0.0, 25.5, -2.5, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 10.0, 4.5);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 6.0);
        ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, Hash40::new("special_hi"), false, 0.0);
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_TURN_CHECK);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 368, 100, 30, 0, 5.3, 0.0, 8.0, 6.9, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 17.0, y: 42.0}, 5, false);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_dead_all(boma, true, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, *WEAPON_MASTER_SWORD_STATUS_KIND_EXTEND, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.01, 368, 100, 30, 0, 5.5, 2.0, -1.0, -1.0, Some(2.0), Some(-1.0), Some(-1.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 4.5, 2.0, -1.0, -1.0, Some(2.0), Some(-1.0), Some(-1.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 17.0, y: 42.0}, 5, false);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_dead_all(boma, true, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_ENABLE_CATCH);
        ATTACK(agent, 0, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 3.5, 0.0, 3.0, 0.75, Some(0.0), Some(-4.0), Some(-1.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 2.0, 0.0, 2.0, 0.75, Some(0.0), Some(-4.0), Some(-1.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 3.5, 0.0, 9.5, 2.5, Some(0.0), Some(-4.0), Some(-1.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 2.0, 0.0, 10.0, 3.0, Some(0.0), Some(-4.0), Some(-1.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, *WEAPON_MASTER_SWORD_STATUS_KIND_BACK, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.3);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 6.0);
        ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, Hash40::new("special_air_hi_start"), false, -1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_TURN_CHECK);
    }
    frame(lua_state, 10.0);
        FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 1.7, 0.0, 30.0, 9.0, Some(0.0), Some(46.0), Some(16.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY, false);
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_MASTER_CLIFF_HANG_DATA_AIR_LASSO as u32);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        ArticleModule::change_status(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, *WEAPON_MASTER_SWORD_STATUS_KIND_EXTEND, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.01, 368, 100, 30, 0, 5.5, 2.0, -1.0, -1.0, Some(2.0), Some(-1.0), Some(-1.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 4.5, 2.0, -1.0, -1.0, Some(2.0), Some(-1.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 16.0, y: 40.0}, 4, false);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_dead_all(boma, true, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.01, 368, 100, 30, 0, 3.5, 0.0, 3.0, 0.75, Some(0.0), Some(-4.0), Some(-1.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("throw"), 10.0, 75, 100, 0, 40, 2.0, 0.0, 2.0, 0.75, Some(0.0), Some(-4.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 16.0, y: 41.0}, 2, false);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::master::instance::SPECIAL_AIR_HI_CATCH) {
            ATTACK(agent, 0, 0, Hash40::new("throw"), 10.0, 75, 100, 0, 40, 4.5, 2.0, -1.0, -1.0, Some(2.0), Some(-1.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);    
        }
        else {
            WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_ENABLE_CATCH);
            ATTACK(agent, 0, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 3.5, 0.0, 3.0, 0.75, Some(0.0), Some(-4.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 2.0, 0.0, 2.0, 0.75, Some(0.0), Some(-4.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_no_dead_all(boma, true, false);
            AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 3.5, 0.0, 9.5, 2.5, Some(0.0), Some(-4.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("throw"), 10.0, 75, 100, 0, 40, 2.0, 0.0, 10.0, 3.0, Some(0.0), Some(-4.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, *WEAPON_MASTER_SWORD_STATUS_KIND_BACK, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialairhiovertake(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
		/*
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){
            VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        }
		*/
        VarModule::on_flag(agent.battle_object, vars::master::instance::SPECIAL_AIR_HI_CATCH);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::phx::Hash40::new("special_air_hi_overtake"), false, 0.0);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            // Hold input throw
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, 2, 10.0, 260, 100, 0, 14, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
        }
        else{
            // Normal throw
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.5, 110, 70, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);    
        }
        // Grab break
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 1, 6.5, 114, 65, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
        
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 5.0);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 1.0);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_TARGET_AIR){
            if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
                ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
                //ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            }
            else{
                ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            }
        }
        else{
            ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        } 
    }
}

unsafe extern "C" fn effect_specialairhiovertake(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold_end"), Hash40::new("top"), 0, 0.0, 0, 0.0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_COLOR(agent, 0.1, 0.0, 3.0);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 2.2, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 3.0);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash_particle"), Hash40::new("top"), 0, 0, 0, 0.0, 0, 0, 0.65, true);
            LAST_EFFECT_SET_COLOR(agent, 0.1, 0.0, 3.0);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT(agent, Hash40::new("master_wire_hit"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.1, 0.0, 3.0);
        }
        else{
            EFFECT(agent, Hash40::new("master_wire_hit"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::master::status::AYMR_CHARGE_LEVEL, 0);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.0, 3.0);
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){
            VarModule::set_int(agent.battle_object, vars::master::status::AYMR_CHARGE_LEVEL, 1);
        }
        else{
            VarModule::set_int(agent.battle_object, vars::master::status::AYMR_CHARGE_LEVEL, 0);
            let motion_rate = 5.0/(42.0-14.0);
            FT_MOTION_RATE(agent, motion_rate);
            ArticleModule::set_rate(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, 1.0/motion_rate);
        }
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        // if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){
        //     VarModule::set_int(agent.battle_object, vars::master::status::AYMR_CHARGE_LEVEL, 2);
        //     let motion_rate = 48.0/(42.0-34.0);
        //     FT_MOTION_RATE(agent, motion_rate);
        //     ArticleModule::set_rate(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, 1.0/motion_rate);
        //     DamageModule::set_damage_mul(boma, 1.25);
        // }
        if VarModule::get_int(agent.battle_object, vars::master::status::AYMR_CHARGE_LEVEL) > 0 {
            WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_START_SUPER_ARMOR);
        }
        if VarModule::get_int(agent.battle_object, vars::master::status::AYMR_CHARGE_LEVEL) == 1 {
            let motion_rate = 5.0/(42.0-34.0);
            FT_MOTION_RATE(agent, motion_rate);
            ArticleModule::set_rate(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, 1.0/motion_rate);
        }
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ArticleModule::set_rate(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, 1.0);
        WorkModule::off_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK);
    }
    frame(lua_state, 51.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
        
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_END_SUPER_ARMOR);
        ArticleModule::set_flag(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, true, *WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        ArticleModule::set_flag(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, *WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
        WorkModule::off_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
        
    }
    frame(lua_state, 96.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY);
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED);
    }
    frame(lua_state, 117.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
    }
    frame(lua_state, 118.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::get_int(agent.battle_object, vars::master::status::AYMR_CHARGE_LEVEL) > 0 { 
            EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
            EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold2"), false, true);
            EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold_end"), false, true);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 6.0);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 6.0);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold_end"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 6.0);

            //COL_PRI(agent, 200);
            //FLASH(agent, 1.0, 0.115, 0.71, 0.75);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        if VarModule::get_int(agent.battle_object, vars::master::status::AYMR_CHARGE_LEVEL) > 1 {
            EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
            EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold2"), false, true);
            EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold_end"), false, true);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold_end"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.3, 0.3, 0.3);

            //COL_PRI(agent, 200);
            //FLASH(agent, 0.325, 0.157, 0.05, 0.75);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold_end"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold2"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash_reverb"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, false);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash_particle"), Hash40::new("top"), 0, 7, 1, 0, 0, 0, 1, true);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("master_axe_slash_reverb"), -1);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axe_slash"), false, true);
    }
}

unsafe extern "C" fn effect_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::get_int(agent.battle_object, vars::master::status::AYMR_CHARGE_LEVEL) > 0 { 
            EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
            EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold2"), false, true);
            EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold_end"), false, true);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 6.0);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 6.0);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold_end"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 6.0);

            //COL_PRI(agent, 200);
            //FLASH(agent, 1.0, 0.115, 0.71, 0.75);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        if VarModule::get_int(agent.battle_object, vars::master::status::AYMR_CHARGE_LEVEL) > 1 {
            EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
            EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold2"), false, true);
            EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold_end"), false, true);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold_end"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.3, 0.3, 0.3);

            //COL_PRI(agent, 200);
            //FLASH(agent, 0.325, 0.157, 0.05, 0.75);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold_end"), Hash40::new("haver"), 0, 13, 0.6, 0, 0, 0, 1, true);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold2"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash_reverb"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, false);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash_particle"), Hash40::new("top"), 0, 7, 1, 0, 0, 0, 1, true);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("master_axe_slash_reverb"), -1);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_axe_slash"), false, true);
    }
}

unsafe extern "C" fn game_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 54.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_PULL_AXE);
        let motion_rate = 1.0;
        FT_MOTION_RATE(agent, motion_rate);
        ArticleModule::set_rate(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, 1.0/motion_rate);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_DEGREE_X);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY);
        WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED);
    }
    frame(lua_state, 78.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.9);
    }
    // frame(lua_state, 1.0);
    // if is_excute(agent) {
    //     if VarModule::get_int(agent.battle_object, vars::master::status::AYMR_CHARGE_LEVEL) == 2 {
    //         EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash_particle"), Hash40::new("top"), 0, 4, 0, -10, 0, 0, 3.0, true);
    //         EFFECT_OFF_KIND(agent, Hash40::new("master_axe_slash_air_reverb"), true, true);
    //     }
    //     else{
    //         EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash_particle"), Hash40::new("top"), 0, 4, 0, -10, 0, 0, 1, true);
    //         EFFECT_OFF_KIND(agent, Hash40::new("master_axe_slash_air_reverb"), true, true);
    //     }
    // }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_axe_rock"), Hash40::new("haver"), 0, 15, 1, 0, 0, 0, 1, true);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("master_axe_rock"), -1);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialairnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialn", game_specialn, Priority::Low);
    agent.acmd("game_specialairn", game_specialn, Priority::Low);

    agent.acmd("game_specialsf", game_specialsf, Priority::Low);
    agent.acmd("effect_specialsf", effect_specialsf, Priority::Low);
    agent.acmd("game_specialsfdash", game_specialsfdash, Priority::Low);
    agent.acmd("game_specialairsf", game_specialairsf, Priority::Low);
    agent.acmd("game_specialairsfdash", game_specialairsfdash, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialairhi, Priority::Low);
    agent.acmd("game_specialairhiovertake", game_specialairhiovertake, Priority::Low);
    agent.acmd("effect_specialairhiovertake", effect_specialairhiovertake, Priority::Low);
    
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_specialairlw, Priority::Low);
    agent.acmd("game_speciallwhit", game_speciallwhit, Priority::Low);
    agent.acmd("effect_speciallwhit", effect_speciallwhit, Priority::Low);
    agent.acmd("game_specialairlwhit", game_speciallwhit, Priority::Low);
}