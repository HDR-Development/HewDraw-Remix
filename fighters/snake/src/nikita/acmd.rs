use super::*;

unsafe extern "C" fn stub(agent : &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_start", stub);
    agent.acmd("sound_shoot", stub);
}