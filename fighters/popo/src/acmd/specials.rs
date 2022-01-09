
use super::*;


#[acmd_script( agent = "popo", script = "game_specialn" , category = ACMD_GAME , low_priority)]
unsafe fn popo_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.556);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        ArticleModule::shoot_exist(boma, *FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    
}

#[acmd_script( agent = "popo", script = "game_specialairn" , category = ACMD_GAME , low_priority)]
unsafe fn popo_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.556);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        ArticleModule::shoot_exist(boma, *FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        popo_special_n_game,
        popo_special_air_n_game,
    );
}

