use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_KILLING_BLOW) {
            PLAY_SE(agent, Hash40::new("vc_ike_damagefly02"));
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
                PLAY_FLY_VOICE(agent, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));
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
            PLAY_SE(agent, Hash40::new("vc_ike_damagefly02"));
        } else {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));
        }
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
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_ike_dash_start"), true, false, false, false, app::enSEType(0));
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
        if VarModule::is_flag(boma.object(), vars::ike::status::SPECIAL_S_INSTAKILL) {
            let costume_type = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2;
            // Path of Radiance voiceline
            if costume_type == 0 {
                PLAY_SE(agent, Hash40::new("vc_ike_win02"));
            }
            // Radiant Dawn voiceline
            else {
                PLAY_SE(agent, Hash40::new("vc_ike_win01"));
            }
        }
        PLAY_SE(agent, Hash40::new("se_ike_appeal_stab"));
    }
    wait(lua_state, 27.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::SPECIAL_S_INSTAKILL) {
            //PLAY_SE(fighter, Hash40::new("vc_ike_win01"));
        } else {
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
        if VarModule::is_flag(boma.object(), vars::ike::status::SPECIAL_S_INSTAKILL) {
            let costume_type = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2;
            // Path of Radiance voiceline
            if costume_type == 0 {
                PLAY_SE(agent, Hash40::new("vc_ike_win02"));
            }
            // Radiant Dawn voiceline
            else {
                PLAY_SE(agent, Hash40::new("vc_ike_win01"));
            }
        }
        PLAY_SE(agent, Hash40::new("se_ike_appeal_stab"));
    }
    wait(lua_state, 27.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::SPECIAL_S_INSTAKILL) {
            //PLAY_SE(fighter, Hash40::new("vc_ike_win01"));
        } else {
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
    if is_excute(agent) {
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ArticleModule::generate_article(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, Hash40::new("appeal_lw"), false, 0.0);
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
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword_hdr"), Hash40::new("tex_ike_sword2"), 7, Hash40::new("sword"), 0, 1, 0, Hash40::new("sword"), 0, 16.2, 0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn effect_downattackd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword_hdr"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0, 1, 0, Hash40::new("sword"), 0, 16.2, 0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn effect_downattacku(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword_hdr"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0, 1, 0, Hash40::new("sword"), 0, 16.2, 0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn effect_slipattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword_hdr"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0, 1, 0, Hash40::new("sword"), 0, 16.2, 0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn effect_win1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 35.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword_hdr"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0, 1, 0, Hash40::new("sword"), 0, 16.2, 0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 69.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword_hdr"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0, 1, 0, Hash40::new("sword"), 0, 16.2, 0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 78.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn effect_win3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_quickdraw"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0, 1, 0, Hash40::new("sword"), 0, 16.2, 0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 4, 0, -5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 85.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 90.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ike_tenku_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 180, 0, 1, true);
    }
    frame(lua_state, 92.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword"), 0, 16, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 106.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ike_tenku_sword2"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("ike_sword"), false, false);
    }
    frame(lua_state, 123.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 4, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_quickdraw"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0, 1, 0, Hash40::new("sword"), 0, 16.2, 0, true, Hash40::new("null"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 126.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_landing_smoke_s"), true, true);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 127.0);
    frame(lua_state, 128.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ike_sword"), false, false);
        AFTER_IMAGE_OFF(agent, 0);
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

    agent.acmd("sound_appeallwl", sound_appeallwl, Priority::Low);
    agent.acmd("sound_appeallwr", sound_appeallwr, Priority::Low);
    agent.acmd("game_appeallwl", game_appeallw, Priority::Low);
    agent.acmd("game_appeallwr", game_appeallw, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("effect_cliffattack", effect_cliffattack, Priority::Low);
    agent.acmd("effect_downattackd", effect_downattackd, Priority::Low);
    agent.acmd("effect_downattacku", effect_downattacku, Priority::Low);
    agent.acmd("effect_slipattack", effect_slipattack, Priority::Low);

    agent.acmd("effect_win1", effect_win1, Priority::Low);
    agent.acmd("effect_win3", effect_win3, Priority::Low);
}
