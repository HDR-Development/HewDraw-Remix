use super::*;

unsafe extern "C" fn game_flythrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 80, 65, 0, 85, 8.0, 0.0, 0.0, 1.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 80, 65, 0, 85, 8.0, 0.0, 0.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.1);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_flythrow", game_flythrow);
}