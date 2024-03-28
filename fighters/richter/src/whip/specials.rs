use super::*;

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials1", game_specials1);
    agent.acmd("game_specialairs1", game_specials1);
}