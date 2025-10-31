use super::*;

unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("toonlink_boomerang_t_hdr"), Hash40::new("all"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(agent, Hash40::new("toonlink_boomerang"), Hash40::new("all"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

unsafe extern "C" fn effect_haved(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("toonlink_boomerang_t_hdr"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("toonlink_boomerang"), false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_fly", effect_fly, Priority::Low);
    agent.acmd("effect_haved", effect_haved, Priority::Low);
}
