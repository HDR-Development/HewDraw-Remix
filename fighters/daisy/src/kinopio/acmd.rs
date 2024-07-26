use super::*;

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_catchattack", acmd_stub, Priority::Low);
    agent.acmd("effect_catchwait", acmd_stub, Priority::Low);
}