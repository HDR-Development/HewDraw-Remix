use super::*;

// see pzenigame/specials.rs for game acmd

unsafe extern "C" fn effect_pzenigamespecialnshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false); 
        }
        if agent.lr() < 0.0 {
            EFFECT_FLW_POS(agent, Hash40::new("pzenigame_mizuteppo_shoot"), Hash40::new("body"), 0, 4, 0, 0, 0, -13, 0.8, true);
        }
        else {
            EFFECT_FLW_POS(agent, Hash40::new("pzenigame_mizuteppo_shoot"), Hash40::new("body"), 0, 4, 0, 0, 0, 13, 0.8, true);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pzenigame_mizuteppo_shoot"), false, false);
    }
}

unsafe extern "C" fn sound_pzenigamespecialnshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pzenigame_special_n03"));
        let rand = sv_math::rand(hash40("fighter"), 5);
        if rand == 1 {
            PLAY_SE(agent, Hash40::new("vc_kirby_copy_pzenigame_01"));
        }
        else {
            PLAY_SE(agent, Hash40::new("vc_kirby_attack03"));
        }
    }
}

unsafe extern "C" fn expression_pzenigamespecialnshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_waterjets"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_pzenigamespecialnshot", effect_pzenigamespecialnshot, Priority::Low);
    agent.acmd("effect_pzenigamespecialairnshot", effect_pzenigamespecialnshot, Priority::Low);
    agent.acmd("sound_pzenigamespecialnshot", sound_pzenigamespecialnshot, Priority::Low);
    agent.acmd("sound_pzenigamespecialairnshot", sound_pzenigamespecialnshot, Priority::Low);
    agent.acmd("expression_pzenigamespecialnshot", expression_pzenigamespecialnshot, Priority::Low);
    agent.acmd("expression_pzenigamespecialairnshot", expression_pzenigamespecialnshot, Priority::Low);
}