use super::*;

unsafe extern "C" fn game_specialairnend(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.5, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 7.5, 17.0, Some(0.0), Some(7.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 7.5, 23.5, Some(0.0), Some(7.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialairnendhi(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.5, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 8.5, 8.0, Some(0.0), Some(12.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 14.0, 25.0, Some(0.0), Some(14.6), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 9.8, 17.0, Some(0.0), Some(12.3), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 13.0, 23.5, Some(0.0), Some(14.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 6.5, 8.0, Some(0.0), Some(10.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 11.5, 25.0, Some(0.0), Some(12.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 9.0, 17.0, Some(0.0), Some(10.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 11.0, 23.5, Some(0.0), Some(12.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialairnendlw(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.5, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 6.5, 8.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 3.0, 25.0, Some(0.0), Some(2.7), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 5.6, 17.0, Some(0.0), Some(3.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 3.0, 23.5, Some(0.0), Some(1.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 4.5, 8.0, Some(0.0), Some(1.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, -0.5, 25.0, Some(0.0), Some(-1.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 2.0, 17.0, Some(0.0), Some(0.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 0.0, 23.5, Some(0.0), Some(-1.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialairnendmax(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 2.0, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 7.5, 17.0, Some(0.0), Some(7.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 7.5, 23.5, Some(0.0), Some(7.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialairnendmaxhi(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 2.0, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(12.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 14.0, 25.0, Some(0.0), Some(14.6), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 9.8, 17.0, Some(0.0), Some(12.3), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 13.0, 23.5, Some(0.0), Some(14.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 6.5, 8.0, Some(0.0), Some(10.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 11.5, 25.0, Some(0.0), Some(12.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 9.0, 17.0, Some(0.0), Some(10.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 11.0, 23.5, Some(0.0), Some(12.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialairnendmaxlw(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 2.0, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 6.5, 8.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 3.0, 25.0, Some(0.0), Some(2.7), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 5.6, 17.0, Some(0.0), Some(3.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 3.0, 23.5, Some(0.0), Some(1.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 4.5, 8.0, Some(0.0), Some(1.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, -0.5, 25.0, Some(0.0), Some(-1.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 2.0, 17.0, Some(0.0), Some(0.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 0.0, 23.5, Some(0.0), Some(-1.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialnend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 7.5, 17.0, Some(0.0), Some(7.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 7.5, 23.5, Some(0.0), Some(7.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialnendhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 8.5, 8.0, Some(0.0), Some(12.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 14.0, 25.0, Some(0.0), Some(14.6), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 9.8, 17.0, Some(0.0), Some(12.3), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 13.0, 23.5, Some(0.0), Some(14.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 6.5, 8.0, Some(0.0), Some(10.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 11.5, 25.0, Some(0.0), Some(12.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 9.0, 17.0, Some(0.0), Some(10.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 11.0, 23.5, Some(0.0), Some(12.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialnendlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 6.5, 8.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 3.0, 25.0, Some(0.0), Some(2.7), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 5.6, 17.0, Some(0.0), Some(3.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 3.0, 23.5, Some(0.0), Some(1.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 4.5, 8.0, Some(0.0), Some(1.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, -0.5, 25.0, Some(0.0), Some(-1.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 2.0, 17.0, Some(0.0), Some(0.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 0.0, 23.5, Some(0.0), Some(-1.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialnendmax(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 7.5, 17.0, Some(0.0), Some(7.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 7.5, 23.5, Some(0.0), Some(7.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialnendmaxhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(12.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 14.0, 25.0, Some(0.0), Some(14.6), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 9.8, 17.0, Some(0.0), Some(12.3), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 13.0, 23.5, Some(0.0), Some(14.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 6.5, 8.0, Some(0.0), Some(10.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 11.5, 25.0, Some(0.0), Some(12.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 9.0, 17.0, Some(0.0), Some(10.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 11.0, 23.5, Some(0.0), Some(12.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialnendmaxlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 6.5, 8.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 3.0, 25.0, Some(0.0), Some(2.7), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 5.6, 17.0, Some(0.0), Some(3.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 3.0, 23.5, Some(0.0), Some(1.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 4.5, 8.0, Some(0.0), Some(1.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, -0.5, 25.0, Some(0.0), Some(-1.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 2.0, 17.0, Some(0.0), Some(0.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 0.0, 23.5, Some(0.0), Some(-1.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 70, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialairnend", game_specialairnend, Priority::Low);
    agent.acmd("game_specialairnendhi", game_specialairnendhi, Priority::Low);
    agent.acmd("game_specialairnendlw", game_specialairnendlw, Priority::Low);
    
    agent.acmd("game_specialairnendmax", game_specialairnendmax, Priority::Low);
    agent.acmd("game_specialairnendmaxhi", game_specialairnendmaxhi, Priority::Low);
    agent.acmd("game_specialairnendmaxlw", game_specialairnendmaxlw, Priority::Low);
    
    agent.acmd("game_specialnend", game_specialnend, Priority::Low);
    agent.acmd("game_specialnendhi", game_specialnendhi, Priority::Low);
    agent.acmd("game_specialnendlw", game_specialnendlw, Priority::Low);
    
    agent.acmd("game_specialnendmax", game_specialnendmax, Priority::Low);
    agent.acmd("game_specialnendmaxhi", game_specialnendmaxhi, Priority::Low);
    agent.acmd("game_specialnendmaxlw", game_specialnendmaxlw, Priority::Low);
}
