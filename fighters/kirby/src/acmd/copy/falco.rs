use super::*;

unsafe extern "C" fn sound_falcospecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_special_n02"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_falcospecialnstart", sound_falcospecialnstart);
    agent.acmd("sound_falcospecialairnstart", sound_falcospecialnstart);
}
