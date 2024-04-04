use super::*;

unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_air_bullet"), Hash40::new("top"), 0, -0.5, 0.5, 0, 180, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_fly", effect_fly);
}