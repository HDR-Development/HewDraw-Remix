use super::*;

unsafe extern "C" fn game_edgespecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 20.0, 32.0, 8.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_S, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 32.0);
    FT_MOTION_RATE_RANGE(agent, 32.0, 79.0, 51.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_M, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 79.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 99.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 100.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_L, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 105.0);
    FT_MOTION_RATE(agent, 1.6);
    frame(lua_state, 115.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 120.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, -1);
    }
    frame(lua_state, 140.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_edgespecialnstart", game_edgespecialnstart, Priority::Low);
    agent.acmd("game_edgespecialairnstart", game_edgespecialnstart, Priority::Low);
}