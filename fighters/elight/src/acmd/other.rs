use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_elight_damagefly02"));
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
                PLAY_FLY_VOICE(agent, Hash40::new("seq_elight_rnd_futtobi01"), Hash40::new("seq_elight_rnd_futtobi02"));
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
            PLAY_SE(agent, Hash40::new("vc_elight_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_elight_rnd_futtobi01"), Hash40::new("seq_elight_rnd_futtobi02"));
        }
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_elight_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_elight_step_left_l"));
    }
}

unsafe extern "C" fn game_escapen(agent: &mut L2CAgentBase) { }

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

unsafe extern "C" fn effect_justshieldoff(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
        COL_PRI(agent, 250);
        FLASH(agent, 0, 0.1, 0.6, 0.8);
        agent.clear_lua_stack();
        lua_args!(agent, -1, 0, 0);
        sv_animcmd::FLASH_SET_DIRECTION(lua_state);
        EFFECT(agent, Hash40::new("elight_foresight2"), Hash40::new("top"), 0, 7.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("elight_foresight_lensflare"), Hash40::new("top"), 0, 7.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("elight_foresight_body"), Hash40::new("hip"), 2, 0, 0, 0, 0, 90, 1, true);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 6, 0, 0.1, 0.6, 0);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        ColorBlendModule::set_disable_camera_depth_influence(boma, false);
    }
}

unsafe extern "C" fn sound_justshieldoff(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let rand = sv_math::rand(hash40("fighter"), 10);
        if rand < 3 {
            PLAY_SEQUENCE(agent, Hash40::new("seq_elight_rnd_foresight"));
        }
        PLAY_SE(agent, Hash40::new("se_elight_escapeforesight01"));
    }
}

unsafe extern "C" fn effect_cliffattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 7, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword_hdr"), Hash40::new("tex_elight_sword2"), 6, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 14.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.7, 0, 0, 4, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.85, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light"), false, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_beam_s"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
}

unsafe extern "C" fn effect_downattackd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword_hdr"), Hash40::new("tex_elight_sword2"), 6, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 14.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light_few"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light_few"), false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword_hdr"), Hash40::new("tex_elight_sword2"), 7, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 14.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light"), false, true);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_beam_s"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
}

unsafe extern "C" fn effect_downattacku(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword_hdr"), Hash40::new("tex_elight_sword2"), 6, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 14.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light_few"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light_few"), false, true);
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword_hdr"), Hash40::new("tex_elight_sword2"), 7, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 14.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light"), false, true);
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_beam_s"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
}

unsafe extern "C" fn effect_slipattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword_hdr"), Hash40::new("tex_elight_sword2"), 6, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 14.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light"), false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword_hdr"), Hash40::new("tex_elight_sword2"), 7, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 14.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light"), false, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_beam_s"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
}

unsafe extern "C" fn effect_win1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_win1"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light_win1"), Hash40::new("sword1"), -5, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light_win1"), false, true);
    }
    frame(lua_state, 88.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword_hdr"), Hash40::new("tex_elight_sword2"), 5, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 15.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light_win1"), Hash40::new("sword1"), -2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 89.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 8, 0, 80, 0, 0.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 92.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light_win1"), false, true);
        AFTER_IMAGE_OFF(agent, 14);
    }
    frame(lua_state, 125.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword_hdr"), Hash40::new("tex_elight_sword2"), 5, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 15.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light_win1"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 130.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light_win1"), false, true);
        AFTER_IMAGE_OFF(agent, 8);
    }
    frame(lua_state, 132.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_beam_win1"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

unsafe extern "C" fn game_jumpaerialfront(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, 1);
    }
}

unsafe extern "C" fn game_jumpaerialback(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, 1);
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

    agent.acmd("game_escapen", game_escapen, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("effect_justshieldoff", effect_justshieldoff, Priority::Low);
    agent.acmd("sound_justshieldoff", sound_justshieldoff, Priority::Low);

    agent.acmd("effect_cliffattack", effect_cliffattack, Priority::Low);
    agent.acmd("effect_downattackd", effect_downattackd, Priority::Low);
    agent.acmd("effect_downattacku", effect_downattacku, Priority::Low);
    agent.acmd("effect_slipattack", effect_slipattack, Priority::Low);
    agent.acmd("effect_win1", effect_win1, Priority::Low);

    agent.acmd("game_jumpaerialfront", game_jumpaerialfront, Priority::Low);
    agent.acmd("game_jumpaerialback", game_jumpaerialback, Priority::Low);
}
