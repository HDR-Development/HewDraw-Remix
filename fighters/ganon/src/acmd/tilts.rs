use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 6.0/(8.0 - 1.0));
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0/(10.1 - 10.0));
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 13.0, 22, 81, 0, 32, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 13.0, 22, 81, 0, 32, 5.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneel"), 14.0, 22, 81, 0, 32, 5.5, 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 10.1);
    FT_MOTION_RATE(agent, 1.0/(12.0 - 10.1));
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 20.0/(35.0 - 14.0));
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn game_attacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 15.0, 22, 72, 0, 25, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 15.0, 22, 72, 0, 25, 5.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneel"), 16.0, 22, 72, 0, 25, 5.5, 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 20.0/(35.0 - 14.0));
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn game_attacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 4.0/(8.0 - 1.0));
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0/(10.1 - 10.0));
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 9.0, 22, 95, 0, 35, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 9.0, 22, 95, 0, 35, 5.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneel"), 10.0, 22, 95, 0, 35, 5.5, 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 10.1);
    FT_MOTION_RATE(agent, 1.0/(12.0 - 10.1));
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 20.0/(35.0 - 14.0));
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_attacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 5, -11, -35, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 22.0, 13.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 0.8);
    }
}

unsafe extern "C" fn effect_attacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 16.0, -10, 27, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 2.0, 17.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 0.8);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, 0);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 14.0, 78, 75, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 14.0, 78, 75, 0, 50, 4.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 14.0, 78, 75, 0, 50, 4.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 14.0, 78, 75, 0, 50, 4.5, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("armr"), 14.0, 78, 75, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 3, false);
        AttackModule::clear(boma, 4, false);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 78, 75, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 78, 75, 0, 50, 4.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 78, 75, 0, 50, 4.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -10, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -10, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -8, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -8, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -6, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -6, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -4, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -4, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        FOOT_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -2, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -2, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, 0, 0, -80, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, false);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appeal_l01"));
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_attack06"));
        PLAY_SE(agent, Hash40::new("se_ganon_swing_m"));
    }
}

unsafe extern "C" fn expression_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_TOP);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_LR, 3);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    FT_MOTION_RATE(agent, 8.0 / 10.0);
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 40, 10, 40, 3.0, 0.0, 3.0, 2.0, Some(0.0), Some(3.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    // frame(lua_state, 30.0);
    // if is_excute(agent) {
    //     WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    // }
}

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
}

unsafe extern "C" fn expression_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
    }
}

unsafe extern "C" fn game_attacklw32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 65, 100, 0, 85, 4.0, 0.0, 4.0, -5.0, Some(0.0), Some(4.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attacklw32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attacklw32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
}

unsafe extern "C" fn expression_attacklw32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3);
    agent.acmd("game_attacks3hi", game_attacks3hi);
    agent.acmd("game_attacks3lw", game_attacks3lw);
    agent.acmd("effect_attacks3hi", effect_attacks3hi);
    agent.acmd("effect_attacks3lw", effect_attacks3lw);

    agent.acmd("game_attackhi3", game_attackhi3);
    agent.acmd("effect_attackhi3", effect_attackhi3);
    agent.acmd("sound_attackhi3", sound_attackhi3);
    agent.acmd("expression_attackhi3", expression_attackhi3);

    agent.acmd("game_attacklw3", game_attacklw3);
    agent.acmd("effect_attacklw3", effect_attacklw3);
    agent.acmd("sound_attacklw3", sound_attacklw3);
    agent.acmd("expression_attacklw3", expression_attacklw3);

    agent.acmd("game_attacklw32", game_attacklw32);
    agent.acmd("effect_attacklw32", effect_attacklw32);
    agent.acmd("sound_attacklw32", sound_attacklw32);
    agent.acmd("expression_attacklw32", expression_attacklw32);
}
