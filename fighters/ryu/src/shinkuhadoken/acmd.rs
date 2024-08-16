use super::*;

unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::enable_safe_pos(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 366, 100, 75, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -7.0, 0.0, 5, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 366, 100, 75, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -2.2, 0.0, 5, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 366, 100, 75, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -0.6, 0.0, 5, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn effect_move(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_ryu_final_shinkuhado2"), false, false, false);
        EFFECT_FOLLOW(agent, Hash40::new("ryu_final_shinkuhado_bullet"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1.2, true);
        // EFFECT_FOLLOW(agent, Hash40::new("ryu_final_shinkuhado_bullet2"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.5, true);
    }
}

unsafe extern "C" fn game_finish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 7.0, 97, 130, 0, 50, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn effect_finish(agent: &mut L2CAgentBase) {{
        EFFECT(agent, Hash40::new("ryu_final_shinkuhado_finish"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_move", game_move, Priority::Low);
    agent.acmd("effect_move", effect_move, Priority::Low);

    agent.acmd("game_finish", game_finish, Priority::Low);
    agent.acmd("effect_finish", effect_finish, Priority::Low);
}