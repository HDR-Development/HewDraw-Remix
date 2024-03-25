use super::*;

unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot23"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot18"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot14"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot4"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
}

unsafe extern "C" fn game_landingairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

unsafe extern "C" fn effect_landingairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("richter_whip_light"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("richter_whip_flash_top"), true, true);
    }
}

unsafe extern "C" fn game_attackairfhi(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn game_attackairflw(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_straight"), Hash40::new("hookshot7"), -8, 0, 0, 0, -75, -90, 0.7, true);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot9"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 2);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot14"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(agent, 2);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot4"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("richter_whip_straight"), true, true);
    }
}

unsafe extern "C" fn game_landingairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.5);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1, false, 0.65);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn game_landingairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    

}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {

}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_attackairn", effect_attackairn);
    agent.acmd("game_landingairn", game_landingairn);
    agent.acmd("effect_landingairn", effect_landingairn);

    agent.acmd("game_attackairfhi", game_attackairfhi);
    agent.acmd("effect_attackairfhi", effect_attackairf);
    agent.acmd("game_attackairf", game_attackairf);
    agent.acmd("effect_attackairf", effect_attackairf);
    agent.acmd("game_attackairflw", game_attackairflw);
    agent.acmd("effect_attackairflw", effect_attackairf);
    agent.acmd("game_landingairf", game_landingairf);

    agent.acmd("game_attackairbhi", game_attackairb);
    agent.acmd("effect_attackairbhi", effect_attackairb);
    agent.acmd("game_attackairb", game_attackairb);
    agent.acmd("effect_attackairb", effect_attackairb);
    agent.acmd("game_attackairblw", game_attackairb);
    agent.acmd("effect_attackairblw", effect_attackairb);
    agent.acmd("game_landingairb", game_landingairb);

    agent.acmd("game_attackairhi", game_attackairhi);
    agent.acmd("effect_attackairhi", effect_attackairhi);
}