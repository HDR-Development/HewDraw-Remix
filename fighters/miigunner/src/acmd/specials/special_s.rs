use super::*;

// ================================================================================================
// ======================================== FLAME PILLAR ==========================================
// ================================================================================================

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE_RANGE(agent, 1.0, 21.0, 17.0);
	frame(lua_state, 21.0);
	FT_MOTION_RATE_RANGE(agent, 21.0, 64.0, 36.0);
	if is_excute(agent) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_FLAMEPILLAR, false, 0);
	}
}

// ================================================================================================
// ======================================== PULSE MISSILE =========================================
// ================================================================================================

unsafe extern "C" fn game_specials32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 23.0);
	if is_excute(agent) {
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON);
	}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials1", game_specials1);
    agent.acmd("game_specialairs1", game_specials1);

    agent.acmd("game_specials32", game_specials32);
    agent.acmd("game_specialairs32", game_specials32);
}