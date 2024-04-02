use super::*;

unsafe extern "C" fn game_diddyspecialncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, -1);
        FT_MOTION_RATE(agent, 8.0/(31.0 - 1.0));
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_diddyspecialncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn sound_diddyspecialncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn expression_diddyspecialncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn game_diddyspecialairncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, -1);
        FT_MOTION_RATE(agent, 8.0/(35.0 - 1.0));
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_diddyspecialairncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn sound_diddyspecialairncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn expression_diddyspecialairncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

pub fn install(agent: &mut Agent) {
        agent.acmd("game_diddyspecialncancel", game_diddyspecialncancel);
        agent.acmd("effect_diddyspecialncancel", effect_diddyspecialncancel);
        agent.acmd("sound_diddyspecialncancel", sound_diddyspecialncancel);
        agent.acmd(
            "expression_diddyspecialncancel",
            expression_diddyspecialncancel,
        );
        agent.acmd(
            "game_diddyspecialairncancel",
            game_diddyspecialairncancel,
        );
        agent.acmd(
            "effect_diddyspecialairncancel",
            effect_diddyspecialairncancel,
        );
        agent.acmd(
            "sound_diddyspecialairncancel",
            sound_diddyspecialairncancel,
        );
        agent.acmd(
            "expression_diddyspecialairncancel",
            expression_diddyspecialairncancel,
        );
    }