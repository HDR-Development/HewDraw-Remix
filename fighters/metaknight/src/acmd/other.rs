use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_metaknight_damagefly02"));
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
                PLAY_FLY_VOICE(agent, Hash40::new("seq_metaknight_rnd_futtobi01"), Hash40::new("seq_metaknight_rnd_futtobi02"));
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
            PLAY_SE(agent, Hash40::new("vc_metaknight_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_metaknight_rnd_futtobi01"), Hash40::new("seq_metaknight_rnd_futtobi02"));
        }
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
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_metaknight_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_metaknight_step_right_l"));
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_metaknight_step_left_l"));
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

    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }

}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }

}

unsafe extern "C" fn effect_cliffattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 14.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_cliff_hdr"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
}

unsafe extern "C" fn effect_downattackd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_down_d_hdr"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
}

unsafe extern "C" fn effect_downattacku(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_down_u_hdr"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
}

unsafe extern "C" fn effect_slipattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_slip_wind"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_slip_hdr"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_slip_wind"), false, false);
    }
}

unsafe extern "C" fn effect_win1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 59.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, -9, 0, 20, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 62.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 63.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_win1_trace_hdr"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 75.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
    frame(agent.lua_state_agent, 127.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -15, 7.2, -5.263, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 135.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_slip_wind"), false, false);
    }
    frame(agent.lua_state_agent, 147.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_win1_wind"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 148.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 166.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("trans"), 0, 0, 0, 0, 90, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 168.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_win1_line"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 185.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_win1_wind"), false, false);
    }
}

unsafe extern "C" fn effect_win3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 42.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 5, 0, 130, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 46.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 47.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_win3_trace_hdr"), Hash40::new("top"), -0.0, 0, 0, 0, 130, 0, 1, true);
    }
    frame(agent.lua_state_agent, 48.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
    frame(agent.lua_state_agent, 107.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0.01, 10.0, -0.071, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn game_jumpaerialf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 21.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_JUMP_FLY_NEXT);
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

    agent.acmd("effect_cliffattack", effect_cliffattack, Priority::Low);
    agent.acmd("effect_downattackd", effect_downattackd, Priority::Low);
    agent.acmd("effect_downattacku", effect_downattacku, Priority::Low);
    agent.acmd("effect_slipattack", effect_slipattack, Priority::Low);

    agent.acmd("effect_win1", effect_win1, Priority::Low);
    agent.acmd("effect_win3", effect_win3, Priority::Low);

    agent.acmd("game_jumpaerialf1", game_jumpaerialf, Priority::Low);
    agent.acmd("game_jumpaerialf2", game_jumpaerialf, Priority::Low);
    agent.acmd("game_jumpaerialf3", game_jumpaerialf, Priority::Low);
    agent.acmd("game_jumpaerialf4", game_jumpaerialf, Priority::Low);
    agent.acmd("game_jumpaerialf5", game_jumpaerialf, Priority::Low);
}
