use super::*;

// ================================================================================================
// ================================ CHARGE BLAST / NEON CANNON ====================================
// ================================================================================================

unsafe extern "C" fn game_specialn1start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 10.0);
	if is_excute(agent) {
		ArticleModule::generate_article_enable(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, false, 0);
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_BULLET_DISP);
	}
}

unsafe extern "C" fn game_specialnifiremax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
		FT_MOTION_RATE_RANGE(agent, 1.0, 3.0, 10.0);
	}
    if is_excute(agent) {
		VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
			WorkModule::off_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_BULLET_DISP);
        }
    }
	frame(lua_state, 3.0);
	if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
		FT_MOTION_RATE_RANGE(agent, 3.0, 5.0, 5.0);
	}
	else {
		FT_MOTION_RATE(agent, 1.0);
	}
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			ArticleModule::remove_exist(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_SHOOT);
			ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 361, 74, 0, 61, 7.0, 0.0, 7.5, 8.0, Some(0.0), Some(7.5), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
			ATTACK(agent, 1, 0, Hash40::new("top"), 22.0, 361, 74, 0, 61, 5.0, 0.0, 7.5, 8.0, Some(0.0), Some(7.5), Some(21.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		}
		else {
			ArticleModule::shoot_exist(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
			WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_SHOOT);
		}
	} 
	frame(lua_state, 5.0);
	if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
		FT_MOTION_RATE_RANGE(agent, 8.0, 40.0, 37.0);
	}
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	} 
}

unsafe extern "C" fn effect_specialn1firemax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 0, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(agent, 2.0);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 100.0, 10.0);
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 90, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(agent, 2.0);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 100.0, 3.0);
		}
	}
	frame(lua_state, 2.6);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("miigunner_sb_tama"), false, false);
		EFFECT_DETACH_KIND(agent, Hash40::new("miigunner_sb_tama"), -1);
	}
	frame(lua_state, 2.8);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_air_bullet"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 5.0, 0.55);
			LAST_EFFECT_SET_RATE(agent, 0.85);
		}
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, false);
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_laser"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_SCALE_W(agent, 1.0, 0.7, 1.0);
			LAST_EFFECT_SET_RATE(agent, 0.8);
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot_s"), Hash40::new("armr"), 6.3, 0, 0, 0, 0, -90, 1, false);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 5.0, 5.0);
			LAST_EFFECT_SET_RATE(agent, 0.5);
			if agent.is_situation(*SITUATION_KIND_GROUND) {
				LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
				FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
			}
		}
		else {
			EFFECT(agent, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			if agent.is_situation(*SITUATION_KIND_GROUND) {
				LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			}
		}
	}
}

unsafe extern "C" fn sound_specialn1firemax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		PLAY_SEQUENCE(agent, Hash40::new("seq_miigunner_rnd_special_c1_n01"));
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			PLAY_SE(agent, Hash40::new("se_miigunner_final01"));
		}
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			STOP_SE(agent, Hash40::new("se_miigunner_final01"));
			PLAY_SE(agent, Hash40::new("se_miigunner_final04"));
		}
	}
}

// ================================================================================================
// ======================================== LASER BLAZE ===========================================
// ================================================================================================

unsafe extern "C" fn game_specialn2loop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 1.0);
	frame(lua_state, 2.0);
	if is_excute(agent) {
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
	}
	frame(lua_state, 4.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 45, 100, 0, 30, 4.0, 2.0, 0.0, 0.0, Some(4.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, false, 0);
	}
    frame(lua_state, 6.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

// ================================================================================================
// ========================================= BOMBS AWAY ===========================================
// ================================================================================================

unsafe extern "C" fn game_specialn3start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		VarModule::set_float(agent.battle_object, vars::miigunner::instance::GRENADE_CHARGE, 0.0);
	}
	frame(lua_state, 16.0);
	FT_MOTION_RATE_RANGE(agent, 16.0, 30.0, 6.0);
	frame(lua_state, 25.0);
}

unsafe extern "C" fn effect_specialn3start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_flame_shot"), Hash40::new("armr"), 5.5, 0, 0, 0, 90, 0, 0.49, true);
        LAST_EFFECT_SET_RATE(agent, 0.15);
    }
}

unsafe extern "C" fn game_specialn3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE_RANGE(agent, 1.0, 23.0, 30.0);
	if is_excute(agent) {
		ArticleModule::generate_article_enable(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GRENADELAUNCHER, false, 0);
	}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn1start", game_specialn1start);
    agent.acmd("game_specialairn1start", game_specialn1start);

    agent.acmd("game_specialn1firemax", game_specialnifiremax);
    agent.acmd("game_specialairn1firemax", game_specialnifiremax);
    agent.acmd("effect_specialn1firemax", effect_specialn1firemax);
    agent.acmd("effect_specialairn1firemax", effect_specialn1firemax);
    agent.acmd("sound_specialn1firemax", sound_specialn1firemax);
    agent.acmd("sound_specialairn1firemax", sound_specialn1firemax);

    agent.acmd("game_specialn2loop", game_specialn2loop);
    agent.acmd("game_specialairn2loop", game_specialn2loop);

    agent.acmd("game_specialn3start", game_specialn3start);
    agent.acmd("game_specialairn3start", game_specialn3start);
    agent.acmd("effect_specialn3start", effect_specialn3start);

    agent.acmd("game_specialn3end", game_specialn3end);
    agent.acmd("game_specialairn3end", game_specialn3end);
}