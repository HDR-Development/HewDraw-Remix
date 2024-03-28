use super::*;

unsafe extern "C" fn effect_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("vacuum"), 4.0, 0.0, 0.0, 0, 90, 90, 0.6, true);
        LAST_EFFECT_SET_RATE(agent, 0.75);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_catch", effect_catch);
}
