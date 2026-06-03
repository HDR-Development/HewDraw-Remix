use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_younglink_damagefly02"));
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
                PLAY_FLY_VOICE(agent, Hash40::new("seq_younglink_rnd_futtobi01"), Hash40::new("seq_younglink_rnd_futtobi02"));
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
            PLAY_SE(agent, Hash40::new("vc_younglink_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_younglink_rnd_futtobi01"), Hash40::new("seq_younglink_rnd_futtobi02"));
        }
    }
}

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 40.0);
    FT_MOTION_RATE(agent, 2.5);
    frame(lua_state, 61.0);
    if is_excute(agent) {
        DamageModule::heal(boma, -2.5, 0);
    }
    frame(lua_state, 71.0);
    FT_MOTION_RATE(agent, 1.0);
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
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_younglink_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_younglink_step_left_m"));
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_younglink_step_right_m"));
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
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 2.8, 1, 3.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_younglink_sword_hdr"), Hash40::new("younglink_sword2"), 4, Hash40::new("sword"), 0.5, 0, 0, Hash40::new("sword"), 9.7, 0, -0.25, true, Hash40::new("younglink_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn effect_downattackd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_younglink_sword_hdr"), Hash40::new("younglink_sword2"), 5, Hash40::new("sword"), 0.5, 0, 0, Hash40::new("sword"), 9.7, 0, -0.25, true, Hash40::new("younglink_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.6, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("younglink_kaiten_sword_s"), Hash40::new("sword"), 9.2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("younglink_kaiten_sword_s"), false, false);
        AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn effect_downattacku(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("younglink_kaiten_sword_s"), Hash40::new("sword"), 9.2, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_younglink_sword_hdr"), Hash40::new("younglink_sword2"), 5, Hash40::new("sword"), 0.5, 0, 0, Hash40::new("sword"), 9.7, 0, -0.25, true, Hash40::new("younglink_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.6, 0.2);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("younglink_kaiten_sword_s"), false, false);
        AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn effect_slipattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_younglink_sword_hdr"), Hash40::new("younglink_sword2"), 3, Hash40::new("sword"), 0.5, 0, 0, Hash40::new("sword"), 9.7, 0, -0.25, true, Hash40::new("younglink_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("younglink_kaiten_sword_s"), Hash40::new("sword"), 8.8, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("younglink_kaiten_sword_s"), false, false);
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_younglink_sword_hdr"), Hash40::new("younglink_sword2"), 3, Hash40::new("sword"), 0.5, 0, 0, Hash40::new("sword"), 9.7, 0, -0.25, true, Hash40::new("younglink_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("younglink_kaiten_sword_s"), Hash40::new("sword"), 8.8, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 3, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("younglink_kaiten_sword_s"), false, false);
        AFTER_IMAGE_OFF(agent, 3);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_cliffescape", acmd_stub, Priority::Low);

    agent.acmd("sound_damageflyhi", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflylw", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyn", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflytop", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyroll", sound_damageflyroll, Priority::Low);

    agent.acmd("game_appeallwl", game_appeallw, Priority::Low);
    agent.acmd("game_appeallwr", game_appeallw, Priority::Low);

    agent.acmd("game_dash", game_dash, Priority::Low);
    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("game_turndash", game_turndash, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("effect_cliffattack", effect_cliffattack, Priority::Low);
    agent.acmd("effect_downattackd", effect_downattackd, Priority::Low);
    agent.acmd("effect_downattacku", effect_downattacku, Priority::Low);
    agent.acmd("effect_slipattack", effect_slipattack, Priority::Low);
}
