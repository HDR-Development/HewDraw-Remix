use smash::app::sv_animcmd::EFFECT_FLW_POS_UNSYNC_VIS;
use core::f32;

use super::*;

unsafe extern "C" fn effect_specialnhold(agent: &mut L2CAgentBase) {
    if is_excute(agent) && agent.is_situation(*SITUATION_KIND_GROUND) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.5, 10, 0, 4, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 8.0);
}

unsafe extern "C" fn sound_specialnhold(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_mewtwo_special_n01"));
    }
}

unsafe extern "C" fn effect_specialnmax(agent: &mut L2CAgentBase) {
    if agent.is_prev_status(*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD) {
        if is_excute(agent) {
            EFFECT_FLW_POS(agent, Hash40::new("mewtwo_shadowball_max_sign"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.4, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        if is_excute(agent) {
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.8, 10, 0, 4, 0, 0, 0, false);
            }
            FLASH(agent, 0.9, 0.7, 1, 0.5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            FLASH_FRM(agent, 5, 0.9, 0.4, 1, 0.1);
        }
        wait(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    } else {
        return effect_specialnhold(agent);
    }
}

unsafe extern "C" fn sound_specialnmax(agent: &mut L2CAgentBase) {
    if agent.is_prev_status(*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD) {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_mewtwo_special_n02"));
            STOP_SE(agent, Hash40::new("se_mewtwo_special_n01"));
            PLAY_STATUS(agent, Hash40::new("se_mewtwo_special_n07"));
        }
    } else {
        if is_excute(agent) {
            PLAY_STATUS(agent, Hash40::new("se_mewtwo_special_n07"));
        }
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 7.4, 0.0, 9.3, 14.3, None, None, None, *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 9.5, 0.0, 9.3, 14.3, None, None, None, *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *COLLISION_SITUATION_MASK_G);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 361, 100, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, 0, 1.0, 280, 50, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_catch_only(boma, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, true, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        agent.on_flag(*FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_GRAVITY_NORMAL);
    }
    frame(lua_state, 36.0);
    FT_MOTION_RATE_RANGE(agent, 36.0, 45.0, 14.0); // 50 faf
    frame(lua_state, 45.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1.5, 0, 2, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mewtwo_nenriki"), Hash40::new("top"), 0, 9.3, 14.3, 0, 90, 0, 0.36, true);
        EffectModule::enable_sync_init_pos_last(boma);
        let effect = EffectModule::get_last_handle(boma) as u64;
        VarModule::set_int64(agent.battle_object, vars::mewtwo::status::EFFECT_HANDLER, effect);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_pk_hand"), false, false);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_nenriki"), false, true);
    }
}

unsafe extern "C" fn game_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(agent, 16.0, 48.0, 50.0);
    for _ in 0..4 {
        if is_excute(agent) {
            let target = agent.get_int64(*FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_TARGET_OBJECT_ID);
            let target_group = agent.get_int64(*FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_THROWN_HIT_GROUP);
            let target_no = agent.get_int64(*FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_THROWN_HIT_NO);
            ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, Hash40::new("throw"), target, target_group, target_no);
        }
        wait(lua_state, 6.0);
    }
    frame(lua_state, 48.0);
    FT_MOTION_RATE_RANGE(agent, 48.0, 59.0, 14.0);
    if is_excute(agent) {
        let final_pos = VarModule::get_vec2(agent.battle_object, vars::mewtwo::status::SPECIAL_S_THROW_POS);
        let pos_diff = Vector2f{x: (final_pos.x - 14.3)*boma.lr(), y: final_pos.y - 9.5}; 
        if pos_diff.x.abs() + pos_diff.y.abs() > 0.0 {
            let angle_max = ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_s.angle_max");
            let angle_diff = pos_diff.y.atan2(pos_diff.x * boma.lr());
            let comb_angle = (angle_diff.to_degrees() + 75.0)/2.0;
            let mut angle_norm = if comb_angle-75.0 > angle_max {(75.0+angle_max) as u64} else {comb_angle.round() as u64};
            if comb_angle-75.0 < -angle_max {angle_norm = (75.0-angle_max) as u64;}
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, angle_norm, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_grudge"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        AttackModule::set_add_reaction_frame(boma, 0, -1.0, false); // eq to 15 frames endlag
        agent.on_flag(*FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_GRAVITY_NORMAL);
        agent.on_flag(*FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_HIT);
    }
    frame(lua_state, 59.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn sound_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    for _ in 0..4 {
        if is_excute(agent) {
            PLAY_STATUS(agent, Hash40::new("se_mewtwo_attack100"));
        }
        wait(lua_state, 6.0);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_mewtwo_attack03"));
        PLAY_SE(agent, Hash40::new("se_mewtwo_attack100_02"));
    }
}

unsafe extern "C" fn effect_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 39.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari_eye"), Hash40::new("head"), 0.7, 1.9, -1.4*boma.lr(), 0, 0, 0, 0.25, true);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        let eff_offset = VarModule::get_vec2(agent.battle_object, vars::mewtwo::status::SPECIAL_S_THROW_CAPTURED_POS_OFFSET);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, eff_offset.y, eff_offset.x, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_COLOR(agent, 0.4, 0.0, 1.0);
        LAST_EFFECT_SET_RATE(agent, 6.0/7.0);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("mewtwo_final_aura"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

unsafe extern "C" fn expression_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

unsafe extern "C" fn game_specialairhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        KineticModule::clear_speed_all(boma);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_MEWTWO_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_1);
    } // make use of unused flag to enable drift
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 6.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MEWTWO_GENERATE_ARTICLE_BINDBALL, false, -1);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 53.0, 21.0);
    frame(lua_state, 53.0);
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_specialnhold", effect_specialnhold, Priority::Low);
    agent.acmd("sound_specialnhold", sound_specialnhold, Priority::Low);
    agent.acmd("effect_specialnmax", effect_specialnmax, Priority::Low);
    agent.acmd("sound_specialnmax", sound_specialnmax, Priority::Low);
    agent.acmd("effect_specialairnhold", effect_specialnhold, Priority::Low);
    agent.acmd("sound_specialairnhold", sound_specialnhold, Priority::Low);
    agent.acmd("effect_specialairnmax", effect_specialnmax, Priority::Low);
    agent.acmd("sound_specialairnmax", sound_specialnmax, Priority::Low);

    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("effect_specials", effect_specials, Priority::Low);

    agent.acmd("game_specialsthrow", game_specialsthrow, Priority::Low);
    agent.acmd("effect_specialsthrow", effect_specialsthrow, Priority::Low);
    agent.acmd("sound_specialsthrow", sound_specialsthrow, Priority::Low);
    agent.acmd("expression_specialsthrow", expression_specialsthrow, Priority::Low);

    agent.acmd("game_specialairhistart", game_specialairhistart, Priority::Low);
    agent.acmd("game_specialairhi", game_specialairhi, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
}
