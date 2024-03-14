
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));}
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));}
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_ike_rnd_futtobi01"), Hash40::new("seq_ike_rnd_futtobi02"));}
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
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_ike_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_ike_step_right_s"));
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_ike_step_left_s"));
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

unsafe extern "C" fn ike_appeal_lw_l_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ike_swing_l"));
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            let costume_type = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2;
            // Path of Radiance voiceline
            if costume_type == 0 {
                PLAY_SE(fighter, Hash40::new("vc_ike_win02"));
            }
            // Radiant Dawn voiceline
            else{
                PLAY_SE(fighter, Hash40::new("vc_ike_win01"));
            }
        }
        PLAY_SE(fighter, Hash40::new("se_ike_appeal_stab"));
    }
    wait(lua_state, 27.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            //PLAY_SE(fighter, Hash40::new("vc_ike_win01"));
        }
        else{
            PLAY_SE(fighter, Hash40::new("vc_ike_appeal03"));
        }
        PLAY_SE(fighter, Hash40::new("se_ike_appeal_04"));
    }
    frame(lua_state, 98.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ike_appeal_pullout"));
    }
}

unsafe extern "C" fn ike_appeal_lw_r_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ike_swing_l"));
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            let costume_type = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2;
            // Path of Radiance voiceline
            if costume_type == 0 {
                PLAY_SE(fighter, Hash40::new("vc_ike_win02"));
            }
            // Radiant Dawn voiceline
            else{
                PLAY_SE(fighter, Hash40::new("vc_ike_win01"));
            }
        }
        PLAY_SE(fighter, Hash40::new("se_ike_appeal_stab"));
    }
    wait(lua_state, 27.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            //PLAY_SE(fighter, Hash40::new("vc_ike_win01"));
        }
        else{
            PLAY_SE(fighter, Hash40::new("vc_ike_appeal03"));
        }
        PLAY_SE(fighter, Hash40::new("se_ike_appeal_04"));
    }
    frame(lua_state, 98.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ike_appeal_pullout"));
    }
}

unsafe extern "C" fn ike_appeal_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 17.0/13.0);
    if is_excute(fighter) {
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ArticleModule::generate_article(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, Hash40::new("appeal_lw"), false, 0.0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

unsafe extern "C" fn ike_sword_appeal_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 17.0/13.0);
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        /* Air-only */
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 9.0, 270, 33, 0, 30, 3.5, 0.0, 6.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 9.0, 270, 33, 0, 30, 4.5, 0.0, 12.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        /* Ground-only */
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 9.0, 270, 100, 275, 0, 3.5, 0.0, 6.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 9.0, 270, 100, 275, 0, 4.5, 0.0, 12.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
	wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

unsafe extern "C" fn ike_sword_appeal_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            EFFECT(fighter, Hash40::new("ike_volcano_ground"), Hash40::new("top"), 0, 0.0, -8.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("ike_volcano"), Hash40::new("top"), 0, 0.0, -8.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
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

pub fn install() {
    smashline::Agent::new("ike_sword")
        .acmd("game_appeallw", ike_sword_appeal_lw_game)
        .acmd("effect_appeallw", ike_sword_appeal_lw_effect)
        .install();
    smashline::Agent::new("ike")
        .acmd("sound_damageflyhi", damageflyhi_sound)
        .acmd("sound_damageflylw", damageflylw_sound)
        .acmd("sound_damageflyn", damageflyn_sound)
        .acmd("sound_damageflyroll", damageflyroll_sound)
        .acmd("sound_damageflytop", damageflytop_sound)
        .acmd("game_dash", dash_game)
        .acmd("sound_dash", dash_sound)
        .acmd("game_turndash", turn_dash_game)
        .acmd("sound_appeallwl", ike_appeal_lw_l_sound)
        .acmd("sound_appeallwr", ike_appeal_lw_r_sound)
        .acmd("game_appeallwl", ike_appeal_lw_game)
        .acmd("game_appeallwr", ike_appeal_lw_game)
        .acmd("game_escapeair", escape_air_game)
        .acmd("game_escapeairslide", escape_air_slide_game)
        .install();
}
