use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 40, 74, 0, 40, 1.6, 0.0, 0.0, -1.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_bowarrow"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn effect_haved(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_bow_hold1"), Hash40::new("top"), 0, 0, -0.5, 0, 0, 0, 0.85, true);
        LAST_EFFECT_SET_RATE(agent, 25.0/14.0); // spawn flash before fire
    }
}

unsafe extern "C" fn effect_haved2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let frame_2 = 45.0 + ((105.0-45.0)/0.9*(56.0/75.0));
    let frame_3 = 45.0 + ((118.0-45.0)/0.9*(56.0/75.0));
    frame(lua_state, 48.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_bow_hold2"), Hash40::new("top"), 0, 0, -0.5, 0, 0, 0, 0.85, true);
        LAST_EFFECT_SET_RATE(agent, (0.9*75.0/56.0));
    }
    frame(lua_state, frame_2);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("master_bow_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, frame_3);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("master_bow_hold2"), -1);
        EFFECT_OFF_KIND(agent, Hash40::new("master_bow_hold2"), false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly, Priority::Low);

    agent.acmd("effect_haved", effect_haved, Priority::Low);

    agent.acmd("effect_haved2", effect_haved2, Priority::Low);
}