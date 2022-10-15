
use super::*;


#[acmd_script( agent = "miigunner", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.476);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 9.0, 361, 45, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.0, 361, 45, 0, 50, 4.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("handr"), 11.0, 361, 80, 0, 55, 5.5, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("handr"), 11.0, 361, 80, 0, 55, 4.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "miigunner", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.5);
	}
	frame(lua_state, 8.0);
    if is_excute(fighter) {
		// Trigger boosted aerial
		if boma.is_button_on(Buttons::Attack) {
			VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
			FT_MOTION_RATE(fighter, 2.5);
		}
    }
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_ATTACKAIRF_BULLET, false, 0);
		
		// Boosted aerial
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 1.0);
			let addSpeed1 = Vector3f{ x: -1.0, y: 0.0, z: 0.0 };
			KineticModule::add_speed(boma, &addSpeed1);
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 5.0, 120, 85, 0, 60, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("armr"), 5.0, 125, 85, 0, 60, 3.5, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("arml"), 5.0, 120, 85, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("hip"), 5.0, 120, 85, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 4, 0, Hash40::new("handr"), 7.0, 125, 85, 0, 60, 4.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		}
		// Normal
		else {
			FT_MOTION_RATE(fighter, 1.0);
		}
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		// Boosted aerial
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 0.667);
			let addSpeed1 = Vector3f{ x: -1.0, y: 0.0, z: 0.0 };
			KineticModule::add_speed(boma, &addSpeed1);
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 4.0, 361, 85, 0, 50, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("armr"), 4.0, 361, 85, 0, 50, 3.5, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("arml"), 4.0, 361, 85, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("hip"), 4.0, 361, 85, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 4, 0, Hash40::new("handr"), 4.0, 361, 85, 0, 50, 4.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		}
	}
	wait(lua_state, 18.0); // Frame 42
	if is_excute(fighter) {
		// Boosted aerial
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 1.0);
			AttackModule::clear_all(boma);
		}
		WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

#[acmd_script( agent = "miigunner", script = "effect_attackairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x16688e0af6), Hash40::new("armr"), 6.0, 0, 0, 0, 90, 0, 1.0, true);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("arml"), 4.289, -0.272, -0.135, 0, 0, 0, 1.1, true);
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x16688e0af6), Hash40::new("armr"), 6.0, 0, 0, 0, 90, 0, 1.5, true);
			LAST_EFFECT_SET_RATE(fighter, 2.0);
			
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x185b39be1a), Hash40::new("handr"), 2.5, 0, 0, 0, 0, -90, 0.65, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0); // Blue value seems fine, increase the green value to make it a bit more of a realistic blue hue
			LAST_EFFECT_SET_RATE(fighter, 0.8);
			
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x14393ffad3), Hash40::new("handr"), 5.5, 0, 0, 0, 0, -90, 0.65, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0); // Blue value seems fine, increase the green value to make it a bit more of a realistic blue hue
		}
	}
}

#[acmd_script( agent = "miigunner", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
		VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
	frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
	frame(lua_state, 7.0);
    if is_excute(fighter) {
		// Trigger boosted aerial
		let cat1 = 0;
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || fighter.is_cat_flag(Cat1::AttackS3) || fighter.is_cat_flag(Cat1::AttackS4) || fighter.is_cat_flag(Cat1::AttackN) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
			FT_MOTION_RATE(fighter, 2.5);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
		// Boosted hitboxes
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 1.0);
			let addSpeed1 = Vector3f{ x: 0.6, y: 0.1, z: 0.0 };
			KineticModule::add_speed(boma, &addSpeed1);
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 8.0, 60, 90, 0, 55, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 60, 90, 0, 55, 4.5, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("kneel"), 8.0, 60, 90, 0, 55, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("handr"), 10.0, 55, 90, 0, 55, 6.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 4, 0, Hash40::new("handr"), 10.0, 55, 90, 0, 55, 4.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
		}
		// Normal hitboxes
		else{
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 10.0, 361, 103, 0, 29, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 103, 0, 29, 4.5, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("handr"), 13.0, 361, 103, 0, 41, 6.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("handr"), 12.0, 361, 103, 0, 36, 4.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
		}
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
		// Boosted
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 0.667);
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 6.5, 361, 100, 0, 35, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("armr"), 6.5, 361, 100, 0, 35, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("kneel"), 6.5, 361, 100, 0, 35, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("handr"), 6.5, 361, 100, 0, 35, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 4, 0, Hash40::new("handr"), 6.5, 361, 100, 0, 35, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
		}
		// Normal
		else {
			AttackModule::clear_all(boma);
		}
    }
	frame(lua_state, 22.0);
    if is_excute(fighter) {
        // Clear boosted hitboxes
		AttackModule::clear_all(boma);
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 0.6);
		}
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 1.0);
		}
		else{
			WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
    }
	frame(lua_state, 40.0);
    if is_excute(fighter) {
		// Boosted autocancel
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
    }
    
}

#[acmd_script( agent = "miigunner", script = "effect_attackairb" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 7.0);
    if is_excute(fighter) {
        // Flash to indicate boosted aerial
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("arml"), 4.289, -0.272, -0.135, 0, 0, 0, 1.1, true);
		}
    }
	frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x14393ffad3), Hash40::new("armr"), 5.5, 0, 0, 0, 0, -90, 1.1, true);
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			//LAST_EFFECT_SET_COLOR(fighter, 0.24, 0.55, 5.0);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0); // Blue value seems fine, increase the green value to make it a bit more of a realistic blue hue
		}
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x185b39be1a), Hash40::new("armr"), 5.5, 0, 0, 0, 0, -90, 1.1, false);
		LAST_EFFECT_SET_RATE(fighter, 0.8);
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			//LAST_EFFECT_SET_COLOR(fighter, 0.24, 0.55, 5.0);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0); // Blue value seems fine, increase the green value to make it a bit more of a realistic blue hue
		}
    }
    
}

#[acmd_script( agent = "miigunner", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.667);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 140, 100, 45, 0, 4.5, 0.0, 15.0, -3.0, Some(0.0), Some(15.0), Some(3.0), 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 367, 100, 25, 0, 2.6, 0.0, 55.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.8, 96, 100, 25, 0, 2.6, 0.0, 55.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.8, 130, 100, 25, 0, 3.2, 0.0, 13.0, -0.8, Some(0.0), Some(13.0), Some(0.8), 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(boma, 0, 1.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 1.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 1.0, false);
        FT_MOTION_RATE(fighter, 0.500);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 72, 182, 0, 50, 4.0, 0.0, 58.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 1.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
    }
	frame(lua_state, 37.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 72, 182, 0, 50, 4.0, 0.0, 61.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 1.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
    }
	frame(lua_state, 38.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 72, 182, 0, 50, 5.0, 0.0, 50.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 1.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "miigunner", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
		VarModule::off_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED);
		VarModule::set_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL, 0.0);
    }
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        }
		else {
			FT_MOTION_RATE(fighter, 1.0);
		}
	}
	// Charge build loop
	for _ in 0..5 { // F9-14 - 6 charge levels in total
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			// If a boosted aerial and the charge hasn't been finished
			if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) && !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED){
				// If holding down the button, increment the charge and continue the slowed animation
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
					VarModule::add_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL, 1.0); // Increment the charge by 1
					FT_MOTION_RATE(fighter, 5.0);
				}
				// If no longer holding the button, play out the rest of the animation as normal
				else{
					VarModule::on_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED);
					FT_MOTION_RATE(fighter, 1.0);
				}
			}
		}
	}
	
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		// Boosted aerial
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 0.5);
		}
		else {
			FT_MOTION_RATE(fighter, 0.5);
		}
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			let charge_attack_damage_mul = 1.0 + (VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL) * 0.05);
			ATTACK(fighter, 0, 0, Hash40::new("handr"), 8.0 * charge_attack_damage_mul, 75, 65, 0, 50, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("handr"), 12.0 * charge_attack_damage_mul, 75, 65, 0, 50, 5.5, 8.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			// If fully charged, add body hitboxes
			if VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL) >= 5.0 {
				SET_SPEED_EX(fighter, 0.0, 3.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				ATTACK(fighter, 2, 0, Hash40::new("head"), 18.0, 80, 70, 0, 40, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			}
			// Add non-fully-charged boosted speed
			else {
				//let addSpeed1 = Vector3f{ x: 0.0, y: 1.0 + 0.5 * VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL), z: 0.0 };
				//KineticModule::add_speed(boma, &addSpeed1);
				let boost_speed = 1.0 + (0.375 * VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL));
				SET_SPEED_EX(fighter, KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(boma), boost_speed, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			}
		}
		else {
			/* Ground-only */
			ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.0, 270, 90, 0, 5, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("handr"), 15.0, 270, 90, 0, 5, 5.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			/* Air-only */
			ATTACK(fighter, 2, 0, Hash40::new("handr"), 10.0, 270, 88, 0, 5, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("handr"), 15.0, 270, 66, 0, 5, 5.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
		}
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			ATTACK(fighter, 0, 0, Hash40::new("handr"), 6.0, 65, 90, 0, 30, 2.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("handr"), 6.0, 65, 90, 0, 30, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			/*
			if VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL) => 4.0 {
				ATTACK(fighter, 2, 0, Hash40::new("head"), 8.0, 77, 80, 0, 50, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			}
			*/
		}
		else {
			ATTACK(fighter, 0, 0, Hash40::new("handr"), 8.0, 361, 90, 0, 30, 2.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("handr"), 12.0, 361, 90, 0, 30, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			AttackModule::clear(boma, 2, false);
			AttackModule::clear(boma, 3, false);
		}
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		// Boosted aerial
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 0.75);
		}
		else {
			AttackModule::clear_all(boma);
		}
	}
	frame(lua_state, 43.0);
	if is_excute(fighter) {
		// Clear the fully charged lingering hitboxes
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			// High-charged boost
			if VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL) >= 5.0 {
				FT_MOTION_RATE(fighter, 4.0);
			}
			else {
				FT_MOTION_RATE(fighter, 1.0);
			}
			AttackModule::clear_all(boma);
		}
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		// Clear the fully charged lingering hitboxes
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			// High-charged boost
			if VarModule::get_float(fighter.battle_object, vars::miigunner::status::CHARGE_ATTACK_LEVEL) >= 5.0 {
				// Activate Ledge Grab
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			}
		}
	}
	frame(lua_state, 54.0);
	if is_excute(fighter) {
		WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
    
}

#[acmd_script( agent = "miigunner", script = "effect_attackairlw" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_attack_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("arml"), 5.0, 0, 0, 0, 0, 0, 1.11, true);
		LAST_EFFECT_SET_RATE(fighter, 1.3);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
			LAST_EFFECT_SET_RATE(fighter, 0.33);
		}
	}
	for _ in 0..5 { // F9-14 - 6 charge levels in total
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			// If a boosted aerial and the charge hasn't been finished
			if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
				// If no longer holding the button, play out the rest of the animation as normal
				if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED) {
					LAST_EFFECT_SET_RATE(fighter, 1.0);
				}
				// If holding down the button, charge is being incremented
				else {
					LAST_EFFECT_SET_RATE(fighter, 0.33);
				}
			}
		}
	}
	
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x139e44e9f0), Hash40::new("haver"), 0, 0, -3, 0, 0, 0, 1.1, true);
		LAST_EFFECT_SET_RATE(fighter, 1.3);
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0); // Blue value seems fine, increase the green value to make it a bit more of a realistic blue hue
		}
		EFFECT_DETACH_KIND(fighter, Hash40::new_raw(0x139e44e9f0), -1);
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x13e943d966), Hash40::new("haver"), 0, 0, 2.5, 90, 0, 0, 0.3, true);
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) && !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED){
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0); // Blue value seems fine, increase the green value to make it a bit more of a realistic blue hue
		}
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x185b39be1a), Hash40::new("armr"), 6, 0, 0, 0, 0, -90, 0.75, true);
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0); // Blue value seems fine, increase the green value to make it a bit more of a realistic blue hue
		}
	}

}

pub fn install() {
    install_acmd_scripts!(
        miigunner_attack_air_n_game,
		miigunner_attack_air_f_game,
		miigunner_attack_air_f_effect,
        miigunner_attack_air_b_game,
		miigunner_attack_air_b_effect,
		miigunner_attack_air_lw_game,
		miigunner_attack_air_lw_effect,
        miigunner_attack_air_hi_game,
    );
}

