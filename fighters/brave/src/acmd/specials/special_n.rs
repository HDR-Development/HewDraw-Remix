use super::*;

unsafe extern "C" fn game_specialn1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
            ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 44.0, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_REVERT_FALL_SPEED);
    }
    frame(lua_state, 44.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 8.0, 6.0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_HOP);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
            ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE_RANGE(agent, 26.0, 52.0, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(lua_state, 52.0);
    FT_MOTION_RATE(agent, 1.0);

}

unsafe extern "C" fn game_specialn3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 16.0, 19.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 9.0, 6.0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
            ArticleModule::generate_article(boma, *FIGHTER_BRAVE_GENERATE_ARTICLE_FIREBALL, false, 0);
        }    
    }
    frame(lua_state, 38.0);
    FT_MOTION_RATE_RANGE(agent, 38.0, 68.0, 20.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 4.0);
    }
    frame(lua_state, 68.0);
    FT_MOTION_RATE(agent, 1.0);

}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn1", game_specialn1, Priority::Low);
    agent.acmd("game_specialairn1", game_specialn1, Priority::Low);

    agent.acmd("game_specialn2", game_specialn2, Priority::Low);
    agent.acmd("game_specialair2", game_specialn2, Priority::Low);

    agent.acmd("game_specialn3", game_specialn3, Priority::Low);
    agent.acmd("game_specialairn3", game_specialn3, Priority::Low);
}