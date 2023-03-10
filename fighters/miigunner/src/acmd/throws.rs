
use super::*;


#[acmd_script( agent = "miigunner", script = "game_throwf" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_throw_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 361, 90, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 48, 100, 140, 10, 5.5, 0.0, 6.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_catch_only_all(boma, true, false);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CHECK_FINISH_CAMERA(fighter, 13.0, 0.0);
		//FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
		//FighterCutInManager::set_throw_finish_offset(boma, 9, -2, 0);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		AttackModule::clear_all(boma);
		FT_MOTION_RATE(fighter, 18.0/(34.0 - 12.0));
	}
}

#[acmd_script( agent = "miigunner", script = "game_throwb" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_throw_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
		// Heavy throw
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
			FT_MOTION_RATE(fighter, 8.0/(3.0 - 1.0));
			ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 52, 56, 2, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
		// Normal throw
		else {
			ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 50, 45, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		}
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 1.0);
		}
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		REVERSE_LR(fighter);
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		// Heavy throw
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 9.0/(21.0 - 10.0));
		}
		// Normal throw
		else {
			FT_MOTION_RATE(fighter, 25.0/(50.0 - 10.0));
		}
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		// Explosion on heavy throw
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			FT_MOTION_RATE(fighter, 24.0/(50.0 - 21.0));
			ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 142, 40, 0, 98, 18.0, 0.0, 22.0, -31.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
		}
	}
	frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miigunner", script = "effect_throwb" , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_throw_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		// Heavy throw
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 18.0, -27.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 10.0, 0.15, 0.15);
			LAST_EFFECT_SET_RATE(fighter, 1.25);
			
			EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 5.0, 0.0, 0.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
		}
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	frame(lua_state, 19.0);
	if is_excute(fighter) {
		// Heavy throw
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 13, -13, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
		}
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		// Heavy throw
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT(fighter, Hash40::new_raw(0x1287005d5d), Hash40::new("armr"), 5.3, 0, 0, 70, 0, 90, 0.9, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 10.0, 0.7, 0.7);
			EFFECT(fighter, Hash40::new_raw(0x168b3949cd), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.9, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 10.0, 0.7, 0.7);
			LAST_EFFECT_SET_RATE(fighter, 1.6);
			
			EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 19.0, -26.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
		}
	}
}

#[acmd_script( agent = "miigunner", script = "sound_throwb" , category = ACMD_SOUND , low_priority)]
unsafe fn miigunner_throw_b_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
	}
	wait(lua_state, 8.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_attack01"));
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
	}
	wait(lua_state, 9.0);
	if is_excute(fighter) {
		// Heavy throw
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_n01"));
		}
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		// Heavy throw
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_s04"));
		}
	}
}

#[acmd_script( agent = "miigunner", script = "game_throwhi" , category = ACMD_GAME , low_priority)]
unsafe fn miigunner_throw_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 91, 54, 2, 76, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, false, 0);
		ArticleModule::change_motion(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, Hash40::new("fly_throw_hi"), false, 0.0);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, false, 0);
		ArticleModule::change_motion(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, Hash40::new("fly_throw_hi_2"), false, 0.0);
		FT_MOTION_RATE(fighter, 20.0/(52.0 - 28.0));
	}
}
   

pub fn install() {
    install_acmd_scripts!(
		miigunner_throw_f_game,
		miigunner_throw_b_game,
		miigunner_throw_b_effect,
		miigunner_throw_b_sound,
		miigunner_throw_hi_game,
	);
}

