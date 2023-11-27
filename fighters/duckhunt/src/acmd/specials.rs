
use super::*;

#[acmd_script( agent = "duckhunt", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_REQUEST_SPECIAL_HI_CANCEL);
    }
    
}

#[acmd_script( agent = "duckhunt" , scripts = ["game_specialairlw", "game_speciallw"], category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state: u64 = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 5.0/(4.0-1.0));
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_STATUS_SPECIAL_LW_FLAG_CALL_TRIGGER);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 1.15);
}

#[acmd_script( agent = "duckhunt" , scripts = ["game_specialairn", "game_specialn"], category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state: u64 = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(fighter, 16.0, 42.0, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_RELEASE_CAN);
    }
    frame(lua_state, 42.0);
    FT_MOTION_RATE(fighter, 1.0);

}

#[acmd_script( agent = "duckhunt", scripts = ["game_specials", "game_specialairs"], category = ACMD_GAME, low_priority )]
unsafe fn duckhunt_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 14.0, 18.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE_RANGE(fighter, 14.0, 50.0, 41.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_RELEASE_CLAY);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
    frame(lua_state, 50.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}

pub fn install() {
    install_acmd_scripts!(
        duckhunt_special_hi_game,
        duckhunt_special_n_game,
        duckhunt_special_lw_game,
        duckhunt_special_s_game,
    );
}
