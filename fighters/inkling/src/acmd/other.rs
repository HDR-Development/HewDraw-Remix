use super::*;
use globals::*;
pub const BRAKE_FRAME: f32 = 7.0;
pub const SQUID_FRAME: f32 = 11.0;

unsafe extern "C" fn game_turnrun(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
        if is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, -1);
        }
    }
    let motion = MotionModule::motion_kind(agent.module_accessor);
    let motion_frame = MotionModule::frame(agent.module_accessor);
    let rate = MotionModule::rate(agent.module_accessor);
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(motion), false, -1.0);
        ArticleModule::set_frame(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID,  motion_frame);
        ArticleModule::set_rate(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, rate);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
        let status = StatusModule::status_kind(agent.module_accessor);
        if status != *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
        }
    }
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_whole(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, BRAKE_FRAME); 
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_RUN_BRAKE_FLAG_STOP);
    }
    frame(agent.lua_state_agent, SQUID_FRAME);
    if is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_turnrun(agent: &mut L2CAgentBase) {
    let r = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R);
    let g = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G);
    let b = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B);
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("inkling_squid_change"), Hash40::new("head"), 3, 0, 0, 0, 0, 0, 1, true);
        LAST_PARTICLE_SET_COLOR(agent,r,g,b);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, BRAKE_FRAME-1.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, SQUID_FRAME-1.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, SQUID_FRAME);
    if is_excute(agent) {
        EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("inkling_squid_change"), Hash40::new("top"), 0, 2.5, -2.5, 0, 0, 0, 1, true);
        LAST_PARTICLE_SET_COLOR(agent,r,g,b);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_turnrun(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, BRAKE_FRAME-2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_inkling_dash_stop"));
        SET_PLAY_INHIVIT(agent, Hash40::new("se_inkling_dash_stop"), 20);
    }
    frame(agent.lua_state_agent, SQUID_FRAME+1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_inkling_dash_start"));
    }
}

unsafe extern "C" fn expression_turnrun(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        InkPaintModule::set_special_paint(agent.module_accessor, SpecialPaintKind{_address: *SPECIAL_PAINT_SQUID_TO_HUMAN as u8});
    }
    frame(agent.lua_state_agent, BRAKE_FRAME);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, SQUID_FRAME+2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

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
            if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_inkling_rnd_futtobi01"), Hash40::new("seq_inkling_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_inkling_rnd_futtobi01"), Hash40::new("seq_inkling_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn sound_damageflyroll(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(boma) {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_inkling_rnd_futtobi01"), Hash40::new("seq_inkling_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        PLAY_FLY_VOICE(agent, Hash40::new("seq_inkling_rnd_futtobi01"), Hash40::new("seq_inkling_rnd_futtobi02"));
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
        }
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
        if is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, -1);
        }
    }
    let motion_kind = MotionModule::motion_kind(agent.module_accessor);
    let motion_frame = MotionModule::frame(agent.module_accessor);
    let motion_rate = MotionModule::rate(agent.module_accessor);
    ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(motion_kind), false, -1.0);
    if is_excute(agent) {
        ArticleModule::set_frame(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, motion_frame);
        ArticleModule::set_rate(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, motion_rate);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
        let status_kind = StatusModule::status_kind(agent.module_accessor);
        if status_kind != *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
        }
    }
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("dash_l"), false, -1.0);
        }
    }
    else {
        if is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("dash_r"), false, -1.0);
        }
    }
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_inkling_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn game_escapeair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
    }
    if !WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
        if is_excute(agent) {
            ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, -1);
        }
    }
    let motion_kind = MotionModule::motion_kind(boma);
    let motion_frame = MotionModule::frame(boma);
    let motion_rate = MotionModule::rate(boma);
    ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(motion_kind), false, -1.0);
    if is_excute(agent) {
        ArticleModule::set_frame(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, motion_frame);
        ArticleModule::set_rate(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, motion_rate);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
        let status_kind = StatusModule::status_kind(boma);
        if status_kind != *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, true);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
    }
    if !WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
        if is_excute(agent) {
            ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, -1);
        }
    }
    let motion_kind = MotionModule::motion_kind(boma);
    let motion_frame = MotionModule::frame(boma);
    let motion_rate = MotionModule::rate(boma);
    ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(motion_kind), false, -1.0);
    if is_excute(agent) {
        ArticleModule::set_frame(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, motion_frame);
        ArticleModule::set_rate(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, motion_rate);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
        let status_kind = StatusModule::status_kind(boma);
        if status_kind != *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, true);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_damageflyhi", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflylw", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyn", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflytop", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyroll", sound_damageflyroll, Priority::Low);

    agent.acmd("game_dash", game_dash, Priority::Low);
    agent.acmd("sound_dash", sound_dash, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("game_turnrun", game_turnrun, Priority::Low);
    agent.acmd("effect_turnrun", effect_turnrun, Priority::Low);
    agent.acmd("sound_turnrun", sound_turnrun, Priority::Low);
    agent.acmd("expression_turnrun", expression_turnrun, Priority::Low);
}
