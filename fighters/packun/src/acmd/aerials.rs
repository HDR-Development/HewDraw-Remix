use super::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 1.0);
    if stance.label == STANCE_PRICKLY {
        FT_MOTION_RATE_RANGE(agent, 1.0, 5.0, 7.0);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        let (bkb, kbg) = if stance.label == STANCE_PRICKLY { (15, 90) } else { (20, 45) };
        ATTACK(agent, 0, 0, Hash40::new("arml"), 2.0 * stance.damage_other, 365, kbg, 0, bkb, 4.5, 1.8, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 2.0 * stance.damage_other, 365, kbg, 0, bkb, 4.5, 1.8, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 3.0 * stance.damage_other, 51, 141, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 3.0 * stance.damage_other, 51, 141, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 3.0);
    if stance.label != STANCE_PRICKLY {
        FT_DESIRED_RATE(agent, 6.0, 4.0);
    }
    else {
        FT_DESIRED_RATE(agent, 6.0, 7.0);
    }
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 10.0);
    if is_excute(agent) {
        let bkb = if stance.label == STANCE_PUTRID { 10 } else { 0 };
        ATTACK(agent, 0, 0, Hash40::new("potc"), 9.0 * stance.damage_other, 361, 72, 0, 40 + bkb, 4.5, 3.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("potc"), 11.0 * stance.damage_other, 361, 72, 0, 40 + bkb, 7.0, -3.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        if stance.label == STANCE_PRICKLY {
            HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_XLU);
            HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_XLU);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn expression_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    if stance.label == STANCE_PIRANHA {
        // Piranha
        frame(lua_state, 5.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 14.0);
        FT_MOTION_RATE(agent, 1.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 15.0 * stance.damage_other, 50, 108, 0, 25, 9.0, 0.0, 4.0, -10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 1, 0, Hash40::new("top"), 11.0 * stance.damage_other, 55, 100, 0, 25, 4.5, 0.0, 8.35, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
        wait(lua_state, 4.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 36.0);
        if is_excute(agent) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
    else {
        // Putrid
        frame(lua_state, 5.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 14.0);
        if is_excute(agent) {
            FT_MOTION_RATE(agent, 1.0);
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 90, 0, 25, 7.0, 0.0, 4.0, -8.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 366, 90, 0, 25, 4.5, 0.0, 8.35, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 50, 108, 0, 25, 9.0, 0.0, 4.0, -8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 50, 108, 0, 25, 4.5, 0.0, 8.35, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 2, 1, Hash40::new("top"), 0.0, 0, 0, 0, 0, 9.0, 0.0, 4.0, -8.5, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 3, 1, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, 8.35, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BOMB);
            AttackModule::set_poison_param(boma, 0, 241, 60, 0.5, false);
            AttackModule::set_poison_param(boma, 1, 241, 60, 0.5, false);
        }
        wait(lua_state, 4.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 36.0);
        if is_excute(agent) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE);
    if stance == STANCE_PIRANHA {
        // Piranha
        frame(lua_state, 6.0);
        for _ in 0..3 {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("packun_atk_air_b_fire"), Hash40::new("mouth"), 7.5, 0, 0, 0, 0, 0, 0.6, true);
            }
            wait(lua_state, 2.0);
        }
        frame(lua_state, 13.0);
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("packun_atk_air_b_breath"), Hash40::new("mouth"), 7, 0.2, 0, 0, 0, -90, 1, true);
        }
        frame(lua_state, 14.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 4.5, -12, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.25);
        }
    }
    else {
        // Putrid
        frame(lua_state, 6.0);
        for h in 0..3 {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("mouth"), 7.5, 0, 0, 0, 0, 0, 0.6, true);
                if h >= 2 {
                    EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 4.0, -9.5, 0, 0, 0, 1.0, true);
                    LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 0.5);
                    EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 4.0, -9.5, 0, 0, 0, 0.9, true);
                }
            }
            wait(lua_state, 2.0);
        }
        frame(lua_state, 13.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("top"), -1.2, 4.0, -9.5, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 2.0);
        }
        frame(lua_state, 16.0);
        for _ in 0..3 {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 4.0, -9.5, 0, 0, 0, 1.0, true);
                LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 0.5);
                EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 4.0, -9.5, 0, 0, 0, 0.9, true);
                EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("top"), -1.2, 4.0, -9.5, 0, 0, 0, 1.0, true);
                LAST_EFFECT_SET_RATE(agent, 2.0);
            }
            wait(lua_state, 3.0);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 4.0, -10.5, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.01, 0.5);
            LAST_EFFECT_SET_RATE(agent, 1.25);
        }
    }
}

unsafe extern "C" fn sound_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE);
    if stance == STANCE_PIRANHA {
        frame(lua_state, 8.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_packun_attackair_b01"));
        }
    }
    else {  // STANCE_PUTRID
        frame(lua_state, 13.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_packun_special_s03"));
        }
        frame(lua_state, 20.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_packun_attackair_b01"));
        }
    }
}

// Prickly
unsafe extern "C" fn game_attackairbs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("virtualhit2"), 16.0, 45, 90, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("virtualhit3"), 16.0, 45, 90, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("mouth"), 19.0, 45, 95, 0, 25, 7.0, 2.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("neck6"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("neck8"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("neck6"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("neck8"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairbs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        let offset = -5.0 * PostureModule::lr(boma);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), offset, 16, 8, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("packun_smash_s_arc_l"), Hash40::new("top"), 6, 7.5, -9.5, 155, -50, 0, 1.0, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("packun_smash_s_arc_r"), Hash40::new("top"), -6, 7.5, -9.5, 155, 50, 0, 1.0, true);
        }
    }
}

unsafe extern "C" fn sound_attackairbs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_smash_s01"));
    }
}

unsafe extern "C" fn expression_attackairbs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    if stance.label != STANCE_PRICKLY {
        FT_DESIRED_RATE(agent, 6.0, 7.0);
    }
    else {
        FT_DESIRED_RATE(agent, 6.0, 9.0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        match stance.label {
            STANCE_PIRANHA => {
                // Piranha
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 10.0 * stance.damage_head, 64, 72, 0, 38, 6.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
                ATTACK(agent, 1, 0, Hash40::new("virtualhit2"), 10.0 * stance.damage_head, 64, 72, 0, 38, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            },
            STANCE_PUTRID => {
                // Putrid
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 10.0 * stance.damage_head, 64, 80, 0, 48, 6.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
                ATTACK(agent, 1, 0, Hash40::new("virtualhit2"), 10.0 * stance.damage_head, 64, 80, 0, 48, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            },
            _ => {
                // Prickly
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 10.0 * stance.damage_head, 75, 90, 0, 60, 6.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
                ATTACK(agent, 1, 0, Hash40::new("virtualhit2"), 10.0 * stance.damage_head, 75, 90, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
                HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_XLU);
                HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_XLU);
                HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_XLU);
            }
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        match stance.label {
            STANCE_PIRANHA => {
                // Piranha
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 10.0 * stance.damage_head, 64, 72, 0, 38, 6.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
                ATTACK(agent, 1, 0, Hash40::new("virtualhit2"), 10.0 * stance.damage_head, 64, 72, 0, 38, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            },
            STANCE_PUTRID => {
                // Putrid
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 10.0 * stance.damage_head, 64, 80, 0, 48, 6.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
                ATTACK(agent, 1, 0, Hash40::new("virtualhit2"), 10.0 * stance.damage_head, 64, 80, 0, 48, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            },
            _ => {
                // Prickly
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 10.0 * stance.damage_head, 75, 90, 0, 60, 6.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
                ATTACK(agent, 1, 0, Hash40::new("virtualhit2"), 10.0 * stance.damage_head, 75, 90, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
            }
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if agent.lr() < 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 6.5, -2.5, 0, 280, -70, 1.45, true);
            LAST_EFFECT_SET_SCALE_W(agent, 1.6, 1.45, 1.45);
            LAST_EFFECT_SET_RATE(agent, 0.7);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 6.5, -2.5, 0, 280, -110, 1.45, true);
            LAST_EFFECT_SET_SCALE_W(agent, 1.6, 1.45, 1.45);
            LAST_EFFECT_SET_RATE(agent, 0.7);
        }
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    if stance.label == STANCE_PRICKLY {
        FT_MOTION_RATE(agent, (11.0/5.0));
    }
    else {
        FT_MOTION_RATE(agent, (11.0/9.0));
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 5.0, 3.0, 8.0, 1.0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 5.0);
    if stance.label == STANCE_PRICKLY {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("potc"), 13.0 * stance.damage_other, 270, 62, 0, 20, 4.5, -0.3,  0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("potc"), 13.0 * stance.damage_other, 270, 62, 0, 20, 4.5, -7.0, -0.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("potc"), 9.0 * stance.damage_other, 361, 92, 0, 15, 4.5, -0.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("potc"), 9.0 * stance.damage_other, 361, 92, 0, 15, 4.5, -4.0, -0.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn, Priority::Low);

    agent.acmd("game_attackairf", game_attackairf, Priority::Low);
    agent.acmd("expression_attackairf", expression_attackairf, Priority::Low);

    agent.acmd("game_attackairb", game_attackairb, Priority::Low);
    agent.acmd("effect_attackairb", effect_attackairb, Priority::Low);
    agent.acmd("sound_attackairb", sound_attackairb, Priority::Low);

    agent.acmd("game_attackairbs", game_attackairbs, Priority::Low);
    agent.acmd("effect_attackairbs", effect_attackairbs, Priority::Low);
    agent.acmd("sound_attackairbs", sound_attackairbs, Priority::Low);
    agent.acmd("expression_attackairbs", expression_attackairbs, Priority::Low);

    agent.acmd("game_attackairhi", game_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi", effect_attackairhi, Priority::Low);

    agent.acmd("game_attackairlw", game_attackairlw, Priority::Low);
}