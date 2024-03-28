use super::*;

unsafe extern "C" fn game_final(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	let pos2dim = Vector3f {x: 0.0, y: 40.0, z: 0.0};
	PostureModule::set_pos(boma, &pos2dim);
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_final", game_final);
}