use super::*;

unsafe extern "C" fn miigunner_attack_s3_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 2.0);
	FT_MOTION_RATE_RANGE(agent, 2.0, 10.0, 5.0);
	frame(lua_state, 10.0);
	FT_MOTION_RATE(agent, 1.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("armr"), 9.0, 361, 56, 0, 56, 3.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
		ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 361, 56, 0, 56, 2.5, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
	}
	frame(lua_state, 11.0);
	if is_excute(agent) {
		ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 361, 56, 0, 56, 2.5, 0.0, 8.0, 6.0, Some(0.0), Some(15.5), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
		ATTACK(agent, 3, 0, Hash40::new("top"), 13.0, 361, 56, 0, 56, 2.5, 0.0, 8.0, 6.0, Some(0.0), Some(2.5), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
	}
	frame(lua_state, 12.0);
	if is_excute(agent) {
		ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 361, 56, 0, 56, 3.0, 0.0, 8.0, 6.0, Some(0.0), Some(8.0), Some(25.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
	}
	frame(lua_state, 13.0);
	if is_excute(agent) {
		ATK_POWER(agent, 1, 10.0);
		ATK_POWER(agent, 2, 10.0);
		ATK_POWER(agent, 3, 10.0);
	}
	frame(lua_state, 14.0);
	if is_excute(agent) {
		ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 361, 56, 0, 56, 1.5, 0.0, 8.0, 6.0, Some(0.0), Some(8.0), Some(25.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
		AttackModule::set_size(boma, 2, 2.0);
		AttackModule::set_size(boma, 3, 2.0);
	}
	frame(lua_state, 16.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

unsafe extern "C" fn miigunner_attack_s3_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("miigunner_atk_laser"), Hash40::new("armr"), 6.3, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn miigunner_attack_hi3_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 5.0);
	if is_excute(agent) {
		HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
		ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 9.0, 90, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 95, 100, 0, 50, 6.0, 0.0, 6.0, 9.5, Some(0.0), Some(13.0), Some(9.25), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
		ATTACK(agent, 2, 0, Hash40::new("handr"), 9.0, 90, 80, 0, 50, 2.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 6.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("armr"), 9.0, 90, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(agent, 1, 0, Hash40::new("handr"), 10.0, 95, 100, 0, 50, 6.0, 9.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 8.0);
	if is_excute(agent) {
		ATTACK(agent, 1, 0, Hash40::new("handr"), 8.0, 95, 70, 0, 50, 6.0, 9.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 11.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
}

unsafe extern "C" fn miigunner_attack_lw3_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 8.0);
	if is_excute(agent) {
		ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 9.0, 85, 50, 0, 82, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BOMB);
		ATTACK(agent, 1, 0, Hash40::new("armr"), 9.0, 85, 50, 0, 82, 3.5, 4.5, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BOMB);
		ATTACK(agent, 2, 0, Hash40::new("top"), 14.0, 80, 79, 0, 70, 6.0, 0.0, 1.6, 17.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
		AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	wait(lua_state, 7.0);
	FT_MOTION_RATE_RANGE(agent, 15.0, 41.0, 20.0);
	if is_excute(agent) {
		AttackModule::clear_all(boma);
	}
}

pub fn install() {
    smashline::Agent::new("miigunner")
        .acmd("game_attacks3", miigunner_attack_s3_game)
        .acmd("effect_attacks3", miigunner_attack_s3_effect)
        .acmd("game_attackhi3", miigunner_attack_hi3_game)
        .acmd("game_attacklw3", miigunner_attack_lw3_game)
        .install();
}
