use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        JostleModule::set_team(boma, 1);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        VarModule::set_int(agent.battle_object, vars::dolly::status::HIT_CANCEL_TIMER, 12);
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 48, 75, 0, 60, 4.0, 0.0, 10.0, 6.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 48, 75, 0, 60, 4.0, 0.0, 10.0, 13.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        JostleModule::set_team(boma, 0);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        agent.on_flag( *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        VarModule::set_int(agent.battle_object, vars::dolly::status::HIT_CANCEL_TIMER, 12);
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 85, 90, 0, 32, 5.0, 0.0, 12.0, 8.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 90, 83, 0, 32, 5.0, 0.0, 21.0, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        VarModule::set_int(agent.battle_object, vars::dolly::status::HIT_CANCEL_TIMER, 12);
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 32, 82, 0, 36, 3.5, 0.0, 5.5, 7.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 32, 82, 0, 36, 3.5, 0.0, 6.5, 11.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 32, 87, 0, 31, 3.5, 0.0, 7.5, 15.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        JostleModule::set_team(boma, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);
    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);
}