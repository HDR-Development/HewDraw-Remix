use super::*;

unsafe extern "C" fn game_passive(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
} 

unsafe extern "C" fn effect_passive(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, false);
    }
} 

unsafe extern "C" fn sound_passive(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swim_middle_01"));
    }
} 

pub fn install(agent: &mut Agent) {
    agent.acmd("game_passive", game_passive);
    agent.acmd("effect_passive", effect_passive);
    agent.acmd("sound_passive", sound_passive);
}
