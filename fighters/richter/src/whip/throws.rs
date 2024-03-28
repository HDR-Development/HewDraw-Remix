use super::*;

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn effect_throwhi(agent: &mut L2CAgentBase) {
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_throwhi", game_throwhi);
    agent.acmd("effect_throwhi", effect_throwhi);
}