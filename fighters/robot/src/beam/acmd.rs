use super::*;

unsafe extern "C" fn effect_flymax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("robot_robobeam_l"), Hash40::new("top"), 0, 0, 2.5, 0, 0, 0, 0.65, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_flymax", effect_flymax);
}
