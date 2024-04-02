use super::*;

unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 62, 80, 0, 60, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
		AREA_WIND_2ND_RAD_arg9(agent, 0, 2, 0.05, 200, 1, 0, 0, 12, 60);
	}
	frame(lua_state, 6.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 20.0);
	if is_excute(agent) {
		AreaModule::erase_wind(boma, 0);
	}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_move", game_move);
}
