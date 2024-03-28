use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.67);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 7.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 10.0, 361, 100, 0, 10, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 361, 100, 0, 10, 3.5, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneer"), 11.0, 361, 120, 0, 20, 3.5, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 10, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), -1.5, 11.5, 14, 2, 5, 165, 0.95, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

unsafe extern "C" fn game_attacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.67);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 7.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
		HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 10.0, 361, 100, 0, 10, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 361, 100, 0, 10, 3.5, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneer"), 11.0, 361, 120, 0, 20, 3.5, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 10, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_attacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -3, 13.5, 14.5, 32, 6, -192, 0.9, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

unsafe extern "C" fn game_attacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.67);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 7.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 10.0, 361, 100, 0, 10, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 361, 100, 0, 10, 3.5, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneer"), 11.0, 361, 120, 0, 20, 3.5, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 10, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_attacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), -2, 6, 11, -15, 0, 195, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 13.0, 270, 100, 0, 40, 5.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 13.0, 270, 100, 0, 40, 5.8, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneer"), 13.0, 361, 107, 0, 40, 5.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("kneer"), 13.0, 361, 107, 0, 40, 5.8, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 1, 11, -5, 1.7, -39, -92, 1.65, true);
        LAST_EFFECT_SET_RATE(agent, 1);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 11, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 100, 77, 0, 80, 8.7, 0.0, 2.5, 13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3);
    agent.acmd("effect_attacks3", effect_attacks3);
    agent.acmd("game_attacks3hi", game_attacks3hi);
    agent.acmd("effect_attacks3hi", effect_attacks3hi);
    agent.acmd("game_attacks3lw", game_attacks3lw);
    agent.acmd("effect_attacks3lw", effect_attacks3lw);

    agent.acmd("game_attackhi3", game_attackhi3);
    agent.acmd("effect_attackhi3", effect_attackhi3);

    agent.acmd("game_attacklw3", game_attacklw3);
}
