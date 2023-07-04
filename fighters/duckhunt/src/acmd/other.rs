
use super::*;

#[acmd_script( agent = "duckhunt", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "duckhunt", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflylw_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "duckhunt", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyn_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "duckhunt", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "duckhunt", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflytop_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "duckhunt", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_catch_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "duckhunt", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "duckhunt", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_duckhunt_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

#[acmd_script( agent = "duckhunt", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "duckhunt_can", script = "game_explode" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_can_explode_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 60, 100, 0, 40, 13.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
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

#[acmd_script( agent = "duckhunt_clay" , script = "game_fly" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_clay_fly_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "duckhunt_clay" , script = "game_hit" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_clay_hit_game(fighter: &mut L2CAgentBase) {
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
        ATTACK(fighter,  0,  0,  Hash40::new("top"),  3.0,  80,  70,  0,  70,  3.0,  0.0,  9.0,  -6.0,  Some(0.0),  Some(-9.0),  Some(6.0),  0.2,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.5,  0.0,  0,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        ATTACK(fighter,  1,  0,  Hash40::new("top"),  3.0,  80,  70,  0,  70,  3.0,  0.0,  0.5,  -11.0,  Some(0.0),  Some(-1.1),  Some(11.0),  0.2,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.5,  0.0,  0,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        ATTACK(fighter,  2,  0,  Hash40::new("top"),  3.0,  80,  70,  0,  70,  3.0,  0.0,  -8.0,  -8.0,  Some(0.0),  Some(8.0),  Some(8.0),  0.2,  0.5,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  -1.5,  0.0,  0,  true,  true,  false,  false,  false,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_OBJECT);
        }
}

#[acmd_script( agent = "duckhunt_gunman" , scripts = ["sound_readyr", "sound_readyl"] , category = ACMD_SOUND , low_priority)]
unsafe fn duckhunt_gunman_ready_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "duckhunt_gunman" , script = "effect_readyl" , category = ACMD_EFFECT , low_priority)]
unsafe fn duckhunt_gunman_ready_effectl(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "duckhunt_gunman" , script = "effect_readyr" , category = ACMD_EFFECT , low_priority)]
unsafe fn duckhunt_gunman_ready_effectr(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "duckhunt_gunmanbullet", script = "game_move", category = ACMD_GAME, low_priority )]
unsafe fn duckhunt_gunmanbullet_move_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 65, 60, 0, 37, 1.2, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 65, 90, 0, 37, 1.2, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 50, 60, 0, 37, 1.2, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(agent, 3, 0, Hash40::new("top"), 11.0, 70, 60, 0, 37, 4.0, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(agent, 4, 0, Hash40::new("top"), 10.0, 65, 60, 0, 37, 4.0, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "duckhunt", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "duckhunt", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
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
    install_acmd_scripts!(
        escape_air_game,
        escape_air_slide_game,
        duckhunt_catch_game,
        dash_game,
        dash_sound,
        turn_dash_game,
        duckhunt_can_explode_game,
        duckhunt_clay_fly_game,
        duckhunt_clay_hit_game,
        duckhunt_gunman_ready_effectr,
        duckhunt_gunman_ready_effectl,
        duckhunt_gunman_ready_sound,
        duckhunt_gunmanbullet_move_game,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound
    );
}

