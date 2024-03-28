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
            if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn sound_damageflyroll(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(agent.module_accessor) {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        PLAY_FLY_VOICE(agent, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));
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
        let dash_sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_snake_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_snake_step_left_m"));
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_snake_step_right_m"));
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

unsafe extern "C" fn sound_appealsr(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_snake_win03"));
    }
}

unsafe extern "C" fn sound_appealhir(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 2, 360, 10, 1, 0, 12, 30, 30, 80);
        // physics!(agent, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.2, 0.2, -1, 0.7, 0.5, -1, Hash40::new("invalid"));
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_snake_appealhi"));
    }
}

unsafe extern "C" fn game_appealendexplode(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_DESIRED_RATE(agent, 80.0, 50.0);
    frame(lua_state, 30.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0 );
        ArticleModule::shoot(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, false, 0);
    }
    frame(lua_state, 80.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        // WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
        // SNAKE_C4_FLAG_IS_SHOWTIME[entry_id] = true;
        ArticleModule::change_status(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_EXPLOSION, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 90.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn expression_appealendexplode(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0 );
    }
    frame(lua_state, 30.0);
    slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    frame(lua_state, 75.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn sound_appealendexplode(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_snake_appealend"));
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_snake_special_l04"));
        PLAY_SE(agent, Hash40::new("se_snake_squat"));
    }
    frame(lua_state, 75.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_snake_special_l05"));
    }
}

unsafe extern "C" fn effect_appealendexplode(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 75.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
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

    agent.acmd("sound_appealsr", sound_appealsr);
    agent.acmd("sound_appealhir", sound_appealhir);

    agent.acmd("game_appealendexplode", game_appealendexplode);
    agent.acmd("effect_appealendexplode", effect_appealendexplode);
    agent.acmd("sound_appealendexplode", sound_appealendexplode);
    agent.acmd("expression_appealendexplode", expression_appealendexplode);

    agent.acmd("game_escapeair", game_escapeair);
    agent.acmd("game_escapeairslide", game_escapeairslide);
}