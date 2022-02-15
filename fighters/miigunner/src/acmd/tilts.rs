
use super::*;

#[acmd_script( agent = "miigunner", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn attack_s3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 0.625);
	}
	// Early Hit
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 7.5, 361, 56, 0, 56, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 361, 56, 0, 56, 2.5, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
		// Cannon "splash" hitboxes
		ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 361, 56, 0, 56, 2.5, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 13.0, 361, 56, 0, 56, 2.5, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		// Cannon "splash" hitboxes
		ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 361, 56, 0, 56, 2.5, 0.0, 8.0, 8.0, Some(0.0), Some(13.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 13.0, 361, 56, 0, 56, 2.5, 0.0, 8.0, 8.0, Some(0.0), Some(4.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
	}
	// Mid Hit
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.5, 361, 56, 0, 56, 3.0, 0.0, 8.0, 6.0, Some(0.0), Some(8.0), Some(25.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
		// Cannon "splash" hitboxes
		ATTACK(fighter, 2, 0, Hash40::new("top"), 10.5, 361, 56, 0, 56, 2.5, 0.0, 8.0, 6.0, Some(0.0), Some(16.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 10.5, 361, 56, 0, 56, 2.5, 0.0, 8.0, 6.0, Some(0.0), Some(2.5), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		// Cannon "splash" hitboxes
		ATTACK(fighter, 2, 0, Hash40::new("top"), 10.5, 361, 56, 0, 56, 2.5, 0.0, 8.0, 6.0, Some(0.0), Some(12.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 10.5, 361, 56, 0, 56, 2.5, 0.0, 8.0, 6.0, Some(0.0), Some(4.5), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 56, 0, 56, 1.5, 0.0, 8.0, 6.0, Some(0.0), Some(8.0), Some(25.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
		// Cannon "splash" hitboxes
		ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 361, 56, 0, 56, 2.0, 0.0, 8.0, 6.0, Some(0.0), Some(12.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 361, 56, 0, 56, 2.0, 0.0, 8.0, 6.0, Some(0.0), Some(4.5), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 56, 0, 56, 1.5, 0.0, 8.0, 6.0, Some(0.0), Some(8.0), Some(21.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
		// Cannon "splash" hitboxes
		ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 361, 56, 0, 56, 2.0, 0.0, 8.0, 6.0, Some(0.0), Some(12.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 361, 56, 0, 56, 2.0, 0.0, 8.0, 6.0, Some(0.0), Some(4.5), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
}

#[acmd_script( agent = "miigunner", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
		ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 9.0, 90, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 95, 100, 0, 50, 6.0, 0.0, 6.0, 9.5, Some(0.0), Some(13.0), Some(9.25), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("handr"), 9.0, 90, 80, 0, 50, 2.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.0, 90, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("handr"), 10.0, 95, 100, 0, 50, 6.0, 9.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("handr"), 8.0, 95, 70, 0, 50, 6.0, 9.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
}

#[acmd_script( agent = "miigunner", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 9.0, 85, 50, 0, 82, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.0, 85, 50, 0, 82, 3.5, 4.5, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 80, 79, 0, 70, 6.0, 0.0, 1.6, 17.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
		AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	wait(lua_state, 7.0);
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

