use super::*;

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", acmd_stub, Priority::Low);

    agent.acmd("game_explosion", acmd_stub, Priority::Low);
}