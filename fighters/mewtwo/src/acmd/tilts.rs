use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 4.375);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 13, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("s_tail2"), 7.0, 361, 100, 0, 15, 4.4, 0.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 1, 0, Hash40::new("s_tail4"), 9.0, 361, 100, 0, 15, 4.0, 1.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("s_tail6"), 11.0, 361, 100, 0, 15, 3.7, 1.9, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_OFF);
        HIT_NO(agent, 13, *HIT_STATUS_OFF);
    }
}

unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        match color {
            0 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 2, 8.0, 6.2, 0, -85, -15.3, 1, true, *EF_FLIP_YZ),
            1 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("top"), 2, 8.0, 6.2, 0, -85, -15.3, 1, true, *EF_FLIP_YZ),
            2 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("top"), 2, 8.0, 6.2, 0, -85, -15.3, 1, true, *EF_FLIP_YZ),
            3 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("top"), 2, 8.0, 6.2, 0, -85, -15.3, 1, true, *EF_FLIP_YZ),
            4 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("top"), 2, 8.0, 6.2, 0, -85, -15.3, 1, true, *EF_FLIP_YZ),
            5 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("top"), 2, 8.0, 6.2, 0, -85, -15.3, 1, true, *EF_FLIP_YZ),
            6 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("top"), 2, 8.0, 6.2, 0, -85, -15.3, 1, true, *EF_FLIP_YZ),
            7 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("top"), 2, 8.0, 6.2, 0, -85, -15.3, 1, true, *EF_FLIP_YZ),
            _ => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 2, 8.0, 6.2, 0, -85, -15.3, 1, true, *EF_FLIP_YZ),
        };
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("s_tail1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z_MINUS), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

unsafe extern "C" fn game_attacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 4.375);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 13, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("s_tail2"), 7.0, 361, 100, 0, 15, 4.4, 0.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 1, 0, Hash40::new("s_tail4"), 9.0, 361, 100, 0, 15, 4.0, 1.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("s_tail6"), 11.0, 361, 100, 0, 15, 3.7, 1.9, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_OFF);
        HIT_NO(agent, 13, *HIT_STATUS_OFF);
    }
}

unsafe extern "C" fn effect_attacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        match color {
            0 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 2, 11.0, 6.4, -34, -72, 49.6, 1, true, *EF_FLIP_YZ),
            1 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("top"), 2, 11.0, 6.4, -34, -72, 49.6, 1, true, *EF_FLIP_YZ),
            2 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("top"), 2, 11.0, 6.4, -34, -72, 49.6, 1, true, *EF_FLIP_YZ),
            3 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("top"), 2, 11.0, 6.4, -34, -72, 49.6, 1, true, *EF_FLIP_YZ),
            4 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("top"), 2, 11.0, 6.4, -34, -72, 49.6, 1, true, *EF_FLIP_YZ),
            5 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("top"), 2, 11.0, 6.4, -34, -72, 49.6, 1, true, *EF_FLIP_YZ),
            6 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("top"), 2, 11.0, 6.4, -34, -72, 49.6, 1, true, *EF_FLIP_YZ),
            7 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("top"), 2, 11.0, 6.4, -34, -72, 49.6, 1, true, *EF_FLIP_YZ),
            _ => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 2, 11.0, 6.4, -34, -72, 49.6, 1, true, *EF_FLIP_YZ),
        };
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_attacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("s_tail1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z_MINUS), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

unsafe extern "C" fn game_attacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 4.375);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 13, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("s_tail2"), 7.0, 361, 100, 0, 15, 4.4, 0.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 1, 0, Hash40::new("s_tail4"), 9.0, 361, 100, 0, 15, 4.0, 1.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("s_tail6"), 11.0, 361, 100, 0, 15, 3.7, 1.9, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_OFF);
        HIT_NO(agent, 13, *HIT_STATUS_OFF);
    }
}

unsafe extern "C" fn effect_attacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        match color {
            0 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 2, 5, 6.5, 82, -85, -95.5, 1, true, *EF_FLIP_YZ),
            1 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("top"), 2, 5, 6.5, 82, -85, -95.5, 1, true, *EF_FLIP_YZ),
            2 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("top"), 2, 5, 6.5, 82, -85, -95.5, 1, true, *EF_FLIP_YZ),
            3 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("top"), 2, 5, 6.5, 82, -85, -95.5, 1, true, *EF_FLIP_YZ),
            4 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("top"), 2, 5, 6.5, 82, -85, -95.5, 1, true, *EF_FLIP_YZ),
            5 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("top"), 2, 5, 6.5, 82, -85, -95.5, 1, true, *EF_FLIP_YZ),
            6 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("top"), 2, 5, 6.5, 82, -85, -95.5, 1, true, *EF_FLIP_YZ),
            7 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("top"), 2, 5, 6.5, 82, -85, -95.5, 1, true, *EF_FLIP_YZ),
            _ => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 2, 5, 6.5, 82, -85, -95.5, 1, true, *EF_FLIP_YZ),
        };
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_attacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("s_tail1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z_MINUS), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.714);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 13, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("s_tail1"), 9.0, 80, 81, 0, 63, 5.0, 0.5, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 1, 0, Hash40::new("s_tail3"), 8.0, 85, 81, 0, 54, 4.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("s_tail5"), 6.0, 65, 71, 0, 45, 3.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 3, 0, Hash40::new("s_tail7"), 5.0, 65, 71, 0, 45, 3.5, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("s_tail1"), 9.0, 65, 81, 0, 45, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 1, 0, Hash40::new("s_tail3"), 8.0, 65, 81, 0, 45, 4.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 3, 0, Hash40::new("s_tail7"), 5.0, 65, 71, 0, 45, 3.5, 0.5, 0.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_OFF);
        HIT_NO(agent, 13, *HIT_STATUS_OFF);
    }
}

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        match color {
            0 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0, 14, 1.5, 0, 5, 90, 1, true, *EF_FLIP_YZ),
            1 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("top"), 0, 14, 1.5, 0, 5, 90, 1, true, *EF_FLIP_YZ),
            2 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("top"), 0, 14, 1.5, 0, 5, 90, 1, true, *EF_FLIP_YZ),
            3 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("top"), 0, 14, 1.5, 0, 5, 90, 1, true, *EF_FLIP_YZ),
            4 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("top"), 0, 14, 1.5, 0, 5, 90, 1, true, *EF_FLIP_YZ),
            5 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("top"), 0, 14, 1.5, 0, 5, 90, 1, true, *EF_FLIP_YZ),
            6 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("top"), 0, 14, 1.5, 0, 5, 90, 1, true, *EF_FLIP_YZ),
            7 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("top"), 0, 14, 1.5, 0, 5, 90, 1, true, *EF_FLIP_YZ),
            _ => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0, 14, 1.5, 0, 5, 90, 1, true, *EF_FLIP_YZ),
        };
        LAST_EFFECT_SET_RATE(agent, 0.9);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 13, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("s_tail2"), 9.0, 80, 84, 0, 60, 4.3, -0.2, -0.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 1, 0, Hash40::new("s_tail4"), 8.0, 80, 84, 0, 60, 4.0, 1.2, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("s_tail7"), 5.0, 90, 84, 0, 60, 3.8, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_OFF);
        HIT_NO(agent, 13, *HIT_STATUS_OFF);
    }
}

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        match color {
            0 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0, 5, 4.5, 0, -70, 190, 1.0, true, *EF_FLIP_YZ),
            1 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("top"), 0, 5, 4.5, 0, -70, 190, 1.0, true, *EF_FLIP_YZ),
            2 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("top"), 0, 5, 4.5, 0, -70, 190, 1.0, true, *EF_FLIP_YZ),
            3 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("top"), 0, 5, 4.5, 0, -70, 190, 1.0, true, *EF_FLIP_YZ),
            4 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("top"), 0, 5, 4.5, 0, -70, 190, 1.0, true, *EF_FLIP_YZ),
            5 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("top"), 0, 5, 4.5, 0, -70, 190, 1.0, true, *EF_FLIP_YZ),
            6 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("top"), 0, 5, 4.5, 0, -70, 190, 1.0, true, *EF_FLIP_YZ),
            7 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("top"), 0, 5, 4.5, 0, -70, 190, 1.0, true, *EF_FLIP_YZ),
            _ => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0, 5, 4.5, 0, -70, 190, 1.0, true, *EF_FLIP_YZ),
        };
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0.5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}

unsafe extern "C" fn expression_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}


pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3);
	agent.acmd("effect_attacks3", effect_attacks3);
    agent.acmd("expression_attacks3", expression_attacks3);
    agent.acmd("game_attacks3hi", game_attacks3hi);
	agent.acmd("effect_attacks3hi", effect_attacks3hi);
    agent.acmd("expression_attacks3hi", expression_attacks3hi);
    agent.acmd("game_attacks3lw", game_attacks3lw);
	agent.acmd("effect_attacks3lw", effect_attacks3lw);
    agent.acmd("expression_attacks3lw", expression_attacks3lw);

    agent.acmd("game_attackhi3", game_attackhi3);
    //agent.acmd("effect_attackhi3", effect_attackhi3);

    agent.acmd("game_attacklw3", game_attacklw3);
    //agent.acmd("effect_attacklw3", effect_attacklw3);
    agent.acmd("expression_attacklw3", expression_attacklw3);
}
