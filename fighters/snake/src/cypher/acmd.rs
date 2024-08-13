use super::*;

pub fn install(agent: &mut Agent) {
    agent.acmd("game_detach", acmd_stub, Priority::Low);
}