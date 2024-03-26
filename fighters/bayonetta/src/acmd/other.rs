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
            if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn sound_damageflyroll(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(agent.module_accessor) {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        PLAY_FLY_VOICE(agent, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_bayonetta_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_bayonetta_step_right_l"));
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn game_escapeair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, 4.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), false);
    }
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
    
    frame(lua_state, 4.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), false);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_escapen(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn game_escapef(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 19.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
}

unsafe extern "C" fn game_escapeb(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn game_appeallwl(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 175.0);
    if is_excute(agent) {
        StatusModule::change_status_request(agent.module_accessor, *FIGHTER_STATUS_KIND_SQUAT_WAIT, false);
    }
}

unsafe extern "C" fn game_appealsr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 144/(90-1));
}

unsafe extern "C" fn game_appealhir(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 210/(100-1));
}

unsafe extern "C" fn sound_appealhil  (agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	frame(lua_state, 1.0);
		if is_excute(agent) {
			agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("vc_bayonetta_appeal01"));
            sv_animcmd::PLAY_DAMAGESTOP(agent.lua_state_agent);
		}
		frame(lua_state, 4.0);
		if is_excute(agent) {
			PLAY_SE(agent, Hash40::new("se_bayonetta_appeal_h01"));
		}
		frame(lua_state, 8.0);
		if is_excute(agent) {
			PLAY_SE(agent, Hash40::new("se_bayonetta_appeal_h02"));
		}
		frame(lua_state, 25.0);
		if is_excute(agent) {
			PLAY_SE(agent, Hash40::new("se_bayonetta_step_right_s"));
		}
        frame(lua_state, 38.0);
        if is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("vc_bayonetta_appeal01_02"));
            sv_animcmd::PLAY_DAMAGESTOP(agent.lua_state_agent);
        }
		frame(lua_state, 41.0);
		if is_excute(agent) {
			PLAY_SE(agent, Hash40::new("se_bayonetta_appeal_h03"));
		}
		frame(lua_state, 45.0);
		if is_excute(agent) {
			PLAY_SE(agent, Hash40::new("se_bayonetta_step_right_m"));
		}
	}

    
unsafe extern "C" fn sound_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_win01"));
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_win01_02"));
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_win01_03"));
    }
    frame(agent.lua_state_agent, 67.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_bayonetta_win08"));
    }
    frame(agent.lua_state_agent, 90.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_step_right_m"));
        PLAY_SE(agent, Hash40::new("se_bayonetta_win01_04"));
    }
    frame(agent.lua_state_agent, 106.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_win01_05"));
    }
    frame(agent.lua_state_agent, 127.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_win01_06"));
    }
}

unsafe extern "C" fn sound_appeallwl(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_win02_02"));
    }
    frame(agent.lua_state_agent, 55.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_win02_03"));
    }
    frame(agent.lua_state_agent, 85.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_bayonetta_win09"));
    }
}

unsafe extern "C" fn effect_justshieldoff(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("bayonetta_batwithin"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 1, true);
        let rate = WorkModule::get_float(boma, *WEAPON_BAYONETTA_BAT_INSTANCE_WORK_ID_FLOAT_MOTION_RATE);
        LAST_EFFECT_SET_RATE(agent, rate);
    }
    // if agent.get_int(*WEAPON_BAYONETTA_BAT_INSTANCE_WORK_ID_INT_COSTUME_KIND) == *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 {
    //     if is_excute(agent) {
    //         EFFECT_FOLLOW(agent, Hash40::new("bayonetta_batwithin_bat"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 0.6, true);
    //     } 
    // } else {
    //     if is_excute(agent) {
    //         EFFECT_FOLLOW(agent, Hash40::new("bayonetta_batwithin_bat2"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 0.6, true);
    //     }
    // }
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("bayonetta_batwithin_change"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_justshieldoff(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_batwithin02"));
        let rng = app::sv_math::rand(hash40("fighter"), 6) as i32;
        if rng >= 2 {
            match rng {
                2 => PLAY_SE(agent, Hash40::new("vc_bayonetta_special_l02")),
                3 => PLAY_SE(agent, Hash40::new("vc_bayonetta_special_l03")),
                4 => PLAY_SE(agent, Hash40::new("vc_bayonetta_special_l04")),
                _ => PLAY_SE(agent, Hash40::new("vc_bayonetta_win09")),
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_damageflyhi", sound_damagefly);
    agent.acmd("sound_damageflylw", sound_damagefly);
    agent.acmd("sound_damageflyn", sound_damagefly);
    agent.acmd("sound_damageflytop", sound_damagefly);
    agent.acmd("sound_damageflyroll", sound_damageflyroll);
    
    agent.acmd("game_dash", game_dash);
    agent.acmd("sound_dash", sound_dash);
    agent.acmd("game_turndash", game_turndash);

    agent.acmd("game_escapeair", game_escapeair);
    agent.acmd("game_escapeairslide", game_escapeairslide);
    agent.acmd("game_escapen", game_escapen);
    agent.acmd("game_escapef", game_escapef);
    agent.acmd("game_escapeb", game_escapeb);

    agent.acmd("game_appeallwl", game_appeallwl);
    agent.acmd("game_appealsr", game_appealsr);
    agent.acmd("game_appealsl", game_appealsr);
    agent.acmd("game_appealhir", game_appealhir);
    agent.acmd("game_appealhil", game_appealhir);
    agent.acmd("sound_appealhir", sound_appealhil);
    agent.acmd("sound_appealhil", sound_appealhil);
    agent.acmd("sound_appeallwr", sound_appeallwr);
    agent.acmd("sound_appeallwl", sound_appeallwl);

    
    agent.acmd("effect_justshieldoff", effect_justshieldoff);
    agent.acmd("sound_justshieldoff", sound_justshieldoff);
}
