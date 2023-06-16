
use super::*;


#[acmd_script( agent = "yoshi", script = "game_attacks3hi" , category = ACMD_GAME , low_priority)]
unsafe fn yoshi_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	FT_DESIRED_RATE(fighter, 5.0, 6.0);
	frame(lua_state, 5.0);
	FT_DESIRED_RATE(fighter, 4.0, 3.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("tail1"), 13.0, 70, 80, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 1, 0, Hash40::new("tail1"), 13.0, 70, 80, 0, 40, 4.4, 6.56, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 2, 0, Hash40::new("tail1"), 13.0, 70, 80, 0, 40, 3.3, 13.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 9.0);
	FT_MOTION_RATE(fighter, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
}


#[acmd_script( agent = "yoshi", script = "effect_attacks3hi" , category = ACMD_EFFECT , low_priority)]
unsafe fn yoshi_attack_s3_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 11, 6, 18, -27, 156, 1.15, true, *EF_FLIP_YZ);
		LAST_EFFECT_SET_RATE(fighter, 1.5);
	}
}


#[acmd_script( agent = "yoshi", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn yoshi_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	FT_DESIRED_RATE(fighter, 5.0, 6.0);
	frame(lua_state, 5.0);
	FT_DESIRED_RATE(fighter, 4.0, 3.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("tail1"), 12.0, 70, 80, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 1, 0, Hash40::new("tail1"), 12.0, 70, 80, 0, 40, 4.4, 6.56, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 2, 0, Hash40::new("tail1"), 12.0, 70, 80, 0, 40, 3.3, 13.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 9.0);
	FT_MOTION_RATE(fighter, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
}


#[acmd_script( agent = "yoshi", script = "effect_attacks3" , category = ACMD_EFFECT , low_priority)]
unsafe fn yoshi_attack_s3_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 9, 4, 173, -156, 2, 1.15, true, *EF_FLIP_YZ);
		LAST_EFFECT_SET_RATE(fighter, 1.5);
	}
}


#[acmd_script( agent = "yoshi", script = "game_attacks3lw" , category = ACMD_GAME , low_priority)]
unsafe fn yoshi_attack_s3_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	FT_DESIRED_RATE(fighter, 5.0, 6.0);
	frame(lua_state, 5.0);
	FT_DESIRED_RATE(fighter, 4.0, 3.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("tail1"), 11.0, 70, 80, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 1, 0, Hash40::new("tail1"), 11.0, 70, 80, 0, 40, 4.4, 6.56, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 2, 0, Hash40::new("tail1"), 11.0, 70, 80, 0, 40, 3.3, 13.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 9.0);
	FT_MOTION_RATE(fighter, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
}
  

#[acmd_script( agent = "yoshi", script = "effect_attacks3lw" , category = ACMD_EFFECT , low_priority)]
unsafe fn yoshi_attack_s3_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 6, 4, 166, -162, 9, 1.15, true, *EF_FLIP_YZ);
		LAST_EFFECT_SET_RATE(fighter, 1.5);
	}
}


#[acmd_script( agent = "yoshi", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn yoshi_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("tail2"), 9.0, 90, 112, 0, 50, 4.5, 5.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 1, 0, Hash40::new("tail2"), 9.0, 90, 112, 0, 50, 4.5, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 2, 0, Hash40::new("tail1"), 9.0, 90, 112, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}


#[acmd_script( agent = "yoshi", script = "effect_attackhi3" , category = ACMD_EFFECT , low_priority)]
unsafe fn yoshi_attack_hi3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 16, -1, 0, 52, 88, 1.15, true, *EF_FLIP_YZ);
	}

}



#[acmd_script( agent = "yoshi", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn yoshi_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("tail1"), 9.0, 361, 30, 0, 67, 5.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 1, 0, Hash40::new("tail2"), 9.0, 361, 30, 0, 67, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 2, 0, Hash40::new("tail3"), 10.0, 28, 40, 0, 67, 4.5, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
		HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        FT_MOTION_RATE(fighter, 0.900);
    }
    
}


#[acmd_script( agent = "yoshi", script = "effect_attacklw3" , category = ACMD_EFFECT , low_priority)]
unsafe fn yoshi_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4, 10, 0, 35, 180, 1.2, true, *EF_FLIP_YZ);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, -2.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
	}

}


pub fn install() {
    install_acmd_scripts!(
		yoshi_attack_s3_hi_game,
		yoshi_attack_s3_s_game,
		yoshi_attack_s3_lw_game,
		yoshi_attack_hi3_game,
		yoshi_attack_lw3_game,
		yoshi_attack_s3_hi_effect,
		yoshi_attack_s3_s_effect,
		yoshi_attack_s3_lw_effect,
		yoshi_attack_hi3_effect,
		yoshi_attack_lw3_effect,
    );
}

