use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.5 * stance.damage_bite, 366, 20, 0, 20, 6.3, 0.0, 7.5, 12.0, Some(0.0), Some(10.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        ATTACK(agent, 1, 0, Hash40::new("virtualhit3"), 5.5 * stance.damage_bite, 361, 20, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        ATTACK(agent, 2, 0, Hash40::new("virtualhit2"), 5.5 * stance.damage_bite, 361, 20, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        AttackModule::set_add_reaction_frame(boma, 0, 11.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 11.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 11.0, false);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

// Piranha
unsafe extern "C" fn game_attacks3a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("virtualhit2"), 6.0, 361, 102, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("mouth"), 11.0, 361, 102, 0, 40, 6.3, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("virtualhit2"), 6.0, 50, 120, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("mouth"), 6.0, 50, 120, 0, 40, 6.3, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_attacks3a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1, 0, 0, 0, 180, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("mouth"), 6.0, -1.0, 0, 0, 0, 0, 0.6, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let color_hash = match color {
            0 => Hash40::new("packun_atk_hi_trace_01"),
            1 => Hash40::new("packun_atk_hi_trace_02"),
            2 => Hash40::new("packun_atk_hi_trace_03"),
            3 => Hash40::new("packun_atk_hi_trace_04"),
            4 => Hash40::new("packun_atk_hi_trace_05"),
            5 => Hash40::new("packun_atk_hi_trace_06"),
            6 => Hash40::new("packun_atk_hi_trace_07"),
            7 => Hash40::new("packun_atk_hi_trace_08"),
            _ => Hash40::new("packun_atk_hi_trace_01")
        };
        EFFECT_FOLLOW(agent, color_hash, Hash40::new("top"), 0, 8, -7, -180, -210, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 8, -7, -180, -210, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("packun_atk_air_b_fire"), Hash40::new("mouth"), 9, 0, 0, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("packun_atk_air_b_fire"), Hash40::new("mouth"), 9, 0, 0, 0, 0, 0, 1.2, true);
    }
}

unsafe extern "C" fn sound_attacks3a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_attackhard_h01"));
        PLAY_SE(agent, Hash40::new("se_common_fire_m"));
    }
}

unsafe extern "C" fn expression_attacks3a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}

// Putrid
unsafe extern "C" fn game_attacks32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0 * stance.damage_bite, 361, 125, 0, 40, 6.5, 0.0, 7.5, 15.0, Some(0.0), Some(10.5), Some(15.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        ATTACK(agent, 1, 0, Hash40::new("virtualhit3"), 6.0 * stance.damage_bite, 361, 125, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        ATTACK(agent, 2, 0, Hash40::new("virtualhit2"), 6.0 * stance.damage_bite, 361, 125, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        ATTACK(agent, 3, 1, Hash40::new("top"), 0.0, 0, 0, 0, 0, 6.5, 0.0, 7.5, 15.0, Some(0.0), Some(10.5), Some(15.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BITE);
        ATTACK(agent, 4, 1, Hash40::new("virtualhit3"), 0.0, 0, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BITE);
        ATTACK(agent, 5, 1, Hash40::new("virtualhit2"), 0.0, 0, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BITE);
        AttackModule::set_poison_param(boma, 0, 241, 60, 1.8, false);
        AttackModule::set_poison_param(boma, 1, 241, 60, 1.8, false);
        AttackModule::set_poison_param(boma, 2, 241, 60, 1.8, false);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 12.0, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attacks32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 11, 18, 0, 0, 0, 0.95, true);
        LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 11, 18, 0, 0, 0, 0.85, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("packun_bite_line"), Hash40::new("packun_bite_line"), Hash40::new("top"), -5, 11, 19, 0, -130, 35, 1, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("top"), -12, 9, 20, 10, 50, 10, 0.8, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("top"), -9, 11, 18, 0, -120, 20, 1, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn sound_attacks32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_attackhard_s03"));
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_attackhard_s04"));
    }
}

unsafe extern "C" fn expression_attacks32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    let atk_frame = if stance.label == 2 { 6.0 } else { 5.0 };
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, atk_frame);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

// Prickly
unsafe extern "C" fn game_attacks3ss(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 10.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0 * stance.damage_head, 361, 105, 0, 40, 6.5, 0.0, 8.25, 15.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("neck5"), 10.0 * stance.damage_head, 361, 105, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
        ATTACK(agent, 2, 0, Hash40::new("neck3"), 10.0 * stance.damage_head, 361, 105, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE_RANGE(agent, 15.0, 26.0, 15.0);
    frame(lua_state, 26.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attacks3ss(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11, -3, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, 0.5);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 15.0, 8.0, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}

unsafe extern "C" fn sound_attacks3ss(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_swing_l01"));
        PLAY_SE(agent, Hash40::new("se_packun_attackair_h01"));
    }
}

unsafe extern "C" fn expression_attacks3ss(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    let atk_frame = if stance.label == STANCE_PRICKLY { 6.0 } else { 5.0 };
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, atk_frame);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    if stance.label == STANCE_PRICKLY {
        FT_MOTION_RATE(agent, (9.0/6.0));
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let collision_sfx = if stance.label != STANCE_PRICKLY { *COLLISION_SOUND_ATTR_PUNCH } else { *COLLISION_SOUND_ATTR_HEAVY };
        ATTACK(agent, 0, 0, Hash40::new("virtualhit1"), 7.0 * stance.damage_head, 80, 100, 0, 55, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, collision_sfx, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("virtualhit2"), 7.0 * stance.damage_head, 80, 100, 0, 55, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, collision_sfx, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("mouth"), 9.0 * stance.damage_head, 85, 100, 0, 55, 6.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, collision_sfx, *ATTACK_REGION_HEAD);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn expression_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm_l"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 1.0);
    if stance.label != STANCE_PRICKLY {
        FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 4.0);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 8.75, 2.0);
    frame(lua_state, 8.75);
    FT_MOTION_RATE_RANGE(agent, 8.75, 12.0, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0 * stance.damage_other, 60, 67, 0, 45, 4.5, 0.0, 4.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0 * stance.damage_other, 60, 67, 0, 45, 4.5, 0.0, 3.4, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0 * stance.damage_other, 80, 54, 0, 60, 4.5, 0.0, 2.8, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE_RANGE(agent, 12.0, 20.0, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("packun_atk_lw_arc"), Hash40::new("top"), 0.7, 4.6, 4.6, -170, -20, -12, 1, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("packun_atk_lw_arc"), Hash40::new("top"), -0.7, 4.6, 4.6, -10, 200, 12, 1, true);
        }
    }
}

unsafe extern "C" fn expression_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 14);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3, Priority::Low);
    agent.acmd("game_attacks3a", game_attacks3a, Priority::Low);
    agent.acmd("effect_attacks3a", effect_attacks3a, Priority::Low);
    agent.acmd("sound_attacks3a", sound_attacks3a, Priority::Low);
    agent.acmd("expression_attacks3a", expression_attacks3a, Priority::Low);

    agent.acmd("game_attacks32", game_attacks32, Priority::Low);
    agent.acmd("effect_attacks32", effect_attacks32, Priority::Low);
    agent.acmd("sound_attacks32", sound_attacks32, Priority::Low);
    agent.acmd("expression_attacks32", expression_attacks32, Priority::Low);

    agent.acmd("game_attacks3ss", game_attacks3ss, Priority::Low);
    agent.acmd("effect_attacks3ss", effect_attacks3ss, Priority::Low);
    agent.acmd("sound_attacks3ss", sound_attacks3ss, Priority::Low);
    agent.acmd("expression_attacks3ss", expression_attacks3ss, Priority::Low);

    agent.acmd("game_attackhi3", game_attackhi3, Priority::Low);
    agent.acmd("expression_attackhi3", expression_attackhi3, Priority::Low);

    agent.acmd("game_attacklw3", game_attacklw3, Priority::Low);
    agent.acmd("effect_attacklw3", effect_attacklw3, Priority::Low);
    agent.acmd("expression_attacklw3", expression_attacklw3, Priority::Low);
}