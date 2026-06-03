use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if stance.label == STANCE_PRICKLY {
        FT_MOTION_RATE_RANGE(agent, 7.0, 15.0, 11.0);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 16.0);
    if is_excute(agent) {
        let (kbg, sound_lvl, sound_attr) = match stance.label {
            STANCE_PIRANHA => { (110, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH) },
            STANCE_PUTRID => { (140, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH) },
            _ => { (100, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY) }
        };
        ATTACK(agent, 0, 0, Hash40::new("virtualhit2"), 14.0 * stance.damage_head, 45, 100, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_lvl, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("virtualhit3"), 14.0 * stance.damage_head, 45, 100, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_lvl, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("mouth"), 15.0 * stance.damage_head, 45, kbg, 0, 25, 7.0, 2.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, sound_attr, *ATTACK_REGION_HEAD);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 20.0, 58.0, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 58.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, 11, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.4);
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

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    if stance.label == STANCE_PRICKLY {
        FT_DESIRED_RATE(agent, 8.0, 11.0);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0 * stance.damage_bite, 114, 100, 80, 0, 5.5, 0.0, 12.0, -4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0 * stance.damage_bite, 114, 100, 80, 0, 5.5, 0.0, 12.0, 4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        HIT_NODE(agent, Hash40::new("mouth"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if stance.label == STANCE_PUTRID {
            ATTACK(agent, 0, 0, Hash40::new("mouth"), 13.0 * stance.damage_bite, 90, 149, 0, 46, 9.0, 2.0, 0.0, 0.0, Some(2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            ATTACK(agent, 1, 1, Hash40::new("mouth"), 0.0, 0, 0, 0, 0, 9.0, 2.0, 0.0, 0.0, Some(2.5), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BITE);
            AttackModule::set_poison_param(boma, 0, 241, 60, 2.5, false);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("mouth"), 13.0 * stance.damage_bite, 90, 109, 0, 46, 9.0, 2.0, 0.0, 0.0, Some(2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
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
        let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
        if stance.label == STANCE_PUTRID {
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
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_smash_h03"));
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 3.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if stance.label != STANCE_PRICKLY {
        FT_MOTION_RATE_RANGE(agent, 7.0, 14.0, 3.0);
    }
    else {
        FT_MOTION_RATE_RANGE(agent, 7.0, 14.0, 6.0);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if stance.label == STANCE_PIRANHA {
            // Piranha
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0 * stance.damage_other, 94, 64, 0, 48, 5.0, 0.0, 4.0, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 13.0 * stance.damage_other, 94, 64, 0, 48, 4.0, 0.0, 3.5, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 13.0 * stance.damage_other, 94, 64, 0, 48, 4.0, 0.0, 3.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        else {
            // Putrid/Prickly
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0 * stance.damage_other, 35, 99, 0, 25, 5.0, 0.0, 4.0, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 13.0 * stance.damage_other, 35, 99, 0, 25, 4.0, 0.0, 3.5, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 13.0 * stance.damage_other, 35, 99, 0, 25, 4.0, 0.0, 3.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if stance.label == STANCE_PIRANHA {
            // Piranha
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0 * stance.damage_other, 94, 64, 0, 48, 5.0, 0.0, 4.0, -13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 13.0 * stance.damage_other, 94, 64, 0, 48, 4.0, 0.0, 3.6, -8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 13.0 * stance.damage_other, 94, 64, 0, 48, 4.0, 0.0, 3.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        else {
            // Putrid/Prickly
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0 * stance.damage_other, 35, 99, 0, 25, 5.0, 0.0, 4.0, -13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 13.0 * stance.damage_other, 35, 99, 0, 25, 4.0, 0.0, 3.6, -8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 13.0 * stance.damage_other, 35, 99, 0, 25, 4.0, 0.0, 3.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 5, 4, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 9.5);
    if is_excute(agent) {
        if stance.label == STANCE_PIRANHA {
            EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("mouth"), 6.0, 1.0, 0, 0, 0, 0, 0.6, true);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if stance.label == STANCE_PIRANHA {
            EFFECT_FOLLOW(agent, Hash40::new("packun_atk_air_b_fire"), Hash40::new("potc"), -8, 0, 0, 0, 0, 0, 1.2, true);
        }
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_smash_lw_arc"), Hash40::new("packun_smash_lw_arc"), Hash40::new("top"), 0, 5, 3, -180, 150, 7, 1.4, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if stance.label == STANCE_PIRANHA {
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
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);
    agent.acmd("effect_attacks4", effect_attacks4, Priority::Low);

    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
    agent.acmd("effect_attackhi4", effect_attackhi4, Priority::Low);
    agent.acmd("sound_attackhi4", sound_attackhi4, Priority::Low);

    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);
    agent.acmd("effect_attacklw4", effect_attacklw4, Priority::Low);
}