use super::*;


#[acmd_script( agent = "trail", script = "effect_specialssearch" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialssearch(fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "trail", script = "effect_specialairssearch" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialairssearch(fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "trail", script = 0x15bfed9702 , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialsturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("trail_sonic_turn"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "trail", script = 0x1152d69c4b , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialsup(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("trail_sonic_turn"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "trail", script = "effect_specialsdown" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialsdown(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("trail_sonic_turn"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "trail", script = 0x18f6f0b024 , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialairsturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("trail_sonic_turn"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "trail", script = 0x1424373288 , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialairsup(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("trail_sonic_turn"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "trail", script = "effect_specialairsdown" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialairsdown(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("trail_sonic_turn"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        effect_specialssearch,
        effect_specialairssearch,
        effect_specialsturn,
        effect_specialsup,
        effect_specialsdown,
        effect_specialairsturn,
        effect_specialairsup,
        effect_specialairsdown
    );
}