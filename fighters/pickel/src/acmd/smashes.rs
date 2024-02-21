
use super::*;

// shrinks the magma block effects to match its reduced size

unsafe extern "C" fn effect_attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -6, 9, -6, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pickel_block_magma_heat"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("null"), Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        EFFECT_OFF_KIND(fighter, Hash40::new("pickel_block_magma_heat"), false, true);
        EFFECT(fighter, Hash40::new("pickel_block_break_magma"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}




pub fn install() {
    smashline::Agent::new("pickel")
        .acmd("effect_attackhi4", effect_attackhi4)
        .install();
}
