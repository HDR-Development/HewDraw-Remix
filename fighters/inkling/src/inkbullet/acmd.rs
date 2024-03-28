use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 45, 100, 6, 0, 3.0, 0.0, 0.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        AttackModule::enable_safe_pos(boma);
        AttackModule::set_ink_value(boma, 0, 0.0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.0, 0.0, 0.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        AttackModule::set_ink_value(boma, 0, 0.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly);
}