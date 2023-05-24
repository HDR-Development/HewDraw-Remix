
use super::*;

#[acmd_script( agent = "duckhunt", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
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
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "duckhunt" , scripts = ["game_specialairn", "game_specialn"], category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state: u64 = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 12.0/(16.0-0.0));
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_RELEASE_CAN);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN, false, -1);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_RELEASE_CAN);
    }
    frame(lua_state, 17.0);
    FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    install_acmd_scripts!(
        duckhunt_special_hi_game,
        duckhunt_special_n_game,
        duckhunt_special_lw_game,
    );
}

