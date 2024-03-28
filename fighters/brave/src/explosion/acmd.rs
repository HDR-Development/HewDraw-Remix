use super::*;

unsafe extern "C" fn game_explode2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::disable_tip(boma);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 366, 100, 100, 0, 25.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 366, 100, 50, 0, 25.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AREA_WIND_2ND_RAD(agent, 0, 0.5, 0.01, 1000, 1, 0, 0, 40);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 14, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ATTACK(agent, 0, 0, Hash40::new("top"), 26.0, 65, 50, 0, 80, 13.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -13, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 21.0, 60, 50, 0, 80, 18.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosionl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 18.0);
        AttackModule::set_size(boma, 1, 23.0);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 23.0);
        AttackModule::set_size(boma, 1, 28.0);
        AREA_WIND_2ND_RAD(agent, 0, 1, 0.1, 1000, 1, 0, 0, 40);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_explode2", game_explode2);
}