use super::*;

unsafe extern "C" fn effect_specialn1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire1_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
}

unsafe extern "C" fn game_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::disable_tip(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 82, 15, 0, 55, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn game_specialn3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::disable_tip(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 82, 15, 0, 55, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn game_bursts(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_furafura"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosionm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        AttackModule::disable_tip(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.5, 70, 64, 0, 77, 12.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_bursts(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("edge_fire1_sign"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("edge_fire1_burst"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn game_burstm(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_furafura"), 14, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        AttackModule::disable_tip(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.7, 366, 100, 50, 0, 19.3, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, -1.0, 3, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.7, 366, 100, 60, 0, 19.3, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, -1.0, 3, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 9.0, 58, 96, 0, 84, 19.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, -1.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_burstm(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("edge_fire2_sign"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("edge_fire2_burst"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("edge_fire2_burst2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn game_burstl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_furafura"), 16, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosionl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        AttackModule::disable_tip(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.7, 115, 100, 50, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.7, 115, 100, 35, 0, 17.5, 0.0, 0.0, 0.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 115, 100, 35, 0, 24.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.7, 366, 100, 75, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.7, 366, 100, 65, 0, 17.5, 0.0, 0.0, 0.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 366, 100, 65, 0, 24.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 11.0, 54, 115, 0, 105, 10.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 1, Hash40::new("top"), 8.0, 52, 115, 0, 95, 18.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 2, 1, Hash40::new("top"), 4.0, 52, 95, 0, 95, 24.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_burstl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("edge_fire3_sign"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EffectModule::req_screen(boma, Hash40::new("edge_fire3_screen2"), false, false, false);
        EFFECT(agent, Hash40::new("edge_fire3_burst"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("edge_fire3_burst2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn stub(agent: &mut L2CAgentBase) {}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn1", stub);
    agent.acmd("game_specialn2", game_specialn2);
    agent.acmd("game_specialn3", game_specialn3);
    
    agent.acmd("game_bursts", game_bursts);
    agent.acmd("effect_bursts", effect_bursts);

    agent.acmd("game_burstm", game_burstm);
    agent.acmd("effect_burstm", effect_burstm);

    agent.acmd("game_burstl", game_burstl);
    agent.acmd("effect_burstl", effect_burstl);
}