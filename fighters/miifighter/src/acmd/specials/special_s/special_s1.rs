use super::*;

// ================================================================================================
// ======================================== ONSLAUGHT =============================================
// ================================================================================================

unsafe extern "C" fn game_specials1start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING_MUL_SPEED_X);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 14.0, 12.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_MOTION_SPEED_MUL);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 3.0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 60, 50, 130, 0, 4.0, 0.0, 7.0, -0.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT_CHECK_ONOFF);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT_CHECK_ONOFF);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 3.0);
    }
}

unsafe extern "C" fn game_specialairs1start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING_MUL_SPEED_X);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 14.0, 12.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_MOTION_SPEED_MUL);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 3.0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 75, 100, 60, 0, 4.0, 0.0, 5.5, -0.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT_CHECK_ONOFF);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_GRAVITY_ONOFF);
        WorkModule::off_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT_CHECK_ONOFF);
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 3.0);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_specials1end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 70, 100, 11, 0, 7.0, 0.0, 8.5, 8.5, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 100, 40, 0, 6.0, 0.0, 7.5, 2.5, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 16, 0, 7.5, 0.0, 9.0, 9.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 100, 40, 0, 6.0, 0.0, 8.0, 3.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 50, 0, 6.5, 0.0, 9.0, 9.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 100, 40, 0, 5.0, 0.0, 8.0, 3.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 50, 0, 6.5, 0.0, 9.0, 9.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 100, 40, 0, 5.0, 0.0, 8.0, 3.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_DISABLE_OPPONENT_PASSIVE);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 60, 70, 40, 30, 6.5, 0.0, 8.0, 8.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 60, 70, 40, 30, 5.0, 0.0, 7.0, 2.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 8.0);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 8.0);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_DISABLE_OPPONENT_PASSIVE);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("toer"), 5.5, 90, 184, 0, 58, 7.0, 0.0, 0.0, 0.0, Some(-6.0), Some(-2.0), Some(1.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.5, 90, 184, 0, 58, 7.0, 0.0, 10.0, 8.0, Some(0.0), Some(9.0), Some(12.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specials1end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miifighter_hyakuretsukick_line"), Hash40::new("top"), -2, 8, -8, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("miifighter_hyakuretsukick_arc_02"), Hash40::new("miifighter_hyakuretsukick_arc_02"), Hash40::new("top"), -1, 8, 2, 17, -7, -170, 0.6, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("miifighter_hyakuretsukick_arc_02"), Hash40::new("miifighter_hyakuretsukick_arc_02"), Hash40::new("top"), -1, 8, 2, -2, 10, -154, 0.55, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("miifighter_hyakuretsukick_arc_02"), Hash40::new("miifighter_hyakuretsukick_arc_02"), Hash40::new("top"), -1, 10, 2, 6.5, -28, 265, 0.6, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.8);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("miifighter_hyakuretsukick_arc_02"), Hash40::new("miifighter_hyakuretsukick_arc_02"), Hash40::new("top"), -1, 8, 2, -12, -2, -267, 0.55, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.8);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("miifighter_hyakuretsukick_arc_02"), Hash40::new("miifighter_hyakuretsukick_arc_02"), Hash40::new("top"), -1, 8, 2, 19, -45, -102.7, 0.55, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.8);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("miifighter_hyakuretsukick_arc_02"), Hash40::new("miifighter_hyakuretsukick_arc_02"), Hash40::new("top"), -1, 10, 2, 30, 5, -90, 0.6, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_hyakuretsukick"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("miifighter_hyakuretsukick_arc"), Hash40::new("miifighter_hyakuretsukick_arc"), Hash40::new("top"), -1, 13, 5, 0, 0, 90, 0.7, true, *EF_FLIP_YZ);
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("miifighter_hyakuretsukick_arc"), Hash40::new("miifighter_hyakuretsukick_arc"), Hash40::new("top"), -1, 8, 4, 0, 0, 90, 0.7, true, *EF_FLIP_YZ);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miifighter_hyakuretsukick"), false, false);
    }
}

unsafe extern "C" fn game_specialairs1end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        SET_SPEED_EX(agent, 1, 1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING_MUL_SPEED_X);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING_MUL_SPEED_X);
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 30, 30, 7.0, 0.0, 8.0, 6.3, Some(0.0), Some(5.5), Some(6.3), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 100, 40, 0, 6.0, 0.0, 6.75, 0.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0.3, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 16, 0, 6.5, 0.0, 8.0, 6.0, Some(0.0), Some(5.5), Some(6.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 100, 40, 0, 6.0, 0.0, 6.75, 0.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0.3, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 50, 0, 6.5, 0.0, 9.0, 9.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 100, 40, 0, 5.0, 0.0, 6.75, 0.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f{x: 0.3, y: 0.5, z: 0.0});
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 50, 0, 6.5, 0.0, 9.0, 9.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 100, 40, 0, 5.0, 0.0, 6.75, 0.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f{x: -0.3, y: 0.7, z: 0.0});
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 60, 70, 40, 30, 6.5, 0.0, 8.0, 8.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 60, 70, 40, 30, 5.0, 0.0, 6.75, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 8.0);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 8.0);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AttackModule::clear_inflict_kind_status(boma);
        KineticModule::add_speed(boma, &Vector3f{x: 0.85, y: 3.0, z: 0.0});
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.5, 85, 158, 0, 58, 7.0, 0.0, 16.0, 9.5, Some(0.0), Some(10.0), Some(8.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 5, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.5, 85, 158, 0, 58, 7.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(8.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 5, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_CONTROL_X);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials1start", game_specials1start, Priority::Low);
    agent.acmd("game_specialairs1start", game_specialairs1start, Priority::Low);
    agent.acmd("game_specials1end", game_specials1end, Priority::Low);
    agent.acmd("game_specialairs1end", game_specialairs1end, Priority::Low);
    agent.acmd("effect_specials1end", effect_specials1end, Priority::Low);
    agent.acmd("effect_specialairs1end", effect_specials1end, Priority::Low);
}