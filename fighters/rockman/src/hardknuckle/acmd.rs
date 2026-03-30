use super::*;

unsafe extern "C" fn game_regular(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	if is_excute(agent) {
        PostureModule::add_pos(boma, &Vector3f::new(0.0, 4.5, 0.0));
        AttackModule::enable_safe_pos(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 270, 48, 0, 30, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 3.0);
	if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_ROCKMAN_HARDKNUCKLE_INSTANCE_WORK_ID_FLAG_ATTACK_VECTOR_REVERSE_UD_CHECK);
    }
    frame(lua_state, 4.0);
	if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 90, 60, 0, 30, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        //sv_kinetic_energy!(set_speed, agent, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -0.5);
        sv_kinetic_energy!(set_stable_speed, agent, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 1.0);
        sv_kinetic_energy!(set_brake, agent, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.3);
    }
}

unsafe extern "C" fn effect_regular(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("rockman_hardknuckle"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    // frame(lua_state, 6.0);
    // if is_excute(agent) {
    //     EFFECT_OFF_KIND(agent, Hash40::new("rockman_hardknuckle"), true, true);
    // }
}

unsafe extern "C" fn game_stick(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 75, 70, 0, 60, 2.0, 7.0, 0.0, -1.0, Some(-7.0), Some(0.0), Some(-1.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_stick(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, -1, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", game_regular, Priority::Low);
    agent.acmd("effect_regular", effect_regular, Priority::Low);

    agent.acmd("game_stick", game_stick, Priority::Low);
    agent.acmd("effect_stick", effect_stick, Priority::Low);
}