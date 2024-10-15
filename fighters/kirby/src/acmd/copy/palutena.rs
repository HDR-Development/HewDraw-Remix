use super::*;

unsafe extern "C" fn effect_palutenaspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("havel"), 0, 6.5, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("havel"), 0, 6.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -1, 21, 1, 0, 90, 0, 1, true, 0.7);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

unsafe extern "C" fn sound_palutenaspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_palutena_special_s01"));
    }
    wait(lua_state, 23.0);
    if is_excute(agent) {
        let rand = sv_math::rand(hash40("fighter"), 4) as i32;
        match rand {
            0 => PLAY_SE(agent, Hash40::new("vc_kirby_attack02")),
            1 => PLAY_SE(agent, Hash40::new("vc_kirby_attack04")),
            _ => PLAY_SE(agent, Hash40::new("vc_kirby_copy_palutena_03")),
        }
    }
}

unsafe extern "C" fn expression_palutenaspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_palutenaspecialn", effect_palutenaspecialn, Priority::Low);
    agent.acmd("effect_palutenaspecialairn", effect_palutenaspecialn, Priority::Low);
    agent.acmd("sound_palutenaspecialn", sound_palutenaspecialn, Priority::Low);
    agent.acmd("sound_palutenaspecialairn", sound_palutenaspecialn, Priority::Low);
    agent.acmd("expression_palutenaspecialn", expression_palutenaspecialn, Priority::Low);
    agent.acmd("expression_palutenaspecialairn", expression_palutenaspecialn, Priority::Low);
}