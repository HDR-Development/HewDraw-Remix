use super::*;

unsafe extern "C" fn effect_check(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn game_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 135, 100, 40, 0, 2.0, 0.0, -3.1, 0.0, None, None, None, 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.7, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 165, 100, 40, 0, 5.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.7, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 3.3);
        AttackModule::set_size(boma, 1, 6.4);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 4.6);
        AttackModule::set_size(boma, 1, 7.7);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 5.9);
        AttackModule::set_size(boma, 1, 9.0);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 7.2);
        AttackModule::set_size(boma, 1, 10.3);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ATTACK(agent, 0, 1, Hash40::new("top"), 5.5, 84, 141, 0, 60, 13.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, (30.0/28.0));
        EFFECT(agent, Hash40::new("palutena_bomb_appear"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_bomb_finish"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.325);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_bomb"), false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_check", effect_check, Priority::Low);

    agent.acmd("game_explode", game_explode, Priority::Low);
    agent.acmd("effect_explode", effect_explode, Priority::Low);
}