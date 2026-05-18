use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_shulk_damagefly02"));
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
                PLAY_FLY_VOICE(agent, Hash40::new("seq_shulk_rnd_futtobi01"), Hash40::new("seq_shulk_rnd_futtobi02"));
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
            PLAY_SE(agent, Hash40::new("vc_shulk_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_shulk_rnd_futtobi01"), Hash40::new("seq_shulk_rnd_futtobi02"));
        }
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.3);
    }
    frame(lua_state, 11.0); // Effectively F14
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
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_shulk_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_shulk_step_left_l"));
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_shulk_step_right_l"));
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.143);
    }
    frame(lua_state, 13.0); // Effectively F14
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

}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }

}

unsafe extern "C" fn game_landingfallspecial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if agent.is_prev_status(*FIGHTER_STATUS_KIND_SPECIAL_S) {
        FT_MOTION_RATE_RANGE(agent, 0.0, 30.0, 25.0);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_win2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword3"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_circle"), Hash40::new("swordr"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword3_end"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_sword3"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_sword3_2"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_circle"), false, false);
    }
}

unsafe extern "C" fn sound_win2a_us_en(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 60.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_shulk_win02_01"));
    }
    frame(lua_state, 118.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shulk_win02"));
    }
}

unsafe extern "C" fn sound_win2b_us_en(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_shulk_win02_02"));
    }
    frame(lua_state, 118.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shulk_win02"));
    }
}

unsafe extern "C" fn effect_cliffattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword2"), Hash40::new("haver"), 0, 2.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_circle"), Hash40::new("swordr"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 0, 0, -3, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        let trail_hash = shulk_get_trail(agent).to_hash();
        AFTER_IMAGE4_ON_arg29(agent, trail_hash, Hash40::new("tex_shulk_sword2"), 8, Hash40::new("haver"), 0, 3, 0.9, Hash40::new("haver"), 0, 19, 1.1, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword2_end"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.98, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_sword2"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_circle"), false, false);
    }
    }

unsafe extern "C" fn effect_downattackd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword2"), Hash40::new("haver"), 0, 2.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_circle"), Hash40::new("swordr"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        let trail_hash = shulk_get_trail(agent).to_hash();
        AFTER_IMAGE4_ON_arg29(agent, trail_hash, Hash40::new("tex_shulk_sword2"), 8, Hash40::new("haver"), 0, 3, 0.9, Hash40::new("haver"), 0, 19, 1.1, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword2_end"), Hash40::new("haver"), 0, 2.5, 0, 0, 0, 0, 0.98, true);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_sword2"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_circle"), false, false);
    }
}

unsafe extern "C" fn effect_downattacku(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword2"), Hash40::new("haver"), 0, 2.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_circle"), Hash40::new("swordr"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        let trail_hash = shulk_get_trail(agent).to_hash();
        AFTER_IMAGE4_ON_arg29(agent, trail_hash, Hash40::new("tex_shulk_sword2"), 8, Hash40::new("haver"), 0, 3, 0.9, Hash40::new("haver"), 0, 19, 1.1, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword2_end"), Hash40::new("haver"), 0, 2.5, 0, 0, 0, 0, 0.98, true);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_sword2"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_circle"), false, false);
    }
}

unsafe extern "C" fn effect_slipattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword2"), Hash40::new("haver"), 0, 2.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_circle"), Hash40::new("swordr"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        let trail_hash = shulk_get_trail(agent).to_hash();
        AFTER_IMAGE4_ON_arg29(agent, trail_hash, Hash40::new("tex_shulk_sword2"), 8, Hash40::new("haver"), 0, 3, 0.9, Hash40::new("haver"), 0, 19, 1.1, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 0);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword2_end"), Hash40::new("haver"), 0, 2.5, 0, 0, 0, 0, 0.98, true);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_sword2"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_circle"), false, false);
    }
}

unsafe extern "C" fn effect_win1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_shulk_sword_jab"), Hash40::new("tex_shulk_sword2"), 10, Hash40::new("haver"), 2.8, 0, 0.9, Hash40::new("haver"), 14, 0, 0.9, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 78.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_sword"), false, false);
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

    agent.acmd("game_landingfallspecial", game_landingfallspecial, Priority::Low);
    
    agent.acmd("effect_cliffattack", effect_cliffattack, Priority::Low);
    agent.acmd("effect_downattackd", effect_downattackd, Priority::Low);
    agent.acmd("effect_downattacku", effect_downattacku, Priority::Low);
    agent.acmd("effect_slipattack", effect_slipattack, Priority::Low);

    agent.acmd("effect_win1", effect_win1, Priority::Low);
    agent.acmd("effect_win2", effect_win2, Priority::Low);
    agent.acmd("sound_win2a_us_en", sound_win2a_us_en, Priority::Low);
    agent.acmd("sound_win2b_us_en", sound_win2b_us_en, Priority::Low);
}
