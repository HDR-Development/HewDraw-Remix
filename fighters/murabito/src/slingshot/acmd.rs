use super::*;
unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
    
    }    
    
}
pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairb", game_attackairb);
}
