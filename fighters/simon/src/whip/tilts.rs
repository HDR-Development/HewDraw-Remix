use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_MOVE);
    }
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3);
}