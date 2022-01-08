
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

// Vanilla animation
/*
#[acmd_script( agent = "miiswordsman", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn attack_s3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area(boma, 0.0, 4.0);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		// Body hitboxes
		ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 12.0, 361, 96, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 361, 96, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("haver"), 12.0, 361, 96, 0, 20, 3.5, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("haver"), 12.0, 361, 96, 0, 20, 3.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("haver"), 15.0, 361, 100, 0, 20, 3.5, 0.0, 13.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		// 5 and 6 are copies of 0 and 4 respectively
		ATTACK(fighter, 5, 0, Hash40::new("shoulderr"), 12.0, 361, 96, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 6, 0, Hash40::new("haver"), 15.0, 361, 100, 0, 20, 3.5, 0.0, 13.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 4, 6, 1.6);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		// Capsule hitbox
		ATTACK(fighter, 5, 0, Hash40::new("top"), 12.0, 361, 96, 0, 20, 3.0, 0.0, 5.4, 4.0, Some(0.0), Some(5.4), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 6, 0, Hash40::new("top"), 15.0, 361, 100, 0, 20, 3.0, 0.0, 5.4, 4.0, Some(0.0), Some(5.4), Some(24.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 4, 6, 1.6);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear(boma, 5, false);
		AttackModule::clear(boma, 6, false);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
    
}
*/

// New animation
#[acmd_script( agent = "miiswordsman", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn attack_s3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area(boma, 0.0, 4.0);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		// Body hitboxes
		ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 9.0, 361, 100, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.0, 361, 100, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("haver"), 10.0, 72, 45, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("haver"), 10.0, 72, 45, 0, 60, 3.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("haver"), 12.0, 96, 50, 0, 60, 4.0, 0.0, 13.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(boma, 2, 4.0, false);
		AttackModule::set_add_reaction_frame(boma, 3, 4.0, false);
		AttackModule::set_add_reaction_frame(boma, 4, 4.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 1.6);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
		//FT_MOTION_RATE(fighter, 0.85);
	}
    
}

#[acmd_script( agent = "miiswordsman", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 7.0, 80, 123, 0, 30, 3.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.0, 85, 130, 0, 30, 3.5, 0.0, 2.0, -1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("haver"), 7.5, 90, 130, 0, 30, 3.5, 0.0, 7.0, -1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("haver"), 8.5, 95, 122, 0, 30, 3.5, 0.0, 13.5, -1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.5);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
}

#[acmd_script( agent = "miiswordsman", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 7.0, 70, 80, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 7.0, 70, 80, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("haver"), 7.0, 70, 80, 0, 50, 3.0, 0.0, 1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("haver"), 8.0, 85, 80, 0, 50, 3.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("haver"), 9.0, 100, 80, 0, 50, 3.0, 0.0, 13.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 1.5);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}

}

pub fn install() {
    install_acmd_scripts!(
		attack_s3_game,
		attack_hi3_game,
		attack_lw3_game,
		
	);
}

