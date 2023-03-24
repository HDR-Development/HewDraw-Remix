use super::*;

#[acmd_script( agent = "donkey", script = "game_specialn" , category = ACMD_GAME , low_priority)]
unsafe fn special_n(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.25);
		ATTACK(fighter, 0, 0, Hash40::new("arml"), 9.0, 70, 70, 0, 30, 9.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::add_power(boma, 0, WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);

        // Kirby
		if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
			ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.0, 361, 80, 0, 30, 6.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 361, 80, 0, 30, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 10.0, 361, 80, 0, 30, 3.0, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			AttackModule::add_power(boma, 0, WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
			AttackModule::add_power(boma, 1, WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
			AttackModule::add_power(boma, 2, WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
		}
	}
    frame(lua_state, 16.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.0, 361, 80, 0, 30, 6.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);		
		AttackModule::add_power(boma, 0, WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
    }
    frame(lua_state, 19.0);
	if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		HIT_RESET_ALL(fighter);
		AttackModule::clear_all(boma);
	}
    
}

#[acmd_script( agent = "donkey", script = "game_specialnmax" , category = ACMD_GAME , low_priority)]
unsafe fn special_n_max(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
	}
	frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.25);
		WorkModule::set_int(boma, 3, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_IGNORE_CRITICAL_ATTACK_ID);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 25.0, 70, 70, 0, 30, 6.0, 0.0, 17.0, -17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 25.0, 70, 70, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		
		if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
			ATTACK(fighter, 0, 0, Hash40::new("arml"), 28.0, 361, 80, 0, 30, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("arml"), 28.0, 361, 80, 0, 30, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 28.0, 361, 80, 0, 30, 4.0, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("waist"), 18.0, 361, 80, 0, 30, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		}
	}
    frame(lua_state, 16.0);
    if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("arml"), 28.0, 361, 80, 0, 30, 5.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
	frame(lua_state, 19.0);
	if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 20.0);
	if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
    }
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(boma);
	}

}

#[acmd_script( agent = "donkey", script = "game_specialairn" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_n(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.25);
		ATTACK(fighter, 0, 0, Hash40::new("arml"), 9.0, 70, 65, 0, 30, 9.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::add_power(boma, 0, WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
		
		// Kirby
		if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
			ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.0, 361, 75, 0, 30, 6.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 361, 75, 0, 30, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 10.0, 361, 75, 0, 30, 3.0, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			AttackModule::add_power(boma, 0, WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
			AttackModule::add_power(boma, 1, WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
			AttackModule::add_power(boma, 2, WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
		}
	}
    frame(lua_state, 16.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.0, 361, 75, 0, 30, 6.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::add_power(boma, 0, WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
    }
	frame(lua_state, 19.0);
	if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		HIT_RESET_ALL(fighter);
		AttackModule::clear_all(boma);
	}
    
}

#[acmd_script( agent = "donkey", script = "game_specialairnmax" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_n_max(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
	}
	frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.25);
		WorkModule::set_int(boma, 3, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_IGNORE_CRITICAL_ATTACK_ID);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 25.0, 70, 70, 0, 30, 6.0, 0.0, 18.0, -18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 25.0, 70, 65, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
			ATTACK(fighter, 0, 0, Hash40::new("arml"), 28.0, 361, 75, 0, 30, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("arml"), 28.0, 361, 75, 0, 30, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 28.0, 361, 75, 0, 30, 4.0, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("waist"), 28.0, 361, 75, 0, 30, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		}
	}
    frame(lua_state, 15.0);
    if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("arml"), 28.0, 361, 75, 0, 30, 5.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
	frame(lua_state, 19.0);
	if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 20.0);
	if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
    }
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(boma);
	}

}

#[acmd_script( agent = "donkey", scripts = ["game_specials", "game_specialairs"] , category = ACMD_GAME , low_priority)]
unsafe fn special_s_common(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_S_FLAG_FALL_START);
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 270, 50, 0, 40, 6.5, 0.0, 5.0, 10.5, Some(0.0), Some(5.0), Some(1.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        /* Air-only */
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 300, 41, 0, 45, 6.5, 0.0, 5.0, 10.5, Some(0.0), Some(5.0), Some(1.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 300, 41, 0, 45, 9.0, 0.0, 11.0, 7.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "donkey", scripts = ["effect_specials", "effect_specialairs"] , category = ACMD_EFFECT , low_priority)]
unsafe fn special_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("donkey_headbat_line"), Hash40::new("top"), 0, 12, -3, 0, -75, -85, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 1.5, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("donkey_headbat"), Hash40::new("top"), 0, 0, 6, 5, 0, 0, 0.70, true);
    }
}

#[acmd_script( agent = "donkey", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn special_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.500);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.5);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 361, 90, 0, 60, 8.0, 0.0, -1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 12.0, 361, 90, 0, 60, 8.0, 0.0, -1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 361, 90, 0, 60, 6.0, 0.0, 10.5, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_whole(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 19.0);
    
    // looping decreasingly effective multihits
    let mut damage = 6.0;
    for _ in 0..7 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), damage + 1.0, 361, 80, 0, 30, 8.0, 0.0, 11.0, 15.0, Some(0.0), Some(11.0), Some(-6.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            damage = damage * 0.5;
        }
        wait(lua_state, 5.0);
    }
    
    frame(lua_state, 62.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT);
        WorkModule::on_flag(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_SPINEND);
        //old finisher
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 85, 0, 75, 8.0, 0.0, 11.0, 20.0, Some(0.0), Some(11.0), Some(-4.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 2.0);
    }
    
}

#[acmd_script( agent = "donkey", script = "game_speciallwstart" , category = ACMD_GAME , low_priority)]
unsafe fn special_lw_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.750);
    }
    
}

#[acmd_script( agent = "donkey", script = "game_speciallwloop" , category = ACMD_GAME , low_priority)]
unsafe fn special_lw_loop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.750);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 78, 30, 0, 90, 10.3, 0.0, 0.0, -5.0, Some(0.0), Some(0.0), Some(20.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        //ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 84, 30, 0, 90, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        //ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 84, 30, 0, 90, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        //ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0, 78, 30, 0, 90, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.6);
        //WorkModule::on_flag(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_ATTACK);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        //WorkModule::on_flag(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_ATTACK);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 3.0);
    if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        MotionModule::set_frame(boma, MotionModule::frame(boma) + 15.0, true);
    } else {
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 78, 30, 0, 90, 10.3, 0.0, 0.0, -5.0, Some(0.0), Some(0.0), Some(20.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            //ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 84, 30, 0, 90, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            //ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 84, 30, 0, 90, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            //ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0, 78, 30, 0, 90, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.6);
            //WorkModule::on_flag(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_ATTACK);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            //WorkModule::on_flag(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_ATTACK);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.50);
    }
    
}

#[acmd_script( agent = "donkey", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        // if you already have an item, transition into pickup instead
        if ItemModule::is_have_item(boma, 0) {
            fighter.change_status_req(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
        }
        MotionModule::set_rate(boma, 0.67);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 0.5);
    }
    frame(lua_state, 14.5);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 0.1);
        JostleModule::set_status(boma, false);
        //VarModule::on_flag(fighter.battle_object, vars::donkey::status::SPECIAL_AIR_LW_STOP);
        CATCH(fighter, 0, Hash40::new("top"), 6.0, 0.0, 14.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 7.0, 0.0, 10.0, 13.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 2, Hash40::new("top"), 8.0, 0.0, 10.0, 13.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    wait(lua_state, 0.5);
    if is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
        MotionModule::set_rate(boma, 6.0);
        /*
        ItemModule::pickup_item(boma, ItemSize{_address: *ITEM_SIZE_HEAVY as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        ItemModule::pickup_item(boma, ItemSize{_address: *ITEM_SIZE_LIGHT as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        // if there is a heavy item to pick up nearby, grab it
        if !ItemModule::is_have_item(boma, 0) {
            WorkModule::off_flag(boma,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_PICKUP_ITEM);
            ItemModule::pickup_item(boma, ItemSize{_address: *ITEM_SIZE_HEAVY as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        // if there is a light item nearby, grab it
        if !ItemModule::is_have_item(boma, 0) {
            WorkModule::off_flag(boma,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_PICKUP_ITEM);
            ItemModule::pickup_item(boma, ItemSize{_address: *ITEM_SIZE_LIGHT as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        // if you have any items at all, transition into pickup
        if ItemModule::is_have_item(boma, 0) {
            fighter.change_status_req(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
        }
        */
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
        MotionModule::set_rate(boma, 0.9);
    }
}

#[acmd_script( agent = "donkey", script = "sound_specialairlw", category = ACMD_SOUND , low_priority)]
unsafe fn sound_special_air_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_donkey_attackdash"));
    }
}

#[acmd_script( agent = "donkey", script = "effect_specialairlw" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_special_air_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 5, 16, 5, 0, 0, 0, 0.8, true);
    }
    frame(lua_state,14.5);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("donkey_attack_arc"), Hash40::new("top"), 0, /* up/down */ 17, /* forward/back */ 7.0, -30, -2, 180, 1.15, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.75);
        EFFECT_FOLLOW(fighter, Hash40::new("donkey_attack_arc"), Hash40::new("top"), 0, /* up/down */ 13, /* forward/back */ 5.0, 30, -2, 0, 1.15, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.75);
    }
}

#[acmd_script( agent = "donkey", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn game_special_air_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_MOT_FRAME);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 40, 7.0, 0.0, 11.0, -8.0, Some(0.0), Some(11.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        FT_MOTION_RATE(fighter, 0.6);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 4.0);

    // loop increasingly weak multihits
    let mut damage = 6.0;
    for _ in 0..13 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("arml"), damage + 1.0, 361, 80, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), damage + 1.0, 361, 80, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 4.0);
        if is_excute(fighter) {
            damage = damage * 0.85;
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_CLIFF_CHECK);
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 68.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT); // set on 68 in vanilla (this is frame 72 rn)
        FT_MOTION_RATE(fighter, 1);
    }
}

#[acmd_script( agent = "donkey", script = "effect_speciallwloop" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_special_lw_loop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("donkey_handslap"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.67, 0, 0, 0, 0, 0, 0, false);
        //LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("donkey_handslap"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.67, 0, 0, 0, 0, 0, 0, false);
        //LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "donkey", script = "effect_specialhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_special_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 14.0);
    for _ in 0..5 {
        if is_excute(fighter) {
            let movement_lean = PostureModule::lr(boma) * 15.0 * KineticModule::get_sum_speed_x(fighter.boma(), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("top"), 5, 14, 3, /* rotate along z axis */ movement_lean, 6, /* rotate along left-right axis */ 20, 2, true, *EF_FLIP_YZ);
        }
        wait(lua_state, 4.0);
        if is_excute(fighter) {
            let movement_lean = PostureModule::lr(boma) * 15.0 * KineticModule::get_sum_speed_x(fighter.boma(), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_XY);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("top"), 5, 11, 3, /* rotate along z axis */ movement_lean, 6, /* rotate along left-right axis */ 20, 2, true, *EF_FLIP_YZ);
        }
        wait(lua_state, 4.0);
    }
    frame(lua_state, 63.0);
    if is_excute(fighter) {
        //LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(lua_state, 80.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_XY);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind_flash"), Hash40::new("donkey_spin_wind_flash"), Hash40::new("top"), -3.5, 17, 15.5, 0, 0, 60, 1.3, true, *EF_FLIP_YZ);
    }
}

pub fn install() {
    install_acmd_scripts!(
        special_n,
		special_n_max,
		special_air_n,
		special_air_n_max,
		special_s_common,
		special_hi,
		special_lw_start,
        special_lw_loop,
        special_air_lw,
        effect_special_air_lw,
        game_special_air_hi,
        sound_special_air_lw,
        special_s_effect,
        effect_special_lw_loop,
        effect_special_hi
    );
}

