use super::*;

unsafe extern "C" fn game_start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 13.0/(21.0 - 1.0));
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_start", game_start);
}
