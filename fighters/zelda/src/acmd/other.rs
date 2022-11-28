
use super::*;

#[acmd_script( agent = "zelda", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "zelda", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "zelda", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "zelda", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "zelda", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "zelda", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_zelda_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_STEP_FLIPPABLE(fighter, Hash40::new("se_zelda_step_left_l"), Hash40::new("se_zelda_step_right_l"));
    }
}

#[acmd_script( agent = "zelda", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "zelda", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.700);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        CATCH(fighter, 0, Hash40::new("top"), 4.8, 0.0, 9.0, 0.0, Some(0.0), Some(9.0), Some(10.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "zelda_dein_s", script = "game_move" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_dein_move_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 52, 97, 0, 50, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
		AREA_WIND_2ND_RAD_arg9(fighter, 0, 2, 0.05, 200, 1, 0, 0, 12, 60);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		AreaModule::erase_wind(boma, 0);
	}
}

#[acmd_script( agent = "zelda_dein", script = "effect_tame" , category = ACMD_EFFECT , low_priority)]
unsafe fn zelda_dein_tame_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_bullet_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.25, 0.25);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_COLOR(fighter, 0.885, 0.051, 0.051);
	}
	frame(lua_state, 76.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("zelda_atk"), Hash40::new("top"), 0, 0, 0, 90, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_COLOR(fighter, 3.0, 0.5, 0.5);
	}
}

#[acmd_script( agent = "zelda_phantom", script = "game_build" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_phantom_build_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 0.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 19.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 27.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 44.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 49.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}
	frame(lua_state, 180.0);
	if is_excute(fighter) {
		WorkModule::inc_int(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_BUILD_NEXT);
	}   
}

#[acmd_script( agent = "zelda_phantom", script = "game_attackkick" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_phantom_attack_kick_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let rush_speed = 4.0;
	frame(lua_state, 0.0);
	if is_excute(fighter) {
		KineticModule::unable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
		FT_MOTION_RATE(fighter, 6.0/(5.0-0.0));
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		KineticModule::enable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
		FT_MOTION_RATE(fighter, 1.0);
		fighter.clear_lua_stack();
		lua_args!(fighter, WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL, rush_speed * PostureModule::lr(boma));
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 50, 0, 50, 5.5, 0.0, 6.0, 14.0, Some(0.0), Some(6.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
	}
	wait(lua_state, 6.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}

}

#[acmd_script( agent = "zelda_phantom", script = "game_attackpunch" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_phantom_attack_punch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let rush_speed = 4.5;
	frame(lua_state, 0.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 60, 0, 4.0, 0.0, 7.0, 11.0, Some(0.0), Some(7.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 6.0, 0.0, 7.0, 11.0, Some(0.0), Some(7.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		FT_MOTION_RATE(fighter, 2.0/(0.5-0.0));
		KineticModule::unable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
	}
	frame(lua_state, 0.5);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0/(1.0-0.5));
	}
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		KineticModule::enable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
		FT_MOTION_RATE(fighter, 1.0);
		fighter.clear_lua_stack();
		lua_args!(fighter, WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL, rush_speed * PostureModule::lr(boma));
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 65, 0, 70, 5.5, 0.0, 10.0, 11.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 65, 0, 70, 4.5, 0.0, 9.5, 19.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
	}
		wait(lua_state, 7.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}

}

#[acmd_script( agent = "zelda_phantom", script = "game_attacks" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_phantom_attack_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let rush_speed = 5.0;
	frame(lua_state, 0.0);
	if is_excute(fighter) {
		//FT_MOTION_RATE(fighter, 5.0/(3.0-0.0));
		KineticModule::unable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 50, 0, 5.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 7.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		KineticModule::enable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
		fighter.clear_lua_stack();
		lua_args!(fighter, WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL, rush_speed * PostureModule::lr(boma));
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
		ATTACK(fighter, 0, 0, Hash40::new("handr"), 6.0, 87, 40, 0, 100, 5.8, 0.0, 0.0, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 1, 0, Hash40::new("handr"), 6.0, 87, 40, 0, 100, 5.8, 0.0, 0.0, 8.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 2, 0, Hash40::new("handr"), 6.0, 87, 40, 0, 100, 5.8, 0.0, 0.0, 16.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 87, 40, 0, 100, 6.5, 0.0, 8.0, 10.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	wait(lua_state, 5.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
}

#[acmd_script( agent = "zelda_phantom", script = "effect_attacks" , category = ACMD_EFFECT , low_priority)]
unsafe fn zelda_phantom_attack_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 1.0, 20.0, -10.0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_COLOR(fighter, 0.885, 0.051, 0.051);
		LAST_EFFECT_SET_RATE(fighter, 0.75);
		if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		}
		//EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 1.0, 0, 0, 0, 1, true);
		//LAST_EFFECT_SET_RATE(fighter, 0.5);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 8.0, 0, 0, 0, 0.5, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 16.0, 0, 0, 0, 0.75, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if PostureModule::lr(boma) > 0.0 {
			EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_line"), Hash40::new("top"), -5, 10, -5, 0, 0, 0, 1, true);
		}
		else{
			EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_line"), Hash40::new("top"), 5, 10, -5, 0, 0, 0, 1, true);		
		}
		AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_zelda_phantomsword1"), Hash40::new("tex_zelda_phantomsword2"), 6, Hash40::new("handr"), 1.5, 0.4, -1.0, Hash40::new("handr"), 1.5, 0.4, 24.4, true, Hash40::new("zelda_phantom_sword"), Hash40::new("handr"), 1.85, 0.35, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.5);

		//EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 1.0, 0, 0, 0, 1, true);
		//LAST_EFFECT_SET_RATE(fighter, 0.75);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 8.0, 0, 0, 0, 0.5, true);
		LAST_EFFECT_SET_RATE(fighter, 0.75);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 16.0, 0, 0, 0, 0.75, true);
		LAST_EFFECT_SET_RATE(fighter, 0.75);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("zelda_phantom_line"), false, false);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		AFTER_IMAGE_OFF(fighter, 4);
	}
	frame(lua_state, 55.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("zelda_phantom_end"), Hash40::new("top"), 0, 4, -3, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(fighter, 1.4);
	}
}

#[acmd_script( agent = "zelda_phantom", script = "game_attackl" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_phantom_attack_l_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let rush_speed = 5.5;
	frame(lua_state, 0.0);
	if is_excute(fighter) {
		//FT_MOTION_RATE(fighter, 20.0/3.0);
		KineticModule::unable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 110, 0, 5.5, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 100, 80, 0, 7.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		KineticModule::enable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
		fighter.clear_lua_stack();
		lua_args!(fighter, WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL, rush_speed * PostureModule::lr(boma));
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
		// Air-only
		ATTACK(fighter, 0, 0, Hash40::new("handr"), 8.0, 361, 40, 0, 60, 5.0, 2.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 1, 0, Hash40::new("handr"), 8.0, 361, 40, 0, 60, 5.6, 2.0, 0.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 2, 0, Hash40::new("handr"), 8.0, 361, 40, 0, 60, 5.6, 2.0, 0.0, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		//ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 361, 57, 0, 60, 5.0, 0.0, 8.5, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		// Ground-only
		ATTACK(fighter, 3, 0, Hash40::new("handr"), 8.0, 262, 40, 0, 90, 5.0, 2.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 4, 0, Hash40::new("handr"), 8.0, 262, 40, 0, 90, 5.6, 2.0, 0.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 5, 0, Hash40::new("handr"), 8.0, 262, 40, 0, 90, 5.6, 2.0, 0.0, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		//ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 270, 40, 0, 120, 5.0, 0.0, 8.5, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	wait(lua_state, 7.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
}

#[acmd_script( agent = "zelda_phantom", script = "effect_attackl" , category = ACMD_EFFECT , low_priority)]
unsafe fn zelda_phantom_attack_l_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 1.0, 20.0, -10.0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.0, 1.0);
		LAST_EFFECT_SET_RATE(fighter, 0.75);
		if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
		}
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("toel"), 0, 0, 0, 180, 0, 0, 0.9, true);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("toer"), 0, 0, 0, 180, 0, 0, 0.9, true);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 90, 1, true);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 90, 1, true);

		/*
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("handr"), 0, 0, 1.0, 0, 0, 0, 3.0, true);
		LAST_EFFECT_SET_RATE(fighter, 0.75);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("handr"), 0, 0, 8.0, 0, 0, 0, 3.0, true);
		LAST_EFFECT_SET_RATE(fighter, 0.75);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("handr"), 0, 0, 16.0, 0, 0, 0, 3.0, true);
		LAST_EFFECT_SET_RATE(fighter, 0.75);
		*/

		EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 1.0, 0, 0, 0, 0.35, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
		LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.0, 1.0);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 8.0, 0, 0, 0, 0.5, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
		LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.0, 1.0);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 16.0, 0, 0, 0, 0.75, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
		LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.0, 1.0);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if PostureModule::lr(boma) > 0.0 {
			EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_line"), Hash40::new("top"), -5, 10, -5, 0, 0, 0, 1, true);
		}
		else{
			EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_line"), Hash40::new("top"), 5, 10, -5, 0, 0, 0, 1, true);
		}
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_sword"), Hash40::new("handr"), 1.85, 0.35, 0, 0, 0, 0, 1, true);
		AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_zelda_phantomsword1"), Hash40::new("tex_zelda_phantomsword2"), 16, Hash40::new("handr"), 1.5, 0.4, -1.0, Hash40::new("handr"), 1.5, 0.4, 24.4, true, Hash40::new("null"), Hash40::new("handr"), 1.85, 0.35, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.7);
		if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
		}
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 1.0, 0, 0, 0, 0.35, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
		LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.0, 1.0);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 8.0, 0, 0, 0, 0.5, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
		LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.0, 1.0);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_din_start"), Hash40::new("handr"), 0, 0, 16.0, 0, 0, 0, 0.75, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
		LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.0, 1.0);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		AFTER_IMAGE_OFF(fighter, 0);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_sword_trace"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
		}
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		//QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
		QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("zelda_phantom_line"), false, false);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("zelda_phantom_sword"), true, true);
	}
	frame(lua_state, 64.0);
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("zelda_phantom_body_aura"), false, true);
	}
	frame(lua_state, 74.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("zelda_phantom_end"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
}

#[acmd_script( agent = "zelda_phantom", script = "game_attackmax" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_phantom_attack_max_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let rush_speed = 6.5;
	frame(lua_state, 0.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 11.0/(3.0-0.0));
		KineticModule::unable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 130, 0, 6.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 6, 100, 85, 0, 8.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		KineticModule::enable_energy(boma, *WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL);
		fighter.clear_lua_stack();
		lua_args!(fighter, WEAPON_ZELDA_PHANTOM_KINETIC_ENERGY_ID_NORMAL, rush_speed * PostureModule::lr(boma));
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
		ATTACK(fighter, 0, 0, Hash40::new("handr"), 11.0, 46, 80, 0, 60, 6.0, 2.0, 0.0, 1.5, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 1, 0, Hash40::new("handr"), 11.0, 46, 80, 0, 60, 6.0, 2.0, 1.0, 9.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 2, 0, Hash40::new("handr"), 11.0, 46, 80, 0, 60, 6.0, 2.0, 2.0, 16.5, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 3, 0, Hash40::new("shoulderr"), 11.0, 46, 80, 0, 60, 6.0, 0.0, 0.0, 0.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
}

#[acmd_script( agent = "zelda_phantom", script = "effect_attackmax" , category = ACMD_EFFECT , low_priority)]
unsafe fn zelda_phantom_attack_max_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 1.0, 20.0, -10.0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(fighter, 0.75);
		if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
		}
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("toel"), 0, 0, 0, 180, 0, 0, 1.5, true);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("toer"), 0, 0, 0, 180, 0, 0, 1.5, true);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 90, 1.5, true);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 90, 1.5, true);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 90, 1.3, true);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 90, 1.3, true);
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_body_aura"), Hash40::new("hip"), 0, 0, 0, 0, 0, 90, 2, true);
		EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("zelda_phantom_start"), Hash40::new("trans"), 0, 3, 0, -90, 0, 0, 1, true);
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		if PostureModule::lr(boma) > 0.0 {
			EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_line"), Hash40::new("top"), -5, 10, -7, 0, 0, 0, 1, true);
		}
		else{
			EFFECT_FOLLOW(fighter, Hash40::new("zelda_phantom_line"), Hash40::new("top"), 5, 10, -7, 0, 0, 0, 1, true);
		}
		
		EFFECT_FOLLOW(fighter, Hash40::new("zelda_atk_hi_flash"), Hash40::new("handr"), 0, 0, 16.0, 0, 0, 0, 2.0, true);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
		AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_zelda_phantomsword1"), Hash40::new("tex_zelda_phantomsword2"), 9, Hash40::new("handr"), 1.5, 0.4, -1.0, Hash40::new("handr"), 1.5, 0.4, 24.4, true, Hash40::new("zelda_phantom_sword"), Hash40::new("handr"), 1.85, 0.35, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.5);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 7, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
			LAST_EFFECT_SET_ALPHA(fighter, 0.6);
		}
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("zelda_phantom_line"), false, false);
	}
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		AFTER_IMAGE_OFF(fighter, 8);
	}
	frame(lua_state, 119.0);
	if is_excute(fighter) {
		EFFECT_DETACH_KIND(fighter, Hash40::new("zelda_phantom_body_aura"), 32);
		EFFECT(fighter, Hash40::new("zelda_phantom_build"), Hash40::new("trans"), 0, 3, 0, 0, -90, 0, 1, 0, 0, 0, 0, 0, 0, true);
		EFFECT(fighter, Hash40::new("zelda_phantom_end"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, true);
	}
}

#[acmd_script( agent = "zelda", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "zelda", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    install_acmd_scripts!(
        escape_air_game,
        escape_air_slide_game,
		dash_sound,
        zelda_turn_dash_game,
		zelda_catch_game,
		zelda_dein_move_game,
		zelda_dein_tame_effect,
		zelda_phantom_build_game,
		zelda_phantom_attack_kick_game,
		zelda_phantom_attack_punch_game,
		zelda_phantom_attack_s_game,
		zelda_phantom_attack_s_effect,
		zelda_phantom_attack_l_game,
		zelda_phantom_attack_l_effect,
		zelda_phantom_attack_max_game,
		zelda_phantom_attack_max_effect,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound
    );
}

