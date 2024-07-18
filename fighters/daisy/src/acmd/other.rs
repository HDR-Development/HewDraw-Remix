use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(boma) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_daisy_rnd_futtobi01"), Hash40::new("seq_daisy_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_daisy_rnd_futtobi01"), Hash40::new("seq_daisy_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn sound_damageflyroll(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(boma) {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_daisy_rnd_futtobi01"), Hash40::new("seq_daisy_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        PLAY_FLY_VOICE(agent, Hash40::new("seq_daisy_rnd_futtobi01"), Hash40::new("seq_daisy_rnd_futtobi02"));
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_daisy_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
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

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 48.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 4.0, 90, 100, 165, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_HIP);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 10.0, false);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 48.0);
    if is_excute(agent) {
        let lr = PostureModule::lr(boma);
        EFFECT(agent, Hash40::new("sys_steam3"), Hash40::new("top"), (-3.8 * lr), 10, -1.5, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, true);
    }
    wait(lua_state, 32.0);
    EFFECT_OFF_KIND(agent, Hash40::new("sys_steam3"), true, true);
}

unsafe extern "C" fn sound_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_daisy_appeal_l01"));
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_stage_suddendeath_in_end"));
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_daisy_cliffcatch"));
    }
    frame(lua_state, 96.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_daisy_wakeup"));
    }
}

unsafe extern "C" fn expression_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 110.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_appealspecial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 25.0);
    if !ArticleModule::is_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO) {
        if is_excute(agent) {
            ArticleModule::generate_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO, false, 0);
            ArticleModule::change_motion(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO, Hash40::new("catch_wait"), true, 0.0);
            ArticleModule::set_visibility_whole(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            let article = ArticleModule::get_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO);
            let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_boma = sv_battle_object::module_accessor(article_id);
            let offset = Vector3f {
                x: PostureModule::pos_x(boma) + (10.0 * PostureModule::lr(boma)),
                y: PostureModule::pos_y(boma) + 7.0,
                z: -6.0
            };
            PostureModule::set_pos(article_boma, &offset);
            PostureModule::set_scale(article_boma, 1.2, true);
            LinkModule::unlink(article_boma, *WEAPON_LINK_NO_CONSTRAINT); // detaches the article from daisy
            VarModule::set_int(agent.battle_object, vars::daisy::instance::YAPPING_TIMER, 999);
            
            let effect = EffectModule::req_on_joint(article_boma, Hash40::new("sys_erace_smoke"), Hash40::new("top"), &Vector3f::new(0.2, 4.5, 0.0), &Vector3f::zero(), 0.6, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
            EffectModule::set_rate(boma, effect as u32, 1.0);
        }
        wait(lua_state, 3.0);
        if is_excute(agent) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
}

unsafe extern "C" fn effect_appealspecial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.9);
    }
}

unsafe extern "C" fn sound_appealspecial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_daisy_appeal_s01"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_daisy_wear02"));
    }
}

unsafe extern "C" fn expression_appealspecial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 100.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_damageflyhi", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflylw", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyn", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflytop", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyroll", sound_damageflyroll, Priority::Low);

    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("game_turndash", game_turndash, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("game_appeallwl", game_appeallw, Priority::Low);
    agent.acmd("game_appeallwr", game_appeallw, Priority::Low);
    agent.acmd("effect_appeallwl", effect_appeallw, Priority::Low);
    agent.acmd("effect_appeallwr", effect_appeallw, Priority::Low);
    agent.acmd("sound_appeallwl", sound_appeallw, Priority::Low);
    agent.acmd("sound_appeallwr", sound_appeallw, Priority::Low);
    agent.acmd("expression_appeallwl", expression_appeallw, Priority::Low);
    agent.acmd("expression_appeallwr", expression_appeallw, Priority::Low);

    agent.acmd("game_appealspecial", game_appealspecial, Priority::Low);
    agent.acmd("effect_appealspecial", effect_appealspecial, Priority::Low);
    agent.acmd("sound_appealspecial", sound_appealspecial, Priority::Low);
    agent.acmd("expression_appealspecial", expression_appealspecial, Priority::Low);
}
