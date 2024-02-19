
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));}
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));}
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));}
    }
}


unsafe extern "C" fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));}
    }
}


unsafe extern "C" fn duckhunt_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 5.0/(5.0-1.0));
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 6.6, 0.0, Some(0.0), Some(6.5), Some(9.70), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
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
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_duckhunt_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
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


unsafe extern "C" fn duckhunt_can_explode_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 60, 100, 0, 40, 13.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 60, 95, 0, 40, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        AREA_WIND_2ND_RAD_arg9(fighter, 0, 2, 0.05, 300, 1, 0, 0, 26, 60);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}


unsafe extern "C" fn duckhunt_clay_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter,  0,  0,  Hash40::new("top"),  2.0,  75,  50,  0,  20,  1.0,  0.0,  0.0,  0.0,  None,  None,  None,  3.1,  1.0,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1,  0.0,  0,  true,  false,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *WEAPON_DUCKHUNT_CLAY_INSTANCE_WORK_ID_FLAG_IS_ADD_ACCEL_Y);
    }
}


unsafe extern "C" fn duckhunt_clay_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 0.0);
    if is_excute(fighter) {
        ATTACK(fighter,  0,  0,  Hash40::new("top"),  2.5,  60,  100,  20,  0,  6.0,  0.0,  0.0,  0.0,  None,  None,  None,  0.5,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.3,  0.0,  7,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        }
    frame(lua_state, 6.0);
        if is_excute(fighter) {
        ATTACK(fighter,  0,  0,  Hash40::new("top"),  2.5,  100,  100,  20,  0,  10.0,  0.0,  0.0,  0.0,  None,  None,  None,  0.5,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.3,  0.0,  7,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter,  0,  0,  Hash40::new("top"),  3.0,  75,  70,  0,  82,  3.0,  0.0,  9.0,  -6.0,  Some(0.0),  Some(-9.0),  Some(6.0),  0.2,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.5,  0.0,  0,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        ATTACK(fighter,  1,  0,  Hash40::new("top"),  3.0,  75,  70,  0,  82,  3.0,  0.0,  0.5,  -11.0,  Some(0.0),  Some(-1.1),  Some(11.0),  0.2,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.5,  0.0,  0,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        ATTACK(fighter,  2,  0,  Hash40::new("top"),  3.0,  75,  70,  0,  82,  3.0,  0.0,  -8.0,  -8.0,  Some(0.0),  Some(8.0),  Some(8.0),  0.2,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.5,  0.0,  0,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        }
}


unsafe extern "C" fn duckhunt_gunman_ready_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_duckhunt_special_l02"));
    }
    frame(lua_state, 275.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_duckhunt_special_l09"));
    } 
}


unsafe extern "C" fn duckhunt_gunman_ready_effectl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 275.0);
    let gunman_kind = WorkModule::get_int(fighter.boma(), *WEAPON_DUCKHUNT_GUNMAN_INSTANCE_WORK_ID_KIND);
    if is_excute(fighter) {
        match gunman_kind {
            0 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 13.3, 0.74, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 13.3, -0.78, 0, 0, 0, 1, true);
            }
            1 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 15.66, 0.42, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 15.66, -0.5, 0, 0, 0, 1, true);
            }
            2 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 16.92, 0.26, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 16.92, -1.29, 0, 0, 0, 1, true);
            }
            3 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 10.9, 0.85, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 10.9, -0.64, 0, 0, 0, 1, true);
            }
            4 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 14.17, 0.4, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5, 14.16, -1.36, 0, 0, 0, 1, true);
            }
            _ => {
                return
            }
        }
    }
}


unsafe extern "C" fn duckhunt_gunman_ready_effectr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 275.0);
    let gunman_kind = WorkModule::get_int(fighter.boma(), *WEAPON_DUCKHUNT_GUNMAN_INSTANCE_WORK_ID_KIND);
    if is_excute(fighter) {
        match gunman_kind {    
            0 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 13.3, 0.74, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 13.3, -0.78, 0, 0, 0, 1, true);
            }
            1 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 15.66, 0.42, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 15.66, -0.5, 0, 0, 0, 1, true);
            }
            2 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 16.92, 0.26, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 16.92, -1.29, 0, 0, 0, 1, true);
            }
            3 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 10.9, 0.85, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 10.9, -0.64, 0, 0, 0, 1, true);
            }
            4 => {
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 14.17, 0.4, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), -0.5, 14.16, -1.36, 0, 0, 0, 1, true);
            }
            _ => {
                return
            }
        }
    }
}


unsafe extern "C" fn duckhunt_gunmanbullet_move_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 65, 60, 0, 45, 1.2, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 65, 90, 0, 45, 1.2, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 50, 60, 0, 45, 1.2, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(agent, 3, 0, Hash40::new("top"), 11.0, 70, 60, 0, 45, 4.0, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(agent, 4, 0, Hash40::new("top"), 10.0, 65, 60, 0, 45, 4.0, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
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


unsafe extern "C" fn effect_appeals(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}


unsafe extern "C" fn sound_appeals(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se(boma, Hash40::new("se_duckhunt_appeal_s01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 3.0, 0);
    }
}


unsafe extern "C" fn expression_appeals(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_normal") as i64);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 10);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_superleaf"), 72, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 88.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
    }
}

pub fn install() {
    smashline::Agent::new("duckhunt_gunmanbullet")
        .acmd("game_move", duckhunt_gunmanbullet_move_game)
        .install();
    smashline::Agent::new("duckhunt_gunman")
        .acmd("sound_readyr", duckhunt_gunman_ready_sound)
        .acmd("sound_readyl", duckhunt_gunman_ready_sound)
        .acmd("effect_readyl", duckhunt_gunman_ready_effectl)
        .acmd("effect_readyr", duckhunt_gunman_ready_effectr)
        .install();
    smashline::Agent::new("duckhunt_can")
        .acmd("game_explode", duckhunt_can_explode_game)
        .install();
    smashline::Agent::new("duckhunt")
        .acmd("sound_damageflyhi", damageflyhi_sound)
        .acmd("sound_damageflylw", damageflylw_sound)
        .acmd("sound_damageflyn", damageflyn_sound)
        .acmd("sound_damageflyroll", damageflyroll_sound)
        .acmd("sound_damageflytop", damageflytop_sound)
        .acmd("game_catch", duckhunt_catch_game)
        .acmd("game_dash", dash_game)
        .acmd("sound_dash", dash_sound)
        .acmd("game_turndash", turn_dash_game)
        .acmd("game_escapeair", escape_air_game)
        .acmd("game_escapeairslide", escape_air_slide_game)
        .acmd("effect_appealsl", effect_appeals)
        .acmd("effect_appealsr", effect_appeals)
        .acmd("sound_appealsl", sound_appeals)
        .acmd("sound_appealsr", sound_appeals)
        .acmd("expression_appealsl", expression_appeals)
        .acmd("expression_appealsr", expression_appeals)
        .install();
    smashline::Agent::new("duckhunt_clay")
        .acmd("game_fly", duckhunt_clay_fly_game)
        .acmd("game_hit", duckhunt_clay_hit_game)
        .install();
}
