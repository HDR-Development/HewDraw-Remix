
use super::*;

#[acmd_script( agent = "miigunner", script = "game_specialn1start" , category = ACMD_GAME , low_priority)]
unsafe fn special_n1_start_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "miigunner", script = "game_specialairn1start" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_n1_start_game(fighter: &mut L2CAgentBase) {
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
unsafe fn special_n1_fire_max_game(fighter: &mut L2CAgentBase) {
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
			ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 361, 100, 0, 56, 7.0, 0.0, 7.5, 8.0, Some(0.0), Some(7.5), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 361, 100, 0, 56, 5.0, 0.0, 7.5, 8.0, Some(0.0), Some(7.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
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
unsafe fn special_n1_fire_max_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.2);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6.0, 6.0, 0, 0, 0, 0.85, true);
			LAST_EFFECT_SET_RATE(fighter, 0.7);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 10.0, 0.55);
		}
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
			LAST_EFFECT_SET_RATE(fighter, 0.6);
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
unsafe fn special_air_n1_fire_max_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.2);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6.0, 6.0, 0, 0, 0, 0.85, true);
			LAST_EFFECT_SET_RATE(fighter, 0.7);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 10.0, 0.55);
		}
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
			LAST_EFFECT_SET_RATE(fighter, 0.6);
		}
		else {
			EFFECT(fighter, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
	}

}

#[acmd_script( agent = "miigunner", script = "sound_specialn1firemax" , category = ACMD_SOUND , low_priority)]
unsafe fn special_n1_fire_max_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_special_c1_n01"));
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL) >= 20.0 && VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL) < 95.0 {
			PLAY_SE(fighter, Hash40::new("se_miigunner_special_n05"));
		}
	}

}

#[acmd_script( agent = "miigunner", script = "sound_specialairn1firemax" , category = ACMD_SOUND , low_priority)]
unsafe fn special_air_n1_fire_max_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_special_c1_n01"));
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL) >= 20.0 && VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL) < 95.0 {
			PLAY_SE(fighter, Hash40::new("se_miigunner_special_n05"));
		}
	}

}

#[acmd_script( agent = "miigunner", script = "game_specialn2loop" , category = ACMD_GAME , low_priority)]
unsafe fn special_n2_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1);
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		//WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_FULLTHROTTLE, false, 0);
		ArticleModule::change_motion(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_FULLTHROTTLE, Hash40::new("final_start"), true, 0.0);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ArticleModule::shoot_exist(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_FULLTHROTTLE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
		//WorkModule::off_flag(boma, *FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
	}
    
}

#[acmd_script( agent = "miigunner", script = "game_specialn3start" , category = ACMD_GAME , low_priority)]
unsafe fn special_n3_start_game(fighter: &mut L2CAgentBase) {
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
					FT_MOTION_RATE(fighter, 1.0);
				}
			}
		}
		wait(lua_state, 1.0);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 2.0);
	} 

}

#[acmd_script( agent = "miigunner", script = "game_specialairn3start" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_n3_start_game(fighter: &mut L2CAgentBase) {
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
					FT_MOTION_RATE(fighter, 1.0);
				}
			}
		}
		wait(lua_state, 1.0);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 2.0);
	} 

}

#[acmd_script( agent = "miigunner", scripts = ["game_specials1", "game_specialairs1"] , category = ACMD_GAME , low_priority)]
unsafe fn special_s1_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "miigunner", scripts = ["game_specialhi1", "game_specialairhi1"] , category = ACMD_GAME , low_priority)]
unsafe fn special_hi1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		VarModule::set_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL, 0.0);
		VarModule::off_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED);
	}
	frame(lua_state, 6.0);
	// Charge build loop
	for _ in 0..19 {
		if is_excute(fighter) {
			// If charge initiated and not finished
			if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED){
				// If holding down the button, increment the charge and continue the slowed animation
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					let motion_vec = Vector3f{x: 1.0, y: 0.35, z: 1.0};
					KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
					VarModule::add_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL, 1.0); // Increment the charge by 1
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

#[acmd_script( agent = "miigunner", script = "effect_specialhi1" , category = ACMD_EFFECT , low_priority)]
unsafe fn special_hi1_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("miigunner_bottom_shot"), Hash40::new("armr"), 6.5, 0, 0, 0, 0, 0, 1, true);
		EffectModule::enable_sync_init_pos_last(boma);
		//LAST_EFFECT_SET_RATE(fighter, 0.5);
		//EffectModule::set_frame(boma, arg2, arg3);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
	}
}

#[acmd_script( agent = "miigunner", scripts = ["game_specialhi2squat", "game_specialairhi2squat"] , category = ACMD_GAME , low_priority)]
unsafe fn special_hi2_squat_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 3.0);
	FT_MOTION_RATE(fighter, 0.5);
	frame(lua_state, 7.0);
	FT_MOTION_RATE(fighter, 0.25);
	frame(lua_state, 11.0);
	FT_MOTION_RATE(fighter, 1);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 270, 100, 0, 5, 9.0, 0.0, 0.0, 3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 110, 85, 40, 90, 9.0, 0.0, 0.0, 3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
}

#[acmd_script( agent = "miigunner", script = "game_specialhi2" , category = ACMD_GAME , low_priority)]
unsafe fn special_hi2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 100, 0, 83, 5.0, 0.0, 18.0, 1.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 75, 90, 0, 83, 4.0, 0.0, 18.0, 1.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
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
unsafe fn special_hi3_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.5);
	}
}

#[acmd_script( agent = "miigunner", script = "game_specialairhi3start" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_hi3_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.5);
	}
}

#[acmd_script( agent = "miigunner", script = "game_specialhi3" , category = ACMD_GAME , low_priority)]
unsafe fn special_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 0.85);
	}
}

#[acmd_script( agent = "miigunner", script = "game_speciallw1start" , category = ACMD_GAME , low_priority)]
unsafe fn special_lw1_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 0, 24, 0, 58, 8.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 24, 24, 0, 58, 8.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
	}
}

#[acmd_script( agent = "miigunner", script = "game_specialairlw1start" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_lw1_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 4.5, 3.0, 6.5, 4.0);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 10, 30, 0, 60, 8.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 24, 30, 0, 60, 8.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
	}
}

#[acmd_script( agent = "miigunner", scripts = ["game_speciallw3start", "game_specialairlw3start"] , category = ACMD_GAME , low_priority)]
unsafe fn special_lw3_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0/(6.0 - 1.0));
	}
}

#[acmd_script( agent = "miigunner", script = "game_speciallw3hold" , category = ACMD_GAME , low_priority)]
unsafe fn special_lw3_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 69, 100, 0, 70, 8.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
}

#[acmd_script( agent = "miigunner", script = "game_specialairlw3hold" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_lw3_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 69, 100, 0, 70, 8.0, 0.0, 9.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
}

#[acmd_script( agent = "miigunner", scripts = ["effect_speciallw3hold", "effect_specialairlw3hold"] , category = ACMD_EFFECT , low_priority)]
unsafe fn special_lw3_hold_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("miigunner_absorberlight"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.0, true);
		EffectModule::enable_sync_init_pos_last(boma);
	}
	COL_NORMAL(fighter);
	if is_excute(fighter) {
		FLASH(fighter, 0.5, 0.7, 1, 0.6);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		FLASH_FRM(fighter, 3, 0, 0.2, 0.5, 0);
	}
	wait(lua_state, 3.0);
}

#[acmd_script( agent = "miigunner", scripts = ["game_speciallw3end", "game_specialairlw3end"] , category = ACMD_GAME , low_priority)]
unsafe fn special_lw3_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

pub fn install() {
    install_acmd_scripts!(
		special_n1_start_game,
		special_n1_fire_max_game,
		special_n1_fire_max_effect,
		special_air_n1_fire_max_effect,
		special_n1_fire_max_sound,
		special_air_n1_start_game,
		special_air_n1_fire_max_sound,
		//special_n2_loop_game,
		special_n3_start_game,
		special_air_n3_start_game,
		special_s1_game,
		special_hi1_game,
		special_hi1_effect,
		special_hi2_squat_game,
		special_hi2_game,
		special_hi3_start_game,
		special_air_hi3_start_game,
		special_hi3_game,
		special_lw1_start_game,
		special_air_lw1_start_game,
		special_lw3_start_game,
		special_lw3_hold_game,
		special_air_lw3_hold_game,
		special_lw3_hold_effect,
		special_lw3_end_game,
	);
}

