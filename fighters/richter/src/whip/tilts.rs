use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_NONE);
        // agent.clear_lua_stack();
        // let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        // if !object.is_null() {
        //     WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(object as *mut smash::app::Weapon);
        // }
    }
}

unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_light"), Hash40::new("hookshot6"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot26"), 2, 0, 0, 0, 0, 0, 1.5, true);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_NONE);
        // agent.clear_lua_stack();
        // let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        // if !object.is_null() {
        //     WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(object as *mut smash::app::Weapon);
        // }
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

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3);
    agent.acmd("effect_attacks3", effect_attacks3);
    agent.acmd("game_attackhi3", game_attackhi3);
    agent.acmd("effect_attackhi3", effect_attackhi3);
}