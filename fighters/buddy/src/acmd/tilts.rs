use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER,  Hash40::new("attack_s3_s"), false, 0.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 45, 67, 0, 46, 3.2, 0.0, 7.2, 13.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 45, 71, 0, 46, 2.6, 0.0, 7.2, 18.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 45, 67, 0, 46, 3.6, 0.0, 7.2, 7.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
    AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
    ArticleModule::remove_exist(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_attacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER,  Hash40::new("attack_s3_hi"), false, 0.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 50, 67, 0, 46, 3.2, 0.0, 14.2, 12.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 50, 71, 0, 46, 2.6, 0.0, 16.2, 17.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 50, 67, 0, 46, 3.6, 0.0, 12.4, 6.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_attacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER,  Hash40::new("attack_s3_lw"), false, 0.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 40, 67, 0, 46, 3.2, 0.0, 2.8, 12.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 40, 71, 0, 46, 2.6, 0.0, 1.2, 17.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 40, 67, 0, 46, 3.6, 0.0, 4.2, 6.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
    AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
    ArticleModule::remove_exist(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        //Banjo//
        ATTACK(agent, 0, 0, Hash40::new("handl"), 11.0, 70, 81, 0, 74, 3.6, 1.2, 0.0, -0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 11.0, 70, 81, 0, 74, 3.6, 1.2, 0.5, 1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        //Kazooie//
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 82, 58, 0, 71, 5.3, 0.0, 6.25, -7.5, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BUDDY_WING, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        //Kazooie//
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 82, 58, 0, 71, 5.3, 0.0, 16.0, -4.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BUDDY_WING, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        //Kazooie//
        ATTACK(agent, 2, 0, Hash40::new("k_wingr4"), 6.5, 82, 50, 0, 73, 4.8, 0.0, 0.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BUDDY_WING, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        //Banjo//
        ATTACK(agent, 0, 0, Hash40::new("handl"), 8.0, 105, 74, 0, 74, 3.6, 1.2, 0.0, -0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 8.0, 105, 74, 0, 74, 3.6, 1.2, 0.0, -0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    //Remove kazooie hitbox
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear(boma,2,false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("buddy_attack100"), false, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        //Banjo
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 4, 11.5, 6, 0, 4, 115, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent,1.4);
        //Kazooie
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -0.25, 14, 1.5, 180, 4, 76, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent,1.4);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
            LAST_EFFECT_SET_COLOR(agent, 0.8, 0.3, 0);
        }
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
            LAST_EFFECT_SET_COLOR(agent, 0.7, 0.35, 0.1);
        }
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
            LAST_EFFECT_SET_COLOR(agent, 0.8, 0.2, 0.5);
        }
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.2, 0.35);
        }
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
            LAST_EFFECT_SET_COLOR(agent, 0.35, 0.35, 0.35);
        }
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
            LAST_EFFECT_SET_COLOR(agent, 0.4, 0.3, 0.1);
        }
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
            LAST_EFFECT_SET_COLOR(agent, 1, 0.2, 0.2);
        }
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
            LAST_EFFECT_SET_COLOR(agent, 0.1, 0.05, 0.1);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_all"), 3, 0, 0, 0, 0, 0, 0.9, true);
    }
}

unsafe extern "C" fn sound_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    //frame(lua_state, 2.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_buddy_attack100_01"));
        PLAY_SE(agent, Hash40::new("se_buddy_attack100_02"));
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attack100_03"));
    }
}

unsafe extern "C" fn expression_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let slide_speed = 0.9;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0 / 1.571); // originally was MotionModule::set_rate, but let's use the lua macros for consistency
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 12.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 13, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 14, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 15, *HIT_STATUS_NORMAL);
        KineticModule::add_speed(boma, &Vector3f::new(slide_speed, 0.0, 0.0));
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 38, 68, 0, 59, 3.75, 0.0, 4.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 38, 68, 0, 59, 3.75, 0.0, 4.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 38, 75, 0, 59, 3.9, 0.0, 4.2, 12.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 90, 61, 0, 73, 3.25, 0.0, 3.75, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 90, 61, 0, 73, 3.25, 0.0, 3.75, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 90, 67, 0, 73, 3.5, 0.0, 3.9, 10.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 17.5);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_OFF);
        HIT_NO(agent, 13, *HIT_STATUS_OFF);
        HIT_NO(agent, 14, *HIT_STATUS_OFF);
        HIT_NO(agent, 15, *HIT_STATUS_OFF);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 1.0);
    wait(lua_state, 1.0);
    for _ in 0..20 {
        if is_excute(agent) {
            KineticModule::add_speed(boma, &Vector3f::new((-1.0*slide_speed)/20.0, 0.0, 0.0));
        }
        wait(lua_state, 1.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3);
    agent.acmd("game_attacks3hi", game_attacks3hi);
    agent.acmd("game_attacks3lw", game_attacks3lw);

    agent.acmd("game_attackhi3", game_attackhi3);
    agent.acmd("effect_attackhi3", effect_attackhi3);
    agent.acmd("sound_attackhi3", sound_attackhi3);
    agent.acmd("expression_attackhi3", expression_attackhi3);

    agent.acmd("game_attacklw3", game_attacklw3);
}
