use super::*;

unsafe extern "C" fn stub(agent: &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", stub, Priority::Low);
    agent.acmd("effect_regular", stub, Priority::Low);
    agent.acmd("sound_regular", stub, Priority::Low);
    agent.acmd("effect_hit", stub, Priority::Low);
    agent.acmd("effect_clash", stub, Priority::Low);
}