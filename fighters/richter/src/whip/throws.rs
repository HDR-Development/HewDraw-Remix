use super::*;

unsafe extern "C" fn richter_whip_throw_hi_game(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn richter_whip_throw_hi_effect(agent: &mut L2CAgentBase) {
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_throwhi", richter_whip_throw_hi_game);
    agent.acmd("effect_throwhi", richter_whip_throw_hi_effect);
}