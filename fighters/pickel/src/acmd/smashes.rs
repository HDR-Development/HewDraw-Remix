use super::*;

// shrinks the magma block effects to match its reduced size

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -6, 9, -6, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pickel_block_magma_heat"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("null"), Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        EFFECT_OFF_KIND(agent, Hash40::new("pickel_block_magma_heat"), false, true);
        EFFECT(agent, Hash40::new("pickel_block_break_magma"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_attackhi4", effect_attackhi4);
}
