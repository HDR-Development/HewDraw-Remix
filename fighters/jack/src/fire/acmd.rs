use super::*;
unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if agent.is_flag(*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        FT_MOTION_RATE(agent, 8.0/(7.0-1.0));
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 20, 0, 20, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(2.0), Some(18.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-1.0), Some(26.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-4.0), Some(34.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE_RANGE(agent, 12.0, 16.0, 2.0);
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 20, 0, 20, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(2.0), Some(18.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-1.0), Some(26.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-4.0), Some(34.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 35, 50, 0, 60, 2.5, 0.0, 7.0, 5.0, Some(0.0), Some(2.0), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-1.0), Some(26.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 7.0, 5.0, Some(0.0), Some(-4.0), Some(34.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }  

}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_attacklw3", effect_attacklw3);
}
