use super::*;
use smash::app::sv_battle_object;

#[acmd_script( agent = "miigunner", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "miigunner", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "miigunner", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "miigunner", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "miigunner", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_miigunner_rnd_futtobi01"), Hash40::new("seq_miigunner_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "miigunner", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.375);
    }
	frame(lua_state, 9.0); // Effectively F12
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "miigunner", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_miigunner_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_miigunner_step_right_m"));
    }
    wait(lua_state, 18.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_miigunner_step_left_m"));
    }
}

#[acmd_script( agent = "miigunner", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.125);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 11.0); // Effectively F12
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "miigunner", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.200);
    }
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(boma, true);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 6.6, 0.0, Some(0.0), Some(6.6), Some(12.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(boma, false);
	}
}

#[acmd_script( agent = "miigunner", script = "game_catchdash" , category = ACMD_GAME , low_priority)]
unsafe fn catch_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(boma, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.25, 0.0, 6.7, 0.0, Some(0.0), Some(6.7), Some(13.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(boma, false);
	}
}

#[acmd_script( agent = "miigunner", script = "game_catchturn" , category = ACMD_GAME , low_priority)]
unsafe fn catch_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(boma, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.25, 0.0, 6.4, -4.0, Some(0.0), Some(6.4), Some(-15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(boma, false);
	}
}

#[acmd_script( agent = "miigunner_attackairf_bullet", script = "effect_fly" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_attackairf_bullet_fly_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_air_bullet"), Hash40::new("top"), 0, -0.5, 0.5, 0, 180, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}

#[acmd_script( agent = "miigunner_rapidshot_bullet", script = "game_fly" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_rapidshot_bullet_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 100, 3, 0, 1.4, 0.0, 0.0, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
        AttackModule::enable_safe_pos(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 100, 3, 0, 1.4, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 1.4, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
    }
}

#[acmd_script( agent = "miigunner_rapidshot_bullet", script = "game_flythrowb" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_rapidshot_bullet_flythrowb_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 145, 40, 0, 98, 5.0, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(10.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
	}
    
}

#[acmd_script( agent = "miigunner_grenadelauncher", script = "game_explode" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_grenadelauncher_explode_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VisibilityModule::set_whole(boma, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.4, 160, 9, 0, 29, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.7, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        AREA_WIND_2ND_RAD(fighter, 0, 1, 0.02, 1000, 1, 0, 0, 18);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(boma, 0);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 6.6, 50, 151, 0, 20, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

#[acmd_script( agent = "miigunner_fullthrottle", script = "game_final" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_fullthrottle_final_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let pos2dim = Vector3f {x: 0.0, y: 40.0, z: 0.0};
	PostureModule::set_pos(boma, &pos2dim);
	if is_excute(fighter) {
		
	}
    
}

#[acmd_script( agent = "miigunner_stealthbomb", script = "effect_tame" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_stealthbomb_tame_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.15, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(fighter, 0.75);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.20, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 5.0);
	}
    frame(lua_state, 100.0);
	if is_excute(fighter) {
		let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
            let gunner = utils::util::get_battle_object_from_id(owner_id);
            let flash_handle = EffectModule::req_follow(boma, Hash40::new("sys_smash_flash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.5, false, 0, 0, 0, 0 ,0, false, false);
            LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.15, 10.0);
            EffectModule::set_rgb(boma, flash_handle as u32, 0.15, 0.15, 10.0);
            EffectModule::set_rate(boma, flash_handle as u32, 0.3);
            VarModule::set_int64(gunner, vars::miigunner::instance::STEALTHBOMB_EFF_HANDLER, flash_handle);
        }
	}
    for h in 101..=120 {
		if is_excute(fighter) {
			let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    		if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
        		let gunner = utils::util::get_battle_object_from_id(owner_id);
				let flash_handle = VarModule::get_int64(gunner, vars::miigunner::instance::STEALTHBOMB_EFF_HANDLER);
				let start_color = Vector3f { x: 0.15, y: 0.15, z: 10.0 };
                let end_color = Vector3f { x: 10.0, y: 0.15, z: 0.15 };
                // Smoothly interpolate from starting to ending color
                let blend_vector = Vector3f {
                    x: start_color.x + ((end_color.x - start_color.x) * (((h as f32) - 100.0) / 20.0)),
                    y: start_color.y + ((end_color.y - start_color.y) * (((h as f32) - 100.0) / 20.0)),
                    z: start_color.z + ((end_color.z - start_color.z) * (((h as f32) - 100.0) / 20.0))
                };
                // Apply color blend
                EffectModule::set_rgb(boma, flash_handle as u32, blend_vector.x, blend_vector.y, blend_vector.z);
			}
		}
		wait(lua_state, 1.0);
	}
}

#[acmd_script( agent = "miigunner_stealthbomb_s", script = "game_move" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_stealthbomb_s_move_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 65, 94, 0, 48, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		//AREA_WIND_2ND_RAD(0, 1, 0.02, 1000, 1, 0, 0, 12);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		//AreaModule::erase_wind(boma, 0);
	}
    
}

#[acmd_script( agent = "miigunner_stealthbomb_s", script = "effect_move" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_stealthbomb_s_move_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
	}
    
}

#[acmd_script( agent = "miigunner_supermissile", script = "game_straight", category = ACMD_GAME, low_priority)]
unsafe fn miigunner_supermissile_straight_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
       let gunner = utils::util::get_battle_object_from_id(owner_id);
       VarModule::set_int(gunner, vars::miigunner::instance::MISSILE_OBJECT_ID, fighter.battle_object_id as i32);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 50, 10, 0, 30, 2.0, 0.0, 0.0, 1.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
       if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
           let gunner = utils::util::get_battle_object_from_id(owner_id);
           VarModule::on_flag(gunner, vars::miigunner::instance::DETONATE_READY);
       }
    }
}

#[acmd_script( agent = "miigunner_supermissile", script = "game_sburst", category = ACMD_GAME, low_priority)]
unsafe fn miigunner_supermissile_burst_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
       let gunner = utils::util::get_battle_object_from_id(owner_id);
       VarModule::off_flag(gunner, vars::miigunner::instance::DETONATE_READY);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        let gunner = utils::util::get_battle_object_from_id(owner_id);
        if VarModule::is_flag(gunner, vars::miigunner::status::MISSILE_DETONATE) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 50, 68, 0, 70, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BOMB);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

#[acmd_script( agent = "miigunner_supermissile", script = "effect_sburst", category = ACMD_EFFECT, low_priority)]
unsafe fn miigunner_supermissile_burst_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if is_excute(fighter) {
       if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
           let gunner = utils::util::get_battle_object_from_id(owner_id);
           if VarModule::is_flag(gunner, vars::miigunner::status::MISSILE_DETONATE) {
               EFFECT(fighter, Hash40::new("miigunner_atk_shot5"), Hash40::new("top"), -14, 0, 0, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, false);
               LAST_EFFECT_SET_COLOR(fighter, 0.5, 10.0, 25.0);
           }
           else {
               EFFECT(fighter, Hash40::new("sys_misfire"), Hash40::new("top"), 0, -1, 2, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, false);
               EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
               LAST_EFFECT_SET_RATE(fighter, 1.5);
           }
       }
    }
}

#[acmd_script( agent = "miigunner_supermissile", script = "sound_sburst", category = ACMD_SOUND, low_priority)]
unsafe fn miigunner_supermissile_burst_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if is_excute(fighter) {
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
           let gunner = utils::util::get_battle_object_from_id(owner_id);
           if VarModule::is_flag(gunner, vars::miigunner::status::MISSILE_DETONATE) {
               PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_s03"));
               PLAY_SE_REMAIN(fighter, Hash40::new("se_common_bomb_l"));
           }
           else {
                PLAY_SE_REMAIN(fighter, Hash40::new("se_common_bomb_s"));
           }
        }
    }
}

#[acmd_script( agent = "miigunner_bottomshoot", script = "game_shoot" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_bottomshoot_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if is_excute(fighter) {
        if VarModule::get_float(owner_module_accessor.object(), vars::miigunner::status::CHARGE_ATTACK_LEVEL) <= 10.0 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 80, 35, 0, 120, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
            AttackModule::set_add_reaction_frame(boma, 0, -15.0, false);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 105, 0, 57, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
        }
	}
}

#[acmd_script( agent = "miigunner_gunnercharge", script = "game_shoot" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_gunnercharge_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 42, 0, 14, 1.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 21.0, 50, 80, 0, 27, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
		AttackModule::enable_safe_pos(boma);
	}
	for _ in 0..1000 {
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			let motion_vec = Vector3f{x: 0.5, y: 1.0, z: 1.0};
			KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
		}
	}
    
}

#[acmd_script( agent = "miigunner", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "miigunner", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
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
        dash_game,
		dash_sound,
        turn_dash_game,
		catch_game,
		catch_dash_game,
		catch_turn_game,
		miigunner_gunnercharge_shoot_game,
        miigunner_attackairf_bullet_fly_effect,
        miigunner_rapidshot_bullet_fly_game,
		miigunner_rapidshot_bullet_flythrowb_game,
        miigunner_grenadelauncher_explode_game,
		//miigunner_fullthrottle_final_game,
		miigunner_stealthbomb_tame_effect,
		//miigunner_stealthbomb_s_move_game,
		//miigunner_stealthbomb_s_move_effect,
		miigunner_supermissile_straight_game,
        miigunner_supermissile_burst_game,
        miigunner_supermissile_burst_effect,
        miigunner_supermissile_burst_sound,
        miigunner_bottomshoot_shoot_game,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound,
    );
}

