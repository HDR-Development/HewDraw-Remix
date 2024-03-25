use super::*;

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairhi", game_attackairhi);
}