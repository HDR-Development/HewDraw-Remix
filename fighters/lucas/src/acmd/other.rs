
use super::*;

#[acmd_script( agent = "lucas", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "lucas", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_jumpaerialfront" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_jump_aerial_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.500);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.500);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.500);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.333);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.500);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.667);
    }

}

#[acmd_script( agent = "lucas", script = "game_jumpaerialback" , category = ACMD_GAME , low_priority)]
unsafe fn lucas_jump_aerial_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.500);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.500);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.500);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.333);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.500);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.667);
    }

}

pub fn install() {
    install_acmd_scripts!(
        dash_effect,
        //lucas_jump_aerial_f_game,
        //lucas_jump_aerial_b_game,
    );
}
