use super::*;

// tilts

unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot6"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot26"), 2, 0, 0, 0, 0, 0, 1.5, true);
    }
}

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_straight"), Hash40::new("hookshot3"), -6, 0, 0, 180, 50, 90, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.8);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1, false, 0.65);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 2, 0, 0, 0, 0, 0, 1.5, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("richter_whip_straight"), true, true);
    }
}

// smashes

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot11"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.0);    
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot11"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);    
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot11"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);  
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.0);  
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("hookshot27"), 0, 3.6, -2.5, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 3.0);    
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_steam2"), Hash40::new("hookshot27"), 0, 0, 0, 0, 0, 0, 0.2, true);        
        LAST_EFFECT_SET_RATE(agent, 1.5);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 0.3, 0.3);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_steam2"), false, true);
    }
}

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot23"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot18"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot14"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot4"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot23"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot18"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot14"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot4"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
}

// aerials

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

unsafe extern "C" fn effect_landingairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("richter_whip_light"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("richter_whip_flash_top"), true, true);
    }
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

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.0);
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

// disable whip physics

unsafe extern "C" fn disable_physics(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

unsafe extern "C" fn stub(agent: &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    // ground
    agent.acmd("game_attack11", disable_physics);
    agent.acmd("game_attack12", disable_physics);
    agent.acmd("game_attack13", disable_physics);

    agent.acmd("game_attackdash", disable_physics);

    // tilts
    agent.acmd("game_attacks3", disable_physics);
    agent.acmd("effect_attacks3", effect_attacks3);
    
    agent.acmd("game_attackhi3", disable_physics);
    agent.acmd("effect_attackhi3", effect_attackhi3);

    // smashes
    agent.acmd("game_attacks4charge", disable_physics);
    agent.acmd("effect_attacks4", effect_attacks4);
    agent.acmd("game_attacks4hi", disable_physics);
    agent.acmd("effect_attacks4hi", effect_attacks4);
    agent.acmd("game_attacks4lw", disable_physics);
    agent.acmd("effect_attacks4lw", effect_attacks4);

    agent.acmd("game_attackhi4charge", disable_physics);
    agent.acmd("game_attackhi4", disable_physics);
    agent.acmd("effect_attackhi4", effect_attackhi4);
    
    agent.acmd("game_attacklw4", game_attacklw4);

    // aerials
    agent.acmd("effect_attackairn", effect_attackairn);
    agent.acmd("game_landingairn", disable_physics);
    agent.acmd("effect_landingairn", effect_landingairn);

    agent.acmd("game_attackairfhi", disable_physics);
    agent.acmd("effect_attackairfhi", effect_attackairf);
    agent.acmd("game_attackairf", disable_physics);
    agent.acmd("effect_attackairf", effect_attackairf);
    agent.acmd("game_attackairflw", disable_physics);
    agent.acmd("effect_attackairflw", effect_attackairf);
    agent.acmd("game_landingairf", disable_physics);

    agent.acmd("game_attackairbhi", game_attackairb);
    agent.acmd("effect_attackairbhi", effect_attackairb);
    agent.acmd("game_attackairb", game_attackairb);
    agent.acmd("effect_attackairb", effect_attackairb);
    agent.acmd("game_attackairblw", game_attackairb);
    agent.acmd("effect_attackairblw", effect_attackairb);
    agent.acmd("game_landingairb", disable_physics);

    agent.acmd("game_attackairhi", stub);
    agent.acmd("effect_attackairhi", stub);

    // specials
    agent.acmd("game_specials1", stub);
    agent.acmd("game_specialairs1", stub);

    // throws
    agent.acmd("game_throwhi", stub);
    agent.acmd("effect_throwhi", stub);

    // other
    agent.acmd("game_guardon", disable_physics);
}