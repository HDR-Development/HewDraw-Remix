use super::*;
use super::*;

#   if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        master_catch_game,
        dash_game,
        dash_effect,
        turn_dash_game,
    );
}

