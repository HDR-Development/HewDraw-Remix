

unsafe extern "C" fn effect_wolfspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 3, Hash40::new("haver"), 0, -0.3, 3, Hash40::new("haver"), 0, 0.77, 6.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("wolf_bayonet"), false, false);
    }
    if sv_animcmd::get_value_float(agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wolf_blaster_shot"), Hash40::new("top"), 1, 9.35, 13.6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wolf_blaster_shot"), Hash40::new("top"), -1, 9.35, 13.6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 3, Hash40::new("haver"), 0, -0.3, 3, Hash40::new("haver"), 0, 0.77, 6.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("wolf_bayonet"), false, false);
    }
}

unsafe extern "C" fn effect_wolfspecialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 3, Hash40::new("haver"), 0, -0.3, 3, Hash40::new("haver"), 0, 0.77, 6.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(lua_state, 14.0);
    if sv_animcmd::get_value_float(agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wolf_blaster_shot"), Hash40::new("top"), 1, 9.35, 13.6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wolf_blaster_shot"), Hash40::new("top"), -1, 9.35, 13.6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 3, Hash40::new("haver"), 0, -0.3, 3, Hash40::new("haver"), 0, 0.77, 6.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("wolf_bayonet"), false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_wolfspecialn", effect_wolfspecialn);
    agent.acmd("effect_wolfspecialairn", effect_wolfspecialairn);
}
