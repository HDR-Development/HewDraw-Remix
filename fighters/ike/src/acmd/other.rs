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
            if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn sound_damageflyroll(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(agent.module_accessor) {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        PLAY_FLY_VOICE(agent, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_ike_step_right_s"));
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_ike_step_left_s"));
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn sound_appeallwl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            let costume_type = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2;
            // Path of Radiance voiceline
            if costume_type == 0 {
                PLAY_SE(agent, Hash40::new("vc_ike_win02"));
            }
            // Radiant Dawn voiceline
            else{
                PLAY_SE(agent, Hash40::new("vc_ike_win01"));
            }
        }
        PLAY_SE(agent, Hash40::new("se_ike_appeal_stab"));
    }
    wait(lua_state, 27.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            //PLAY_SE(fighter, Hash40::new("vc_ike_win01"));
        }
        else{
            PLAY_SE(agent, Hash40::new("vc_ike_appeal03"));
        }
        PLAY_SE(agent, Hash40::new("se_ike_appeal_04"));
    }
    frame(lua_state, 98.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ike_appeal_pullout"));
    }
}

unsafe extern "C" fn sound_appeallwr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            let costume_type = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2;
            // Path of Radiance voiceline
            if costume_type == 0 {
                PLAY_SE(agent, Hash40::new("vc_ike_win02"));
            }
            // Radiant Dawn voiceline
            else{
                PLAY_SE(agent, Hash40::new("vc_ike_win01"));
            }
        }
        PLAY_SE(agent, Hash40::new("se_ike_appeal_stab"));
    }
    wait(lua_state, 27.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            //PLAY_SE(fighter, Hash40::new("vc_ike_win01"));
        }
        else{
            PLAY_SE(agent, Hash40::new("vc_ike_appeal03"));
        }
        PLAY_SE(agent, Hash40::new("se_ike_appeal_04"));
    }
    frame(lua_state, 98.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ike_appeal_pullout"));
    }
}

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 17.0/13.0);
    if is_excute(agent) {
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ArticleModule::generate_article(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, Hash40::new("appeal_lw"), false, 0.0);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
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
    
    agent.acmd("game_dash", game_dash);
    agent.acmd("sound_dash", sound_dash);
    agent.acmd("game_turndash", game_turndash);

    agent.acmd("sound_appeallwl", sound_appeallwl);
    agent.acmd("sound_appeallwr", sound_appeallwr);
    agent.acmd("game_appeallwl", game_appeallw);
    agent.acmd("game_appeallwr", game_appeallw);
    
    agent.acmd("game_escapeair", game_escapeair);
    agent.acmd("game_escapeairslide", game_escapeairslide);
}
