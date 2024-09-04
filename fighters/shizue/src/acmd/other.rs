use super::*;

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.4);
    }
	frame(lua_state, 11.0); // Effectively F15
    if is_excute(agent) {
		FT_MOTION_RATE(agent, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_shizue_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.2);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0); // Effectively F15
    if is_excute(agent) {
		FT_MOTION_RATE(agent, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
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

unsafe extern "C" fn sound_cliffcatch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_cliff_catch"));
        if sv_math::rand(hash40("fighter"), 2) == 0 {
            let handle = SoundModule::play_se(boma, Hash40::new("vc_shizue_cliffcatch"), true, false, false, false, app::enSEType(0));
            SoundModule::set_se_vol(boma, handle as i32, 0.75, 0);
        }
    }
}

unsafe extern "C" fn sound_cliffattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_dash_start"));
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_attackair_n01"));
        SHIZUE_VC_SEQUENCE_ATTACK(agent);
    }
}

unsafe extern "C" fn sound_ottotto(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if sv_math::rand(hash40("fighter"), 4) == 0 {
            let handle = SoundModule::play_se(boma, Hash40::new("vc_shizue_ottootto"), true, false, false, false, app::enSEType(0));
            SoundModule::set_se_vol(boma, handle as i32, 0.5, 0);
        }
    }
}

unsafe extern "C" fn sound_furafura(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        let handle = SoundModule::play_se(boma, Hash40::new("vc_shizue_furafura"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 0.6, 0);
    }
}


unsafe extern "C" fn sound_passive(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_blowaway_s"));
        STOP_SE(agent, Hash40::new("se_common_blowaway_m"));
        STOP_SE(agent, Hash40::new("se_common_blowaway_l"));
        let damage_vc: [&str;5] = [ // damage voice lines
            "vc_shizue_damage01",
            "vc_shizue_damage02",
            "vc_shizue_damage03",
            "vc_shizue_damagefly01",
            "vc_shizue_damagefly02"
        ];
        for clip in damage_vc { STOP_SE(agent, Hash40::new(clip)) };
        PLAY_LANDING_SE(agent, Hash40::new("se_shizue_landing01"));
        let handle = SoundModule::play_se(boma, Hash40::new("vc_shizue_passive"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 0.75, 0);
    }
}

unsafe extern "C" fn sound_appeals(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        let handle = SoundModule::play_se(boma, Hash40::new("vc_shizue_appeal_s01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 0.75, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_appeal_s01"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_appeal_s01"));
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_appeal_s01"));
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_appeal_s01"));
    }
}

unsafe extern "C" fn sound_appealhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_appeal_h01"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        let handle = SoundModule::play_se(boma, Hash40::new("vc_shizue_appeal_h01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 0.75, 0);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_appeal_h01"));
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_appeal_h01"));
    }
}

unsafe extern "C" fn sound_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_appeal_l01"));
        PLAY_SE(agent, Hash40::new("se_shizue_step_left_s"));
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_step_right_s"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_step_left_s"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_step_right_s"));
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_step_left_s"));
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_step_right_s"));
    }
    frame(lua_state, 49.0);
    if is_excute(agent) {
        let handle = SoundModule::play_se(boma, Hash40::new("vc_shizue_appeal_l01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 0.75, 0);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shizue_appeal_l02"));
    }
    frame(lua_state, 69.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_shizue_landing01"));
    }
}

unsafe extern "C" fn sound_damage(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        SHIZUE_VC_SEQUENCE_DAMAGE(agent);
    }
}

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        SHIZUE_VC_SEQUENCE_DAMAGEFLY(agent);
    }
}

unsafe extern "C" fn sound_win1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_appeal_h01_win01"));
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_bell01_win01"));
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_appeal_h01_win01"));
    }
    frame(lua_state, 61.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_appeal_h01_win01"));
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_bell02_win01"));
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_appeal_h01_win01"));
    }
    frame(lua_state, 94.0);
    if is_excute(agent) {
        let handle = SoundModule::play_se(boma, Hash40::new("vc_shizue_win03"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 1.2, 0);
    }
    frame(lua_state, 113.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_appeal_l02_win01"));
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_bell03_win01"));
    }
}

unsafe extern "C" fn sound_win3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_win03_cloth_swish"));
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_win03_cloth_swish"));
    }
    frame(lua_state, 67.0);
    if is_excute(agent) {
        let handle = SoundModule::play_se_no3d(boma, Hash40::new("vc_shizue_win02"), true, false);
        SoundModule::set_se_vol(boma, handle as i32, 1.3, 0);
    }
    frame(lua_state, 71.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_appeal_l02_win03"));
    }
    frame(lua_state, 91.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_landing01_win03"));
    }
    frame(lua_state, 97.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_appeal_l02_win03"));
    }
    frame(lua_state, 118.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_shizue_landing01_win03"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_dash", game_dash, Priority::Low);
    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("game_turndash", game_turndash, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("sound_cliffcatch", sound_cliffcatch, Priority::Low);
    agent.acmd("sound_cliffattack", sound_cliffattack, Priority::Low);
    agent.acmd("sound_ottotto", sound_ottotto, Priority::Low);
    agent.acmd("sound_furafura", sound_furafura, Priority::Low);

    agent.acmd("sound_passive", sound_passive, Priority::Low);
    agent.acmd("sound_passivewall", sound_passive, Priority::Low);
    agent.acmd("sound_passiveceil", sound_passive, Priority::Low);

    agent.acmd("sound_appealsl", sound_appeals, Priority::Low);
    agent.acmd("sound_appealsr", sound_appeals, Priority::Low);
    agent.acmd("sound_appealhil", sound_appealhi, Priority::Low);
    agent.acmd("sound_appealhir", sound_appealhi, Priority::Low);
    agent.acmd("sound_appeallwl", sound_appeallw, Priority::Low);
    agent.acmd("sound_appeallwr", sound_appeallw, Priority::Low);

    agent.acmd("sound_damagen1", sound_damage, Priority::Low);
    agent.acmd("sound_damagen2", sound_damage, Priority::Low);
    agent.acmd("sound_damagen3", sound_damage, Priority::Low);
    agent.acmd("sound_damagehi1", sound_damage, Priority::Low);
    agent.acmd("sound_damagehi2", sound_damage, Priority::Low);
    agent.acmd("sound_damagehi3", sound_damage, Priority::Low);
    agent.acmd("sound_damagelw1", sound_damage, Priority::Low);
    agent.acmd("sound_damagelw2", sound_damage, Priority::Low);
    agent.acmd("sound_damagelw3", sound_damage, Priority::Low);

    agent.acmd("sound_damageflyhi", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflylw", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyn", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflytop", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyroll", sound_damagefly, Priority::Low);

    agent.acmd("sound_win1", sound_win1, Priority::Low);
    agent.acmd("sound_win3", sound_win3, Priority::Low);
}
