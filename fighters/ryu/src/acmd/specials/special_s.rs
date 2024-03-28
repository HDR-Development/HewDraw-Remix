use super::*;

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 1.0, 3.5, 8.5, 8.5);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION) == *SITUATION_KIND_GROUND {
            shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 11.0, 0.0, 11.0, 7.0, 0.0, 11.0, -7.0, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 5.5, 3.0, 9.0, 3.0);
        if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION) == *SITUATION_KIND_GROUND {
            shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 11.0, 0.0, 11.0, 7.0, 0.0, 11.0, -7.0, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_INVINCIBLE);
        }
    }
    frame(lua_state, 1.0);
    if VarModule::is_flag(boma.object(), vars::shotos::instance::IS_USE_EX_SPECIAL) {
        FT_MOTION_RATE(agent, 0.5);
        agent.set_int(*FIGHTER_RYU_STRENGTH_S, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    } else {
        FT_MOTION_RATE(agent, 1.0);
    }
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT) > 2 {
            agent.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        }
        if VarModule::is_flag(boma.object(), vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 366, 100, 70, 0, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(12.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 2.5, 366, 100, 70, 0, 3.5, 0.0, 8.5, -5.5, Some(0.0), Some(8.5), Some(6.25), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        } else {
            MeterModule::watch_damage(agent.battle_object, true);
            if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 67, 40, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 67, 40, 0, 80, 3.5, 0.0, 8.5, 6.25, Some(0.0), Some(8.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            } else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 70, 71, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 70, 71, 0, 80, 3.5, 0.0, 8.5, 6.25, Some(0.0), Some(8.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            }
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 366, 100, 70, 0, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(12.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 2.5, 366, 100, 70, 0, 3.5, 0.0, 8.5, -5.5, Some(0.0), Some(8.5), Some(6.25), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        } else {
            MeterModule::watch_damage(agent.battle_object, true);
            if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 67, 40, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 67, 40, 0, 80, 3.5, 0.0, 8.5, -5.5, Some(0.0), Some(8.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            } else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 50, 56, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
                ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 50, 56, 0, 80, 3.5, 0.0, 8.5, -5.5, Some(0.0), Some(8.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            }
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
    }
}

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION) == *SITUATION_KIND_GROUND {
            shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
        if VarModule::is_flag(boma.object(), vars::shotos::instance::IS_USE_EX_SPECIAL) {
            AttackModule::clear_all(boma);
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.5, 75, 70, 0, 85, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(12.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 13.5, 75, 70, 0, 85, 3.5, 0.0, 8.5, -5.5, Some(0.0), Some(8.5), Some(6.25), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(lua_state, 6.0);
    if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        FT_MOTION_RATE_RANGE(agent, 6.0, 31.0, 12.0);
    }
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(agent, 200);
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("ryu_savingattack_line_r"), Hash40::new("ryu_savingattack_line_l"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.1, false, *EF_FLIP_NONE);

            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if  agent.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND){
            if agent.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND){
                FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_ALPHA(agent, 0.5);
            }
        }
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("ryu_tatsumaki_smoke_r"), Hash40::new("ryu_tatsumaki_smoke_l"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.1, false, *EF_FLIP_NONE);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(agent, 200);
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(agent);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(agent, 200);
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
}

unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT(agent, Hash40::new("ryu_savingattack_impact"), Hash40::new("top"), 14, 10.5, 3, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND){
            LANDING_EFFECT_FLIP(agent, Hash40::new("ryu_tatsumaki_smoke_r"), Hash40::new("ryu_tatsumaki_smoke_l"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        }
    }
}

unsafe extern "C" fn effect_specialairsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT(agent, Hash40::new("ryu_savingattack_impact"), Hash40::new("top"), 14, 10.5, 3, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn effect_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(agent, 200);
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("ryu_savingattack_line_r"), Hash40::new("ryu_savingattack_line_l"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.1, false, *EF_FLIP_NONE);

            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(agent, 200);
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(agent);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(agent, 200);
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialsstart", game_specialsstart);
    agent.acmd("game_specialairsstart", game_specialsstart);
    agent.acmd("game_specials", game_specials);
    agent.acmd("game_specialairs", game_specials);
    agent.acmd("game_specialsend", game_specialsend);
    agent.acmd("game_specialairsend", game_specialsend);
    agent.acmd("effect_specials", effect_specials);
    agent.acmd("effect_specialsend", effect_specialsend);
    agent.acmd("effect_specialairsend", effect_specialairsend);
    agent.acmd("effect_specialairs", effect_specialairs);
}
