use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut attr = Hash40::new("collision_attr_normal");;
    let mut dmg = 1.0;
    if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
        attr = Hash40::new("collision_attr_fire");
        dmg = 1.05;
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0 * dmg, 361, 103, 0, 28, 3.0, 0.0, 12.5, 2.3, Some(0.0), Some(11.5), Some(4.5), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 16.0 * dmg, 361, 102, 0, 28, 3.3, 3.5, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        boma.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(agent, 8.0, 11.0, 1.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        boma.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("handl"), 13.5, 80, 86, 0, 32, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 13.5, 80, 86, 0, 32, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handl"), 15.8, 80, 95, 0, 30, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 15.8, 80, 95, 0, 30, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE_RANGE(agent, 13.0, 24.0, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        boma.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        boma.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 53.0, 19.0);
}

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 7, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("ken_attack_arc"), Hash40::new("ken_attack_arc"), Hash40::new("top"), -2, 13, -2, 16, -30, 76, 1, true, *EF_FLIP_YZ, 0.5);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 360.0, true, 0.65);
    }
}

unsafe extern "C" fn sound_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ken_smash_h01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_ken_rnd_attack_ll"));
    }
}

unsafe extern "C" fn expression_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        AREA_WIND_2ND_arg10(agent, 0, 0.8, 110, 8, 0.8, 0, 4, 32, 8, 80);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut attr = Hash40::new("collision_attr_normal");;
    let mut dmg = 1.0;
    if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
        attr = Hash40::new("collision_attr_fire");
        dmg = 1.05;
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        agent.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0 * dmg, 35, 47, 0, 50, 3.6, 0.0, 2.5, 12.0, Some(0.0), Some(3.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0 * dmg, 80, 50, 0, 50, 2.5, 0.0, 3.0, 2.5, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0 * dmg, 80, 50, 0, 50, 2.5, 0.0, 3.0, 2.5, Some(0.0), Some(3.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        agent.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("ken_attack_arc2"), Hash40::new("ken_attack_arc2"), Hash40::new("top"), 1, 1.5, 0, 180, 180, -5, 1, true, *EF_FLIP_YZ, 0.5);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);

    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
    agent.acmd("effect_attackhi4", effect_attackhi4, Priority::Low);
    agent.acmd("sound_attackhi4", sound_attackhi4, Priority::Low);
    agent.acmd("expression_attackhi4", expression_attackhi4, Priority::Low);
    
    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);
    agent.acmd("effect_attacklw4", effect_attacklw4, Priority::Low);
}
