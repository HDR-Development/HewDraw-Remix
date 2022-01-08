
use smash::app::{sv_system, sv_animcmd::{frame, wait}, self, lua_bind::*};
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::utility::*;
use smash_script::*;
use smashline::*;
use smash_script::macros::*;
use crate::utils::hdr;
use crate::vars::*;


#[acmd_script( agent = "miigunner", script = "game_specialn1start" , category = ACMD_GAME , low_priority)]
unsafe fn special_n1_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		charge_attack_level[hdr::get_player_number(boma)] = 0.0;
		charge_finished[hdr::get_player_number(boma)] = false;
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		ArticleModule::generate_article_enable(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, false, 0);
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_BULLET_DISP);
	} 

}

#[acmd_script( agent = "miigunner", script = "game_specialairn1start" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_n1_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		charge_attack_level[hdr::get_player_number(boma)] = 0.0;
		charge_finished[hdr::get_player_number(boma)] = false;
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		ArticleModule::generate_article_enable(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, false, 0);
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_BULLET_DISP);
	} 

}

#[acmd_script( agent = "miigunner", script = "game_specialn1firemax" , category = ACMD_GAME , low_priority)]
unsafe fn special_n1_fire_max_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		charge_attack_level[hdr::get_player_number(boma)] = 0.0;
		charge_finished[hdr::get_player_number(boma)] = false;
	}
	frame(lua_state, 1.0);
	// Charge build loop
	for _ in 0..99 { // F1-2 - 100 charge levels in total
		if is_excute(fighter) {
			// If charge initiated and not finished
			if !charge_finished[hdr::get_player_number(boma)]{
				// If holding down the button, increment the charge and continue the slowed animation
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					charge_attack_level[hdr::get_player_number(boma)] += 1.0; // Increment the charge by 1
					FT_MOTION_RATE(fighter, 100.0);
				}
				// If no longer holding the button, play out the rest of the animation as normal
				else{
					charge_finished[hdr::get_player_number(boma)] = true;
					FT_MOTION_RATE(fighter, 1.0);
				}
			}
		}
		wait(lua_state, 0.01);
	}
	frame(lua_state, 2.01);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
	} 
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		if charge_attack_level[hdr::get_player_number(boma)] < 10.0 {
			ArticleModule::shoot_exist(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
			WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_SHOOT);
		}
		else if charge_attack_level[hdr::get_player_number(boma)] >= 10.0 {
			WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_BULLET_DISP);
			let blast_damage = 16.0 + ((charge_attack_level[hdr::get_player_number(boma)] - 10.0) * 10.0 / (100.0 - 20.0));
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ blast_damage, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 56, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(10.0), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_ENERGY);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ blast_damage - 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 56, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(50.0), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_ENERGY);
		}
	} 
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	} 
}

#[acmd_script( agent = "miigunner", script = "effect_specialn1firemax" , category = ACMD_EFFECT , low_priority)]
unsafe fn special_n1_fire_max_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.20);
	if is_excute(fighter) {
		// Flash to signify first charge level reached
		if !charge_finished[hdr::get_player_number(boma)] {
			EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 5.0, 3.0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_RATE(fighter, 0.45);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 10.0, 0.55);
		}
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if charge_attack_level[hdr::get_player_number(boma)] < 10.0 {
			EFFECT(fighter, Hash40::new_raw(0x143f92a0db), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		else if charge_attack_level[hdr::get_player_number(boma)] >= 10.0 {
			EFFECT(fighter, Hash40::new_raw(0x143f92a0db), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			
			//EFFECT_FLW_POS(fighter, Hash40::new_raw(0x13afa10b1e), Hash40::new("armr"), 6.3, 0, 0, 0, 0, 0, 3.0, true);
			EFFECT(fighter, Hash40::new_raw(0x13afa10b1e), Hash40::new("armr"), 6.3, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
			FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
			
		}
	}

}

#[acmd_script( agent = "miigunner", script = "sound_specialn1firemax" , category = ACMD_SOUND , low_priority)]
unsafe fn special_n1_fire_max_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_special_c1_n01"));
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if charge_attack_level[hdr::get_player_number(boma)] >= 20.0 && charge_attack_level[hdr::get_player_number(boma)] < 95.0 {
			PLAY_SE(fighter, Hash40::new("se_miigunner_special_n05"));
		}
	}

}

#[acmd_script( agent = "miigunner", script = "game_specialairn1firemax" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_n1_fire_max_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		charge_attack_level[hdr::get_player_number(boma)] = 0.0;
		charge_finished[hdr::get_player_number(boma)] = false;
	}
	frame(lua_state, 1.0);
	// Charge build loop
	for _ in 0..99 { // F1-2 - 100 charge levels in total
		if is_excute(fighter) {
			// If charge initiated and not finished
			if !charge_finished[hdr::get_player_number(boma)]{
				// If holding down the button, increment the charge and continue the slowed animation
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					charge_attack_level[hdr::get_player_number(boma)] += 1.0; // Increment the charge by 1
					FT_MOTION_RATE(fighter, 100.0);
				}
				// If no longer holding the button, play out the rest of the animation as normal
				else{
					charge_finished[hdr::get_player_number(boma)] = true;
					FT_MOTION_RATE(fighter, 1.0);
				}
			}
		}
		wait(lua_state, 0.01);
	}
	frame(lua_state, 2.01);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
	} 
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		if charge_attack_level[hdr::get_player_number(boma)] < 20.0 {
			ArticleModule::shoot_exist(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
			WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_SHOOT);
		}
		else if charge_attack_level[hdr::get_player_number(boma)] >= 20.0 && charge_attack_level[hdr::get_player_number(boma)] < 20.0 {
			WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_BULLET_DISP);
			let blast_damage = 16.0 + ((charge_attack_level[hdr::get_player_number(boma)] - 25.0) * 10.0 / (95.0 - 25.0));
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ blast_damage, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 56, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(10.0), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_ENERGY);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ blast_damage - 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 56, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(50.0), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_ENERGY);
		}
	} 
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	} 
}

#[acmd_script( agent = "miigunner", script = "effect_specialairn1firemax" , category = ACMD_EFFECT , low_priority)]
unsafe fn special_air_n1_fire_max_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.20);
	if is_excute(fighter) {
		// Flash to signify first charge level reached
		if !charge_finished[hdr::get_player_number(boma)] {
			EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 5.0, 3.0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_RATE(fighter, 0.45);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 10.0, 0.55);
		}
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if charge_attack_level[hdr::get_player_number(boma)] < 20.0 {
			EFFECT(fighter, Hash40::new_raw(0x143f92a0db), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		else if charge_attack_level[hdr::get_player_number(boma)] >= 20.0 && charge_attack_level[hdr::get_player_number(boma)] < 95.0 {
			EFFECT(fighter, Hash40::new_raw(0x143f92a0db), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			
			//EFFECT_FLW_POS(fighter, Hash40::new_raw(0x13afa10b1e), Hash40::new("armr"), 6.3, 0, 0, 0, 0, 0, 3.0, true);
			EFFECT(fighter, Hash40::new_raw(0x13afa10b1e), Hash40::new("armr"), 6.3, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
			FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
			
		}
	}

}

#[acmd_script( agent = "miigunner", script = "sound_specialairn1firemax" , category = ACMD_SOUND , low_priority)]
unsafe fn special_air_n1_fire_max_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_special_c1_n01"));
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if charge_attack_level[hdr::get_player_number(boma)] >= 20.0 && charge_attack_level[hdr::get_player_number(boma)] < 95.0 {
			PLAY_SE(fighter, Hash40::new("se_miigunner_special_n05"));
		}
	}

}

#[acmd_script( agent = "miigunner", script = "game_specialn2loop" , category = ACMD_GAME , low_priority)]
unsafe fn special_n2_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, /*FSM*/ 1);
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		//WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_FULLTHROTTLE, false, 0);
		ArticleModule::change_motion(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_FULLTHROTTLE, Hash40::new("final_start"), true, 0.0);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ArticleModule::shoot_exist(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_FULLTHROTTLE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
		//WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
	}
    
}

#[acmd_script( agent = "miigunner", script = "game_specialn3start" , category = ACMD_GAME , low_priority)]
unsafe fn special_n3_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		charge_attack_level[hdr::get_player_number(boma)] = 0.0;
		charge_finished[hdr::get_player_number(boma)] = false;
	}
	frame(lua_state, 10.0);
	// Charge build loop
	for _ in 0..10 { // F10-20 - 10 charge levels in total
		if is_excute(fighter) {
			// If charge initiated and not finished
			if !charge_finished[hdr::get_player_number(boma)]{
				// If holding down the button, increment the charge and continue the slowed animation
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					charge_attack_level[hdr::get_player_number(boma)] += 1.0; // Increment the charge by 1
					FT_MOTION_RATE(fighter, 2.0);
				}
				// If no longer holding the button, play out the rest of the animation as normal
				else{
					charge_finished[hdr::get_player_number(boma)] = true;
					FT_MOTION_RATE(fighter, 1.0);
				}
			}
		}
		wait(lua_state, 1.0);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
	} 

}

#[acmd_script( agent = "miigunner", script = "game_specialairn3start" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_n3_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		charge_attack_level[hdr::get_player_number(boma)] = 0.0;
		charge_finished[hdr::get_player_number(boma)] = false;
	}
	frame(lua_state, 10.0);
	// Charge build loop
	for _ in 0..10 { // F10-20 - 10 charge levels in total
		if is_excute(fighter) {
			// If charge initiated and not finished
			if !charge_finished[hdr::get_player_number(boma)]{
				// If holding down the button, increment the charge and continue the slowed animation
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					charge_attack_level[hdr::get_player_number(boma)] += 1.0; // Increment the charge by 1
					FT_MOTION_RATE(fighter, 2.0);
				}
				// If no longer holding the button, play out the rest of the animation as normal
				else{
					charge_finished[hdr::get_player_number(boma)] = true;
					FT_MOTION_RATE(fighter, 1.0);
				}
			}
		}
		wait(lua_state, 1.0);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
	} 

}

#[acmd_script( agent = "miigunner", script = "game_specialhi1" , category = ACMD_GAME , low_priority)]
unsafe fn special_hi1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		charge_attack_level[hdr::get_player_number(boma)] = 0.0;
		charge_finished[hdr::get_player_number(boma)] = false;
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
	}
	wait(lua_state, 1.0);
	// Charge build loop
	for _ in 0..29 { // F4-6 - 30 charge levels in total
		if is_excute(fighter) {
			// If charge initiated and not finished
			if !charge_finished[hdr::get_player_number(boma)]{
				// If holding down the button, increment the charge and continue the slowed animation
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					charge_attack_level[hdr::get_player_number(boma)] += 1.0; // Increment the charge by 1
					FT_MOTION_RATE(fighter, 10.0);
					HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
				}
				// If no longer holding the button, play out the rest of the animation as normal
				else{
					charge_finished[hdr::get_player_number(boma)] = true;
					FT_MOTION_RATE(fighter, 1.0);
					HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
				}
			}
		}
		wait(lua_state, 0.1);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
		FT_MOTION_RATE(fighter, 1.0);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_MIIGUNNER_STATUS_BOTTOM_SHOOT_FLAG_JUMP);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_BOTTOMSHOOT, false, 0);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	} 
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	} 
}

#[acmd_script( agent = "miigunner", script = "game_specialairhi1" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_hi1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		charge_attack_level[hdr::get_player_number(boma)] = 0.0;
		charge_finished[hdr::get_player_number(boma)] = false;
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
	}
	wait(lua_state, 1.0);
	// Charge build loop
	for _ in 0..29 { // F4-6 - 30 charge levels in total
		if is_excute(fighter) {
			// If charge initiated and not finished
			if !charge_finished[hdr::get_player_number(boma)]{
				// If holding down the button, increment the charge and continue the slowed animation
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					let motion_vec = Vector3f{x: 1.0, y: 0.85, z: 1.0};
					KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); //
					charge_attack_level[hdr::get_player_number(boma)] += 1.0; // Increment the charge by 1
					FT_MOTION_RATE(fighter, 10.0);
					HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
				}
				// If no longer holding the button, play out the rest of the animation as normal
				else{
					charge_finished[hdr::get_player_number(boma)] = true;
					FT_MOTION_RATE(fighter, 1.0);
					HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
				}
			}
		}
		wait(lua_state, 0.1);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
		FT_MOTION_RATE(fighter, 1.0);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_MIIGUNNER_STATUS_BOTTOM_SHOOT_FLAG_JUMP);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_BOTTOMSHOOT, false, 0);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	} 
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	} 
}

#[acmd_script( agent = "miigunner", script = "game_specialhi2" , category = ACMD_GAME , low_priority)]
unsafe fn special_hi2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 83, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 75, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 83, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
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
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.5);
	}
}

#[acmd_script( agent = "miigunner", script = "game_specialairhi3start" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_hi3_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.5);
	}
}

#[acmd_script( agent = "miigunner", script = "game_specialhi3" , category = ACMD_GAME , low_priority)]
unsafe fn special_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 0.85);
	}
}


pub fn install() {
    install_acmd_scripts!(
		special_n1_start_game,
		special_n1_fire_max_game,
		special_n1_fire_max_effect,
		special_n1_fire_max_sound,
		special_air_n1_start_game,
		special_air_n1_fire_max_game,
		special_air_n1_fire_max_effect,
		special_air_n1_fire_max_sound,
		//special_n2_loop_game,
		special_n3_start_game,
		special_air_n3_start_game,
		special_hi1_game,
		special_air_hi1_game,
		special_hi2_game,
		special_hi3_start_game,
		special_air_hi3_start_game,
		special_hi3_game,

	);
}

