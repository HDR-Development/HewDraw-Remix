use super::*;

unsafe extern "C" fn game_lucariospecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.0, 2.0);
        ArticleModule::generate_article(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, false, -1);
    }
}

unsafe extern "C" fn game_lucariospecialncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if [
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_JUMP_SQUAT,
    ].contains(&agent.get_int(*FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS)) {
        FT_MOTION_RATE_RANGE(agent, 1.0, 9.0, 4.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_lucariospecialnstart", game_lucariospecialnstart, Priority::Low);
    agent.acmd("game_lucariospecialairnstart", game_lucariospecialnstart, Priority::Low);
    agent.acmd("game_lucariospecialncancel", game_lucariospecialncancel, Priority::Low);
}
