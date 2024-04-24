use super::*;

unsafe extern "C" fn stub(agent : &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_start", stub, Priority::Low);
    agent.acmd("sound_shoot", stub, Priority::Low);
}