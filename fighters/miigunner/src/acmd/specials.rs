
use super::*;

#[acmd_script( agent = "miigunner", scripts = ["game_specialn1start", "game_specialairn1start"] , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_n1_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		VarModule::set_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL, 0.0);
		VarModule::off_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		ArticleModule::generate_article_enable(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, false, 0);
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_BULLET_DISP);
	}

}

#[acmd_script( agent = "miigunner", scripts = ["game_specialn1firemax", "game_specialairn1firemax"] , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_n1_fire_max_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
	}
	frame(lua_state, 1.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
			WorkModule::off_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_BULLET_DISP);
			FT_MOTION_RATE(fighter, 10.0/(3.0 - 1.0));
        }
    }
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		// Melee blast
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			ArticleModule::remove_exist(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_SHOOT);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 361, 100, 0, 56, 7.0, 0.0, 7.5, 8.0, Some(0.0), Some(7.5), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 361, 100, 0, 56, 5.0, 0.0, 7.5, 8.0, Some(0.0), Some(7.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
			FT_MOTION_RATE(fighter, 5.0/(5.0 - 3.0));
		}
		// Normal
		else {
			ArticleModule::shoot_exist(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
			WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_SHOOT);
		}
	} 
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 37.0/(40.0 - 8.0));
		}
	} 
}

#[acmd_script( agent = "miigunner", script = "effect_specialn1firemax" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_special_n1_fire_max_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 0, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(fighter, 2.0);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 100.0, 10.0);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 90, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(fighter, 2.0);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 100.0, 3.0);
		}
	}
	frame(lua_state, 2.6);
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("miigunner_sb_tama"), false, false);
		EFFECT_DETACH_KIND(fighter, Hash40::new("miigunner_sb_tama"), -1);
	}
	frame(lua_state, 2.8);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_air_bullet"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 5.0, 0.55);
			LAST_EFFECT_SET_RATE(fighter, 0.85);
		}
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, false);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_laser"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_SCALE_W(fighter, 1.0, 0.7, 1.0);
			LAST_EFFECT_SET_RATE(fighter, 0.8);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_shot_s"), Hash40::new("armr"), 6.3, 0, 0, 0, 0, -90, 1, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 5.0, 5.0);
			LAST_EFFECT_SET_RATE(fighter, 0.5);
			LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
		}
		else {
			EFFECT(fighter, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
	}

}

#[acmd_script( agent = "miigunner", script = "effect_specialairn1firemax" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_special_air_n1_fire_max_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 0, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(fighter, 2.0);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 100.0, 5.0);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 90, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(fighter, 2.0);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 100.0, 3.0);
		}
	}
	frame(lua_state, 2.6);
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("miigunner_sb_tama"), false, false);
		EFFECT_DETACH_KIND(fighter, Hash40::new("miigunner_sb_tama"), -1);
	}
	frame(lua_state, 2.8);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_air_bullet"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 5.0, 0.55);
			LAST_EFFECT_SET_RATE(fighter, 0.85);
		}
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, false);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_laser"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_SCALE_W(fighter, 1.0, 0.7, 1.0);
			LAST_EFFECT_SET_RATE(fighter, 0.8);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_shot_s"), Hash40::new("armr"), 6.3, 0, 0, 0, 0, -90, 1, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 5.0, 5.0);
			LAST_EFFECT_SET_RATE(fighter, 0.5);
		}
		else {
			EFFECT(fighter, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
	}

}

#[acmd_script( agent = "miigunner", scripts = ["sound_specialn1firemax", "sound_specialairn1firemax"] , category = ACMD_SOUND , low_priority)]
unsafe fn miigunner_special_n1_fire_max_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_special_c1_n01"));
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			PLAY_SE(fighter, Hash40::new("se_miigunner_final01"));
		}
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			STOP_SE(fighter, Hash40::new("se_miigunner_final01"));
			PLAY_SE(fighter, Hash40::new("se_miigunner_final04"));
		}
	}

}

#[acmd_script( agent = "miigunner", scripts = ["game_specialn2loop", "game_specialairn2loop"] , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_n2_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 45, 100, 0, 30, 4.0, 2.0, 0.0, 0.0, Some(4.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, false, 0);
	}
    frame(lua_state, 6.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}

}

#[acmd_script( agent = "miigunner", scripts = ["game_specialn3start", "game_specialairn3start"] , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_n3_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		VarModule::set_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL, 0.0);
		VarModule::off_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED);
	}
	frame(lua_state, 10.0);
	// Charge build loop
	for _ in 0..10 { // F10-20 - 10 charge levels in total
		if is_excute(fighter) {
			// If charge initiated and not finished
			if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED){
				// If holding down the button, increment the charge and continue the slowed animation
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					VarModule::add_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL, 1.0); // Increment the charge by 1
					FT_MOTION_RATE(fighter, 2.0);
				}
				// If no longer holding the button, play out the rest of the animation as normal
				else{
					VarModule::on_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED);
					FT_MOTION_RATE(fighter, 0.5);
				}
			}
		}
		wait(lua_state, 1.0);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
	}

}

#[acmd_script( agent = "miigunner", scripts = ["game_specialn3end", "game_specialairn3end"] , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_n3_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		ArticleModule::generate_article_enable(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GRENADELAUNCHER, false, 0);
		FT_MOTION_RATE(fighter, 30.0/(23.0 - 1.0));
	}
}

#[acmd_script( agent = "miigunner", scripts = ["game_specials1", "game_specialairs1"] , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_s1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 17.0/(21.0 - 1.0));
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 36.0/(64.0 - 21.0));
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_FLAMEPILLAR, false, 0);
	}
}

#[acmd_script( agent = "miigunner", scripts = ["game_specials32", "game_specialairs32"] , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_s3_super_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON);
	}
}

#[acmd_script( agent = "miigunner", scripts = ["game_specialhi1", "game_specialairhi1"] , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_hi1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		VarModule::set_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL, 0.0);
		VarModule::off_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		// Charge build loop
		for _ in 0..19 {
			if is_excute(fighter) {
				if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED){
					// If holding down the button, increment the charge and continue the slowed animation
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						let motion_vec = if VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL) <= 10.0
						{Vector3f{x: 1.0, y: 0.55, z: 1.0}}			// become floaty while charging
						else {Vector3f{x: 1.0, y: 0.35, z: 1.0}};	// slow descent a little bit more to get more height from higher charge
						KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
						// Increment the charge by 1
						VarModule::add_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL, 1.0);
						FT_MOTION_RATE(fighter, 10.0);
					}
					// If no longer holding the button, play out the rest of the animation as normal
					else{
						VarModule::on_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED);
						FT_MOTION_RATE(fighter, 1.0);
					}
				}
			}
			wait(lua_state, 0.1);
		}
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_BOTTOM_SHOOT_FLAG_JUMP);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_BOTTOMSHOOT, false, 0);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
}

#[acmd_script( agent = "miigunner", scripts = ["effect_specialhi1", "effect_specialairhi1"] , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_special_hi1_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		// Store effect handle for later modification
		let handle = EffectModule::req_follow(boma, Hash40::new("miigunner_bottom_shot"), Hash40::new("armr"), &Vector3f::new(6.5, 0.0, 0.0), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
		VarModule::set_int64(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER, handle);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		let handle = VarModule::get_int64(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER);
		// Charge build loop
		for _ in 0..19 {
			if is_excute(fighter) {
				if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED){
					// why you no work
					// if 10.0 <= VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL) {
					// 	EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6, 6, 0, 0, 0, 1.11, true);
					// }
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						// Why did this work so well
						EffectModule::set_rate(boma, handle as u32, 1.0/fighter.motion_frame());
					}
					else{
						EffectModule::set_rate(boma, handle as u32, 1.0);
					}
				}
			}
		}
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		let handle = VarModule::get_int64(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER);
		EffectModule::set_rate(boma, handle as u32, 1.0);
		// Recolor launch effect if actionable
		if 10.0 >= VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL) && !VarModule::is_flag(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_AIR_USED) {
			EffectModule::set_rgb(boma, handle as u32, 0.15, 0.55, 10.0);
		}
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
	}
}

#[acmd_script( agent = "miigunner", script = "effect_landingfallspecial" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_special_hi1_landing_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		EFFECT_OFF_KIND(fighter, Hash40::new("miigunner_bottom_shot"), false, false);
	}
}

#[acmd_script( agent = "miigunner", scripts = ["game_specialhi2squat", "game_specialairhi2squat"] , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_hi2_squat_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 3.0);
	FT_MOTION_RATE(fighter, 0.5);
	frame(lua_state, 7.0);
	FT_MOTION_RATE(fighter, 0.25);
	frame(lua_state, 11.0);
	FT_MOTION_RATE(fighter, 1);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 110, 85, 40, 90, 9.0, 0.0, 0.0, 3.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
}

#[acmd_script( agent = "miigunner", script = "game_specialhi2" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_hi2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 80, 100, 0, 83, 5.0, 0.0, 18.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 75, 90, 0, 83, 4.0, 0.0, 18.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 27.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

#[acmd_script( agent = "miigunner", script = "game_specialhi3start" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_hi3_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.5);
	}
}

#[acmd_script( agent = "miigunner", script = "game_specialairhi3start" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_air_hi3_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.5);
	}
}

#[acmd_script( agent = "miigunner", script = "game_specialhi3" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 0.85);
	}
}

#[acmd_script( agent = "miigunner", script = "game_speciallw1start" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_lw1_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 69, 100, 0, 70, 8.0, 0.0, 9.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
	}
}

#[acmd_script( agent = "miigunner", script = "game_specialairlw1start" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_air_lw1_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 4.5, 3.0, 6.5, 4.0);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 69, 100, 0, 70, 8.0, 0.0, 9.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
	}
}

#[acmd_script( agent = "miigunner", scripts = ["game_speciallw3start", "game_specialairlw3start"] , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_lw3_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 4.0/(6.0 - 1.0));
	}
}

#[acmd_script( agent = "miigunner", scripts = ["game_speciallw3hold", "game_specialairlw3hold"] , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_lw3_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		let offset_y = if fighter.is_situation(*SITUATION_KIND_GROUND) { 6.5 } else { 9.5 };
		ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 0, 15, 0, 59, 12.5, 0.0, offset_y, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 24, 15, 0, 59, 12.5, 0.0, offset_y, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
}

#[acmd_script( agent = "miigunner", scripts = ["game_speciallw3end", "game_specialairlw3end"] , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_special_lw3_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

pub fn install() {
    install_acmd_scripts!(
		miigunner_special_n1_start_game,
		miigunner_special_n1_fire_max_game,
		miigunner_special_n1_fire_max_effect,
		miigunner_special_air_n1_fire_max_effect,
		miigunner_special_n1_fire_max_sound,
		miigunner_special_n2_loop_game,
		miigunner_special_n3_start_game,
		miigunner_special_n3_end_game,
		miigunner_special_s1_game,
		miigunner_special_s3_super_game,
		miigunner_special_hi1_game,
		miigunner_special_hi1_effect,
		miigunner_special_hi1_landing_effect,
		miigunner_special_hi2_squat_game,
		miigunner_special_hi2_game,
		miigunner_special_hi3_start_game,
		miigunner_special_air_hi3_start_game,
		miigunner_special_hi3_game,
		miigunner_special_lw1_start_game,
		miigunner_special_air_lw1_start_game,
		miigunner_special_lw3_start_game,
		miigunner_special_lw3_hold_game,
		miigunner_special_lw3_end_game,
	);
}

