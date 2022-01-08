
use super::*;

#[acmd_script( agent = "kamui", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn kamui_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
	frame(lua_state, 4.0);
	if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.5);
    }
    frame(lua_state, 10.0); // Effectively F16
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        kamui_catch_game,
        kamui_dash_game,
        dash_effect,
        kamui_turn_dash_game,
    );
}

