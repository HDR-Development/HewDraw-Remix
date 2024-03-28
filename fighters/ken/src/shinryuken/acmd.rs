use super::*;
unsafe extern "C" fn ken_shinryuken_game_final(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 110, 120, 130, 0, 11.0, 0.0, -10.0, 8.0, Some(0.0), Some(50.0), Some(8.0), 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 100, 100, 0, 11.0, 0.0, 50.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        boma.on_flag(*WEAPON_KEN_SHINRYUKEN_INSTANCE_WORK_ID_FLAG_ADD_ATTACK);
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.9, 110, 85, 70, 0, 11.0, 0.0, -10.0, 8.0, Some(0.0), Some(50.0), Some(8.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.9, 367, 100, 100, 0, 11.0, 0.0, 50.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 90.0);
    if is_excute(agent) {
        boma.off_flag(*WEAPON_KEN_SHINRYUKEN_INSTANCE_WORK_ID_FLAG_ADD_ATTACK);
        AttackModule::clear_all(boma);
        ATTACK(agent, 3, 0, Hash40::new("top"), 13.0, 90, 59, 0, 100, 11.0, 0.0, -10.0, 8.0, Some(0.0), Some(50.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 90, 59, 0, 100, 11.0, 0.0, 50.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}
unsafe extern "C" fn ken_shinryuken_effect_final(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 51.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ken_final_shinryuken_beam"), Hash40::new("top"), 0, 0, 8, 0, 90, 0, 1.0, false);
        EffectModule::set_scale_last(boma, &Vector3f::new(0.6, 0.36, 0.6));
    }
    frame(lua_state, 97.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("ken_final_shinryuken_beam"), -1);
    }
}
pub fn install(agent: &mut Agent) {
    agent.acmd("game_final", ken_shinryuken_game_final);
    agent.acmd("effect_final", ken_shinryuken_effect_final);
}