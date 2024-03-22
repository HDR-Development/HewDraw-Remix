use super::*;

unsafe extern "C" fn game_attack_air_hi(agent: &mut L2CAgentBase) {
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairhi", game_attack_air_hi);
}