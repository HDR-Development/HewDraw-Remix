use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        let kbg = if agent.is_situation(*SITUATION_KIND_GROUND) { 3 } else { 0 };
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 9.0, 0.0, 10.0, 7.0, 0.0, 10.0, -7.0, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 45, 42 + kbg, 0, 85, 5.8, 0.0, 11.0, 4.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 45, 45 + kbg, 0, 80, 5.4, 0.0, 11.0, -4.0, Some(0.0), Some(11.0), Some(-4.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 45, 45 + kbg, 0, 80, 4.6, 0.0, 8.0, 2.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            HIT_NODE(agent, Hash40::new("body"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        }
        else {
            HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        HIT_RESET_ALL(agent);
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        let kbg = if agent.is_situation(*SITUATION_KIND_GROUND) { 5 } else { 0 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 45 + kbg, 0, 85, 4.2, 0.0, 11.0, 4.0, Some(0.0), Some(11.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 45, 35 + kbg, 0, 80, 3.8, 0.0, 11.0, -2.0, Some(0.0), Some(11.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 45, 35 + kbg, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        let kbg = if agent.is_situation(*SITUATION_KIND_GROUND) { 5 } else { 0 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 45 + kbg, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 45, 35 + kbg, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 45, 35 + kbg, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        let kbg = if agent.is_situation(*SITUATION_KIND_GROUND) { 5 } else { 0 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 45, 45 + kbg, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 45, 35 + kbg, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 45, 35 + kbg, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        let kbg = if agent.is_situation(*SITUATION_KIND_GROUND) { 5 } else { 0 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 45, 45 + kbg, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 45, 35 + kbg, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 45, 35 + kbg, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        let kbg = if agent.is_situation(*SITUATION_KIND_GROUND) { 5 } else { 0 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 45, 45 + kbg, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 45, 35 + kbg, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 45, 35 + kbg, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        let kbg = if agent.is_situation(*SITUATION_KIND_GROUND) { 5 } else { 0 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 45, 45 + kbg, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 45, 35 + kbg, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 45, 35 + kbg, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        let kbg = if agent.is_situation(*SITUATION_KIND_GROUND) { 5 } else { 0 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 45, 45 + kbg, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 45, 35 + kbg, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 45, 35 + kbg, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        let kbg = if agent.is_situation(*SITUATION_KIND_GROUND) { 5 } else { 0 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 45, 45 + kbg, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 45, 45 + kbg, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 45, 45 + kbg, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
    }
    frame(lua_state, 85.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_REQUEST_GRAVITY_DEFAULT);
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.8);
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_START);
        VarModule::off_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB);
        VarModule::off_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB);
        VarModule::off_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB);
    }
    
    frame(lua_state, 11.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if ControlModule::get_stick_y(boma) < -0.5 {
            VarModule::on_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB);
            VarModule::on_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB);
        }
        else if ControlModule::get_stick_y(boma) > 0.5 {
            VarModule::on_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB);
            VarModule::on_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        // Spawn the special windboxes if an alternate grab is detected
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // OTG Grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 20, 100, 80, 0, 5.0, 0.0, 5.0, 2.0, Some(0.0), Some(5.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            }
            // Anti-air grab
            else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 350, 100, 80, 0, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
            }
        }
        // Otherwise produce the normal windbox
        else{
            // Original Windbox
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 350, 100, 80, 0, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // Regular grab windbox
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 350, 100, 30, 0, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // Spawn the air grab/OTG grab box if we've detected we hit the windbox
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB) {
                AttackModule::clear_all(boma);
                CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 4.0, 2.0, Some(0.0), Some(4.0), Some(8.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_GA);
            }
            else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB) {
                // Clear windbox and spawn the grab
                AttackModule::clear_all(boma);
                CATCH(agent, 0, Hash40::new("top"), 7.0, 0.0, 15.0, 0.0, Some(0.0), Some(15.0), Some(6.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_A);
            }
        }
        // If the regular grab, then just clear the original windbox and spawn the grabbox
        else{
            // Clear original windbox
            AttackModule::clear_all(boma);
            CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(8.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_GA);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if !VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        }
        GrabModule::set_rebound(boma, false);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        }
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_END);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.8);
    }
    frame(lua_state, 67.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
    frame(lua_state, 74.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1);
    }
}

unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 10, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        // OTG/Anti-air grab effects
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // OTG grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB){
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.25, true);
                LAST_EFFECT_SET_RATE(agent, 0.5);
                
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                LAST_EFFECT_SET_RATE(agent, 0.5);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
                LAST_EFFECT_SET_RATE(agent, 0.5);

                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                LAST_EFFECT_SET_RATE(agent, 0.5);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
                LAST_EFFECT_SET_RATE(agent, 0.5);

                EFFECT_FOLLOW(agent, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.7, true);
                LAST_EFFECT_SET_RATE(agent, 1.1);
            }
            // Anti-air grab
            else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB){
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.25, true);
                LAST_EFFECT_SET_RATE(agent, 0.5);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                LAST_EFFECT_SET_RATE(agent, 0.5);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
                LAST_EFFECT_SET_RATE(agent, 0.5);

                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                LAST_EFFECT_SET_RATE(agent, 0.5);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
                LAST_EFFECT_SET_RATE(agent, 0.5);

                EFFECT_FOLLOW(agent, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.7, true);
                LAST_EFFECT_SET_RATE(agent, 1.1);
            }
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 18.0);
    for _ in 0..8{
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            // OTG/Anti-air grab effects
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB) {
                // OTG grab
                if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB){
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.25, true);
                    
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.35, true);

                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
                }
                // Anti-air grab
                else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB){
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.3, true);

                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.35, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
        
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.35, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
                }
            }
        }
    wait(lua_state, 2.0);
    }
}

unsafe extern "C" fn game_specialairsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_START);
    }
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 350, 100, 30, 0, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 350, 100, 30, 0, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(8.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_G);
        CATCH(agent, 1, Hash40::new("top"), 2.5, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(9.5), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_END);
    }
    frame(lua_state, 44.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 67.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
    frame(lua_state, 74.0);
    FT_MOTION_RATE(agent, 1);
}

unsafe extern "C" fn game_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 37.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_THROW_ROPE);
    }
    frame(lua_state, 60.0);
    FT_MOTION_RATE(agent, 1);
    frame(lua_state, 95.0);
}

unsafe extern "C" fn effect_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_catch"), Hash40::new("sys_catch"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.9, true, *EF_FLIP_YZ);
        // OTG/Anti-air grab effects
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // OTG grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB){
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.25, true);
                
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.35, true);

                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
            }
            // Anti-air grab
            else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB){
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.3, true);

                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.35, true);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
    
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.35, true);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
            }
        }
    }
    frame(lua_state, 6.0);
    for _ in 0..12{
        if is_excute(agent) {
            // OTG/Anti-air grab effects
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB) {
                // OTG grab
                if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB){
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.25, true);
                    
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.35, true);

                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
                }
                // Anti-air grab
                else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB){
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.3, true);

                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.35, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
        
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.35, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
                }
            }
        }
    wait(lua_state, 2.0);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 32.0);
    for _ in 0..8{
        if is_excute(agent) {
            // OTG/Anti-air grab effects
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB) {
                // OTG grab
                if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB){
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.25, true);
                    
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.35, true);

                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.35, true);
                }
                // Anti-air grab
                else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB){
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.3, true);

                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.35, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
        
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.35, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
                }
            }
        }
    wait(lua_state, 2.0);
    }
}

unsafe extern "C" fn game_specialslariat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        // OTG/Anti-air grab throw boxes
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // OTG grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB) {
                ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 15, 0, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
            // Anti-air grab
            else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB) {
                ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 77, 100, 0, 95, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        }
        // Regular grab
        else{
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 145, 474, 0, 20, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        AttackModule::set_force_reaction(boma, 0, true, true);
        WorkModule::set_float(boma, 9.0, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLOAT_LARIAT_HIT_FRAME);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        // OTG/Anti-air grab hitboxes
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // OTG grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB) {
                ATTACK(agent, 0, 0, Hash40::new("arml"), 15.0, 15, 0, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            }
            // Anti-air grab
            else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB) {
                ATTACK(agent, 0, 0, Hash40::new("arml"), 15.0, 105, 35, 0, 88, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            }
        }
        // Regular grab
        else{
            ATTACK(agent, 0, 0, Hash40::new("arml"), 20.0, 145, 40, 0, 88, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        //CHECK_FINISH_CAMERA(fighter, 0, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        //FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
    }
    frame(lua_state, 20.0);
    if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB)
    || VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB) {
        FT_MOTION_RATE_RANGE(agent, 20.0, 50.0, 26.0);
    }
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        REVERSE_LR(agent);
    }
    frame(lua_state, 50.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

unsafe extern "C" fn game_specialairslariat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 145, 474, 0, 20, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_force_reaction(boma, 0, true, true);
        WorkModule::set_float(boma, 9.0, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLOAT_LARIAT_HIT_FRAME);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 20.0, 145, 40, 0, 88, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        //CHECK_FINISH_CAMERA(fighter, 0, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        //FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        REVERSE_LR(agent);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

unsafe extern "C" fn game_specialsshoulder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // OTG grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB){
                ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 179, 0, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
            // Anti-air grab
            else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB){
                ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 105, 280, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        }
        // Regular grab
        else{
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 100, 100, 0, 72, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        AttackModule::set_force_reaction(boma, 0, true, true);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        // OTG/Anti-air grab hitboxes
        if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // OTG grab
            if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_GROUND_GRAB){
                ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 105, 34, 0, 128, 6.0, 0.0, 6.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            }
            // Anti-air grab
            else if VarModule::is_flag(boma.object(), vars::gaogaen::instance::IS_SPECIAL_S_AIR_GRAB){
                ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 105, 34, 0, 128, 6.0, 0.0, 6.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            }
        }
        // Regular grab
        else{
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 105, 34, 0, 128, 6.0, 0.0, 6.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        //CHECK_FINISH_CAMERA_IF_NOT_HP_MODE(fighter, 0, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        //FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        REVERSE_LR(agent);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
}

unsafe extern "C" fn game_specialairsshoulder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 100, 100, 0, 72, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_force_reaction(boma, 0, true, true);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 105, 34, 0, 128, 6.0, 0.0, 6.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        //CHECK_FINISH_CAMERA_IF_NOT_HP_MODE(fighter, 0, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        //FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        REVERSE_LR(agent);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::gaogaen::status::IS_INPUT_CROSS_CHOP_CANCEL);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        //FT_MOTION_RATE(fighter, 6.0/(6.0-4.0));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        //FT_MOTION_RATE(fighter, 1.0);
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 81, 1, 0, 82, 7.0, 0.0, 9.0, 2.0, None, None, None, 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 87, 1, 0, 81, 7.0, 0.0, 9.0, 2.0, Some(0.0), Some(9.0), Some(6.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 80, 1, 0, 80, 4.5, 0.0, 11.5, 2.0, Some(0.0), Some(11.5), Some(3.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 78, 1, 0, 90, 2.5, 0.0, 4.5, 0.0, Some(0.0), Some(4.5), Some(5.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 4, 0, Hash40::new("top"), 4.0, 90, 1, 0, 91, 7.0, 0.0, 9.0, 2.5, Some(0.0), Some(9.0), Some(6.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
        AttackModule::set_force_reaction(boma, 2, true, false);
        AttackModule::set_force_reaction(boma, 3, true, false);
        AttackModule::set_force_reaction(boma, 4, true, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        FT_MOTION_RATE(agent, 0.9);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
        AttackModule::clear(boma, 4, false);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 4.0, 66, 5, 0, 58, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 4.0, 66, 5, 0, 58, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 4.0, 45, 5, 0, 49, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 4.0, 45, 5, 0, 49, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 2.5, 30, 5, 0, 34, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 2.5, 30, 5, 0, 34, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 2.5, 5, 5, 0, 30, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 2.5, 5, 5, 0, 30, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::status::IS_INPUT_CROSS_CHOP_CANCEL){
            FT_MOTION_RATE(agent, 1.5);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::gaogaen::status::IS_INPUT_CROSS_CHOP_CANCEL);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        FT_MOTION_RATE(agent, 4.0/(6.0-4.0));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 81, 1, 0, 82, 7.0, 0.0, 9.0, 2.0, None, None, None, 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 87, 1, 0, 81, 7.0, 0.0, 9.0, 2.0, Some(0.0), Some(9.0), Some(6.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 80, 1, 0, 80, 4.5, 0.0, 11.5, 2.0, Some(0.0), Some(11.5), Some(3.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 78, 1, 0, 90, 2.5, 0.0, 4.5, 0.0, Some(0.0), Some(4.5), Some(5.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 4, 0, Hash40::new("top"), 4.0, 90, 1, 0, 91, 7.0, 0.0, 9.0, 2.5, Some(0.0), Some(9.0), Some(6.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        FT_MOTION_RATE(agent, 0.9);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
        AttackModule::clear(boma, 4, false);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 4.0, 66, 5, 0, 58, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 4.0, 66, 5, 0, 58, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 4.0, 45, 5, 0, 49, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 4.0, 45, 5, 0, 49, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 2.5, 30, 5, 0, 34, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 2.5, 30, 5, 0, 34, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 2.5, 5, 5, 0, 30, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 2.5, 5, 5, 0, 30, 3.0, 1.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::status::IS_INPUT_CROSS_CHOP_CANCEL){
            FT_MOTION_RATE(agent, 1.25);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialairhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("gaogaen_belt_fire2"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::status::SHOULD_CROSS_CHOP_DIVE_EARLY) {
            EFFECT(agent, Hash40::new("sys_hit_fire"), Hash40::new("bust"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.25);

            EFFECT_FOLLOW(agent, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 1.25);

            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(agent, 0.25);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(agent, 0.25);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(agent, 0.25);

            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(agent, 0.25);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(agent, 0.25);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 7, 14.5, -2, -111, -28, 77, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("gaogaen_chop_arc"), Hash40::new("gaogaen_chop_arc"), Hash40::new("trans"), 7, 14.5, 0, -48, 23, 71, 1.2, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("gaogaen_chop_line_start"), Hash40::new("trans"), 0, 22, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn game_specialairhiturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if VarModule::is_flag(boma.object(), vars::gaogaen::status::IS_INPUT_CROSS_CHOP_CANCEL){
        FT_MOTION_RATE(agent, 5.0/(3.0 - 1.0));
    }
    else {
        FT_MOTION_RATE(agent, 1.0);
    }
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 3.0);
    if VarModule::is_flag(boma.object(), vars::gaogaen::status::IS_INPUT_CROSS_CHOP_CANCEL){
        FT_MOTION_RATE(agent, 6.0/(6.0 - 3.0));
    }
    else {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::gaogaen::status::IS_INPUT_CROSS_CHOP_CANCEL){
            VarModule::off_flag(boma.object(), vars::gaogaen::status::IS_INPUT_CROSS_CHOP_CANCEL);
            VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_FALL_TYPE_CHECK);
        }
    }
}

unsafe extern "C" fn effect_specialairhiturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        // if VarModule::is_flag(boma.object(), vars::gaogaen::status::IS_INPUT_CROSS_CHOP_CANCEL){
        //     EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 1.0, true);
        //     LAST_EFFECT_SET_COLOR(fighter, 0.3, 0.3, 1.5);
        //     LAST_EFFECT_SET_RATE(fighter, 1.25);
        // }
        // else if VarModule::is_flag(boma.object(), vars::gaogaen::status::SHOULD_CROSS_CHOP_DIVE_EARLY) {
        //     EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.4, true);
        //     LAST_EFFECT_SET_RATE(fighter, 0.25);
        //     EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
        //     LAST_EFFECT_SET_RATE(fighter, 0.25);
        //     EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
        //     LAST_EFFECT_SET_RATE(fighter, 0.25);

        //     EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.4, true);
        //     LAST_EFFECT_SET_RATE(fighter, 0.25);
        //     EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.4, true);
        //     LAST_EFFECT_SET_RATE(fighter, 0.25);
        // }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::gaogaen::status::IS_INPUT_CROSS_CHOP_CANCEL){
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 8, 5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn game_specialairhifall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 70, 100, 0, 50, 7.0, 0.0, 12.0, 1.0, Some(0.0), Some(7.0), Some(3.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.423);
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_DISABLE_OPPONENT_PASSIVE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 312, 100, 175, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.15, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 54, 48, 0, 135, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(3.0), Some(4.5), 1.15, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 312, 100, 100, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.15, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 312, 100, 80, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.15, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 312, 100, 60, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.15, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 312, 100, 40, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.15, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
}

unsafe extern "C" fn game_specialhibound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.8, 361, 72, 0, 89, 8.0, 0.0, 4.0, 8.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.5, 361, 80, 0, 85, 9.0, 0.0, 5.5, 10.5, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.25);
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_START);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_END);
    }
    frame(lua_state, 28.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 32.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 46.0);
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn);
    agent.acmd("game_specialairn", game_specialn);
    agent.acmd("game_specialsstart", game_specialsstart);
    agent.acmd("effect_specialsstart", effect_specialsstart);
    agent.acmd("game_specialairsstart", game_specialairsstart);
    agent.acmd("game_specialsthrow", game_specialsthrow);
    agent.acmd("game_specialairsthrow", game_specialsthrow);
    agent.acmd("effect_specialsthrow", effect_specialsthrow);
    agent.acmd("game_specialslariat", game_specialslariat);
    agent.acmd("game_specialairslariat", game_specialairslariat);
    agent.acmd("game_specialsshoulder", game_specialsshoulder);
    agent.acmd("game_specialairsshoulder",game_specialairsshoulder,);
    agent.acmd("game_specialhistart", game_specialhistart);
    agent.acmd("game_specialairhistart", game_specialairhistart);
    agent.acmd("effect_specialairhistart",effect_specialairhistart,);
    agent.acmd("game_specialairhiturn", game_specialairhiturn);
    agent.acmd("effect_specialairhiturn",effect_specialairhiturn,);
    agent.acmd("game_specialairhifall", game_specialairhifall);
    agent.acmd("game_specialhibound", game_specialhibound);
    agent.acmd("game_speciallwstart", game_speciallwstart);
    agent.acmd("game_specialairlwstart", game_speciallwstart);
}