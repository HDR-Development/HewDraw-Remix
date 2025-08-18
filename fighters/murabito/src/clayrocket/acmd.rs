use super::*;

unsafe extern "C" fn game_ready(agent: &mut L2CAgentBase) {
    TeamModule::set_hit_team(agent.boma(), -1);
}

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    TeamModule::set_hit_team(agent.boma(), -1);
	frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 110, 55, 0, 75, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn effect_burst(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 90, 0.5, 0, 0, 0, 0, 0, 0, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("murabito_clayrocket_jet"), 10);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_ready", game_ready, Priority::Low);
    agent.acmd("game_fly", game_fly, Priority::Low);
    agent.acmd("effect_burst", effect_burst, Priority::Low);
    agent.acmd("effect_lodgedburst", effect_burst, Priority::Low);
}
