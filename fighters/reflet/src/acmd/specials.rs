
use super::*;

unsafe extern "C" fn game_specialntronstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 15.0/(19.0-1.0));
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairntronstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 15.0/(19.0-1.0));
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialntronend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.7); //FAF is frame 61
}

unsafe extern "C" fn game_specialairntronend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.35); //FAF is frame 63
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
        }
        else{
            MotionModule::set_rate(boma, 2.0);
        }
    }
    wait(lua_state, 1.0);
    for _ in 0..30 {
        if is_excute(agent) {
            if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
                WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
            }
        }
        wait(lua_state, 1.0);
    }
    
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
        }
        else{
            MotionModule::set_rate(boma, 2.0);
        }
    }
    wait(lua_state, 1.0);
    for _ in 0..30 {
        if is_excute(agent) {
            if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
                WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
            }
        }
        wait(lua_state, 1.0);
    }
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialntronstart", game_specialntronstart);
    agent.acmd( "game_specialairntronstart",game_specialairntronstart);
    agent.acmd("game_specialntronend", game_specialntronend);
    agent.acmd("game_specialairntronend",game_specialairntronend);
    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("game_specialairhi", game_specialairhi);
}
