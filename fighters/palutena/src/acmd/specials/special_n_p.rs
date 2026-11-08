use super::*;

unsafe extern "C" fn game_specialnp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 18.0, 11.0);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, articles::palutena::METEOR, false, -1);
    }
}

unsafe extern "C" fn effect_specialnp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 0.025, 0.9);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2_grey"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 0.025, 0.9);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight_grey"), Hash40::new("top"), -1, 21, -1, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 0.025, 0.9);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_atk1"), Hash40::new("top"), 0, 40, 6, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_COLOR(agent, 1.875, 0.025, 0.875);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("palutena_wand_atk1"), 0);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace_grey"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2_grey"), false, false);
    }
}

unsafe extern "C" fn sound_specialnp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_attack"));
    }
}

unsafe extern "C" fn expression_specialnp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_beamm"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnp", game_specialnp, Priority::Low);
    agent.acmd("game_specialairnp", game_specialnp, Priority::Low);
    agent.acmd("effect_specialnp", effect_specialnp, Priority::Low);
    agent.acmd("effect_specialairnp", effect_specialnp, Priority::Low);
    agent.acmd("sound_specialnp", sound_specialnp, Priority::Low);
    agent.acmd("sound_specialairnp", sound_specialnp, Priority::Low);
    agent.acmd("expression_specialnp", expression_specialnp, Priority::Low);
    agent.acmd("expression_specialairnp", expression_specialnp, Priority::Low);
}