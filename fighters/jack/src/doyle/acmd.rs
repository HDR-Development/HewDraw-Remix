use super::*;

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("jack_doyle_magic_flash"), Hash40::new("handl"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("jack_doyle_magic_flash2"), Hash40::new("top"), 2, 10, 11, 0, 0, 0, 0.07, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("jack_doyle_magic_flash2"), Hash40::new("handl"), 2, 0, 0, 0, 0, 0, 0.07, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("jack_doyle_magic_flash2"), Hash40::new("handl"), 2, 0, 0, 0, 0, 0, 0.07, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_attacklw3", effect_attacklw3);
}