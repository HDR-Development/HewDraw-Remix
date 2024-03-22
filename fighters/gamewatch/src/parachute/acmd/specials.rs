use super::*;

unsafe extern "C" fn game_special_hi_open(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("para3") as i64, hash40("on") as i64);
        VisibilityModule::set_int64(boma, hash40("para4") as i64, hash40("on") as i64);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhiopen", game_special_hi_open);
}