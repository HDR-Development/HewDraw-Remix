use super::*;

unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
	frame(lua_state, 5.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 65, 94, 0, 48, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
	}
	frame(lua_state, 8.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 9.0);
	if is_excute(agent) {
		//AREA_WIND_2ND_RAD(0, 1, 0.02, 1000, 1, 0, 0, 12);
	}
	frame(lua_state, 15.0);
	if is_excute(agent) {
		//AreaModule::erase_wind(boma, 0);
	}
    
}

unsafe extern "C" fn effect_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 5.0);
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
	}
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_move", game_move);
    agent.acmd("effect_move", effect_move);
}