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

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    MotionModule::set_rate(boma, (12.0 - 1.0)/13.0);
    frame(lua_state, 12.0);
    MotionModule::set_rate(boma, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 30.0);
    MotionModule::set_rate(boma, (40.0 - 30.0)/8.0);
    frame(lua_state, 40.0);
    MotionModule::set_rate(boma, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_catch", effect_catch, Priority::Low);
    agent.acmd("game_throwhi", game_throwhi, Priority::Low);
}