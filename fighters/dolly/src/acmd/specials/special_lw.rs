use super::*;

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 0.5);
    }
    frame(lua_state, 5.5);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 368, 100, 91, 0, 5.0, 0.0, 10.0, 7.5, Some(0.0), Some(6.0), Some(7.5), 1.75, 0.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 12.5, y: 19.9}, 6, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, false);
    }
}

unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if agent.is_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 8, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.3);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("dolly_down_start"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.5);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairlwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if agent.is_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 8, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.3);
        }
    }
    frame(lua_state, 5.5);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_rush"), 8, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    if agent.is_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(lua_state, 5.5);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialairlwrise(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        agent.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::clear_speed_all(boma);
        agent.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        if agent.is_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 85, 100, 40, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 0.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }

    frame(lua_state, 5.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 65, 100, 50, 20, 7.0, 0.0, 9.0, 3.0, None, None, None, 1.0, 0.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 40, 100, 30, 10, 7.0, 0.0, 9.0, 3.0, None, None, None, 1.0, 0.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }

    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 40, 100, 30, 10, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 0.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }

    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        agent.off_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        MotionModule::set_rate(boma, 1.1875);
    }

    frame(lua_state, 28.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::add_speed(boma, &Vector3f{x: 0.3, y: -1.5, z: 0.0});
        agent.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
}

unsafe extern "C" fn game_specialairlwrisew(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        agent.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::clear_speed_all(boma);
        agent.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        if agent.is_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 85, 100, 25, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 0.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }

    frame(lua_state, 5.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 65, 100, 25, 20, 7.0, 0.0, 9.0, 3.0, None, None, None, 1.0, 0.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 40, 100, 20, 10, 7.0, 0.0, 9.0, 3.0, None, None, None, 1.0, 0.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }

    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 40, 100, 20, 10, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 0.25, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }

    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        agent.off_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }

    frame(lua_state, 22.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::add_speed(boma, &Vector3f{x: 5.0, y: -1.5, z: 0.0});
        agent.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 0.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        agent.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.3, y: -1.0, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 1.3, y: -1.5, z: 0.0});
        }
        agent.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }

    frame(lua_state, 1.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -0.3, z: 0.0});
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.7, 50, 116, 0, 50, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -1.0, z: 0.0});
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.4, 50, 104, 0, 50, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }

    frame(lua_state, 2.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -0.5, z: 0.0});
        }
    }
    
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            MotionModule::set_rate(boma, 1.2);
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.7, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        } else {
            MotionModule::set_rate(boma, 1.0);
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.7, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }

    frame(lua_state, 6.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 8.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 10.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 12.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 13.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 15.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 10.0);
    }

    frame(lua_state, 35.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_DOLLY_SPECIAL_LW_FALL);
        MotionModule::set_rate(boma, 1.0);
    }

    frame(lua_state, 37.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_LANDING_HEAVY);
    }
}

unsafe extern "C" fn game_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        SET_SPEED_EX(agent, 1, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::dolly::status::INHERIT_FINAL_CANCEL_ON_END);
    }
}

unsafe extern "C" fn game_speciallwbreaking(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_XLU);
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 66, 151, 0, 54, 5.5, 0.0, 10.0, 7.5, Some(0.0), Some(6.0), Some(7.5), 0.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 25.0, 19.0);
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_speciallwbreaking(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 0.3, 0.8);
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 12, -4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        if agent.lr() < 0.0 {
            // I will find and defenestrate whomsoever made this effect unable to be easily mirrored
            EFFECT_FOLLOW(agent, Hash40::new("dolly_drive_start0"), Hash40::new("top"), 0, 14, 8, 100, 0, -240, 0.95, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("dolly_drive_start0"), Hash40::new("top"), 0, 6, 8, 90, 0, -10, 0.95, true);
        }
        LAST_EFFECT_SET_COLOR(agent, 0.5, 0.05, 2.5);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            FLASH(agent, 0.1, 0.1, 1, 0.5);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH_FRM(agent, 2, 0.1, 0.1, 0.9, 0);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn sound_speciallwbreaking(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_dolly_ottotto"));
        let handle = SoundModule::play_se_no3d(boma, Hash40::new("se_dolly_command_success"), true, true);
        SoundModule::set_se_vol(boma, handle as i32, 1.25, 0);
        let rand = sv_math::rand(hash40("fighter"), 2) as i32;
        match rand {
            0 => PLAY_SE(agent, Hash40::new("vc_dolly_escapef")),
            _ => PLAY_SE(agent, Hash40::new("vc_dolly_escapeb")),
        }
    }
}

unsafe extern "C" fn expression_speciallwbreaking(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
        ControlModule::set_rumble(
            boma,
            Hash40::new("rbkind_rush"),
            8,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallwstart", game_speciallwstart, Priority::Low);
    agent.acmd("effect_speciallwstart", effect_speciallwstart, Priority::Low);
    agent.acmd("expression_speciallwstart", expression_speciallwstart, Priority::Low);

    agent.acmd("game_specialairlwstart", game_speciallwstart, Priority::Low);
    agent.acmd("effect_specialairlwstart", effect_specialairlwstart, Priority::Low);
    agent.acmd("expression_specialairlwstart", expression_speciallwstart, Priority::Low);

    agent.acmd("game_specialairlwrise", game_specialairlwrise, Priority::Low);
    agent.acmd("game_specialairlwrisew", game_specialairlwrisew, Priority::Low);

    agent.acmd("game_specialairlw", game_specialairlw, Priority::Low);

    agent.acmd("game_speciallwend", game_speciallwend, Priority::Low);

    agent.acmd("game_speciallwbreaking", game_speciallwbreaking, Priority::Low);
    agent.acmd("effect_speciallwbreaking", effect_speciallwbreaking, Priority::Low);
    agent.acmd("sound_speciallwbreaking", sound_speciallwbreaking, Priority::Low);
    agent.acmd("expression_speciallwbreaking", expression_speciallwbreaking, Priority::Low);
}
