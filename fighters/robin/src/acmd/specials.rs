
use super::*;


#[acmd_script( agent = "reflet", script = "game_specialntronstart" , category = ACMD_GAME , low_priority)]
unsafe fn reflet_special_n_tron_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 15.0/(19.0-1.0));
    frame(lua_state, 19.0);
    FT_MOTION_RATE(fighter, 1.0);
}



#[acmd_script( agent = "reflet", script = "game_specialairntronstart" , category = ACMD_GAME , low_priority)]
unsafe fn reflet_special_air_n_tron_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 15.0/(19.0-1.0));
    frame(lua_state, 19.0);
    FT_MOTION_RATE(fighter, 1.0);
}



#[acmd_script( agent = "reflet", script = "game_specialntronend" , category = ACMD_GAME , low_priority)]
unsafe fn reflet_special_n_tron_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.7); //FAF is frame 61
    }
}


#[acmd_script( agent = "reflet", script = "game_specialairntronend" , category = ACMD_GAME , low_priority)]
unsafe fn reflet_special_air_n_tron_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.35); //FAF is frame 63
    }
}


#[acmd_script( agent = "reflet", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn reflet_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
        }
        else{
            MotionModule::set_rate(boma, 2.0);
        }
    }
    wait(lua_state, 1.0);
    for _ in 0..30 {
        if is_excute(fighter) {
            if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
                WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
            }
        }
        wait(lua_state, 1.0);
    }
    
}


#[acmd_script( agent = "reflet", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn reflet_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
        }
        else{
            MotionModule::set_rate(boma, 2.0);
        }
    }
    wait(lua_state, 1.0);
    for _ in 0..30 {
        if is_excute(fighter) {
            if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
                WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
            }
        }
        wait(lua_state, 1.0);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        reflet_special_n_tron_start_game,
        reflet_special_air_n_tron_start_game,
        reflet_special_n_tron_end_game,
        reflet_special_air_n_tron_end_game,
        reflet_special_hi_game,
        reflet_special_air_hi_game,
    );
}

