use super::*;

// ================================================================================================
// ======================================== LUNAR LAUNCH ==========================================
// ================================================================================================

unsafe extern "C" fn game_specialhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 11.0);
	if is_excute(agent) {
		WorkModule::on_flag(boma, *FIGHTER_MIIGUNNER_STATUS_BOTTOM_SHOOT_FLAG_JUMP);
	}
	frame(lua_state, 12.0);
	if is_excute(agent) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_BOTTOMSHOOT, false, 0);
	}
	frame(lua_state, 14.0);
	if is_excute(agent) {
		notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
}

unsafe extern "C" fn effect_specialhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 2.0);
	if is_excute(agent) {
		let handle = EffectModule::req_follow(boma, Hash40::new("miigunner_bottom_shot"), Hash40::new("armr"), &Vector3f::new(6.5, 0.0, 0.0), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
		VarModule::set_int64(agent.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER, handle);
	}
	frame(lua_state, 9.0);
	if is_excute(agent) {
		let handle = VarModule::get_int64(agent.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER);
		EffectModule::set_rate(boma, handle as u32, 1.0);
		if VarModule::get_float(agent.battle_object, vars::miigunner::status::CURRENT_CHARGE) <= 10.0 && !VarModule::is_flag(agent.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_AIR_USED) {
			EffectModule::set_rgb(boma, handle as u32, 0.15, 0.55, 10.0);
		}
	}
	frame(lua_state, 12.0);
	if is_excute(agent) {
		LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
	}
}

// ================================================================================================
// ====================================== FLASH POINT DIVE ========================================
// ================================================================================================

unsafe extern "C" fn game_specialhi2squat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 3.0);
	FT_MOTION_RATE(agent, 0.5);
	frame(lua_state, 7.0);
	FT_MOTION_RATE(agent, 0.25);
	frame(lua_state, 11.0);
	FT_MOTION_RATE(agent, 1.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 110, 85, 40, 90, 7.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 13.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

unsafe extern "C" fn effect_specialhi2squat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot_after"), Hash40::new("armr"), 5.5, 0, 0, 0, 0, -90, 1, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
		if agent.is_situation(*SITUATION_KIND_GROUND) {
			LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
			EFFECT(agent, Hash40::new("miigunner_gimmck_bomb1"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
			EFFECT(agent, Hash40::new("miigunner_gimmck_bomb2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
		}
		else {
			EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
		}
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 2.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 80, 100, 0, 83, 5.0, 0.0, 17.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 80, 100, 0, 83, 4.0, 0.0, 12.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 5.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 75, 90, 0, 83, 4.0, 0.0, 17.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 75, 90, 0, 83, 3.5, 0.0, 12.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 8.0);
	if is_excute(agent) {
		notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
	frame(lua_state, 23.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
	frame(lua_state, 27.0);
	if is_excute(agent) {
		notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

// ================================================================================================
// ======================================= JETPACK JOYRIDE ========================================
// ================================================================================================

unsafe extern "C" fn game_specialhi3start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 1.5);
}

unsafe extern "C" fn game_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 0.85);
}

unsafe extern "C" fn effect_landingfallspecial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		EFFECT_OFF_KIND(agent, Hash40::new("miigunner_bottom_shot"), false, false);
	}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi1", game_specialhi1);
    agent.acmd("game_specialairhi1", game_specialhi1);
    agent.acmd("effect_specialhi1", effect_specialhi1);
    agent.acmd("effect_specialairhi1", effect_specialhi1);

    agent.acmd("game_specialhi2squat", game_specialhi2squat);
    agent.acmd("game_specialairhi2squat", game_specialhi2squat);
    agent.acmd("effect_specialhi2squat", effect_specialhi2squat);
    agent.acmd("effect_specialairhi2squat", effect_specialhi2squat);

    agent.acmd("game_specialhi2", game_specialhi2);

    agent.acmd("game_specialhi3start", game_specialhi3start);
    agent.acmd("game_specialairhi3start", game_specialhi3start);

    agent.acmd("game_specialhi3", game_specialhi3);

	agent.acmd("effect_landingfallspecial", effect_landingfallspecial);
}