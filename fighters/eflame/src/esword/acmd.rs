use super::*;

unsafe extern "C" fn game_flyflickl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::disable_tip(boma);
    }
    frame(lua_state, 0.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.3);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 65, 2.5, 0.0, -1.5, -3.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 55, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 55, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 55, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 55, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 55, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 55, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 50, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 50, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 40, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 35, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 45, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 35, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 45, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 40, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 45, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 40, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 65, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 40, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 65, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        AttackModule::clear(boma, 0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        AttackModule::clear(boma, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
        ATTACK(agent, 6, 1, Hash40::new("top"), 1.5, 90, 100, 10, 20, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 6, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 6, true, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 10, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 5, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 5, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 2, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 3, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 4, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 5, 5.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
}

unsafe extern "C" fn game_flyflickr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::disable_tip(boma);
    }
    frame(lua_state, 0.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.3);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 65, 2.5, 0.0, -1.5, -3.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 55, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 55, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 55, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 55, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 55, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 40, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 55, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 60, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 50, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 50, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 40, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 35, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 45, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 35, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 45, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 40, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 45, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 40, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 7, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 65, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 40, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 65, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        AttackModule::clear(boma, 0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        AttackModule::clear(boma, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
        ATTACK(agent, 6, 1, Hash40::new("top"), 1.5, 90, 100, 10, 20, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 6, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 6, true, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 10, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 5, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 5, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 2, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 3, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 4, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 5, 5.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
}

unsafe extern "C" fn game_flyl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::disable_tip(boma);
    }
    frame(lua_state, 0.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.45);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 65, 2.5, 0.0, -1.5, -3.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 35, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 35, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 35, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 35, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 60, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 45, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 45, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 45, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 45, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 45, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 45, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 45, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 20, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 20, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 25, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 75, 100, 10, 30, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 75, 100, 10, 30, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 75, 100, 10, 30, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        AttackModule::clear(boma, 0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        AttackModule::clear(boma, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
        ATTACK(agent, 6, 1, Hash40::new("top"), 1.5, 90, 100, 10, 20, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 6, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 6, true, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 10, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 5, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 5, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 2, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 3, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 4, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 5, 5.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
}

unsafe extern "C" fn game_flyr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::disable_tip(boma);
    }
    frame(lua_state, 0.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.45);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 65, 2.5, 0.0, -1.5, -3.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 35, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 35, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 35, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 35, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 60, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 45, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 45, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 45, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 45, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 45, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 45, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 45, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 20, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 20, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 25, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 70, 100, 10, 40, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 75, 100, 10, 30, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 75, 100, 10, 30, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 4, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 30, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 75, 100, 10, 30, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        AttackModule::clear(boma, 0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        AttackModule::clear(boma, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
        ATTACK(agent, 6, 1, Hash40::new("top"), 1.5, 90, 100, 10, 20, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 6, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 6, true, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 10, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 5, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 5, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 2, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 3, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 4, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 5, 5.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
}

unsafe extern "C" fn game_reflectedl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::disable_tip(boma);
    }
    frame(lua_state, 0.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.5);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 75, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 65, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 65, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 65, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 65, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 65, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 65, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 60, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 65, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 60, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        AttackModule::clear(boma, 0, false);
    }
    frame(lua_state, 22.0);
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 55, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 50, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 50, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 2, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 3, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 4, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 5, 5.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
}

unsafe extern "C" fn game_reflectedr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::disable_tip(boma);
    }
    frame(lua_state, 0.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.5);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 75, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 65, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 65, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 45, 100, 10, 65, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 65, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 50, 100, 10, 65, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 70, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 55, 100, 10, 60, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 65, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 60, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 100, 10, 65, 2.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 1.5, 60, 100, 10, 50, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 40.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 20.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        AttackModule::clear(boma, 0, false);
    }
    frame(lua_state, 22.0);
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 55, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 50, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 50, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("sword1"), 0.8, 50, 100, 10, 30, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, -1, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 1, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 2, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 3, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 4, 5.0, false);
        // AttackModule::set_add_reaction_frame(boma, 5, 5.0, false);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
}

unsafe extern "C" fn game_rotate(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::disable_tip(boma);
    }
    frame(lua_state, 0.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.2);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 0.8, 0, 60, 20, 10, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 0.8, 90, 70, 20, 10, 2.3, 2.5, -2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 5, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("sword1"), 0.8, 120, 70, 20, 5, 1.8, 6.0, -2.0, 0.0, Some(6.0), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("sword1"), 0.8, 190, 60, 20, 10, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 6, 0, Hash40::new("sword1"), 0.8, 120, 70, 30, 10, 1.8, 9.5, -2.0, 0.0, Some(9.5), Some(2.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 1, Hash40::new("top"), 1.5, 90, 100, 10, 20, 4.5, 0.0, -1.5, -2.0, Some(0.0), Some(-1.5), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -3, -1.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(boma, 1, 25.0, false);
        // AttackModule::set_add_reaction_frame(boma, 2, 25.0, false);
        // AttackModule::set_add_reaction_frame(boma, 3, 25.0, false);
        // AttackModule::set_add_reaction_frame(boma, 4, 25.0, false);
        // AttackModule::set_add_reaction_frame(boma, 5, 25.0, false);
        // AttackModule::set_add_reaction_frame(boma, 6, 25.0, false);
        // AttackModule::set_add_reaction_frame(boma, 0, 25.0, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 367, 100, 100, 50, 2.5, 0.0, 2.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -3, -1.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 10.0, 65, 100, 0, 35, 3.5, 5.0, 0.0, 0.0, None, None, None, 1.15, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 10.0, 65, 100, 0, 35, 3.5, 9.0, 0.0, 0.0, None, None, None, 1.15, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 10.0, 65, 100, 0, 35, 3.5, 11.0, 0.0, 0.0, None, None, None, 1.15, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *WEAPON_EFLAME_ESWORD_STATUS_SPECIAL_S_FLAG_FINISH);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 0.5);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_flyflickl", game_flyflickl);
    agent.acmd("game_flyflickr", game_flyflickr);
    
    agent.acmd("game_flyl", game_flyl);
    agent.acmd("game_flyr", game_flyr);

    agent.acmd("game_reflectedl", game_reflectedl);
    agent.acmd("game_reflectedr", game_reflectedr);

    agent.acmd("game_rotate", game_rotate);
}
