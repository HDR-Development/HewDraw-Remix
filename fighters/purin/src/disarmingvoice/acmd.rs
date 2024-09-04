use super::*;

unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::enable_safe_pos(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn effect_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("poke_meloetta_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 727, true);
    }
    for _ in 0..15 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("rosetta_ring_erase"), false, false);
            EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 0, 0, 0, 0, 25, 727, false);
            EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
        }
        wait(lua_state, 4.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shoot", game_shoot, Priority::Low);
    agent.acmd("effect_shoot", effect_shoot, Priority::Low);
}