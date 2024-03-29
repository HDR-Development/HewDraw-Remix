use super::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    for _ in 0..5 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 235, 100, 15, 20, 4.0, 0.0, 11.0, 4.1, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 235, 100, 15, 20, 4.0, 0.0, 11.0, -6.1, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 115, 100, 15, 20, 4.0, 0.0, 2.0, 4.1, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 115, 100, 15, 20, 4.0, 0.0, 2.0, -6.1, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 87, 0, 60, 2.0, 0.0, 7.0, 4.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 87, 0, 60, 12.0, 0.0, 7.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 84, 96, 0, 43, 4.0, 0.0, 7.2, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 84, 96, 0, 43, 4.5, 0.0, 4.2, 9.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 84, 96, 0, 43, 4.0, 0.0, 7.2, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 84, 96, 0, 43, 4.5, 0.0, 5.0, 10.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 84, 96, 0, 43, 4.0, 0.0, 7.7, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 84, 96, 0, 43, 4.5, 0.0, 7.7, 9.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    } 
}

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 1.2, 0, 1.8, 0, 0, 0, 0.55, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_attack_g"), Hash40::new("mewtwo_pk_attack_g"), Hash40::new("top"), 0, 7.7, 1.8, 15, 0, 30, 1.13, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_pk_hand"), false, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_pk_attack_g"), true, true);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    MotionModule::set_rate(boma, (12.0 - 1.0) / 6.4);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        HIT_NO(agent, 12, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 13, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 12.0);
    MotionModule::set_rate(boma, 1.0);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("s_tail1"), 13.0, 361, 105, 0, 5, 4.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 1, 0, Hash40::new("s_tail5"), 12.0, 361, 95, 0, 5, 4.1, -0.8, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("s_tail7"), 11.0, 361, 95, 0, 5, 3.8, -0.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    frame(lua_state, 17.0);
    MotionModule::set_rate(boma, (37.0 - 17.0) / 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NO(agent, 12, *HIT_STATUS_OFF);
        HIT_NO(agent, 13, *HIT_STATUS_OFF);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        match color {
            0 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0, 13, -6.2, 180, 35, 90, 0.98, true, *EF_FLIP_YZ),
            1 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("top"), 0, 13, -6.2, 180, 35, 90, 0.98, true, *EF_FLIP_YZ),
            2 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("top"), 0, 13, -6.2, 180, 35, 90, 0.98, true, *EF_FLIP_YZ),
            3 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("top"), 0, 13, -6.2, 180, 35, 90, 0.98, true, *EF_FLIP_YZ),
            4 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("top"), 0, 13, -6.2, 180, 35, 90, 0.98, true, *EF_FLIP_YZ),
            5 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("top"), 0, 13, -6.2, 180, 35, 90, 0.98, true, *EF_FLIP_YZ),
            6 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("top"), 0, 13, -6.2, 180, 35, 90, 0.98, true, *EF_FLIP_YZ),
            7 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("top"), 0, 13, -6.2, 180, 35, 90, 0.98, true, *EF_FLIP_YZ),
            _ => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0, 13, -6.2, 180, 35, 90, 0.98, true, *EF_FLIP_YZ),
        };
        LAST_EFFECT_SET_RATE(agent, 0.84);
    }
    frame(lua_state, 17.5);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_01"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_02"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_03"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_04"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_05"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_06"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_07"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_08"), true, true);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    MotionModule::set_rate(boma, (10.0 - 1.0) / 7.0);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 13, *HIT_STATUS_NORMAL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    MotionModule::set_rate(boma, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("s_tail2"), 10.0, 55, 92, 0, 13, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 1, 0, Hash40::new("s_tail5"), 11.0, 55, 93, 0, 13, 4.1, 0.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("s_tail7"), 12.0, 55, 103, 0, 13, 3.7, -0.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    frame(lua_state, 11.0);
    if is_excute (agent) {
        ATTACK(agent, 0, 0, Hash40::new("s_tail2"), 7.0, 65, 92, 0, 13, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 1, 0, Hash40::new("s_tail5"), 8.0, 65, 92, 0, 13, 4.1, 0.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("s_tail7"), 9.0, 65, 92, 0, 13, 3.7, -0.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        HIT_NO(agent, 12, *HIT_STATUS_OFF);
        HIT_NO(agent, 13, *HIT_STATUS_OFF);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        match color {
            0 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -1.5, 8.6, -1.8, 0, 9, 85, 1.08, true, *EF_FLIP_YZ),
            1 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("mewtwo_tail_attack_a_02"), Hash40::new("top"), -1.5, 8.6, -1.8, 0, 9, 85, 1.08, true, *EF_FLIP_YZ),
            2 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("mewtwo_tail_attack_a_03"), Hash40::new("top"), -1.5, 8.6, -1.8, 0, 9, 85, 1.08, true, *EF_FLIP_YZ),
            3 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("mewtwo_tail_attack_a_04"), Hash40::new("top"), -1.5, 8.6, -1.8, 0, 9, 85, 1.08, true, *EF_FLIP_YZ),
            4 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("mewtwo_tail_attack_a_05"), Hash40::new("top"), -1.5, 8.6, -1.8, 0, 9, 85, 1.08, true, *EF_FLIP_YZ),
            5 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("mewtwo_tail_attack_a_06"), Hash40::new("top"), -1.5, 8.6, -1.8, 0, 9, 85, 1.08, true, *EF_FLIP_YZ),
            6 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("mewtwo_tail_attack_a_07"), Hash40::new("top"), -1.5, 8.6, -1.8, 0, 9, 85, 1.08, true, *EF_FLIP_YZ),
            7 => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("mewtwo_tail_attack_a_08"), Hash40::new("top"), -1.5, 8.6, -1.8, 0, 9, 85, 1.08, true, *EF_FLIP_YZ),
            _ => EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -1.5, 8.6, -1.8, 0, 9, 85, 1.08, true, *EF_FLIP_YZ),
        };
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_01"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_02"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_03"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_04"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_05"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_06"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_07"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_tail_attack_a_08"), true, true);
    }
}

unsafe extern "C" fn expression_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 0.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    MotionModule::set_rate(boma, (19.0 - 1.0) / 14.0);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 19.0);
    MotionModule::set_rate(boma, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 270, 62, 0, 12, 5.5, 0.0, -7.7, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 70, 105, 0, 20, 7.5, 0.0, -5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn);

    agent.acmd("game_attackairf", game_attackairf);
    agent.acmd("effect_attackairf", effect_attackairf);

    agent.acmd("game_attackairb", game_attackairb);
	agent.acmd("effect_attackairb", effect_attackairb);

    agent.acmd("game_attackairhi", game_attackairhi);
	agent.acmd("effect_attackairhi", effect_attackairhi);
    agent.acmd("expression_attackairhi", expression_attackairhi);
    
    agent.acmd("game_attackairlw", game_attackairlw);
}
