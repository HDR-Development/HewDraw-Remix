
use super::*;

#[acmd_script( agent = "brave", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.2);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0); // Effectively F15
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        brave_catch_game,
        dash_game,
        dash_effect,
        turn_dash_game,
    );
}

