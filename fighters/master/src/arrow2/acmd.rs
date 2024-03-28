use super::*;

unsafe extern "C" fn game_search(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 30.0, 361, 60, 0, 53, 1.5, 2.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(1.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 9, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_ARROW_MAX, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 30.0, 361, 60, 0, 53, 1.5, -2.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(1.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 9, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_ARROW_MAX, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 30.0, 361, 60, 0, 53, 1.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(1.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 9, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_ARROW_MAX, *ATTACK_REGION_ENERGY);
        AttackModule::disable_tip(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_search", game_search);
}
