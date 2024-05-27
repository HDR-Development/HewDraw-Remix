use super::*;

unsafe extern "C" fn jethammer_special_lw_attack_effect(agent: &mut L2CAgentBase){
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

}

unsafe extern "C" fn jethammer_special_lw_attack_game(agent: &mut L2CAgentBase){
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        FT_MOTION_RATE(agent, 2.0);
    }
}

pub fn install(agent: &mut Agent){
    agent.acmd("effect_speciallwattack", jethammer_special_lw_attack_effect, Priority::Low);
    agent.acmd("game_speciallwattack", jethammer_special_lw_attack_game, Priority::Low);
}
