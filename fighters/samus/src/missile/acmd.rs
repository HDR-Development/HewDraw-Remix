use super::*;

unsafe extern "C" fn effect_homing(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("samus_missile_homing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn effect_hburst(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_bomb_a"), Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.68, 0, 0, 0, 0, 0, 0, true);
        sv_animcmd::EFFECT_BRANCH_SITUATION(agent.lua_state_agent);
        agent.clear_lua_stack();
        LAST_EFFECT_SET_RATE(agent, 1.33);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_homing", effect_homing, Priority::Low);
    agent.acmd("effect_hburst", effect_hburst, Priority::Low);
}