use super::*;

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_light", acmd_stub, Priority::Low);
}