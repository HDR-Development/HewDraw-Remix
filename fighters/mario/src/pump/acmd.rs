use super::*;

unsafe extern "C" fn effect_light(agent: &mut L2CAgentBase) {
    
}

pub fn install(agent: &mut Agent) {
    agent.effect_acmd("effect_light", effect_light);
}