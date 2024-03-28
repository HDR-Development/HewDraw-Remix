use super::*;

unsafe extern "C" fn game_ready(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 100, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
		AttackModule::enable_safe_pos(boma);
	}
}

unsafe extern "C" fn game_straight(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 100, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
	}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_ready", game_ready);
    agent.acmd("game_straight", game_straight);
}
