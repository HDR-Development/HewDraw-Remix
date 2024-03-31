use super::*;

unsafe extern "C" fn game_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_DESIRED_RATE(agent, 26.0, 30.0);
    frame(lua_state, 26.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 17.0, 80, 60, 0, 90, 8.0, 0.0, 8.0, 15.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 80, 60, 0, 90, 6.0, 0.0, 6.0, -3.0, Some(0.0), Some(6.0), Some(7.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 2, true, false);
        if WorkModule::is_flag(boma, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
            AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_kamui_criticalhit"));
        }
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 15.0, 80, 60, 0, 90, 5.0, 0.0, 21.0, 15.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 80, 60, 0, 90, 6.0, 0.0, 6.0, -3.0, Some(0.0), Some(6.0), Some(7.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(boma, 1, true, false);
        AttackModule::set_force_reaction(boma, 2, true, false);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 80, 60, 0, 90, 7.0, 0.0, 10.0, 15.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 80, 60, 0, 90, 5.0, 0.0, 22.0, 15.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
    }
    frame(lua_state, 31.0);
    FT_MOTION_RATE_RANGE(agent, 31.0, 66.0, 37.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallwhit", game_speciallwhit);
    agent.acmd("game_specialairlwhit", game_speciallwhit);
}
