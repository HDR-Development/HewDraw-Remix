use super::*;

// ================================================================================================
// ======================================== FEINT JUMP ============================================
// ================================================================================================

unsafe extern "C" fn game_speciallw2start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 1.5, 3.0);
    frame(lua_state, 1.5);
    FT_MOTION_RATE_RANGE(agent, 1.5, 2.0, 1.0);
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 43.0, 36.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.5);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_KICK_ENABLE_LANDING);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_START_WAIT_INPUT);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_KICK_ENABLE);
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_REVERSE);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_WALL_JUMP_ENABLE);
        if VarModule::is_flag(agent.battle_object, vars::common::status::CHECK_HOLD_INPUT) {
            SEARCH(agent, 0, 0, Hash40::new("hip"), 3.5, 1.0, 0.0, 0.0, Some(-3.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
        }
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

unsafe extern "C" fn game_specialairlw2kick(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0, 0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        SET_SPEED_EX(agent, -2.15, -1.1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 10.0, 35, 75, 0, 65, 4.5, 0.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 60, 75, 0, 65, 4.5, 0.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE(agent, 1.5);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialairlw2kick(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_illusion_line"), Hash40::new("top"), 0, 0, -5, 140, 0, 0, 0.9, true);
    }
}

unsafe extern "C" fn game_specialairlw2autoattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 40, 0, 95, 4.0, 0.0, 0.0, -2.0, Some(0.0), Some(0.0), Some(3.5), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 80, 40, 0, 95, 3.5, 0.0, 5.0, -2.0, Some(0.0), Some(5.0), Some(3.5), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw2start", game_speciallw2start, Priority::Low);
    agent.acmd("game_specialairlw2start", game_speciallw2start, Priority::Low);

    agent.acmd("game_specialairlw2kick", game_specialairlw2kick, Priority::Low);
    agent.acmd("effect_specialairlw2kick", effect_specialairlw2kick, Priority::Low);
    agent.acmd("game_specialairlw2autoattack", game_specialairlw2autoattack, Priority::Low);
}