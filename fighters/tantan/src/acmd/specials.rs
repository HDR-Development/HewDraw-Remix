use super::*;

unsafe extern "C" fn game_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    frame(lua_state, 1.0);
    if(armType == 1){
        FT_MOTION_RATE(agent, 1.35);
    }
    else if (armType==2)
    {
        FT_MOTION_RATE(agent, 0.75);
    }
    else if WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L) {
        AttackModule::set_power_mul(boma, 1.1);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr5"), *HIT_STATUS_XLU);
        if(armType == 1){
            ATTACK(agent, 0, 0, Hash40::new("armr5"), 13.0, 60, 95, 0, 50, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("handr"), 13.0, 60, 95, 0, 50, 5.0, 3.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        else if(armType == 2){
            ATTACK(agent, 0, 0, Hash40::new("armr5"), 8.5, 361, 85, 0, 45, 2.5, 1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("handr"), 8.5, 361, 85, 0, 45, 3.5, 3.5, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        }
        else {
            let is_dragonized = WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L);
            let bigScale = WorkModule::get_param_float(boma,hash40("param_private"),hash40("arm_l_big_scale"));
            let sizeFactor = if is_dragonized {bigScale} else {1.0};
            let powerFactor = if is_dragonized {1.15} else {1.0};
            let sfx_level = if is_dragonized {*ATTACK_SOUND_LEVEL_L} else {*ATTACK_SOUND_LEVEL_M};

            ATTACK(agent, 0, 0, Hash40::new("armr5"), 10.25*powerFactor, 45, 100, 0, 45, 2.5*sizeFactor, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx_level, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("handr"), 10.25*powerFactor, 45, 100, 0, 45, 4.5*sizeFactor, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx_level, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("tantan_jump_line_s"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("tantan_jump_line_l"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if(armType==1){
            EFFECT_FOLLOW(agent, Hash40::new("tantan_atk_air_n2"), Hash40::new("top"), 0, 10, 4, 0, 0, 0, 1.2, true);
        }
        else if (armType==2){
            EFFECT_FOLLOW(agent, Hash40::new("tantan_atk_air_n3"), Hash40::new("top"), 0, 10, 4, 0, 0, 0, 1.2, true);
        }
        else{
            EFFECT_FOLLOW(agent, Hash40::new("tantan_atk_air_n1"), Hash40::new("top"), 0, 10, 4, 0, 0, 0, 1.2, true);
            if WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L) {
                LAST_EFFECT_SET_COLOR(agent,1.0,0.5,0.5);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("pr1_gimmickc"), 0, 0, 0, 0, 0, 0, 0.3, true);
                LAST_EFFECT_SET_RATE(agent,1.5);
            }
        }
    }
    frame(lua_state, 22.0);
    {
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_damage_fire_fly"), -1);
    }
}

unsafe extern "C" fn sound_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    if(armType==1){
        frame(lua_state, 8.0);
        if is_excute(agent) {
            PLAY_STATUS(agent, Hash40::new("se_tantan_attackair_n02"));
            PLAY_SEQUENCE(agent, Hash40::new("seq_tantan_rnd_attack03"));
        }
    }
    else if (armType==2){
        frame(lua_state, 7.0);
        if is_excute(agent) {
            PLAY_STATUS(agent, Hash40::new("se_tantan_attackair_n03"));
        }
        frame(lua_state, 8.0);
        if is_excute(agent) {
            PLAY_SEQUENCE(agent, Hash40::new("seq_tantan_rnd_attack01"));
        }
    }
    else{
        frame(lua_state, 6.0);
        if is_excute(agent) {
            PLAY_STATUS(agent, Hash40::new("se_tantan_attackair_n01"));
        }
        frame(lua_state, 8.0);
        if is_excute(agent) {
            PLAY_SEQUENCE(agent, Hash40::new("seq_tantan_rnd_attack01"));
        }
    }
}

unsafe extern "C" fn expression_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if(armType==1){
            RUMBLE_HIT(agent, Hash40::new("rbkind_76_megabolthit"), 10);
        }
        else if (armType==2){
            RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 12);
        }
        else{
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 9);
        }
    }
}

unsafe extern "C" fn game_specialairnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);

    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 5.0);
    if (armType!=2) {
        //Additional landing lag for using Dragon/Megawatt
        FT_MOTION_RATE(agent, 1.0);
        if (armType==0){
            frame(lua_state, 7.0);
            FT_MOTION_RATE(agent, 0.5);
        }
        else{
            frame(lua_state, 11.0);
            FT_MOTION_RATE(agent, 0.5);
        }
    }
    //Landing lag
    frame(lua_state, 16.0);
    if is_excute(agent) {
        CancelModule::enable_cancel(boma);
    }
}

unsafe extern "C" fn effect_specialairnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialairnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;

    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_tantan_landing02"));
    }
}

unsafe extern "C" fn expression_specialairnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let angle = WorkModule::get_float(boma,*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("tantan_jump_air"), Hash40::new("pl1_have"), 0, 0, 0, 0, 180.0+angle, 0, 1, 0, 0, 0, 0, 0, 0, true);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1.3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if !WorkModule::is_flag(boma, *FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_IS_SPECIAL_HI_AIR_PHYSICS) {
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("tantan_hook_wind"), Hash40::new("handl"), -0.2, -0.03, -0.03, 180, 180, 0, 1, true);
        }
    }
    frame(lua_state, 2.0);
    if !WorkModule::is_flag(boma, *FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_IS_SPECIAL_HI_AIR_PHYSICS) {
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("tantan_wepon_ringwind"), Hash40::new("pl1_have"), 0, 0, 0, 0, 180, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 2);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("tantan_wepon_ringwind"), false, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("tantan_hook_wind"), false, true);
    }
}

unsafe extern "C" fn game_specialairhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let bReverseFrame = if agent.is_prev_status(*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND) {2.0} else {4.0};

    frame(lua_state, bReverseFrame);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
    }
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 5.0/(8.0-5.0));
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_TANTAN_CLIFF_HANG_DATA_AIR_LASSO as u32);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialairn", game_specialairn);
    agent.acmd("effect_specialairn", effect_specialairn);
    agent.acmd("sound_specialairn", sound_specialairn);
    agent.acmd("expression_specialairn", expression_specialairn);
    agent.acmd("game_specialairnend", game_specialairnend);
    agent.acmd("effect_specialairnend", effect_specialairnend);
    agent.acmd("sound_specialairnend", sound_specialairnend);
    agent.acmd("expression_specialairnend",expression_specialairnend);

    agent.acmd("effect_specialairhi", effect_specialairhi);
    agent.acmd("effect_specialairhi2", effect_specialairhi);
    agent.acmd("game_specialairhistart", game_specialairhistart);
    agent.acmd("game_specialairhistart2", game_specialairhistart);
}
