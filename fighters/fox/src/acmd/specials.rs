use super::*;

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 20.0/25.0);
}

unsafe extern "C" fn game_specialairsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.2);
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

unsafe extern "C" fn game_specialhihold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 20.0);
    for _ in 0..7 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.8, 105, 20, 0, 20, 8.0, 0.0, 8.0, 0.0, None, None, None, 0.8, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 16.0, 60, 60, 0, 70, 5.0, 2.5, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 10.0, 60, 50, 0, 85, 5.0, 2.5, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);
    let sound = if rng == 0 { "vc_fox_special_h01" } else { "vc_fox_attack06" };
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new(sound));
        PLAY_SE(agent, Hash40::new("se_fox_special_h02"));
    }
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 4.0, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 360, 100, 80, 0, 8.0, 0.0, 6.5, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 24, 24, 0, 66, 8.0, 0.0, 6.5, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame_revised(boma, 0, -3.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 2.0, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("sys_hit_elec"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("sys_hit_elec"));
        ReflectorModule::set_status(boma, *FIGHTER_FOX_REFLECTOR_KIND_REFLECTOR, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_EXTEND);
        // Reflection begins on same frame shine hitbox is active
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 0.54);
    }
    frame(lua_state, 2.0);
    if is_excute(agent){
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("fox_ref_loop"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("fox_ref_ref"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 5.0, 0, 0, 0, 0, 2.8, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        LAST_EFFECT_SET_RATE(agent, 1.5);
	}
}

unsafe extern "C" fn sound_speciallwstart(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
		PLAY_SE(agent, Hash40::new("se_item_item_get"));
	}
}

unsafe extern "C" fn game_specialairlwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 4.0, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 10, 32, 0, 62, 8.0, 0.0, 6.5, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 24, 45, 0, 66, 8.0, 0.0, 6.5, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("sys_hit_elec"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("sys_hit_elec"));
        ReflectorModule::set_status(boma, *FIGHTER_FOX_REFLECTOR_KIND_REFLECTOR, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_EXTEND);
        // Reflection begins on same frame shine hitbox is active
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 0.54);
    }
    frame(lua_state, 2.0);
    if is_excute(agent){
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialsstart", game_specialsstart);
    agent.acmd("game_specialairsstart", game_specialsstart);

    agent.acmd("game_specialairsend", game_specialairsend);

    agent.acmd("game_specialhihold", game_specialhihold);
    agent.acmd("game_specialhiholdair", game_specialhihold);

    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("sound_specialhi", sound_specialhi);

    agent.acmd("game_speciallwstart", game_speciallwstart);
    agent.acmd("effect_speciallwstart", effect_speciallwstart);
    agent.acmd("sound_speciallwstart", sound_speciallwstart);
    agent.acmd("game_specialairlwstart", game_specialairlwstart);
    agent.acmd("effect_specialairlwstart", effect_speciallwstart);
    agent.acmd("sound_specialairlwstart", sound_speciallwstart);
}
