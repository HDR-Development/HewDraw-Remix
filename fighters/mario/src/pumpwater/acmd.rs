use super::*;

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", acmd_stub, Priority::Low);
    agent.acmd("effect_regular", acmd_stub, Priority::Low);
    agent.acmd("sound_regular", acmd_stub, Priority::Low);
    agent.acmd("effect_hit", acmd_stub, Priority::Low);
    agent.acmd("effect_clash", acmd_stub, Priority::Low);
}