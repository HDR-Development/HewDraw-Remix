use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let owner_boma = agent.get_owner_boma();
        if owner_boma.kind() != *FIGHTER_KIND_MIIGUNNER || !VarModule::is_flag(owner_boma.object(), vars::miigunner::instance::BOOSTED_AERIAL_LANDING) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 48, 91, 0, 40, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 48, 91, 0, 40, 2.8, 0.0, -0.5, 0.0, Some(0.0), Some(-0.5), Some(-11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 48, 91, 0, 40, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
        AttackModule::clear(boma, 1, false);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 52, 91, 0, 40, 1.8, 0.0, 0.0, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_air_bullet"), Hash40::new("top"), 0, -0.5, 0.5, 0, 180, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly, Priority::Low);
    agent.acmd("effect_fly", effect_fly, Priority::Low);
}