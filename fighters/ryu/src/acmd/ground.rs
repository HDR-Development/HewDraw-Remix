use super::*;

unsafe extern "C" fn game_attack11w(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 2.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        agent.on_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        agent.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
        let attr = if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            Hash40::new("collision_attr_elec")
        } else {
            Hash40::new("collision_attr_normal")
        };
        ATTACK(agent, 5, 0, Hash40::new("top"), 4.0, 361, 12, 0, 25, 3.0, 0.0, 6.0, 6.5, Some(0.0), Some(6.0), Some(10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 4.0, 361, 12, 0, 25, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        // ground
        ATTACK(agent, 1, 0, Hash40::new("arml"),      4.0, 180, 12, 0, 25, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G , *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("handl"),     4.0, 180, 12, 0, 25, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G , *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        // air
        ATTACK(agent, 3, 0, Hash40::new("arml"),      4.0, 361, 12, 0, 25, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A , *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 4, 0, Hash40::new("handl"),     4.0, 361, 12, 0, 25, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A , *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE_RANGE(agent, 5.0, 8.0, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 12.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        agent.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        agent.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn effect_attack11w(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 11, 12.2, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 360, false, 0.3);
    }
}

unsafe extern "C" fn sound_attack11w(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ryu_swing_punch_m"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_ryu_rnd_attack_m"));
    }
}

unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        agent.on_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        agent.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("legr") , 3.0, 361, 25, 0, 28, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 361, 25, 0, 28, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footr"), 3.0, 361, 25, 0, 28, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE_RANGE(agent, 7.0, 10.0, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        agent.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        agent.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn effect_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 12.0, 4.5, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 360, false, 0.3);
    }
}

unsafe extern "C" fn sound_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ryu_swing_kick_s"));
        PLAY_SE(agent, Hash40::new("vc_ryu_attack02"));
    }
}

unsafe extern "C" fn game_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 11.0, 5.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 53, 65, 0, 60, 4.0, 0.0, 12.0, 7.5, Some(0.0), Some(12.0), Some(4.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 53, 65, 0, 60, 3.0, 0.0, 7.0, 3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE_RANGE(agent, 18.0, 23.0, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 26.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn effect_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("ryu_attack_arc"), Hash40::new("ryu_attack_arc"), Hash40::new("top"), 0, 12.5, 0, 10, 0, 170, 0.82, true, *EF_FLIP_YZ, 0.35);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 13.75, 10, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.5);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -7, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ryu_swing_kick_l"));
        PLAY_SE(agent, Hash40::new("vc_ryu_attack08"));
    }
}

unsafe extern "C" fn game_attack11s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        agent.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
        let attr = if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            Hash40::new("collision_attr_elec")
        } else {
            Hash40::new("collision_attr_normal")
        };
        ATTACK(agent, 0, 0, Hash40::new("bust"),      8.0, 80, 15, 0, 38, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 8.0, 80, 15, 0, 38, 3.5, 1.7, 0.0, 0.0, None, None, None, 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"),      8.0, 80, 15, 0, 38, 3.5, 2.3, 0.0, 0.0, None, None, None, 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 8.0, 80, 15, 0, 38, 3.0, 0.0, 9.0, 8.0, Some(0.0), Some(9.0), Some(11.0), 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 3, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        agent.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
}

unsafe extern "C" fn effect_attack11nears(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("trans"), 0, 14, -3, 0, 0, 0, 0.7, true, *EF_FLIP_YZ, 0.6);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("armr"), 3.0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, false, 0.4);
    }
}

unsafe extern "C" fn game_attack12s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 50, 103, 0, 41, 4.5, 0.0, 15.0, 13.3, Some(0.0), Some(11.0), Some(6.4), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        // HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn effect_attack12s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("ryu_attack_arc"), Hash40::new("ryu_attack_arc"), Hash40::new("top"), -2, 12, 3, 30, -20, -170, 1.1, true, *EF_FLIP_YZ, 0.35);
        LAST_EFFECT_SET_RATE(agent, 1.35);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attack12s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ryu_swing_kick_l"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_ryu_rnd_attack_l"));
    }
}

unsafe extern "C" fn expression_attack12s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.95);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("legl"),  12.0, 50, 65, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 12.0, 50, 65, 0, 80, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneel"), 12.0, 50, 65, 0, 80, 4.2, 5.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"),  8.0, 80, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 8.0, 80, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneel"), 8.0, 80, 60, 0, 80, 3.7, 5.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attack11w", game_attack11w);
    agent.acmd("effect_attack11w", effect_attack11w);
    agent.acmd("sound_attack11w", sound_attack11w);
    
    agent.acmd("game_attack12", game_attack12);
    agent.acmd("effect_attack12", effect_attack12);
    agent.acmd("sound_attack12", sound_attack12);

    agent.acmd("game_attack13", game_attack13);
    agent.acmd("effect_attack13", effect_attack13);
    agent.acmd("sound_attack13", sound_attack13);

    agent.acmd("game_attack11s", game_attack11s);
    agent.acmd("game_attack11nears", game_attack11s);
    agent.acmd("effect_attack11s", effect_attack11nears);
    agent.acmd("effect_attack11nears", effect_attack11nears);

    agent.acmd("game_attack12s", game_attack12s);
    agent.acmd("effect_attack12s", effect_attack12s);
    agent.acmd("sound_attack12s", sound_attack12s);
    agent.acmd("expression_attack12s", expression_attack12s);

    agent.acmd("game_attackdash", game_attackdash);
}
