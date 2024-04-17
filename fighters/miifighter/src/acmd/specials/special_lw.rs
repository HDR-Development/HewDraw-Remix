use super::*;

// ================================================================================================
// ======================================= EARTHQUAKE FIST ========================================
// ================================================================================================

unsafe extern "C" fn game_speciallw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    let charge_distance = VarModule::get_float(agent.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE_DISTANCE);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 90, 28, 0, 75, 5.0, 0.0, 4.5, 8.0 + charge_distance, Some(0.0), Some(4.5), Some(16.0 + charge_distance), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 90, 28, 0, 75, 5.0, 0.0, 4.5, 12.0 + charge_distance, Some(0.0), Some(13.0), Some(12.0 + charge_distance), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 45, 60, 0, 60, 3.0, 0.0, 3.0, 7.0, Some(0.0), Some(3.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12.0, 5.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let eff_handle = EffectModule::req_follow(boma, Hash40::new("sys_windwave"), Hash40::new("top"), &Vector3f::new(0.0, 0.0, 10.0), &Vector3f::zero(), 0.4, false, 0, 0, 0, 0, 0, false, false);
        EffectModule::set_rate(boma, eff_handle as u32, 0.4);
        VarModule::set_int64(agent.battle_object, vars::miifighter::instance::QUAKE_EFFECT_HANDLER, eff_handle as u64);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_sidekick_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.6, true);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_sidekick_hold"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 0.4);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 8.0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miifighter_sidekick_hold"), true, true);
    }
    frame(lua_state, 13.0);
    let charge_distance = VarModule::get_float(agent.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE_DISTANCE);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miifighter_sidekick_flash"), true, true);
        LANDING_EFFECT(agent, Hash40::new("miifighter_headbut_v_smoke"), Hash40::new("top"), 8.0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("miifighter_headbut_v_smoke"), Hash40::new("top"), 12.0 + charge_distance, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_SCALE_W(agent, 0.6, 1.1, 0.6);
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 12.0 + charge_distance, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 12.0 + charge_distance, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 12.0 + charge_distance, 3.0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_speciallw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_attack03"));
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_smash_s03"));
        PLAY_SE(agent, Hash40::new("se_miifighter_special_l03"));
        PLAY_SE(agent, Hash40::new("se_miifighter_special_s03"));
        PLAY_SE_REMAIN(agent, Hash40::new("se_miifighter_special_c2_s02"));
    }
}

unsafe extern "C" fn expression_speciallw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    };
    frame(lua_state, 13.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        AREA_WIND_2ND_arg10(agent, 0, 2, 80, 300, 0.8, 4, 8, 44, 16, 50);
    }
    wait(lua_state, 18.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    wait(lua_state, 18.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

unsafe extern "C" fn effect_specialairlw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12.0, 5.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_specialairlw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_special_l02"));
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_special_c1_l01"));
        PLAY_SE(agent, Hash40::new("se_miifighter_final06"));
        PLAY_SE(agent, Hash40::new("se_miifighter_final06"));
    }
}

unsafe extern "C" fn game_speciallw1loop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        app::sv_kinetic_energy::clear_speed(lua_state);
        KineticModule::clear_speed_all(boma);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        SET_SPEED_EX(agent, 3.0, -2.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 15.0, 361, 57, 0, 80, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
}

unsafe extern "C" fn effect_speciallw1loop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    for _ in 0..24 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 0, 220, 0, 0, 1, true);
        }
        wait(lua_state, 2.0);
    }
}

unsafe extern "C" fn game_speciallw1landing(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        SET_SPEED_EX(agent, 1.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 60, 48, 0, 120, 4.0, 0.0, 4.0, 5.0, Some(0.0), Some(4.0), Some(-6.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 60, 48, 0, 120, 4.5, 0.0, 6.5, -0.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 14.0, 60, 48, 0, 120, 6.5, 0.0, 6.5, -0.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 60, 48, 0, 120, 4.0, 0.0, 4.0, 5.0, Some(0.0), Some(4.0), Some(-6.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 2, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

// ================================================================================================
// ======================================== FEINT JUMP ============================================
// ================================================================================================

unsafe extern "C" fn game_speciallw2start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 1.5, 3.0);
    frame(lua_state, 1.5);
    FT_MOTION_RATE_RANGE(agent, 1.5, 2.0, 1.0);
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 43.0, 36.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_KICK_ENABLE_LANDING);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_START_WAIT_INPUT);
    }   frame(lua_state, 13.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_KICK_ENABLE);
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_REVERSE);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_WALL_JUMP_ENABLE);
        SEARCH(agent, 0, 0, Hash40::new("hip"), 3.5, 1.0, 0.0, 0.0, Some(-3.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

unsafe extern "C" fn game_specialairlw2start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 1.5, 3.0);
    frame(lua_state, 1.5);
    FT_MOTION_RATE_RANGE(agent, 1.5, 2.0, 1.0);
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 43.0, 36.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_KICK_ENABLE_LANDING);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_START_WAIT_INPUT);
    }   frame(lua_state, 13.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_KICK_ENABLE);
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_REVERSE);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_REVERSAL_KICK_FLAG_WALL_JUMP_ENABLE);
        SEARCH(agent, 0, 0, Hash40::new("hip"), 4.0, 1.0, 0.0, 0.0, Some(-3.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

unsafe extern "C" fn game_specialairlw2kick(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0, 0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        /*
        let dive_speed_x_modifier_stick_mul = 0.5;
        let dive_speed_y_modifier_stick_mul = 0.5;
        let mut dive_speed_x_modifier_raw = dive_speed_x_modifier_stick_mul * ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
        let mut dive_speed_y_modifier_raw = dive_speed_y_modifier_stick_mul * ControlModule::get_stick_y(boma);
        let mut dive_speed_x_modifier = dive_speed_x_modifier_raw.clamp(-dive_speed_x_modifier_stick_mul, dive_speed_x_modifier_stick_mul * 0.5);
        let mut dive_speed_y_modifier = dive_speed_x_modifier_raw.clamp(-dive_speed_y_modifier_stick_mul, dive_speed_y_modifier_stick_mul * 0.5);
        SET_SPEED_EX(fighter, -3.0 + dive_speed_x_modifier, -2.5 + dive_speed_y_modifier, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        */
        SET_SPEED_EX(agent, -2.5, -1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 10.0, 35, 75, 0, 65, 5.8, 4.2, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 60, 75, 0, 65, 5.8, 4.2, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE(agent, 1.5);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairlw2autoattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 40, 0, 95, 4.0, 0.0, 0.0, -2.0, Some(0.0), Some(0.0), Some(3.5), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 80, 40, 0, 95, 3.5, 0.0, 5.0, -2.0, Some(0.0), Some(5.0), Some(3.5), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

// ================================================================================================
// ============================================ WILD THROW ========================================
// ================================================================================================

unsafe extern "C" fn game_speciallw3catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 4.0, 8.0);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        //SEARCH(fighter, 0, 0, Hash40::new("top"), 5.0, 0.0, 7.0, 9.0, Some(0.0), Some(7.0), Some(10.5), *COLLISION_KIND_MASK_ALL, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
        GrabModule::set_rebound(boma, true);
        CATCH(agent, 2, Hash40::new("top"), 5.0, 0.0, 7.0, 9.0, Some(0.0), Some(7.0), Some(10.5), *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.2);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
        //search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

unsafe extern "C" fn game_speciallw3throw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        REVERSE_LR(agent);
        //ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 45, 66, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 275, 100, 25, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_CATCH_STOP(agent, 5, 1);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("armr"), 9.0, 361, 85, 0, 80, 5.0, 6.0, 0.0, 0.0, Some(6.0), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 9.0, 361, 85, 0, 80, 4.0, 0.0, 3.0, -5.0, Some(0.0), Some(3.0), Some(-11.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        FT_CATCH_STOP(agent, 5, 1);
        //CHECK_FINISH_CAMERA(fighter, 14, 0);
        //lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        let opponent_boma = agent.get_grabbed_opponent_boma();
        if opponent_boma.is_fighter() {
            VarModule::on_flag(opponent_boma.object(), vars::common::instance::IS_KNOCKDOWN_THROW);
        }
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn game_specialairlw3throw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING);
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        REVERSE_LR(agent);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 1.0, 270, 100, 1, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_CATCH_STOP(agent, 5, 1);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("armr"), 9.0, 361, 85, 0, 80, 5.0, 6.0, 0.0, 0.0, Some(6.0), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        FT_CATCH_STOP(agent, 5, 1);
        //CHECK_FINISH_CAMERA(fighter, 14, 0);
        //lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 13.0, 280, 65, 0, 10, 12.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_nomal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialairlw3throw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        COL_PRI(agent, 101);
        FLASH(agent, 1, 1, 1, 0);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_catch"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("miifighter_counter_arc"), Hash40::new("miifighter_counter_arc"), Hash40::new("top"), -1, 8, 1, 0, 112, 90, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

pub fn install(agent: &mut Agent) {;
    agent.acmd("game_speciallw1", game_speciallw1);
    agent.acmd("effect_speciallw1", effect_speciallw1);
    agent.acmd("sound_speciallw1", sound_speciallw1);
    agent.acmd("expression_speciallw1", expression_speciallw1);
    agent.acmd("effect_specialairlw1", effect_specialairlw1);
    agent.acmd("sound_specialairlw1", sound_specialairlw1);
    agent.acmd("game_speciallw1loop", game_speciallw1loop);
    agent.acmd("effect_speciallw1loop", effect_speciallw1loop);
    agent.acmd("game_speciallw1landing", game_speciallw1landing);

    agent.acmd("game_speciallw2start", game_speciallw2start);
    agent.acmd("game_specialairlw2start", game_specialairlw2start);
    agent.acmd("game_specialairlw2kick", game_specialairlw2kick);
    agent.acmd("game_specialairlw2autoattack", game_specialairlw2autoattack);
    
    agent.acmd("game_speciallw3catch", game_speciallw3catch);
    agent.acmd("game_specialairlw3catch", game_speciallw3catch);
    agent.acmd("game_speciallw3throw", game_speciallw3throw);
    agent.acmd("game_specialairlw3throw", game_specialairlw3throw);
    agent.acmd("effect_specialairlw3throw", effect_specialairlw3throw);
}