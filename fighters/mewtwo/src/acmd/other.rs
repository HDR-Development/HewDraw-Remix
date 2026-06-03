use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_mewtwo_damagefly02"));
        } else {
            let damage_speed_x = agent.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            let damage_speed_y = agent.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);

            let speed_vector = sv_math::vec2_length(damage_speed_x, damage_speed_y);

            let play_vc = if speed_vector < 3.8 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {
                PLAY_FLY_VOICE(agent, Hash40::new("seq_mewtwo_rnd_futtobi01"), Hash40::new("seq_mewtwo_rnd_futtobi02"));
            }
        }
    }
}

unsafe extern "C" fn sound_damageflyroll(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_mewtwo_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_mewtwo_rnd_futtobi01"), Hash40::new("seq_mewtwo_rnd_futtobi02"));
        }
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.5);
    }
    frame(lua_state, 11.0); // Effectively F16
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
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_mewtwo_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.3);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0); // Effectively F16
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn game_escapeair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, true);
        ArticleModule::remove(boma, *FIGHTER_MEWTWO_GENERATE_ARTICLE_ESCAPEAIRDUMMY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }

}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, true);
        ArticleModule::remove(boma, *FIGHTER_MEWTWO_GENERATE_ARTICLE_ESCAPEAIRDUMMY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }

}

unsafe extern "C" fn sound_fuwafuwa(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        let sfx_handle = SoundModule::play_status_se(boma, Hash40::new("se_common_spirits_pfog_loop"), true, false, false);
        SoundModule::set_se_vol(boma, sfx_handle as i32, 1.1, 0);
    }
}

unsafe extern "C" fn expression_fuwafuwa(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_cliffescape", acmd_stub, Priority::Low);

    agent.acmd("sound_damageflyhi", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflylw", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyn", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflytop", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyroll", sound_damageflyroll, Priority::Low);

    agent.acmd("game_dash", game_dash, Priority::Low);
    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("game_turndash", game_turndash, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("game_fuwafuwastart", acmd_stub, Priority::Low);
    agent.acmd("effect_fuwafuwastart", acmd_stub, Priority::Low);
    agent.acmd("sound_fuwafuwastart", sound_fuwafuwa, Priority::Low);
    agent.acmd("expression_fuwafuwastart", expression_fuwafuwa, Priority::Low);
    agent.acmd("game_fuwafuwa", acmd_stub, Priority::Low);
    agent.acmd("effect_fuwafuwa", acmd_stub, Priority::Low);
    agent.acmd("sound_fuwafuwa", sound_fuwafuwa, Priority::Low);
    agent.acmd("expression_fuwafuwa", expression_fuwafuwa, Priority::Low);
}