use super::*;

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let attr = if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
        Hash40::new("collision_attr_elec")
    } else {
        Hash40::new("collision_attr_normal")
    };
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
        MeterModule::watch_damage(agent.battle_object, true);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(agent.battle_object, false);
            MeterModule::drain_direct(agent.battle_object, 2.0 * MeterModule::meter_per_level(agent.battle_object));
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 105, 100, 125, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.0, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        } else if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W
        || agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 80, 49, 0, 80, 5.0, 0.0, 10.0, 7.6, None, None, None, 1.25, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 80, 60, 0, 80, 5.0, 0.0, 10.0, 7.6, None, None, None, 1.25, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(agent.battle_object, false);
        } else if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W
        || agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
            ATTACK(agent, 0, 0, Hash40::new("handr"), 10.0, 80, 57, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("handr"), 10.0, 80, 57, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(agent.battle_object, false);
            ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 80, 107, 0, 84, 5.5, 4.0, -0.4, 0.0, Some(-4.0), Some(-0.4), Some(0.0), 1.5, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("legr"), 6.0, 80, 107, 0, 84, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        } else if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W
        || agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
            ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.25, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 70, 0, 60, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.25, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn game_specialairhiend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let attr = if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
        Hash40::new("collision_attr_elec")
    } else {
        Hash40::new("collision_attr_normal")
    };
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        if MotionModule::motion_kind(agent.module_accessor) == hash40("special_hi_command") {
            WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
        MeterModule::watch_damage(agent.battle_object, true);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(agent.battle_object, false);
            MeterModule::drain_direct(agent.battle_object, 2.0 * MeterModule::meter_per_level(agent.battle_object));
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 105, 100, 125, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.0, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        } else if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W
        || agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.25, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 80, 69, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.25, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(agent.battle_object, false);
        } else {   
            if MotionModule::motion_kind(agent.module_accessor) != hash40("special_hi_command") {
                WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
            }
            if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W
            || agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
                ATTACK(agent, 0, 0, Hash40::new("handr"), 12.0, 80, 64, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            } else {
                ATTACK(agent, 0, 0, Hash40::new("handr"), 12.0, 80, 64, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        agent.on_flag( *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(agent.battle_object, false);
            ATTACK(agent, 0, 0, Hash40::new("armr"), 6.5, 80, 126, 0, 82, 5.5, 4.0, -0.4, 0.0, Some(-4.0), Some(-0.4), Some(0.0), 1.5, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("legr"), 6.5, 80, 126, 0, 82, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        } else {
            WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
            if agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W
            || agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
                ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.25, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            } else {
                ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.25, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn game_specialhifall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("ryu_syoryuken_arc"), Hash40::new("trans"), 6.5, 5, 0, 5, 0, 25, 1, false);
        } else {
            EFFECT_FOLLOW(agent, Hash40::new("ryu_syoryuken_arc"), Hash40::new("trans"), -6.5, 5, 0, 5, 0, -25, 1, false);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("ryu_syoryuken_arc"), -1);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("ryu_savingattack_line_r"), Hash40::new("ryu_savingattack_line_l"), Hash40::new("top"), 10, 8, 3, -100, 160, 0, 1.4, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ryu_syoryuken_arc"), true, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialairhi", game_specialairhi);
    agent.acmd("game_specialairhicommand", game_specialairhi);
    agent.acmd("game_specialairhiend", game_specialairhiend);
    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("game_specialhicommand", game_specialhi);
    agent.acmd("game_specialhifall", game_specialhifall);
    agent.acmd("effect_specialhi", effect_specialhi);
    agent.acmd("effect_specialairhi", effect_specialhi);
    agent.acmd("effect_specialhicommand", effect_specialhi);
    agent.acmd("effect_specialairhicommand", effect_specialhi);
}
