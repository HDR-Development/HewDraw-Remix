use super::*;

unsafe extern "C" fn game_special_hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("up") as i64, hash40("off") as i64);
        VisibilityModule::set_int64(boma, hash40("down") as i64, hash40("off") as i64);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 11.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("down") as i64, hash40("on") as i64);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("up") as i64, hash40("on") as i64);
        VisibilityModule::set_int64(boma, hash40("down") as i64, hash40("off") as i64);
    }
}

unsafe extern "C" fn game_special_air_hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("up") as i64, hash40("off") as i64);
        VisibilityModule::set_int64(boma, hash40("down") as i64, hash40("off") as i64);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 9.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("down") as i64, hash40("on") as i64);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("up") as i64, hash40("on") as i64);
        VisibilityModule::set_int64(boma, hash40("down") as i64, hash40("off") as i64);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi", game_special_hi);
    agent.acmd("game_specialairhi", game_special_air_hi);
}