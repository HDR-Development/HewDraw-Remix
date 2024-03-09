
use super::*;

unsafe extern "C" fn game_specialairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        boma.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
        MeterModule::watch_damage(fighter.battle_object, true);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(fighter.battle_object, false);
            MeterModule::drain_direct(fighter.battle_object, 2.0 * MeterModule::meter_per_level(fighter.battle_object));
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 90, 100, 90, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.1, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        } else if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 90, 100, 90, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.1, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(fighter.battle_object, false);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 80, 100, 100, 0, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.22, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        } else if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W
        || boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 80, 54, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 80, 100, 100, 0, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.22, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(fighter.battle_object, false);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.5, 80, 107, 0, 76, 6.0, 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        } else if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W
        || boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.5, 80, 112, 0, 78, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.5, 80, 107, 0, 76, 6.0, 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        boma.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn game_specialairhiend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi_command") {
            WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        boma.on_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
        MeterModule::watch_damage(fighter.battle_object, true);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(fighter.battle_object, false);
            MeterModule::drain_direct(fighter.battle_object, 2.0 * MeterModule::meter_per_level(fighter.battle_object));
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 105, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.1, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        } else if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 105, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.1, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(fighter.battle_object, false);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 95, 10, 0, 95, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(8.1), 1.22, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        } else if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W
        || boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 95, 10, 0, 95, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(8.1), 1.22, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) 
    && !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL)
    && MotionModule::motion_kind(fighter.module_accessor) != hash40("special_hi_command") {
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(fighter.battle_object, false);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 80, 126, 0, 80, 5.5, 4.0, -0.4, 0.0, Some(-4.0), Some(-0.4), Some(0.0), 1.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("legr"), 6.5, 80, 126, 0, 80, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        } else {
            WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
            if boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W
            || boma.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 80, 121, 0, 78, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
            } else {
                ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 80, 126, 0, 80, 5.5, 4.0, -0.4, 0.0, Some(-4.0), Some(-0.4), Some(0.0), 1.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
                ATTACK(fighter, 1, 0, Hash40::new("legr"), 6.5, 80, 126, 0, 80, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            }
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MeterModule::watch_damage(fighter.battle_object, false);
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        boma.off_flag(*FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn effect_specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W
        || fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_arc"), Hash40::new("trans"), 6.5, 5, 0, 5, 0, 25, 1, false);
        } else {
            let id = EffectModule::req_follow(fighter.module_accessor, Hash40::new("ken_syoryuken_fire"), Hash40::new("handr"), &Vector3f::new(0.0, 0.0, 0.0), &Vector3f::new(0.0, 0.0, 0.0), 1.0, false, 0, 0, 0, 0, 0, false, false);
            VarModule::set_int(fighter.battle_object, vars::shotos::instance::SPECIAL_HI_FIRE_EFF_ID, id as i32);
            EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
            if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
                EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
                EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
            } else {
                EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
                EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            }
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("ken_syoryuken_firearc"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("ken_savingattack_aura"), -1);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ryu_syoryuken_arc"), true, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        let id = VarModule::get_int(fighter.battle_object, vars::shotos::instance::SPECIAL_HI_FIRE_EFF_ID) as u32;
        EffectModule::kill(fighter.module_accessor, id, true, true);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.7);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.7);
        }
    }
}

unsafe extern "C" fn game_specialhifall(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    smashline::Agent::new("ken")
        .acmd("game_specialairhi", game_specialairhi)
        .acmd("game_specialairhicommand", game_specialairhi)
        .acmd("game_specialairhiend", game_specialairhiend)
        .acmd("game_specialhi", game_specialhi)
        .acmd("game_specialhicommand", game_specialhi)
        .acmd("game_specialhifall", game_specialhifall)
        .acmd("effect_specialhi", effect_specialhi)
        .acmd("effect_specialhicommand", effect_specialhi)
        .acmd("effect_specialairhi", effect_specialhi)
        .acmd("effect_specialairhicommand", effect_specialhi)
        .install();
}
