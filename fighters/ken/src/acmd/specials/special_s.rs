use super::*;

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut attr = Hash40::new("collision_attr_normal");
    let mut dmg = 1.0;
    if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
        attr = Hash40::new("collision_attr_fire");
        dmg = 1.10;
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 1.0, 3.5, 8.5, 8.5);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 3.0, 3.5, 8.5, 4.5);
        if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION) == *SITUATION_KIND_GROUND {
            shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 11.0, 0.0, 11.0, 7.0, 0.0, 11.0, -7.0, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
        MeterModule::watch_damage(agent.battle_object, true);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(agent.battle_object, false);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0 * dmg, 0, 50, 100, 0, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut attr = Hash40::new("collision_attr_normal");
    let mut dmg = 1.0;
    if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
        attr = Hash40::new("collision_attr_fire");
        dmg = 1.10;
    }
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 5.5, 3.0, 9.0, 3.0);
        if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION) == *SITUATION_KIND_GROUND {
            shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 11.0, 0.0, 11.0, 7.0, 0.0, 11.0, -7.0, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
    }
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.6);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT) > 2 {
            boma.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        }
        MeterModule::watch_damage(agent.battle_object, true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MeterModule::watch_damage(agent.battle_object, true);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(agent.battle_object, false);
        }
        if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0 * dmg, 45, 160, 0, 26, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.0 * dmg, 45, 160, 0, 26, 3.5, 0.0, 8.5, 6.25, Some(0.0), Some(8.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        } else if agent.is_situation(*SITUATION_KIND_GROUND) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0 * dmg, 363, 100, 45, 0, 3.5, 0.0, 12.5, 6.25, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 2.0 * dmg, 363, 100, 45, 0, 3.5, 0.0, 8.5, 6.25, Some(0.0), Some(8.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 2.0 * dmg, 365, 100, 45, 0, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0 * dmg, 55, 100, 45, 0, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 2.0 * dmg, 55, 100, 45, 0, 3.5, 0.0, 8.5, 6.25, Some(0.0), Some(8.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 2.0 * dmg, 31, 100, 59, 0, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(agent.battle_object, false);
        }
        if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
            ATTACK(agent, 0, 0, Hash40::new("top"), 4.0 * dmg, 45, 120, 0, 30, 3.5, 0.0, 12.5, -12.5, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 4.0 * dmg, 45, 120, 0, 30, 3.5, 0.0, 8.5, -6.25, Some(0.0), Some(8.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        } else if agent.is_situation(*SITUATION_KIND_GROUND) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0 * dmg, 363, 100, 59, 0, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 2.0 * dmg, 363, 100, 59, 0, 3.5, 0.0, 8.5, -5.5, Some(0.0), Some(8.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0 * dmg, 55, 100, 59, 0, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 2.0 * dmg, 55, 100, 59, 0, 3.5, 0.0, 8.5, -5.5, Some(0.0), Some(8.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
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
    let mut attr = Hash40::new("collision_attr_normal");
    let mut dmg = 1.0;
    if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
        attr = Hash40::new("collision_attr_fire");
        dmg = 1.10;
    }
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION) == *SITUATION_KIND_GROUND {
            shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
        MeterModule::watch_damage(agent.battle_object, false);
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, true);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(agent.battle_object, false);
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.5 * dmg, 75, 69, 0, 72, 4.0, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(-3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 12.5 * dmg, 75, 69, 0, 72, 4.0, 0.0, 8.5, 6.25, Some(0.0), Some(8.5), Some(-3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        } else if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
            // no finisher
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 4.0 * dmg, 50, 150, 0, 50, 4.0, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(-3.0), 1.25, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 4.0 * dmg, 50, 150, 0, 50, 4.0, 0.0, 8.5, 6.25, Some(0.0), Some(8.5), Some(-3.0), 1.25, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        boma.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
    if boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
        if (boma.is_situation(*SITUATION_KIND_GROUND)) {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 10.5, 6, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true, 0.7);
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                EFFECT_FOLLOW(agent, Hash40::new("ken_savingattack_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.0, true);
                EFFECT_FOLLOW(agent, Hash40::new("ken_savingattack_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.0, true);
                EFFECT_FOLLOW(agent, Hash40::new("ken_savingattack_aura"), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1.0, true);
            }
        }
    }
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 3.0);
    if boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND)
    && boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_ALPHA(agent, 0.5);
        }
    }
    if is_excute(agent) {
        // FOOT_EFFECT_FLIP(fighter, Hash40::new("ken_tatsumaki_smoke_r"), Hash40::new("ken_tatsumaki_smoke_l"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        // LAST_EFFECT_SET_RATE(fighter, 1.1);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW(agent, Hash40::new("ken_savingattack_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(agent, Hash40::new("ken_savingattack_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(agent, Hash40::new("ken_savingattack_aura"), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1.0, true);
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
    frame(lua_state, 10.0);
    if boma.is_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND) {
        if is_excute(agent) {
            LANDING_EFFECT_FLIP(agent, Hash40::new("ken_tatsumaki_smoke_r"), Hash40::new("ken_tatsumaki_smoke_l"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_RATE(agent, 1.1);
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
    agent.acmd("effect_specialsstart", effect_specialsstart);
    agent.acmd("effect_specialairsstart", effect_specialsstart);
    agent.acmd("effect_specials", effect_specials);
    agent.acmd("effect_specialairs", effect_specials);
    agent.acmd("effect_specialsend", effect_specialsend);
    agent.acmd("effect_specialairsend", effect_specialsend);
}
