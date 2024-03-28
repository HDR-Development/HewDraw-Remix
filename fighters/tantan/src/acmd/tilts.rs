
use super::*;

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    let kbg = 95;
    let bkb = 85;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 6.0, 96, kbg, 0, bkb, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 6.0, 96, kbg, 0, bkb, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footl"), 7.5, 96, kbg, 0, bkb, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("legl"), 6.0, 96, kbg, 0, bkb, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 4, 0, Hash40::new("kneel"), 6.0, 96, kbg, 0, bkb, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 5, 0, Hash40::new("footl"), 7.5, 96, kbg, 0, bkb, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("legr"), 6.0, 96, kbg, 0, bkb, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 4, 0, Hash40::new("kneer"), 6.0, 96, kbg, 0, bkb, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 5, 0, Hash40::new("footr"), 7.5, 96, kbg, 0, bkb, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 5.0, 93, kbg+5, 0, bkb, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 5.0, 93, kbg+5, 0, bkb, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footl"), 6.5, 93, kbg+5, 0, bkb, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("legr"), 5.0, 93, kbg+5, 0, bkb, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 4, 0, Hash40::new("kneer"), 5.0, 93, kbg+5, 0, bkb, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 5, 0, Hash40::new("footr"), 6.5, 93, kbg+5, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
        AttackModule::clear(boma, 2, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut powerFactor = 1.0;
    let mut sfx_level = *ATTACK_SOUND_LEVEL_M;
    let mut sizeFactor = 1.0;
    let mut powerFactor = 1.0;

    frame(lua_state, 12.0);
    if is_excute(agent) {        
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("arml5"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("arml4"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("arml3"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("arml2"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("arml1"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE_RANGE(agent,13.0,24.0,8.0);
    if is_excute(agent) {     
        let is_dragonized = WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L);
        let bigScale = WorkModule::get_param_float(boma,hash40("param_private"),hash40("arm_l_big_scale"));
        let sizeFactor = if is_dragonized {bigScale} else {1.0};
        let powerFactor = if is_dragonized {1.15} else {1.0};
        let sfx_level = if is_dragonized {*ATTACK_SOUND_LEVEL_L} else {*ATTACK_SOUND_LEVEL_M};
        
        ATTACK(agent, 0, 0, Hash40::new("pl1_gimmickc"), 8.0*powerFactor, 90, 85, 0, 40, 2.9*sizeFactor, 3.0, 0.0, 0.3, Some(0.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx_level, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {    
        ATTACK(agent, 0, 0, Hash40::new("pl1_gimmickc"), 10.0*powerFactor, 70, 85, 0, 50, 2.9*sizeFactor, 3.0, 0.0, 0.3, Some(0.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx_level, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE(agent,1.0);
    if is_excute(agent) {    
        ATTACK(agent, 0, 0, Hash40::new("pl1_gimmickc"), 8.0*powerFactor, 361, 85, 0, 50, 2.9*sizeFactor, 3.0, 0.0, 0.3, Some(0.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx_level, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml5"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml4"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml3"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml2"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml1"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 13.0);
    if is_excute(agent) {        
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {    
        let is_dragonized = WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L);
        let scale = if is_dragonized {1.5} else {1.25};
        //EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("pl1_have"), 0, 0, 0, 0, 90, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 2.5, 10, 0, 0, 0, scale, true);
        LAST_EFFECT_SET_SCALE_W(agent,1.5,1,1);
        LAST_EFFECT_SET_RATE(agent,0.55);

        if is_dragonized {
            LAST_EFFECT_SET_COLOR(agent,0.25,1.0,0.375);
            EFFECT_FOLLOW(agent, Hash40::new("tantan_dragon_attack_fire"), Hash40::new("pl1_have"), 0, 0, 0, 0, 180, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent,2.0);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("tantan_dragon_attack_fire"),false,false);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("tantan_punch_end"), Hash40::new("arml1"), -1, -0.2, 0, 0, 0, 0, 0.8, true);
    }
}

unsafe extern "C" fn sound_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut powerFactor = 1.0;

    frame(lua_state, 12.0);
    if is_excute(agent) {        
        PLAY_SEQUENCE(agent, Hash40::new("seq_tantan_rnd_punch_long"));
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {        
        let sfx = if WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L) {Hash40::new("se_tantan_attack01_doragon_smash")} else {Hash40::new("se_tantan_attack01_long")};
        PLAY_SE(agent, sfx);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {     
        PLAY_SE(agent, Hash40::new("se_tantan_attack01_catch"));
    }
}

unsafe extern "C" fn expression_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut powerFactor = 1.0;
    
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_dragonarm"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackss"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

unsafe extern "C" fn game_attacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 6.0);
    let motion = MotionModule::motion_kind(boma);
    let isHigh = motion == Hash40::new("attack_s3_hi").hash;
    let isLow = motion == Hash40::new("attack_s3_lw").hash;
    let damageBonus = if (!isHigh && !isLow) {0.0} else {0.5};
    
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 9.5+(damageBonus*2.0), 361, 75, 0, 37, 3.8, 7.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 9.5+(damageBonus*2.0), 361, 75, 0, 37, 4.2, 1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneer"), 8.5+damageBonus, 361, 75, 0, 37, 3.8, -5.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        if (isHigh)
            {AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);}
        else if (isLow)
            {AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);}
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 2, 11.5, 2.2, -21.5, 2.5, 13.5, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 2, 8.5, 2.2, -8.5, 17, 9.5, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_attacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 2, 6, 2.2, 5, -1, 11.8, 0.95, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_tantan_attackhard_h01"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_tantan_rnd_attack01"));
    }
}

unsafe extern "C" fn expression_attacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackhi3", game_attackhi3);
    agent.acmd("game_attacklw3", game_attacklw3);
    agent.acmd("effect_attacklw3", effect_attacklw3);
    agent.acmd("sound_attacklw3", sound_attacklw3);
    agent.acmd("expression_attacklw3", expression_attacklw3);
    agent.acmd("game_attacks3hi", game_attacks3hi);
    agent.acmd("game_attacks3", game_attacks3hi);
    agent.acmd("game_attacks3lw", game_attacks3hi);
    agent.acmd("effect_attacks3hi", effect_attacks3hi);
    agent.acmd("effect_attacks3", effect_attacks3);
    agent.acmd("effect_attacks3lw", effect_attacks3lw);
    agent.acmd("sound_attacks3hi", sound_attacks3hi);
    agent.acmd("sound_attacks3", sound_attacks3hi);
    agent.acmd("sound_attacks3lw", sound_attacks3hi);
    agent.acmd("expression_attacks3hi", expression_attacks3hi);
    agent.acmd("expression_attacks3", expression_attacks3hi);
    agent.acmd("expression_attacks3lw", expression_attacks3hi);
}
