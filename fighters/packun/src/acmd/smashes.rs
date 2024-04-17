use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        let kbg = if stance.label == 0 { 110 } else if stance.label == 1 { 140 } else { 100 };
        let hitlag = if stance.label != 2 { 1.2 } else { 1.5 };
        let sound = if stance.label != 2 { *COLLISION_SOUND_ATTR_PUNCH } else { *COLLISION_SOUND_ATTR_HEAVY };
        ATTACK(agent, 0, 0, Hash40::new("virtualhit2"), 14.0 * stance.damage_head, 45, 100, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("virtualhit3"), 14.0 * stance.damage_head, 45, 100, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("mouth"), 15.0 * stance.damage_head, 45, kbg, 0, 25, 7.0, 2.5, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_HEAD);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
        if stance.label != 2 {
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, 11, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.4);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("packun_smash_s_arc_l"), Hash40::new("top"), 0, 13, 3.5, 40, 60, 0, 1.6, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("packun_smash_s_arc_r"), Hash40::new("top"), 0, 13, 3.5, 40, -60, 0, 1.6, true);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_attacks42(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 22.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("virtualhit2"), 20.0, 45, 100, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("virtualhit3"), 20.0, 45, 100, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("mouth"), 25.0, 45, 100, 0, 25, 7.0, 2.5, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("neck6"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("neck8"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 60, 75, 0, 40, 6.0, 0.0, 5.0, 5.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("neck6"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("neck8"), *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_attacks42(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 18, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("packun_smash_s_arc_l"), Hash40::new("top"), 0, 14.5, 8, 50, 50, 90, 1.3, true);
            LAST_EFFECT_SET_COLOR(agent, 1.3, 0.33, 0.11);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("packun_smash_s_arc_r"), Hash40::new("top"), 0, 14.5, 8, 40, -50, -90, 1.3, true);
            LAST_EFFECT_SET_COLOR(agent, 1.3, 0.33, 0.11);
        }
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 16, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 16, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attacks42(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 24.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_smash_s01"));
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_down_soil_l"));
    }
}

unsafe extern "C" fn expression_attacks42(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 23.0);
    smash::app::sv_animcmd::execute(lua_state, 23.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(lua_state, 79.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 14);
    }
}

unsafe extern "C" fn effect_attacks4charge2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
    }
    for _ in 0..999 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("head"), -5, 3, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, true);
        }
        wait(lua_state, 5.0);
    }
}

unsafe extern "C" fn sound_attacks4charge2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_smash_start"));
    }
}

unsafe extern "C" fn expression_attacks4charge2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        physics!(agent, *MA_MSC_CMD_PHYSICS_START_CHARGE, -1, -1, -1, -1, 0.1, -1, Hash40::new("invalid"));
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 61.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    if stance.label == 2 {
        FT_DESIRED_RATE(agent, 8.0, 11.0);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        if stance.label == 2 {
            VarModule::on_flag(boma.object(), vars::packun::status::BITE_START);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::packun::status::BITE_START);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0 * stance.damage_bite, 75, 0, 0, 75, 5.5, 0.0, 12.0, -4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0 * stance.damage_bite, 105, 0, 0, 75, 5.5, 0.0, 12.0, 4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if stance.label == 1 {
            ATTACK(agent, 0, 0, Hash40::new("mouth"), 12.5 * stance.damage_bite, 90, 86, 0, 90, 9.0, 2.0, 0.0, 0.0, Some(3.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            AttackModule::set_poison_param(boma, 0, 121, 30, 3.5, false);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("mouth"), 12.5 * stance.damage_bite, 90, 86, 0, 90, 9.0, 2.0, 0.0, 0.0, Some(3.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            if VarModule::is_flag(boma.object(), vars::packun::status::BURST) {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 14.0 * stance.damage_bite, 90, 86, 0, 90, 9.0, 2.0, 0.0, 0.0, Some(3.5), Some(0.0), Some(0.0), 0.35, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                ATTACK(agent, 1, 0, Hash40::new("mouth"), 8.5, 90, 86, 0, 90, 9.0, 4.0, 0.0, 0.0, Some(3.5), Some(0.0), Some(0.0), 0.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BITE);
            }
        }
        
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::off_flag(boma.object(), vars::packun::status::BURST);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 23, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
        if stance.label == 1 {
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 25.5, 0, 0, 0, 0, 0.95, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 25.5, 0, 0, 0, 0, 0.85, true);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, 0, -90, 0, 0, 1.1, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("packun_bite_line"), Hash40::new("mouth"), 4, 0, 0, 0, 45, 0, 0.7, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("packun_bite_line2"), Hash40::new("top"), 0, 27, 0, 90, 0, 90, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("packun_smash_hi_bite"), Hash40::new("mouth"), 1.5, 0, 0, 0, 0, -90, 1, true);
        if VarModule::is_flag(boma.object(), vars::packun::status::BURST) {
            EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("mouth"), 0, 0, 0, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.01, 0.6);
		    LAST_EFFECT_SET_RATE(agent, 0.7);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_smash_hi_bite"), -1);
    }
}

unsafe extern "C" fn sound_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_smash_h01"));
    }
    wait(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_smash_h02"));
        if VarModule::is_flag(boma.object(), vars::packun::status::BURST) {
            PLAY_SE(agent, Hash40::new("se_common_bomb_s"));
        }
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_smash_h03"));
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    let angle1 = if stance.label == 0 { 175 } else { 32 };
    let angle2 = if stance.label == 0 { 94 } else { 30 };
    let dmg1 = if stance.label == 0 { 7.0 } else { 14.0 };
    let dmg2 = if stance.label == 0 { 6.0 } else { 12.0 };
    let fkb1 = if stance.label == 0 { 150 } else { 0 };
    let bkb1 = if stance.label == 0 { 0 } else { 25 }; 
    let kbg2 = if stance.label == 0 { 70 } else { 99 };
    let bkb2 = if stance.label == 0 { 75 } else { 25 };
    let element = if stance.label == 0 { Hash40::new("collision_attr_fire") } else { Hash40::new("collision_attr_normal") };
    let lvl1 = if stance.label == 0 { *ATTACK_SOUND_LEVEL_M } else { *ATTACK_SOUND_LEVEL_L };
    if stance.label == 2 {
        FT_DESIRED_RATE(agent, 6.0, 9.0);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if stance.label == 0 {
        FT_DESIRED_RATE(agent, 7.0, 3.0);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), dmg1 * stance.damage_other, angle1, 99, fkb1, bkb1, 5.0, 0.0, 4.0, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, element, lvl1, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), dmg1 * stance.damage_other, angle1, 99, fkb1, bkb1, 4.0, 0.0, 3.5, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, element, lvl1, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        if stance.label == 0 {
            VarModule::on_flag(boma.object(), vars::packun::status::FLAME_ACTIVE);
        }
        else {
            AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), dmg2 * stance.damage_other, angle2, kbg2, 0, bkb2, 5.0, 0.0, 4.0, -13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, element, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), dmg2 * stance.damage_other, angle2, kbg2, 0, bkb2, 4.0, 0.0, 3.6, -8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, element, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        if stance.label != 0 {
            AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::off_flag(boma.object(), vars::packun::status::FLAME_ACTIVE);
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 5, 4, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 9.5);
    if is_excute(agent) {
        if stance.label == 0 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("mouth"), 6.0, 1.0, 0, 0, 0, 0, 0.6, true);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if stance.label == 0 {
            EFFECT_FOLLOW(agent, Hash40::new("packun_atk_air_b_fire"), Hash40::new("potc"), -8, 0, 0, 0, 0, 0, 1.2, true);
        }
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_smash_lw_arc"), Hash40::new("packun_smash_lw_arc"), Hash40::new("top"), 0, 5, 3, -180, 150, 7, 1.4, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if stance.label == 0 {
            EFFECT_FOLLOW(agent, Hash40::new("packun_atk_air_b_fire"), Hash40::new("potc"), -8, 0, 0, 0, 0, 0, 1.2, true);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("packun_smash_lw_arc"), true, true);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_smash_lw_arc"), Hash40::new("packun_smash_lw_arc"), Hash40::new("top"), 0, 5, -4, -180, 20, 5, 1.4, true, *EF_FLIP_YZ);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4);
    agent.acmd("effect_attacks4", effect_attacks4);

    agent.acmd("game_attacks42", game_attacks42);
    agent.acmd("effect_attacks42", effect_attacks42);
    agent.acmd("sound_attacks42", sound_attacks42);
    agent.acmd("expression_attacks42", expression_attacks42);

    agent.acmd("effect_attacks4charge2", effect_attacks4charge2);
    agent.acmd("sound_attacks4charge2", sound_attacks4charge2);
    agent.acmd("expression_attacks4charge2", expression_attacks4charge2); 

    agent.acmd("game_attackhi4", game_attackhi4);
    agent.acmd("effect_attackhi4", effect_attackhi4);
    agent.acmd("sound_attackhi4", sound_attackhi4);

    agent.acmd("game_attacklw4", game_attacklw4);
    agent.acmd("effect_attacklw4", effect_attacklw4);
}