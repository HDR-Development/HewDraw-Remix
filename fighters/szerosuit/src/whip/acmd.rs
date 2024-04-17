use super::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 4.0/(8.0-1.0));
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, (28.0-1.0)/23.0);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
}

unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
}

unsafe extern "C" fn game_specials2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, (28.0-1.0)/23.0);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
}

unsafe extern "C" fn effect_specials2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("szero_whip_flash"), Hash40::new("plasmawhip1"), 1, 0, 0, 0, 0, 0, 1, true);
        EFFECT_OFF_KIND(agent, Hash40::new("szero_pwhip"), true, true);
        EFFECT_FLW_POS(agent, Hash40::new("szero_whip"), Hash40::new("attach"), 0, 0, 0, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip2"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip3"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip4"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip5"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip6"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip7"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip8"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("szero_gbeam_lightning"), false, true);
    }
    frame(lua_state, 54.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("szero_whip"), false, true);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("szero_whip_flash"), false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn);

    agent.acmd("game_specials", game_specials);
    agent.acmd("game_specialairs", game_specialairs);
    agent.acmd("game_specials2", game_specials2);
    agent.acmd("effect_specials2", effect_specials2);
}
