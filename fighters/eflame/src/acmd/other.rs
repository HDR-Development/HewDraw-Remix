use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_eflame_damagefly02"));
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
                PLAY_FLY_VOICE(agent, Hash40::new("seq_eflame_rnd_futtobi01"), Hash40::new("seq_eflame_rnd_futtobi02"));
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
            PLAY_SE(agent, Hash40::new("vc_eflame_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_eflame_rnd_futtobi01"), Hash40::new("seq_eflame_rnd_futtobi02"));
        }
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_eflame_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_eflame_step_left_l"));
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
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 7, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_core_start"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 3, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 15, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, -25, 0, 0, 0.75, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 0, 0, 0, 10, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire"), false, true);
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_s"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_core_start"), false, true);
    }
}

unsafe extern "C" fn effect_downattackd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 6, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 14, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.45, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.45, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, -21, 0, 0, 0.75, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 0, 0, 0, -10, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail_end"), false, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 7, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 14, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, -35, 0, 0, 0.75, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, -15, 0, 0, 0.75, true);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 0, 0, 0, 15, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_s"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
}

unsafe extern "C" fn effect_downattacku(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 6, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 14, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.45, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.45, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, -21, 0, 0, 0.75, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 0, 0, 0, -10, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail_end"), false, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 7, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 14, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, -35, 0, 0, 0.75, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, -15, 0, 0, 0.75, true);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 0, 0, 0, 15, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_s"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
}

unsafe extern "C" fn effect_slipattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 6, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 14, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.45, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.45, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, -21, 0, 0, 0.75, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 0, 0, 0, -10, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(agent, 1.6);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire"), false, true);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail_end"), false, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword_hdr"), Hash40::new("tex_eflame_sword2"), 7, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 14, 0, -0.25, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.45, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.45, true);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, -35, 0, 0, 0.75, true);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, -15, 0, 0, 0.75, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 0, 0, 0, 15, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_s"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_cliffescape", acmd_stub, Priority::Low);

    agent.acmd("sound_damageflyhi", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflylw", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyn", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflytop", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyroll", sound_damageflyroll, Priority::Low);

    agent.acmd("sound_dash", sound_dash, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("effect_cliffattack", effect_cliffattack, Priority::Low);
    agent.acmd("effect_downattackd", effect_downattackd, Priority::Low);
    agent.acmd("effect_downattacku", effect_downattacku, Priority::Low);
    agent.acmd("effect_slipattack", effect_slipattack, Priority::Low);
}
