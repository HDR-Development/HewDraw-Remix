use super::*;

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackdash", acmd_stub, Priority::Low);
    agent.acmd("effect_attackdash", acmd_stub, Priority::Low);
    agent.acmd("sound_attackdash", acmd_stub, Priority::Low);
    agent.acmd("expression_attackdash", acmd_stub, Priority::Low);
}
