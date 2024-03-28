use super::*;

unsafe extern "C" fn effect_royspecialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_erupution_hold"), Hash40::new("roy_erupution_hold"), Hash40::new("havel"), 0.0, 0.0, 0.0, -90.0, 90.0, 0.0, 1.4, true, *EF_FLIP_NONE);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_sword"), Hash40::new("roy_sword"), Hash40::new("havel"), 0.0, 0.0, 0.0, -90.0, 90.0, 0.0, 1.0, true, *EF_FLIP_NONE);

        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_attack_fire"), Hash40::new("roy_attack_fire"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.25);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_fire"), Hash40::new("roy_fire"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.25);
        //AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), 7, Hash40::new("havel"), 0.0, 0.0, -0.8, Hash40::new("havel"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
        //EFFECT(agent, Hash40::new("roy_eruption_bomb_main"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        //LAST_EFFECT_SET_RATE(agent, 1.5);
        //EFFECT(agent, Hash40::new("roy_eruption_bomb_start"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 1.05, 0, 0, 0, 0, 0, 0, true);
        //LAST_EFFECT_SET_RATE(agent, 1.5);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_fire"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_attack_fire"), false, false);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_sword"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
    }

}

unsafe extern "C" fn sound_royspecialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_roy_special_n01"));
        PLAY_SE(agent, Hash40::new("se_roy_special_n02"));
        PLAY_SE(agent, Hash40::new("vc_kirby_copy_roy_02"));
        PLAY_SE(agent, Hash40::new("se_roy_attackl_s01"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_royspecialnend", sound_royspecialnend);
    agent.acmd("sound_royspecialairnend", sound_royspecialnend);
    agent.acmd("effect_royspecialnend", effect_royspecialnend);
    agent.acmd("effect_royspecialairnend", effect_royspecialnend);
}