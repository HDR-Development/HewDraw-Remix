use super::*;

unsafe extern "C" fn game_specialhiopen(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("para3"), hash40("on"));
        VisibilityModule::set_int64(boma, hash40("para4"), hash40("on"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhiopen", game_specialhiopen, Priority::Low);
}
