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
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_flame1"), Hash40::new("tex_roy_flame2"), 7, Hash40::new("havel"), 0.0, -0.8, 0.0, Hash40::new("havel"), 0.0, 14.5, 0.0, true, Hash40::new("null"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
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

unsafe extern "C" fn expression_royspecialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        //AREA_WIND_2ND_arg10(fighter, 0, 2, 110, 300, 0.6, 0, 12, 30, 30, 40);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5);
        RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

unsafe extern "C" fn effect_royspecialnend2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_erupution_hold"), Hash40::new("roy_erupution_hold"), Hash40::new("havel"), 0, 0, 0, -90, 90, 0, 0.7, true, *EF_FLIP_NONE);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_sword"), Hash40::new("roy_sword"), Hash40::new("havel"), 0, 0, 0, -90, 90, 0, 1, true, *EF_FLIP_NONE);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_flame1"), Hash40::new("tex_roy_flame2"), 7, Hash40::new("havel"), 0.0, 0.0, -0.8, Hash40::new("havel"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("roy_eruption_middle"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("roy_eruption_bomb_main"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("roy_eruption_bomb_start_middle"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 6);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 26.0);
    frame(lua_state, 38.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_sword"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
    }
}

unsafe extern "C" fn effect_royspecialnend3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_erupution_hold"), Hash40::new("roy_erupution_hold"), Hash40::new("havel"), 0, 0, 0, -90, 90, 0, 1, true, *EF_FLIP_NONE);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_sword"), Hash40::new("roy_sword"), Hash40::new("havel"), 0, 0, 0, -90, 90, 0, 1, true, *EF_FLIP_NONE);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_flame1"), Hash40::new("tex_roy_flame2"), 7, Hash40::new("havel"), 0.0, 0.0, -0.8, Hash40::new("havel"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
        EFFECT(agent, Hash40::new("roy_eruption"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("roy_eruption_bomb_main"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT(agent, Hash40::new("roy_eruption_bomb_start_high"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 6);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 26.0);
    frame(lua_state, 38.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_sword"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
    }
}

unsafe extern "C" fn effect_royspecialnendmax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_erupution_hold"), Hash40::new("roy_erupution_hold"), Hash40::new("havel"), 0, 0, 0, -90, 90, 0, 1.3, true, *EF_FLIP_NONE);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_sword"), Hash40::new("roy_sword"), Hash40::new("havel"), 0, 0, 0, -90, 90, 0, 1, true, *EF_FLIP_NONE);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_flame1"), Hash40::new("tex_roy_flame2"), 7, Hash40::new("havel"), 0.0, 0.0, -0.8, Hash40::new("havel"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
        EFFECT(agent, Hash40::new("roy_eruption_max"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 2.1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT(agent, Hash40::new("roy_eruption_bomb_main"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT(agent, Hash40::new("roy_eruption_bomb_start_max"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 6);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 26.0);
    frame(lua_state, 46.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_sword"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_royspecialnend", effect_royspecialnend, Priority::Low);
    agent.acmd("effect_royspecialairnend", effect_royspecialnend, Priority::Low);
    agent.acmd("sound_royspecialnend", sound_royspecialnend, Priority::Low);
    agent.acmd("sound_royspecialairnend", sound_royspecialnend, Priority::Low);
    agent.acmd("expression_royspecialnend", expression_royspecialnend, Priority::Low);
    agent.acmd("expression_royspecialairnend", expression_royspecialnend, Priority::Low);

    agent.acmd("effect_royspecialnend2", effect_royspecialnend2, Priority::Low);
    agent.acmd("effect_royspecialairnend2", effect_royspecialnend2, Priority::Low);
    agent.acmd("effect_royspecialnend3", effect_royspecialnend3, Priority::Low);
    agent.acmd("effect_royspecialairnend3", effect_royspecialnend3, Priority::Low);
    agent.acmd("effect_royspecialnendmax", effect_royspecialnendmax, Priority::Low);
    agent.acmd("effect_royspecialairnendmax", effect_royspecialnendmax, Priority::Low);
}