use super::*;

unsafe extern "C" fn null(agent: &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairb", null);
}
