use super::*;

// see pfushigisou/specials.rs for game acmd

unsafe extern "C" fn sound_pfushigisouspecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 0.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pfushigisou_special_n01"));
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_copy_pfushigisou_01"));
    }
}

unsafe extern "C" fn effect_pfushigisouspecialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("pfushigisou_tanemg"), Hash40::new("top"), 0, 12, 1, 0, 0, 0, 1.3, true);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_ALPHA(agent, 0.8);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pfushigisou_leaf"), Hash40::new("top"), 0, 12, 1, 0, 0, -90, 1, 0, 0, 0, 0, 360, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("pfushigisou_tanemg"), Hash40::new("top"), 0, 12, 1, 0, 0, 0, 1.3, true);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_ALPHA(agent, 0.8);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_ALPHA(agent, 0.5);
        }
    }
}

unsafe extern "C" fn expression_pfushigisouspecialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_pfushigisouspecialnstart", acmd_stub, Priority::Low);
    agent.acmd("sound_pfushigisouspecialnstart", sound_pfushigisouspecialnstart, Priority::Low);
    agent.acmd("effect_pfushigisouspecialairnstart", acmd_stub, Priority::Low);
    agent.acmd("sound_pfushigisouspecialairnstart", sound_pfushigisouspecialnstart, Priority::Low);

    agent.acmd("game_pfushigisouspecialn", acmd_stub, Priority::Low);
    agent.acmd("game_pfushigisouspecialairn", acmd_stub, Priority::Low);

    agent.acmd("effect_pfushigisouspecialnend", effect_pfushigisouspecialnend, Priority::Low);
    agent.acmd("expression_pfushigisouspecialnend", expression_pfushigisouspecialnend, Priority::Low);
    agent.acmd("effect_pfushigisouspecialairnend", effect_pfushigisouspecialnend, Priority::Low);
    agent.acmd("expression_pfushigisouspecialairnend", expression_pfushigisouspecialnend, Priority::Low);
}