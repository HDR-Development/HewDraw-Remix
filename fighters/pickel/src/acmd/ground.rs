use super::*;

// special_s_failed is now functionally steve's dash attack, when he does not have iron 

unsafe extern "C" fn game_specialsfailed(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 8.0, 361, 40, 0, 75, 3.0, 0.0, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("head"), 8.0, 361, 40, 0, 75, 3.5, 1.5, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialsfailed(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pickel_erace_smoke"), Hash40::new("top"), 0, 4.5, 10, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0.0, 0, -2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0.0, 0, 9, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(lua_state, 34.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialsfailed(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pickel_swing_s02"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pickel_special_s11"));
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_down_m_01"));
    }
}

unsafe extern "C" fn expression_specialsfailed(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 8);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialsfailed", game_specialsfailed, Priority::Low);
    agent.acmd("effect_specialsfailed", effect_specialsfailed, Priority::Low);
    agent.acmd("sound_specialsfailed", sound_specialsfailed, Priority::Low);
    agent.acmd("expression_specialsfailed", expression_specialsfailed, Priority::Low);

    agent.acmd("game_attackdash", acmd_stub, Priority::Low);
    agent.acmd("effect_attackdash", acmd_stub, Priority::Low);
    agent.acmd("sound_attackdash", acmd_stub, Priority::Low);
    agent.acmd("expression_attackdash", acmd_stub, Priority::Low);
}
