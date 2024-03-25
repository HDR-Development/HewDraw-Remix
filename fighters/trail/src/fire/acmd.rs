use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let lr = PostureModule::lr(boma);
        let offset = Vector2f {x: (14.0 * lr), y: 7.5}; // distance for article to be offset
        PostureModule::add_pos_2d(boma, &offset);
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.6, 361, 24, 0, 42, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1.2, -1.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::enable_safe_pos(boma);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 0.0, false);
        ATK_SET_SHIELD_SETOFF_MUL2(agent, 0, 1.05);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.2, 361, 22, 0, 36, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -0.8, -1.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 0.0, false);
        ATK_SET_SHIELD_SETOFF_MUL2(agent, 0, 1.05);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.8, 361, 20, 0, 32, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -0.8, -1.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 0.0, false);
        ATK_SET_SHIELD_SETOFF_MUL2(agent, 0, 1.05);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly);
    agent.acmd("game_fly2", game_fly);
}
