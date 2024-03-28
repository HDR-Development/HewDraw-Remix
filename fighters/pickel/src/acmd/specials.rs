use super::*;

unsafe extern "C" fn sound_specialn1getgold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pickel_special_n02_iron"));
        PLAY_SE(agent, Hash40::new("se_pickel_special_n_item"));
        PLAY_SE(agent, Hash40::new("se_result_coin_silver"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_specialn1getgold", sound_specialn1getgold);
}
