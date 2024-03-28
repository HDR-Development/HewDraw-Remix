use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(agent.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_pzenigame_rnd_futtobi01"), Hash40::new("seq_pzenigame_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_pzenigame_rnd_futtobi01"), Hash40::new("seq_pzenigame_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn sound_damageflyroll(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(agent.module_accessor) {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_pzenigame_rnd_futtobi01"), Hash40::new("seq_pzenigame_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        PLAY_FLY_VOICE(agent, Hash40::new("seq_pzenigame_rnd_futtobi01"), Hash40::new("seq_pzenigame_rnd_futtobi02"));
    }
}

unsafe extern "C" fn effect_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_dash"), Hash40::new("top"), 2.5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.56, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_pzenigame_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn effect_runbrake(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.56, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_runbrakel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.56, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_runbraker(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.56, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_turnrun(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_run"), Hash40::new("top"), 0, 0, 1.5, 0, 180, 0, 0.84, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
}

unsafe extern "C" fn effect_turnrunbrake(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_brake"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 0.56, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
}

unsafe extern "C" fn game_escapeair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));
    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_damageflyhi", sound_damagefly);
    agent.acmd("sound_damageflylw", sound_damagefly);
    agent.acmd("sound_damageflyn", sound_damagefly);
    agent.acmd("sound_damageflytop", sound_damagefly);
    agent.acmd("sound_damageflyroll", sound_damageflyroll);
    
    agent.acmd("effect_dash", effect_dash);
    agent.acmd("sound_dash", sound_dash);

    agent.acmd("effect_runbrake", effect_runbrake);
    agent.acmd("effect_runbrakel", effect_runbrakel);
    agent.acmd("effect_runbraker", effect_runbraker);
    agent.acmd("effect_turnrun", effect_turnrun);
    agent.acmd("effect_turnrunbrake", effect_turnrunbrake);

    agent.acmd("game_escapeair", game_escapeair);
    agent.acmd("game_escapeairslide", game_escapeairslide);
}
