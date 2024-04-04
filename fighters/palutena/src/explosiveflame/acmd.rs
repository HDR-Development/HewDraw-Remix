use super::*;

unsafe extern "C" fn game_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 160, 100, 50, 0, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.7, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 6.0);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 7.2);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 8.4);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 9.6);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 10.8);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 12.0);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        //AREA_WIND_2ND_RAD(agent, 0, 1, 0.02, 1000, 1, 0, 0, 29);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 5.5, 84, 141, 0, 60, 15.5, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_explode", game_explode);
}