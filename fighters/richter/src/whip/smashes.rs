use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

unsafe extern "C" fn game_attacks4charge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

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

unsafe extern "C" fn game_attackhi4charge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
        // PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_MOVE);
        // agent.clear_lua_stack();
        // let object = sv_system::battle_object(lua_state) as *mut BattleObject;
        // if !object.is_null() {
        //     WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(object as *mut smash::app::Weapon);
        //     WeaponSpecializer_SimonWhip::set_node_fix_flag_list(object as *mut smash::app::Weapon, 4, 5, 10, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1);
        // }
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
        // PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_MOVE);
        // agent.clear_lua_stack();
        // let object = sv_system::battle_object(lua_state) as *mut BattleObject;
        // if !object.is_null() {
        //     WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(object as *mut smash::app::Weapon);
        //     WeaponSpecializer_SimonWhip::set_node_fix_flag_list(object as *mut smash::app::Weapon, 4, 5, 9, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1);
        // }
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

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4charge", game_attacks4charge);
    agent.acmd("effect_attacks4", effect_attacks4);
    agent.acmd("game_attacks4hi", game_attacks4);
    agent.acmd("effect_attacks4hi", effect_attacks4);
    agent.acmd("game_attacks4lw", game_attacks4);
    agent.acmd("effect_attacks4lw", effect_attacks4);

    agent.acmd("game_attackhi4charge", game_attackhi4charge);
    agent.acmd("game_attackhi4", game_attackhi4);
    agent.acmd("effect_attackhi4", effect_attackhi4);
    
    agent.acmd("game_attacklw4", game_attacklw4);
}