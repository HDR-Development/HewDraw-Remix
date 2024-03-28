use super::*;

unsafe extern "C" fn stub(agent: &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", stub);
    agent.acmd("effect_regular", stub);
    agent.acmd("sound_regular", stub);
    agent.acmd("effect_hit", stub);
    agent.acmd("effect_clash", stub);
}