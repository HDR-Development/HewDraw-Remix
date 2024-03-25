use super::*;
unsafe extern "C" fn game_specialairs5(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
}
unsafe extern "C" fn effect_specialairs5(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("elight_photon_body_lihgt"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_photon_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -1);
        EFFECT(agent, Hash40::new("elight_photon_appear"), Hash40::new("throw"), 0, 0, -12.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_photon_speedline"), Hash40::new("throw"), 0, 0, 0, 0, -24, -43, 0.4, 0, 0, 0, 0, 0, 0, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword4"), Hash40::new("tex_elight_sword2"), 5, Hash40::new("sword1"), 0.0, 0.0, -0.08, Hash40::new("sword1"), 19.5, 0.0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        //EFFECT(fighter, Hash40::new("elight_photon_appear"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.7);
        //EFFECT(fighter, Hash40::new("elight_photon_start"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("elight_appeal_spark"), Hash40::new("rot"), 0, -1.5, 4.8, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.7);
        EFFECT_FOLLOW(agent, Hash40::new("elight_photon_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT(agent, Hash40::new("elight_photon_speedline"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("elight_photon_speedline"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("elight_photon_speedline"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("elight_photon_slash5"), Hash40::new("top"), 0, 0, -3, 0, 0, -2, 0.9, false);
        LAST_EFFECT_SET_RATE(agent, 1.7);
        EFFECT(agent, Hash40::new("elight_photon_slash5_light"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("elight_photon_slash5"), -1);
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_photon_vanish"), Hash40::new("throw"), 0, 0, -12.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        //EFFECT_OFF_KIND(fighter, Hash40::new("elight_photon_body_lihgt"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_photon_body_lihgt"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_photon_sword"), true, true);
    }
}
pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialairs5", game_specialairs5);
    agent.acmd("game_specials5", game_specialairs5);
    agent.acmd("effect_specialairs5", effect_specialairs5);
    agent.acmd("effect_specials5", effect_specialairs5);
}
