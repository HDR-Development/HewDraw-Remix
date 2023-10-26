use super::*;
use smash::app::sv_module_access::sound;

#[acmd_script( agent = "miigunner", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
	FT_MOTION_RATE_RANGE(fighter, 2.0, 5.0, 1.0);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 5.0);
	FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 11.0, 361, 38, 0, 51, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 11.0, 361, 38, 0, 51, 4.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("handr"), 13.0, 361, 85, 0, 55, 4.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 3, 0, Hash40::new("handr"), 13.0, 361, 85, 0, 55, 5.5, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
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
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
	frame(lua_state, 6.0);
	FT_MOTION_RATE_RANGE(fighter, 6.0, 9.0, 4.0);
	frame(lua_state, 9.0);
	if boma.is_button_on(Buttons::Attack) {
		FT_MOTION_RATE_RANGE(fighter, 9.0, 10.0, 5.0);
	}
    if is_excute(fighter) {
		if boma.is_button_on(Buttons::Attack) {
			VarModule::on_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL);
		}
    }
	frame(lua_state, 10.0);
	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		FT_MOTION_RATE_RANGE(fighter, 10.0, 12.0, 3.0);
	}
	if is_excute(fighter) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_ATTACKAIRF_BULLET, false, 0);
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			let addSpeed1 = Vector3f{ x: -0.8, y: 0.0, z: 0.0 };
			KineticModule::add_speed(boma, &addSpeed1);
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 5.0, 120, 85, 0, 60, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("armr"), 5.0, 120, 85, 0, 60, 3.5, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("handr"), 5.0, 120, 85, 0, 60, 4.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		}
	}
	frame(lua_state, 12.0);
	FT_MOTION_RATE(fighter, 1.0);
	frame(lua_state, 14.0);
	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		FT_MOTION_RATE_RANGE(fighter, 14.0, 23.0, 8.0);
	}
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 4.0, 361, 85, 0, 50, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("armr"), 4.0, 361, 85, 0, 50, 3.5, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("handr"), 4.0, 361, 85, 0, 50, 4.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		}
	}
	frame(lua_state, 23.0);
	FT_MOTION_RATE(fighter, 1.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			AttackModule::clear_all(boma);
		}
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

#[acmd_script( agent = "miigunner", script = "effect_attackairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_air_shot"), Hash40::new("armr"), 6.0, 0, 0, 0, 90, 0, 1.0, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 1, true);
		}
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_air_shot"), Hash40::new("armr"), 6.0, 0, 0, 0, 90, 0, 1.5, true);
			LAST_EFFECT_SET_RATE(fighter, 2.0);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_shot_after"), Hash40::new("handr"), 2.5, 0, 0, 0, 0, -90, 0.65, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
			LAST_EFFECT_SET_RATE(fighter, 0.45);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_shot_s"), Hash40::new("handr"), 5.5, 0, 0, 0, 0, -90, 0.65, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
		}
	}
}

#[acmd_script( agent = "miigunner", script = "game_landingairf" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_landing_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_f"), 0);
        if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
            FT_MOTION_RATE(fighter, (landing_frame + 5.0)/landing_frame);
        }
    }
}

#[acmd_script( agent = "miigunner", script = "effect_landingairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_landing_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("miigunner_atk_shot_after"), false, false);
		LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
}

#[acmd_script( agent = "miigunner", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
	frame(lua_state, 8.0);
	if boma.is_button_on(Buttons::Attack) {
		FT_MOTION_RATE_RANGE(fighter, 8.0, 9.0, 7.0);
	}
    if is_excute(fighter) {
        if boma.is_button_on(Buttons::Attack) {
            VarModule::on_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL);
        }
    }
    frame(lua_state, 9.0);
	FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			let addSpeed1 = Vector3f{ x: 0.6, y: 0.1, z: 0.0 };
			KineticModule::add_speed(boma, &addSpeed1);
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 8.0, 60, 90, 0, 55, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 60, 90, 0, 55, 4.5, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("kneel"), 8.0, 60, 90, 0, 55, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("handr"), 10.0, 55, 90, 0, 55, 6.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 4, 0, Hash40::new("handr"), 10.0, 55, 90, 0, 55, 4.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
		}
		else {
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 10.0, 361, 103, 0, 29, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 103, 0, 29, 4.5, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("handr"), 13.0, 361, 103, 0, 36, 4.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("handr"), 13.0, 361, 103, 0, 41, 6.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
		}
    }
    frame(lua_state, 13.0);
	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		FT_MOTION_RATE_RANGE(fighter, 13.0, 22.0, 7.0);
	}
    if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 6.5, 361, 100, 0, 35, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("armr"), 6.5, 361, 100, 0, 35, 3.5, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 4, 0, Hash40::new("handr"), 6.5, 361, 100, 0, 35, 3.5, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			AttackModule::clear(boma, 2, false);
			AttackModule::clear(boma, 3, false);
		}
		else {
			AttackModule::clear_all(boma);
		}
    }
	frame(lua_state, 22.0);
	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		FT_MOTION_RATE_RANGE(fighter, 22.0, 32.0, 13.0);
	}
    if is_excute(fighter) {
		AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		FT_MOTION_RATE(fighter, 1.0);
	}
    if is_excute(fighter) {
		if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
    }
	frame(lua_state, 35.0);
    if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
    }
    
}

#[acmd_script( agent = "miigunner", script = "effect_attackairb" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 8.0);
    if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, -11, 0, 0, 0, 1, true);
		}
    }
	frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x14393ffad3), Hash40::new("armr"), 5.5, 0, 0, 0, 0, -90, 1.1, true);
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
			EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.0, true);
		}
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x185b39be1a), Hash40::new("armr"), 5.5, 0, 0, 0, 0, -90, 1.1, false);
		LAST_EFFECT_SET_RATE(fighter, 0.8);
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
		}
    }
    
}

#[acmd_script( agent = "miigunner", script = "game_landingairb" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_landing_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_b"), 0);
        if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
            FT_MOTION_RATE(fighter, (landing_frame + 3.0)/landing_frame);
        }
    }
}

#[acmd_script( agent = "miigunner", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
	FT_MOTION_RATE_RANGE(fighter, 1.0, 15.0, 11.0);
    frame(lua_state, 15.0);
	if boma.is_button_on(Buttons::Attack) {
		FT_MOTION_RATE_RANGE(fighter, 15.0, 17.0, 8.0);
	}
	else {
		FT_MOTION_RATE(fighter, 1.0);
	}
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        if boma.is_button_on(Buttons::Attack) {
            VarModule::on_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL);
        }
    }
    frame(lua_state, 17.0);
	FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			let addSpeed = Vector3f{ x: 0.0, y: -4.0, z: 0.0 };
			KineticModule::add_speed(boma, &addSpeed);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 90, 80, 0, 50, 6.5, 0.0, 28.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
			/* Ground-only */
			ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 270, 50, 0, 70, 6.0, 0.0, 19.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
			/* Air-only */
			ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 270, 48, 0, 40, 6.0, 0.0, 19.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
		}
		else {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 140, 100, 45, 0, 4.5, 0.0, 15.0, -3.0, Some(0.0), Some(15.0), Some(3.0), 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
		}
	}
    frame(lua_state, 18.0);
	if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		FT_MOTION_RATE(fighter, 0.5);
	}
    if is_excute(fighter) {
		if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 367, 100, 25, 0, 2.6, 0.0, 55.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 1.8, 96, 100, 25, 0, 2.6, 0.0, 55.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 1.8, 130, 100, 25, 0, 3.2, 0.0, 13.0, -0.8, Some(0.0), Some(13.0), Some(0.8), 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
			AttackModule::set_add_reaction_frame(boma, 0, 1.0, false);
			AttackModule::set_add_reaction_frame(boma, 1, 1.0, false);
			AttackModule::set_add_reaction_frame(boma, 2, 1.0, false);
		}
    }
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 80, 0, 50, 6.5, 0.0, 19.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
			AttackModule::clear(boma, 1, false);
			AttackModule::clear(boma, 2, false);
		}
	}
    frame(lua_state, 22.0);
	FT_MOTION_RATE(fighter, 1.0);
	frame(lua_state, 27.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			AttackModule::clear_all(boma);
		}
	}
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
		if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 72, 172, 0, 50, 4.0, 0.0, 58.0, 0.0, Some(0.0), Some(16.0), Some(0.0), 1.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
		}
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

#[acmd_script( agent = "miigunner", script = "effect_attackairhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_attack_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4.0, 15, 0, 0, 0, 0, 1, true);
		}
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_shot5"), Hash40::new("haver"), 0, 0, -3, 0, 0, 0, 1.1, true);
			LAST_EFFECT_SET_RATE(fighter, 1.3);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
			EFFECT_DETACH_KIND(fighter, Hash40::new("miigunner_atk_shot5"), -1);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_shot4"), Hash40::new("haver"), 0, 0, 2.5, 90, 0, 0, 0.5, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
		}
		else {
			EFFECT_FLW_POS(fighter, Hash40::new("miigunner_atk_gatling"), Hash40::new("armr"), 5.5, 0, 0, -90, 0, 0, 0.6, true);
			LAST_EFFECT_SET_RATE(fighter, 1.4);
		}
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_shot_after"), Hash40::new("armr"), 6, 0, 0, 0, 0, -90, 0.75, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
		}
	}
}

#[acmd_script( agent = "miigunner", script = "sound_attackairhi" , category = ACMD_SOUND , low_priority)]
unsafe fn miigunner_attack_air_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			PLAY_STATUS(fighter, Hash40::new("se_miigunner_attackair_h01_lp"));
			PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_attack02"));
		}
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			PLAY_SE(fighter, Hash40::new("se_miigunner_smash_l01"));
			PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_attack03"));
		}
	}
	frame(lua_state, 39.0);
	if is_excute(fighter) {
		if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			STOP_SE(fighter, Hash40::new("se_miigunner_attackair_h01_lp"));
			STOP_SE(fighter, Hash40::new("seq_miigunner_rnd_attack02"));
			PLAY_SE(fighter, Hash40::new("se_miigunner_attackair_h01"));
		}
	}
}

#[acmd_script( agent = "miigunner", script = "game_landingairhi" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_landing_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_hi"), 0);
        if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
            FT_MOTION_RATE(fighter, (landing_frame + 5.0)/landing_frame);
        }
    }
}

#[acmd_script( agent = "miigunner", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            VarModule::on_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL);
        }
	}
	frame(lua_state, 14.0);
	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		FT_MOTION_RATE_RANGE(fighter, 14.0, 20.0, 4.0);
	}
	else {
		FT_MOTION_RATE_RANGE(fighter, 14.0, 18.0, 1.0);
	}
	frame(lua_state, 18.0);
	if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		FT_MOTION_RATE_RANGE(fighter, 18.0, 20.0, 1.0);
	} 
	frame(lua_state, 20.0);
	FT_MOTION_RATE(fighter, 1.0);
	if is_excute(fighter) {
		let charge = VarModule::get_float(fighter.battle_object, vars::miigunner::status::CURRENT_CHARGE);
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			let charge_mul = 1.0 + (charge * 0.025);
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 12.0 * charge_mul, 80, 65, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("handr"), 12.0 * charge_mul, 80, 65, 0, 50, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("handr"), 14.0 * charge_mul, 80, 65, 0, 50, 5.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			SET_SPEED_EX(fighter,
				KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(boma) * (20.0 - charge)/20.0,
				1.0 + (0.125 * charge),
				*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN
			);
		}
		else {
			/* Ground-only */
			ATTACK(fighter, 0, 0, Hash40::new("handr"), 15.0, 270, 70, 0, 40, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("handr"), 15.0, 270, 70, 0, 40, 5.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			/* Air-only */
			ATTACK(fighter, 2, 0, Hash40::new("handr"), 15.0, 270, 50, 0, 20, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("handr"), 15.0, 270, 50, 0, 20, 5.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
		}
	}
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 8.0, 65, 90, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("handr"), 8.0, 65, 90, 0, 30, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("handr"), 8.0, 65, 90, 0, 30, 4.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		}
		else {
			ATTACK(fighter, 0, 0, Hash40::new("handr"), 12.0, 361, 70, 0, 30, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("handr"), 12.0, 361, 70, 0, 30, 4.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
			AttackModule::clear(boma, 2, false);
			AttackModule::clear(boma, 3, false);
		}
	}
	frame(lua_state, 28.0);
	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		FT_MOTION_RATE(fighter, 0.75);
	}
	if is_excute(fighter) {
		if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			AttackModule::clear_all(boma);
		}
	}
	frame(lua_state, 43.0);
	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		FT_MOTION_RATE_RANGE(fighter, 43.0, 50.0, 9.0 + 2.0 * VarModule::get_float(fighter.battle_object, vars::miigunner::status::CURRENT_CHARGE));
	}
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			AttackModule::clear_all(boma);
		}
	}
	frame(lua_state, 50.0);
	FT_MOTION_RATE(fighter, 1.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
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
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL){
			let handle = EffectModule::req_follow(boma, Hash40::new("sys_smash_flash"), Hash40::new("top"), &Vector3f::new(0.0, 10.0, -1.0), &Vector3f::zero(), 0.75, true, 0, 0, 0, 0, 0, false, false);
			VarModule::set_int64(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER, handle);
			EffectModule::set_rate(boma, handle as u32, 0.1);
		}
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x139e44e9f0), Hash40::new("haver"), 0, 0, -3, 0, 0, 0, 1.1, true);
		LAST_EFFECT_SET_RATE(fighter, 1.3);
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL){
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
		}
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x13e943d966), Hash40::new("haver"), 0, 0, 2.5, 90, 0, 0, 0.3, true);
		EFFECT_DETACH_KIND(fighter, Hash40::new_raw(0x139e44e9f0), -1);
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_gimmckjump"), Hash40::new("armr"), 6, 0, 0, 0, -90, 0, 1, true);
			LAST_EFFECT_SET_RATE(fighter, 1.1);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 1.0, 10.0);
			if VarModule::get_float(fighter.battle_object, vars::miigunner::status::CURRENT_CHARGE) >= 10.0 {
				EFFECT_FLW_POS(fighter, Hash40::new("miigunner_gimmck_attack"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, true);
				LAST_EFFECT_SET_RATE(fighter, 1.1);
			}
		}
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x185b39be1a), Hash40::new("armr"), 6, 0, 0, 0, 0, -90, 0.75, true);
		if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL){
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
		}
	}

}

pub fn install() {
    install_acmd_scripts!(
        miigunner_attack_air_n_game,
		miigunner_attack_air_f_game,
		miigunner_attack_air_f_effect,
		miigunner_landing_air_f_game,
		miigunner_landing_air_f_effect,
        miigunner_attack_air_b_game,
		miigunner_attack_air_b_effect,
		miigunner_landing_air_b_game,
		miigunner_attack_air_hi_game,
		miigunner_attack_air_hi_effect,
		miigunner_landing_air_hi_game,
		miigunner_attack_air_hi_sound,
		miigunner_attack_air_lw_game,
		miigunner_attack_air_lw_effect,
    );
}