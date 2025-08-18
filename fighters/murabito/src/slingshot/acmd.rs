use super::*;

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairb", acmd_stub, Priority::Low);
}
