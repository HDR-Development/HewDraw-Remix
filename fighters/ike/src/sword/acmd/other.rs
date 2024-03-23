use super::*;

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 17.0/13.0);
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
    /* Air-only */
    ATTACK(agent, 0, 0, Hash40::new("haver"), 9.0, 270, 33, 0, 30, 3.5, 0.0, 6.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    ATTACK(agent, 1, 0, Hash40::new("haver"), 9.0, 270, 33, 0, 30, 4.5, 0.0, 12.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    /* Ground-only */
    ATTACK(agent, 2, 0, Hash40::new("haver"), 9.0, 270, 100, 275, 0, 3.5, 0.0, 6.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 9.0, 270, 100, 275, 0, 4.5, 0.0, 12.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            EFFECT(agent, Hash40::new("ike_volcano_ground"), Hash40::new("top"), 0, 0.0, -8.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent, Hash40::new("ike_volcano"), Hash40::new("top"), 0, 0.0, -8.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
}
    
pub fn install(agent: &mut Agent) {
    agent.acmd("game_appeallw", game_appeallw);
    agent.acmd("effect_appeallw", effect_appeallw);
}