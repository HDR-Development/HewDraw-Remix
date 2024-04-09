use super::*;

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_THROW);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 80, 0, 70, 9.0, 0.0, 10.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        //ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 100, 50, 0, 11.5, 0.0, 10.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        //ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 80, 0, 70, 9.0, 0.0, 10.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        //ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 100, 50, 0, 11.5, 0.0, 10.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FT_SHEIK_STATUS_SPECIAL_HI_FLAG_FALL);
    }
}

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), -5.5, 0.5, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sheik_fushin_start"), Hash40::new("top"), -5.5, 0, -1.5, 0, 0, 0, 0.77, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("sys_bomb_b"), Hash40::new("top"), -5.5, 0, -1.5, 0, 0, 0, 0.84, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -5.5, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sheik_fushin_end"), Hash40::new("top"), 0, 7, 2.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn effect_specialairhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 5, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sheik_fushin_start_air"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.77, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sheik_fushin_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn game_speciallwattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 13.0, 361, 92, 0, 26, 3.5, 5.5, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 11.0, 361, 92, 0, 26, 3.0, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
        AttackModule::set_attack_keep_rumble(boma, 1, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.75);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials", game_specials);
    agent.acmd("game_specialairs", game_specials);

    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("game_specialairhi", game_specialairhi);
    agent.acmd("effect_specialhistart", effect_specialhistart);
    agent.acmd("effect_specialairhistart", effect_specialairhistart);

    agent.acmd("game_speciallwattack", game_speciallwattack);
}
