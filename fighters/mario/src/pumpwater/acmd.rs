use super::*;

unsafe extern "C" fn game_regular(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn effect_regular(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn sound_regular(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn effect_hit(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn effect_clash(agent: &mut L2CAgentBase) {
    
}

pub fn install(agent: &mut Agent) {
    agent.game_acmd("game_regular", game_regular);
    agent.effect_acmd("effect_regular", effect_regular);
    agent.sound_acmd("sound_regular", sound_regular);
    agent.effect_acmd("effect_hit", effect_hit);
    agent.effect_acmd("effect_clash", effect_clash);
}