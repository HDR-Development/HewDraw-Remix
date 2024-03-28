use super::*;

unsafe extern "C" fn sound_start(agent : &mut L2CAgentBase) {

}

unsafe extern "C" fn sound_shoot(agent : &mut L2CAgentBase) {

}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_start", sound_start);
    agent.acmd("sound_shoot", sound_shoot);
}