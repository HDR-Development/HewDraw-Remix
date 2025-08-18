use super::*;

unsafe extern "C" fn game_specialairsfstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            MotionModule::set_rate(boma, 1.5);
        } else {
            MotionModule::set_rate(boma, 1.0);
        }
    }
}

unsafe extern "C" fn game_specialsfstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            MotionModule::set_rate(boma, 1.3);
        } else {
            MotionModule::set_rate(boma, 0.75);
        }
    }
}

unsafe extern "C" fn game_specialsfattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 0.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
        if agent.is_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_S_WORK_FLAG_AIR_ATTACK) {
            if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                MotionModule::set_rate(boma, 1.5);
            }
        } else {
            if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                MotionModule::set_rate(boma, 1.2);
            } else {
                MotionModule::set_rate(boma, 1.5);
            }
        }
        if agent.is_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        }
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.3, 40, 72, 0, 55, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 0.74, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 14.2, 40, 67, 0, 65, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 0.74, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.7, 40, 78, 0, 52, 4.8, 0.0, 11.0, 11.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 0.74, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn game_specialairsfend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, false);
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            MotionModule::set_rate(boma, 1.0);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 0.8);
    }
}

unsafe extern "C" fn game_specialsfend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, false);
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            MotionModule::set_rate(boma, 1.2);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::dolly::status::INHERIT_FINAL_CANCEL_ON_END);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialairsfstart", game_specialairsfstart, Priority::Low);
    agent.acmd("game_specialsfstart", game_specialsfstart, Priority::Low);
    agent.acmd("game_specialsfattack", game_specialsfattack, Priority::Low);
    agent.acmd("game_specialairsfend", game_specialairsfend, Priority::Low);
    agent.acmd("game_specialsfend", game_specialsfend, Priority::Low);
}
