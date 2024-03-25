use super::*;
unsafe extern "C" fn game_burn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AREA_WIND_2ND_RAD_arg9(agent, 0, 1, 0.05, 200, 0.8, 0, 9, 15, 60);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 120, 100, 15, 0, 3.8, 0.0, 8.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.5, 0.0, 8, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 115, 100, 15, 0, 4.0, 0.0, 9.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.5, 0.0, 8, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 110, 100, 15, 0, 4.0, 0.0, 9.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.5, 0.0, 8, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
}
unsafe extern "C" fn game_rise(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        AREA_WIND_2ND_RAD_arg9(agent, 0, 1, 0.05, 200, 1, 0, 5, 12, 60);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 96, 100, 15, 0, 3.4, 0.0, 7.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.5, 0.0, 4, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 96, 100, 15, 0, 3.6, 0.0, 7.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.5, 0.0, 4, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
}
unsafe extern "C" fn effect_rise(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("reflet_gigafire_firepillar"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.45);
    }
}
pub fn install(agent: &mut Agent) {
    agent.acmd("game_burn", game_burn);
    agent.acmd("game_rise", game_rise);
    agent.acmd("effect_rise", effect_rise);
}
