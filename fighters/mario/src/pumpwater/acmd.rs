use super::*;

unsafe extern "C" fn null(agent: &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", null);
    agent.acmd("effect_regular", null);
    agent.acmd("sound_regular", null);
    agent.acmd("effect_hit", null);
    agent.acmd("effect_clash", null);
}