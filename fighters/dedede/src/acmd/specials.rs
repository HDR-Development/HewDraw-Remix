use super::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    FT_MOTION_RATE(agent, 0.6);
    frame(lua_state, 17.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE);
        CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(12.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 170, 40, 30, 0, 6.0, 0.0, 9.0, 16.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 170, 40, 10, 0, 7.0, 0.0, 6.0, 23.0, Some(0.0), Some(12.0), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 9.0, 0.0, 8.5, 10.0, Some(0.0), Some(8.5), Some(21.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        SEARCH(agent, 0, 0, Hash40::new("top"), 4.0, 0.0, 13.0, 5.5, Some(0.0), Some(3.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
        search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
        SEARCH(agent, 1, 0, Hash40::new("top"), 9.5, 0.0, 8.5, 11.0, Some(0.0), Some(8.5), Some(20.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
        search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE);
            CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 5.5, 5.0, Some(0.0), Some(5.5), Some(12.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 170, 40, 30, 0, 6.0, 0.0, 6.5, 16.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 170, 40, 10, 0, 7.0, 0.0, 3.5, 23.0, Some(0.0), Some(9.5), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 9.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(21.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            SEARCH(agent, 0, 0, Hash40::new("top"), 4.0, 0.0, 9.5, 5.5, Some(0.0), Some(2.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            SEARCH(agent, 1, 0, Hash40::new("top"), 9.5, 0.0, 6.0, 11.0, Some(0.0),Some(6.0), Some(20.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
        }
    }
}

unsafe extern "C" fn game_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 19.0, 25.0);
}

unsafe extern "C" fn game_specialairnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE);
        CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 12.0, 5.0, Some(0.0), Some(12.0), Some(9.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 170, 40, 30, 0, 6.0, 0.0, 12.0, 16.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 170, 40, 10, 0, 7.0, 0.0, 9.0, 23.0, Some(0.0), Some(15.0), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 9.0, 0.0, 11.5, 10.0, Some(0.0), Some(11.5), Some(21.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        SEARCH(agent, 0, 0, Hash40::new("top"), 4.0, 0.0, 16.0, 5.5, Some(0.0), Some(6.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
        search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
        SEARCH(agent, 1, 0, Hash40::new("top"), 9.5, 0.0, 11.5, 11.0, Some(0.0), Some(11.5), Some(20.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
        search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE);
            CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 5.5, 5.0, Some(0.0), Some(5.5), Some(9.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 170, 40, 30, 0, 6.0, 0.0, 6.5, 16.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 170, 40, 10, 0, 7.0, 0.0, 3.5, 23.0, Some(0.0), Some(9.5), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 9.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(21.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            SEARCH(agent, 0, 0, Hash40::new("top"), 4.0, 0.0, 9.5, 5.5, Some(0.0), Some(2.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            SEARCH(agent, 1, 0, Hash40::new("top"), 9.5, 0.0, 6.0, 11.0, Some(0.0), Some(6.0), Some(20.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
        }
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_GENERATE);
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE_RANGE(agent, 26.0, 65.0, 31.0);
}

unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if (PostureModule::lr(boma) == -1.0) {
            EFFECT_FOLLOW(agent, Hash40::new("dedede_hammer_arc_wind"), Hash40::new("throw"), 2.0, -2.0, -2.0, 90.0, 0.0, 0.0 , 0.6, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("dedede_hammer_arc_wind"), Hash40::new("throw"), -2.0, -2.0, -2.0, 90.0, 180.0, 0.0 , 0.6, true);
        }
        LAST_EFFECT_SET_COLOR(agent, 0.96, 0.6, 0.26); 
    }
    frame(lua_state, 18.0);
    if is_excute(agent){     
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent){
        EFFECT_OFF_KIND(agent, Hash40::new("dedede_hammer_arc_wind"), true, true);
    }
}

unsafe extern "C" fn sound_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_dedede_wad_throw"));
    }
}

unsafe extern "C" fn expression_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialsmiss(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_MISS_SEARCH);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_MISS_SEARCH);
    }
}

unsafe extern "C" fn game_specialsget(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE_RANGE(agent, 26.0, 65.0, 23.0);
}

unsafe extern "C" fn effect_specialsget(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent){
        EFFECT(agent, Hash40::new("sys_item_get"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent){
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_specialsget(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent){
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.0, y:-0.8, z:0.0});

    }
    FT_MOTION_RATE(agent, 0.809);
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);

}

unsafe extern "C" fn game_specialhijump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 4.0, 65, 100, 0, 50, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 65, 100, 0, 50, 6.0, 0.0, 7.0, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 65, 100, 0, 50, 6.0, 0.0, 7.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 140);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_TURN);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(0.0, -4.0, 0.0));
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 140);
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 270, 90, 0, 60, 9.8, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 270, 56, 0, 60, 8.5, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn game_specialhilanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent){
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    if is_excute(agent){
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("virtualwaist"), *HIT_STATUS_OFF); 
    }
    frame(lua_state, 3.0);
    if is_excute(agent){
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 90, 82, 0, 95, 6.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 70, 82, 0, 95, 6.0, 0.0, 6.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 70, 82, 0, 95, 6.0, 0.0, 6.0, -10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(agent){
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_STAR_RIGHT);

        ArticleModule::generate_article(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_STAR, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_STAR_LEFT);

        ArticleModule::generate_article(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_STAR, false, 0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhiturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 13.0 / (20.0 - 1.0));
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("virtualwaist"), *HIT_STATUS_OFF);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_specialhifailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 81.0 / (111.0));
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, false, -1);
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER){
            let article = ArticleModule::get_article(agent.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER);
            let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_boma = sv_battle_object::module_accessor(object_id);
            MotionModule::change_motion(article_boma, Hash40::new("start"), 0.0, 1.0, false, 0.0, false, false);
            MotionModule::set_frame(article_boma, 5.0, true);
        }
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER){
            let article = ArticleModule::get_article(agent.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER);
            let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_boma = sv_battle_object::module_accessor(object_id);
            MotionModule::change_motion(article_boma, Hash40::new("hold"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 27.0, 7.0);

}

unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 19, 9, 0, 0, 0, 1, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("dedede_jethammer_hold"), Hash40::new("hammer3"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("dedede_jethammer_hold2"), Hash40::new("hammer3"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
}

unsafe extern "C" fn sound_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_dedede_special_l01"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_dedede_special_l02"));
    }
}

unsafe extern "C" fn expression_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("hammer") as i64, hash40("hammer_disp_off") as i64);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charge_frames = VarModule::get_int(agent.battle_object, vars::dedede::instance::SPECIAL_LW_CHARGE_FRAME);
    let charge_level = charge_frames as f32 / 30.0;
    FT_MOTION_RATE(agent, 45.0/(23.0));
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::dedede::instance::SPECIAL_LW_CONTINUE_JET_SPIN) {
            KineticModule::clear_speed_attr(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let rush_speed = if boma.is_situation(*SITUATION_KIND_GROUND) { 2.2 } else { 1.5 };
            KineticModule::add_speed(agent.module_accessor, &Vector3f::new(rush_speed + (charge_level * 0.2), 0.0, 0.0));
        }
        ATTACK(agent, 0, 0, Hash40::new("hammer2"), 14.0 + (charge_level * 1.5), 361, 60, 0, 85, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(agent, 1, 0, Hash40::new("hammer1"), 14.0 + (charge_level * 1.5), 361, 60, 0, 85, 4.0, 10.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(agent, 2, 0, Hash40::new("hammer1"), 14.0 + (charge_level * 1.5), 361, 60, 0, 85, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(agent, 3, 0, Hash40::new("hammer1"), 14.0 + (charge_level * 1.5), 361, 60, 0, 85, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
    } 
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::dedede::instance::SPECIAL_LW_CONTINUE_JET_SPIN) && boma.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::add_speed(boma, &Vector3f::new(0.5, 0.0, 0.0));
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if charge_level < 1.0 {
            ATTACK(agent, 0, 0, Hash40::new("hammer2"), 11.0 + (charge_level * 1.5), 361, 60, 0, 85, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 1, 0, Hash40::new("hammer1"), 11.0 + (charge_level * 1.5), 361, 60, 0, 85, 4.0, 10.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 2, 0, Hash40::new("hammer1"), 11.0 + (charge_level * 1.5), 361, 60, 0, 85, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 3, 0, Hash40::new("hammer1"), 11.0 + (charge_level * 1.5), 361, 60, 0, 85, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("hammer2"), 14.0 + (charge_level * 1.5), 361, 60, 0, 85, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 1, 0, Hash40::new("hammer1"), 14.0 + (charge_level * 1.5), 361, 60, 0, 85, 4.0, 10.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 2, 0, Hash40::new("hammer1"), 14.0 + (charge_level * 1.5), 361, 60, 0, 85, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 3, 0, Hash40::new("hammer1"), 14.0 + (charge_level * 1.5), 361, 60, 0, 85, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::dedede::instance::SPECIAL_LW_CONTINUE_JET_SPIN) && boma.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::add_speed(boma, &Vector3f::new(0.3, 0.0, 0.0));
        }
    }
    // todo: refactor
    frame(lua_state, 18.0);
    if is_excute(agent) {
        VarModule::set_flag(agent.battle_object, vars::dedede::status::SPECIAL_LW_CONTINUE_SPIN, true);
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 23.0);
    FT_MOTION_RATE_RANGE(agent, 22.0, 31.0, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        AttackModule::clear_all(boma);
        KineticModule::clear_speed_all(boma);
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.6, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 13, 0, 0, 0, 0, 1.6, true);
        }
        EFFECT_FOLLOW(agent, Hash40::new("dedede_final_jet"), Hash40::new("hammer2"), 0.0, 0.0, 15.0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.2, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 13, 0, 0, 0, 0, 1.6, true);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.8, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 13, 0, 0, 0, 0, 1.6, true);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("dedede_final_jet"), false, true);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::dedede::instance::SPECIAL_LW_CONTINUE_JET_SPIN) {
            let charge_frames = VarModule::get_int(agent.battle_object, vars::dedede::instance::SPECIAL_LW_CHARGE_FRAME);
            let charge_level = charge_frames as f32 / 30.0;
            if charge_level < 2.0 {
                PLAY_SEQUENCE(agent, Hash40::new("seq_dedede_rnd_attack01"));
            }
            else {
                let rand = sv_math::rand(hash40("fighter"), 2);
                if rand == 1 {
                    PLAY_SE(agent, Hash40::new("vc_dedede_final01"));
                }
                else {
                    PLAY_SE(agent, Hash40::new("vc_dedede_final03"));
                }
            } 
        }
        PLAY_STATUS(agent, Hash40::new("se_dedede_special_l05"));
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("hammer") as i64, hash40("hammer_disp_off") as i64);
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        if !VarModule::is_flag(agent.battle_object, vars::dedede::instance::SPECIAL_LW_CONTINUE_JET_SPIN) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("hammer") as i64, hash40("hammer_normal") as i64);
        ArticleModule::remove_exist(agent.boma(), *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}


unsafe extern "C" fn game_speciallwmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let rush_speed = 0.9 + 0.01 * WorkModule::get_float(boma, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 14);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(rush_speed, 0.0, 0.0));
        ATTACK(agent, 0, 0, Hash40::new("hammer1"), 40.0, 361, 46, 0, 60, 9.0, 16.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 24, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(agent, 1, 0, Hash40::new("top"), 30.0, 361, 46, 0, 60, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 24, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 16.0);
    for _ in 0.. 20 {
        if is_excute(agent) {
            KineticModule::add_speed(boma, &Vector3f::new(-1.0 * (rush_speed/30.0), 0.0, 0.0));
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn game_speciallwjumpsquat(agent: &mut L2CAgentBase){
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 5.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialnloop", game_specialnloop, Priority::Low);
    agent.acmd("game_specialnend", game_specialnend, Priority::Low);

    agent.acmd("game_specialairnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialairnloop", game_specialairnloop, Priority::Low);
    agent.acmd("game_specialairnend", game_specialnend, Priority::Low);

    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialairsstart", game_specialsstart, Priority::Low);
    agent.acmd("effect_specialsstart", effect_specialsstart, Priority::Low);
    agent.acmd("effect_specialairsstart", effect_specialsstart, Priority::Low);
    agent.acmd("sound_specialsstart", sound_specialsstart, Priority::Low);
    agent.acmd("sound_specialairsstart", sound_specialsstart, Priority::Low);
    agent.acmd("expression_specialsstart", expression_specialsstart, Priority::Low);
    agent.acmd("expression_specialairsstart", expression_specialsstart, Priority::Low);

    agent.acmd("game_specialsmiss", game_specialsmiss, Priority::Low);
    agent.acmd("game_specialairsmiss", game_specialsmiss, Priority::Low);

    agent.acmd("game_specialsget", game_specialsget, Priority::Low);
    agent.acmd("game_specialairsget", game_specialsget, Priority::Low);
    agent.acmd("effect_specialsget", effect_specialsget, Priority::Low);
    agent.acmd("effect_specialairsget", effect_specialsget, Priority::Low);
    agent.acmd("expression_specialsget", expression_specialsget, Priority::Low);
    agent.acmd("expression_specialairsget", expression_specialsget, Priority::Low);

    agent.acmd("game_specialhijump", game_specialhijump, Priority::Low);

    agent.acmd("game_specialhistartl", game_specialhistart, Priority::Low);
    agent.acmd("game_specialhistartr", game_specialhistart, Priority::Low);

    agent.acmd("game_specialhilandingr", game_specialhilanding, Priority::Low);
    agent.acmd("game_specialhilandingl", game_specialhilanding, Priority::Low);

    agent.acmd("game_specialairhiturnl", game_specialairhiturn, Priority::Low);
    agent.acmd("game_specialairhiturnr", game_specialairhiturn, Priority::Low);

    agent.acmd("game_specialhifailure", game_specialhifailure, Priority::Low);

    agent.acmd("game_speciallwstart", game_speciallwstart, Priority::Low);
    agent.acmd("game_specialairlwstart", game_speciallwstart, Priority::Low);
    agent.acmd("effect_speciallwstart", effect_speciallwstart, Priority::Low);
    agent.acmd("effect_specialairlwstart", effect_speciallwstart, Priority::Low);
    agent.acmd("sound_speciallwstart", sound_speciallwstart, Priority::Low);
    agent.acmd("sound_specialairlwstart", sound_speciallwstart, Priority::Low);
    agent.acmd("expression_speciallwstart",  expression_speciallwstart, Priority::Low);
    agent.acmd("expression_specialairlwstart", expression_speciallwstart, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw, Priority::Low);
    agent.acmd("sound_speciallw", sound_speciallw, Priority::Low);
    agent.acmd("sound_specialairlw", sound_speciallw, Priority::Low);
    agent.acmd("expression_speciallw", expression_speciallw, Priority::Low);
    agent.acmd("expression_specialairlw", expression_speciallw, Priority::Low);

    agent.acmd("game_speciallwmax", game_speciallwmax, Priority::Low);
    agent.acmd("game_specialairlwmax", game_speciallwmax, Priority::Low);

    agent.acmd("game_speciallwjumpsquat", game_speciallwjumpsquat, Priority::Low);
}