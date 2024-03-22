
use super::*;

unsafe extern "C" fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));
    }
}

unsafe extern "C" fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_richter_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
        PLAY_SE(fighter, Hash40::new("se_richter_whip_chain"));
    }
}

unsafe extern "C" fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, 29.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn cliffattack_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 5, 3.5, 12, -29, -27, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
        LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn slipattack_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.4);
    }
    frame(lua_state, 30.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4.5, 2, -1, -30, 170, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 2);
        LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
    }
    else {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4.5, 2, -1, -10, 160, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 2);
        LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
}
}
}
}

unsafe extern "C" fn downattacku_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
        frame(lua_state, 17.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 3, 0, 0, 0, 180, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
        frame(lua_state, 18.0);
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -1, 3, 0, 1, 180, 187, 1, true);
                LAST_EFFECT_SET_RATE(fighter, 1.3);
                LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
            }
            else {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 3, 0, 1, 169, 187, 1, true);
                LAST_EFFECT_SET_RATE(fighter, 1.3);
                LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
            }
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.6);
    }
    frame(lua_state, 25.0);
    if sv_animcmd::get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4.5, 2, -1, -30, 170, 1, true);
            LAST_EFFECT_SET_RATE(fighter, 2);
            LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
        }
        else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4.5, 2, -1, -10, 160, 1, true);
            LAST_EFFECT_SET_RATE(fighter, 2);
            LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
        }
    }
    }
    }

unsafe extern "C" fn downattackd_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 3, 0, 0, 0, 180, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 18.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -1, 3, 0, 1, 180, 187, 1, true);
            LAST_EFFECT_SET_RATE(fighter, 1.3);
            LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
        }
        else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 3, 0, 1, 169, 187, 1, true);
            LAST_EFFECT_SET_RATE(fighter, 1.3);
            LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
        }
    }
}
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.4);
    }
    frame(lua_state, 25.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4.5, 2, -1, -30, 170, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
        LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
    }
    else {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4.5, 2, -1, -10, 160, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
        LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
    }
}
}
}

unsafe extern "C" fn appeallwr_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_appeal_l01"));
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_appeal_l01"));
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_appeal_l02"));
    }
}

unsafe extern "C" fn appeallwl_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_appeal_l01"));
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_appeal_l01"));
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_appeal_l02"));
    }
}

unsafe extern "C" fn richter_whip_attack_guardon_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

pub fn install() {
    smashline::Agent::new("richter")
        .acmd("sound_damageflyhi", damageflyhi_sound)
        .acmd("sound_damageflylw", damageflylw_sound)
        .acmd("sound_damageflyn", damageflyn_sound)
        .acmd("sound_damageflyroll", damageflyroll_sound)
        .acmd("sound_damageflytop", damageflytop_sound)
        .acmd("game_dash", dash_game)
        .acmd("sound_dash", dash_sound)
        .acmd("game_turndash", turn_dash_game)
        .acmd("game_escapeair", escape_air_game)
        .acmd("game_escapeairslide", escape_air_slide_game)
        .acmd("sound_appeallwr", appeallwr_sound)
        .acmd("sound_appeallwl", appeallwl_sound)
        .acmd("effect_cliffattack", cliffattack_effect)
        .acmd("effect_slipattack", slipattack_effect)
        .acmd("effect_downattacku", downattacku_effect)
        .acmd("effect_downattackd", downattackd_effect)
        .install();
    smashline::Agent::new("richter_whip")
        .acmd("game_guardon", richter_whip_attack_guardon_game)
        .install();
}
