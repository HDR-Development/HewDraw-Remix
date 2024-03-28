use super::*;

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_FLOWERPOT, false, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ArticleModule::shoot(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_FLOWERPOT, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 30.0/(44.0-10.0));
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackdash", game_attackdash);
}
