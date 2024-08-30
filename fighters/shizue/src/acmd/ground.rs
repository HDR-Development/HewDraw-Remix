use super::*;

unsafe extern "C" fn sound_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_attackdash_01"));
        SHIZUE_VC_SEQUENCE_DAMAGE(agent);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_attackdash", sound_attackdash, Priority::Low);
}
