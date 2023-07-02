
use super::*;

#[acmd_script( agent = "diddy", script = "game_specialairhistart" , category = ACMD_GAME , low_priority)]
unsafe fn diddy_special_air_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        KineticModule::clear_speed_all(boma);
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}


pub fn install() {
    install_acmd_scripts!(
        diddy_special_air_hi_start_game,
    );
}

