use super::*;

unsafe extern "C" fn game_regular(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 270, 60, 125, 55, 6.0, 0.0, 2.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f::new(0.0, -100.0), 24, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 60, 0, 74, 7.0, 0.0, 4.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }        
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", game_regular);
}
