use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(boma) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_falco_rnd_futtobi01"), Hash40::new("seq_falco_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_falco_rnd_futtobi01"), Hash40::new("seq_falco_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn sound_damageflyroll(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(boma) {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_falco_rnd_futtobi01"), Hash40::new("seq_falco_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        PLAY_FLY_VOICE(agent, Hash40::new("seq_falco_rnd_futtobi01"), Hash40::new("seq_falco_rnd_futtobi02"));
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_falco_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_step_right_m"));
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_step_left_m"));
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn sound_passivewalljump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_jump02"));
        PLAY_SE(agent, Hash40::new("vc_falco_passive"));
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

unsafe extern "C" fn sound_escapeair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_escapeair"));
        PLAY_SE(agent, Hash40::new("vc_falco_cliffcatch"));
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

unsafe extern "C" fn sound_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("falco"));
        PLAY_SE(agent, Hash40::new("vc_falco_cliffcatch"));
    }
}

unsafe extern "C" fn sound_escapen(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_escapeair"));
        PLAY_SE(agent, Hash40::new("vc_falco_cliffcatch"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_damageflyhi", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflylw", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyn", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflytop", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyroll", sound_damageflyroll, Priority::Low);
    
    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("game_turndash", game_turndash, Priority::Low);

    agent.acmd("sound_passivewalljump", sound_passivewalljump, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("sound_escapeair", sound_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);
    agent.acmd("sound_escapeairslide", sound_escapeairslide, Priority::Low);
    agent.acmd("sound_escapen", sound_escapen, Priority::Low);
}
