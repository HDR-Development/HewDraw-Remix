use super::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("up"), hash40("off"));
        VisibilityModule::set_int64(boma, hash40("down"), hash40("off"));
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 11.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("down"), hash40("on"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("up"), hash40("on"));
        VisibilityModule::set_int64(boma, hash40("down"), hash40("off"));
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("up"), hash40("off"));
        VisibilityModule::set_int64(boma, hash40("down"), hash40("off"));
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 9.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("down"), hash40("on"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("up"), hash40("on"));
        VisibilityModule::set_int64(boma, hash40("down"), hash40("off"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialairhi, Priority::Low);
}
