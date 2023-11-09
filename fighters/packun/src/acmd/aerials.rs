
use super::*;


#[acmd_script( agent = "packun", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 1.0);
    if stance.label != 2 {
        FT_MOTION_RATE(fighter, 0.5);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        let bkb = if stance.label == 2 {15} else {20};
        let kbg = if stance.label == 2 {90} else {45};
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 2.0 * stance.damage_other, 365, kbg, 0, bkb, 5.5, 1.8, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 2.0 * stance.damage_other, 365, kbg, 0, bkb, 5.5, 1.8, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 3.0 * stance.damage_other, 55, 125, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0 * stance.damage_other, 55, 125, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "packun", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 3.0);
    if stance.label != 2 {
        FT_DESIRED_RATE(fighter, 6.0, 4.0);
    }
    else {
        FT_DESIRED_RATE(fighter, 6.0, 7.0);
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("potc"), 9.0 * stance.damage_other, 40, 94, 0, 30, 4.5, 3.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("potc"), 11.0 * stance.damage_other, 40, 94, 0, 30, 7.0, -3.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        if stance.label == 2 {
            HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_XLU);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "packun", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    if stance.label != 1 {
        if stance.label == 2 {
            FT_MOTION_RATE(fighter, (18.0/14.0));
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 14.0);
        FT_MOTION_RATE(fighter, 1.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0 * stance.damage_other, 50, 108, 0, 25, 9.0, 0.0, 4.0, -10.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            VarModule::on_flag(boma.object(), vars::packun::status::FLAME_ACTIVE);
        }
        wait(lua_state, 4.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            VarModule::off_flag(boma.object(), vars::packun::status::FLAME_ACTIVE);
        }
        frame(lua_state, 36.0);
        if is_excute(fighter) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
    else {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * stance.damage_other, 366, 90, 0, 25, 7.0, 0.0, 4.0, -8.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
        frame(lua_state, 26.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0 * stance.damage_other, 50, 108, 0, 25, 9.0, 0.0, 4.0, -8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        wait(lua_state, 4.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 36.0);
        if is_excute(fighter) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
    
}

#[acmd_script( agent = "packun", script = "effect_attackairb", category = ACMD_EFFECT, low_priority )]
unsafe fn packun_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    if stance != 1 { 
        frame(lua_state, 6.0);
        for _ in 0..3 {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_air_b_fire"), Hash40::new("mouth"), 7.5, 0, 0, 0, 0, 0, 0.6, true);
            }
            wait(lua_state, 2.0);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("packun_atk_air_b_breath"), Hash40::new("mouth"), 7, 0.2, 0, 0, 0, -90, 1, true);
        }
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_flame"), Hash40::new("top"), 0, 4.5, -12, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(fighter, 1.25);
        }
    }
    else if stance == 1 { 
        frame(lua_state, 6.0);
        for h in 0..3 {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_mouth"), Hash40::new("mouth"), 7.5, 0, 0, 0, 0, 0, 0.6, true);
                if h >= 2 {
                    EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 4.0, -9.5, 0, 0, 0, 1.0, true);
                    LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
                    EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 4.0, -9.5, 0, 0, 0, 0.9, true);
                }
            }
            wait(lua_state, 2.0);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_mouth"), Hash40::new("top"), -1.2, 4.0, -9.5, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(fighter, 2.0);
        }
        frame(lua_state, 16.0);
        for _ in 0..3 {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 4.0, -9.5, 0, 0, 0, 1.0, true);
                LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
                EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 4.0, -9.5, 0, 0, 0, 0.9, true);
                EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_mouth"), Hash40::new("top"), -1.2, 4.0, -9.5, 0, 0, 0, 1.0, true);
                LAST_EFFECT_SET_RATE(fighter, 2.0);
            }
            wait(lua_state, 3.0);
        }
        frame(lua_state, 26.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_flame"), Hash40::new("top"), 0, 4.0, -10.5, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.01, 0.5);
            LAST_EFFECT_SET_RATE(fighter, 1.25);
        }
    }
}

#[acmd_script( agent = "packun", script = "sound_attackairb", category = ACMD_SOUND, low_priority )]
unsafe fn packun_attack_air_b_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if stance != 1 {
            PLAY_SE(fighter, Hash40::new("se_packun_attackair_b01"));
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if stance == 1 {
            PLAY_SE(fighter, Hash40::new("se_packun_special_s03"));
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if stance == 1 {
            PLAY_SE(fighter, Hash40::new("se_packun_attackair_b01"));
        }
    }
}

#[acmd_script( agent = "packun", script = "game_attackairbs" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_air_b_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("virtualhit2"), 18.2, 45, 100, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("virtualhit3"), 18.2, 45, 100, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("mouth"), 19.5, 45, 108, 0, 25, 7.0, 2.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
        HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("neck6"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("neck8"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("neck6"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("neck8"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "packun", script = "effect_attackairbs", category = ACMD_EFFECT, low_priority )]
unsafe fn packun_attack_air_b_s_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("packun_smash_s_arc_l"), Hash40::new("top"), 0, 13, 3.5, 40, -60, 0, 1.0, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("packun_smash_s_arc_r"), Hash40::new("top"), 0, 13, 3.5, 40, 60, 0, 1.0, true);
        }
    }
}

#[acmd_script( agent = "packun", script = "sound_attackairbs", category = ACMD_SOUND, low_priority )]
unsafe fn packun_attack_air_b_s_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_smash_s01"));
    }
}

#[acmd_script( agent = "packun", script = "expression_attackairbs", category = ACMD_EXPRESSION, low_priority )]
unsafe fn packun_attack_air_b_s_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "packun", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    let angle = if stance.label == 1 {105} else {75};
    let bkb = if stance.label == 1 {15} else if stance.label == 0 {10} else {0};
    let sound = if stance.label != 2 { *COLLISION_SOUND_ATTR_PUNCH } else { *COLLISION_SOUND_ATTR_HEAVY };
    if stance.label != 2 {
        FT_DESIRED_RATE(fighter, 6.0, 7.0);
    }
    else {
        FT_DESIRED_RATE(fighter, 6.0, 9.0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("mouth"), 9.0 * stance.damage_head, angle, 87, 0, 53 + bkb, 7.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_HEAD);
        if stance.label == 2 {
            HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("neck6"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("neck8"), *HIT_STATUS_XLU);
        }
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("neck6"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("neck8"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "packun", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn packun_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    if stance.label == 2 {
        FT_MOTION_RATE(fighter, (11.0/5.0));
    }
    else {
        FT_MOTION_RATE(fighter, (11.0/9.0));
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 5.0, 3.0, 8.0, 1.0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 5.0);
    if stance.label == 2 {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new_raw(0x0496187f8d), 13.0 * stance.damage_other, 270, 92, 0, 15, 4.5, -0.3, 0.0, 0.0, Some(-0.3), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new_raw(0x0496187f8d), 13.0 * stance.damage_other, 270, 90, 0, 25, 4.5, -7.0, -0.5, 0.5, Some(-7.0), Some(-0.5), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        /* Air-only */
        ATTACK(fighter, 2, 0, Hash40::new_raw(0x0496187f8d), 13.0 * stance.damage_other, 270, 64, 0, 15, 4.5, -0.3, 0.0, 0.0, Some(-0.3), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new_raw(0x0496187f8d), 13.0 * stance.damage_other, 270, 64, 0, 25, 4.5, -7.0, -0.5, 0.5, Some(-7.0), Some(-0.5), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new_raw(0x0496187f8d), 9.0 * stance.damage_other, 361, 92, 0, 15, 4.5, -0.3, 0.0, 0.0, Some(-0.3), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new_raw(0x0496187f8d), 9.0 * stance.damage_other, 361, 92, 0, 15, 4.5, -7.0, -0.5, 0.5, Some(-7.0), Some(-0.5), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        packun_attack_air_n_game,
        packun_attack_air_f_game,
        packun_attack_air_b_game,
        packun_attack_air_b_effect,
        packun_attack_air_b_sound,
        packun_attack_air_b_s_game,
        packun_attack_air_b_s_effect,
        packun_attack_air_b_s_expression,
        packun_attack_air_b_s_sound,
        packun_attack_air_hi_game,
        packun_attack_air_lw_game,
    );
}

