use super::*;

unsafe extern "C" fn game_miigunnerspecialn1firemax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 3.0);
	if is_excute(agent) {
		ArticleModule::shoot_exist(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_SHOOT);
	} 
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	} 
}

unsafe extern "C" fn effect_miigunnerspecialn1firemax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 3.0);
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		if agent.is_situation(*SITUATION_KIND_GROUND) {
			LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
	}
}

unsafe extern "C" fn sound_miigunnerspecialn1firemax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		PLAY_SEQUENCE(agent, Hash40::new("seq_miigunner_rnd_special_c1_n01"));
	}
}

unsafe extern "C" fn game_miigunnerspecialn1neon(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE_RANGE(agent, 1.0, 3.0, 10.0);
    if is_excute(agent) {
		WorkModule::off_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_BULLET_DISP);
    }
	frame(lua_state, 3.0);
	FT_MOTION_RATE_RANGE(agent, 3.0, 5.0, 5.0);
	if is_excute(agent) {
		ArticleModule::remove_exist(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_SHOOT);
		ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 361, 74, 0, 61, 7.0, 0.0, 5.5, 8.0, Some(0.0), Some(5.5), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		ATTACK(agent, 1, 0, Hash40::new("top"), 22.0, 361, 74, 0, 61, 5.0, 0.0, 5.5, 8.0, Some(0.0), Some(5.5), Some(21.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
	} 
	frame(lua_state, 5.0);
	FT_MOTION_RATE_RANGE(agent, 8.0, 40.0, 37.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

unsafe extern "C" fn effect_miigunnerspecialn1neon(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 7.3, 0, 0, 0, 0, 0, 3.5, true);
		LAST_EFFECT_SET_RATE(agent, 2.0);
		LAST_EFFECT_SET_COLOR(agent, 0.15, 100.0, 10.0);
		EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 7.3, 0, 0, 90, 0, 0, 3.5, true);
		LAST_EFFECT_SET_RATE(agent, 2.0);
		LAST_EFFECT_SET_COLOR(agent, 0.15, 100.0, 3.0);
	}
	frame(lua_state, 2.6);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("miigunner_sb_tama"), false, false);
		EFFECT_DETACH_KIND(agent, Hash40::new("miigunner_sb_tama"), -1);
	}
	frame(lua_state, 2.8);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_air_bullet"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
		LAST_EFFECT_SET_COLOR(agent, 0.15, 5.0, 0.55);
		LAST_EFFECT_SET_RATE(agent, 0.85);
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 5, 3, 0, 0, 0, 0, 1, false);
		EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_laser"), Hash40::new("top"), 0, 5.3, 10.5, 0, 0, 0, 1, false);
		LAST_EFFECT_SET_SCALE_W(agent, 1.0, 0.7, 1.0);
		LAST_EFFECT_SET_RATE(agent, 0.8);
		EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot_s"), Hash40::new("armr"), 6.3, 0, -1, 0, 0, -90, 1, false);
		LAST_EFFECT_SET_COLOR(agent, 0.15, 5.0, 5.0);
		LAST_EFFECT_SET_RATE(agent, 0.5);
		if agent.is_situation(*SITUATION_KIND_GROUND) {
			LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
		}
	}
}

unsafe extern "C" fn sound_miigunnerspecialn1neon(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		PLAY_SE(agent, Hash40::new("se_miigunner_final01"));
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		STOP_SE(agent, Hash40::new("se_miigunner_final01"));
		PLAY_SE(agent, Hash40::new("se_miigunner_final04"));
		PLAY_SE(agent, Hash40::new("vc_kirby_copy_mii_09"));
	}
}

unsafe extern "C" fn expression_miigunnerspecialn1neon(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beaml"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_miigunnerspecialn1firemax", game_miigunnerspecialn1firemax, Priority::Low);
    agent.acmd("game_miigunnerspecialairn1firemax", game_miigunnerspecialn1firemax, Priority::Low);
    agent.acmd("effect_miigunnerspecialn1firemax", effect_miigunnerspecialn1firemax, Priority::Low);
    agent.acmd("effect_miigunnerspecialairn1firemax", effect_miigunnerspecialn1firemax, Priority::Low);
    agent.acmd("sound_miigunnerspecialn1firemax", sound_miigunnerspecialn1firemax, Priority::Low);
    agent.acmd("sound_miigunnerspecialairn1firemax", sound_miigunnerspecialn1firemax, Priority::Low);

	agent.acmd("game_miigunnerspecialn1neon", game_miigunnerspecialn1neon, Priority::Low);
    agent.acmd("game_miigunnerspecialairn1neon", game_miigunnerspecialn1neon, Priority::Low);
    agent.acmd("effect_miigunnerspecialn1neon", effect_miigunnerspecialn1neon, Priority::Low);
    agent.acmd("effect_miigunnerspecialairn1neon", effect_miigunnerspecialn1neon, Priority::Low);
    agent.acmd("sound_miigunnerspecialn1neon", sound_miigunnerspecialn1neon, Priority::Low);
    agent.acmd("sound_miigunnerspecialairn1neon", sound_miigunnerspecialn1neon, Priority::Low);
	agent.acmd("expression_miigunnerspecialn1neon", expression_miigunnerspecialn1neon, Priority::Low);
    agent.acmd("expression_miigunnerspecialairn1neon", expression_miigunnerspecialn1neon, Priority::Low);
}