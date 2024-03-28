use super::*;

// ================================================================================================
// ======================================== ECHO REFLECTOR ========================================
// ================================================================================================

unsafe extern "C" fn game_speciallw1start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		if agent.is_situation(*SITUATION_KIND_GROUND) {
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 4.5, 3.0, 6.5, 4.0);
		}
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		let offset = if agent.is_situation(*SITUATION_KIND_GROUND) { 0.0 } else { 2.0 };
		ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 69, 100, 0, 70, 8.0, 0.0, 6.5 + offset, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
	}
}

// ================================================================================================
// ====================================== ABSORBING VORTEX ========================================
// ================================================================================================

unsafe extern "C" fn game_speciallw3start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE_RANGE(agent, 1.0, 7.0, 5.0);
}

unsafe extern "C" fn game_speciallw3hold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE(agent, 0.8);
	if is_excute(agent) {
		let offset_y = if agent.is_situation(*SITUATION_KIND_GROUND) { 6.5 } else { 9.5 };
		ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 100, 25, 0, 12.5, 0.0, offset_y, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		let offset_y = if agent.is_situation(*SITUATION_KIND_GROUND) { 6.5 } else { 9.5 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 366, 100, 25, 0, 22.0, 0.0, offset_y, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn effect_speciallw3hold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
		EffectModule::kill_kind(boma, Hash40::new("miigunner_absorber"), false, false);
		let offset = if agent.is_situation(*SITUATION_KIND_GROUND) { 3.0 } else { 2.0 };
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_absorber"), Hash40::new("hip"), offset, 0, 0, 0, 0, 0, 0.75, true);
		EFFECT_FOLLOW(agent, Hash40::new("miigunner_absorberlight"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    COL_NORMAL(agent);
    if is_excute(agent) {
        FLASH(agent, 0.5, 0.7, 1, 0.6);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 3, 0, 0.2, 0.5, 0);
    }
    wait(lua_state, 3.0);
}

unsafe extern "C" fn game_speciallw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	FT_MOTION_RATE_RANGE(agent, 1.0, 7.0, 9.0);
	if is_excute(agent) {
		let offset_y = if agent.is_situation(*SITUATION_KIND_GROUND) { 6.5 } else { 9.5 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 55, 70, 0, 30, 14.0, 0.0, offset_y, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
	frame(lua_state, 3.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

unsafe extern "C" fn effect_speciallw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		EFFECT(agent, Hash40::new("sys_dead_flash"), Hash40::new("armr"), 1, 5.5, 0, 0, 0, 0, 0.075, 0, 0, 0, 0, 0, 0, false);
		EFFECT(agent, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
	}
}

unsafe extern "C" fn sound_speciallw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
		PLAY_SE(agent, Hash40::new("se_miigunner_attackhard_s01"));
	}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw1start", game_speciallw1start);
    agent.acmd("game_specialairlw1start", game_speciallw1start);

    agent.acmd("game_speciallw3start", game_speciallw3start);
    agent.acmd("game_specialairlw3start", game_speciallw3start);

    agent.acmd("game_speciallw3hold", game_speciallw3hold);
    agent.acmd("game_specialairlw3hold", game_speciallw3hold);
    agent.acmd("effect_speciallw3hold", effect_speciallw3hold);
    agent.acmd("effect_specialairlw3hold", effect_speciallw3hold);
	
    agent.acmd("game_speciallw3end", game_speciallw3end);
    agent.acmd("game_specialairlw3end", game_speciallw3end);
    agent.acmd("effect_speciallw3end", effect_speciallw3end);
    agent.acmd("effect_specialairlw3end", effect_speciallw3end);
    agent.acmd("sound_speciallw3end", sound_speciallw3end);
    agent.acmd("sound_specialairlw3end", sound_speciallw3end);
}