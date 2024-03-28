use super::*;

unsafe extern "C" fn game_guardon(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_guardon", game_guardon);
}