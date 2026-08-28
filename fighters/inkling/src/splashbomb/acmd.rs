use super::*;

unsafe extern "C" fn game_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 50, 56, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_BOMB);
        AttackModule::set_ink_value(boma, 0, 0.0);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 11.0);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 13.0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("inkling_splashbomb_explosion"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        let r = WorkModule::get_float(boma, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_R);
        let g = WorkModule::get_float(boma, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_G);
        let b = WorkModule::get_float(boma, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_B);
        LAST_PARTICLE_SET_COLOR(agent, r, g, b);
    }
}

unsafe extern "C" fn game_loop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        SEARCH(agent, 0, 0, Hash40::new("top"), 2.8, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NI, 180, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_explode", game_explode, Priority::Low);
    agent.acmd("effect_explode", effect_explode, Priority::Low);

    agent.acmd("game_loop", game_loop, Priority::Low);
}