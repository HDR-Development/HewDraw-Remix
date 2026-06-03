use super::*;

// ================================================================================================
// ======================================== EARTHQUAKE FIST =======================================
// ================================================================================================

unsafe extern "C" fn game_speciallw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_LW1_HOLD);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        let charge_distance = VarModule::get_float(agent.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE_DISTANCE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 90, 54, 0, 45, 2.5, 0.0, 2.0, 10.0 + charge_distance, Some(0.0), Some(2.0), Some(14.0 + charge_distance), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 90, 100, 0, 10, 3.5, 0.0, 3.0, 12.0 + charge_distance, Some(0.0), Some(8.0), Some(12.0 + charge_distance), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 70, 64, 0, 35, 2.5, 0.0, 2.0, 12.0 + charge_distance, Some(0.0), Some(2.0), Some(21.0 + charge_distance), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("top"), 10.0, 110, 64, 0, 35, 2.5, 0.0, 2.0, 3.0 + charge_distance, Some(0.0), Some(2.0), Some(12.0 + charge_distance), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 4, 0, Hash40::new("top"), 8.0, 45, 60, 0, 60, 3.0, 0.0, 3.0, 7.0, Some(0.0), Some(3.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -3, 12, 5, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_sidekick_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        let eff_handle = EffectModule::req_follow(boma, Hash40::new("sys_windwave"), Hash40::new("top"), &Vector3f::new(0.0, 0.0, 10.0), &Vector3f::zero(), 0.4, false, 0, 0, 0, 0, 0, false, false);
        EffectModule::set_rate(boma, eff_handle as u32, 0.4);
        VarModule::set_int64(agent.battle_object, vars::miifighter::status::SPECIAL_LW1_QUAKE_EFFECT_HANDLE, eff_handle as u64);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_sidekick_hold"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 0.4);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miifighter_sidekick_hold"), true, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        let charge_distance = VarModule::get_float(agent.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE_DISTANCE);
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 8.0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
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
    frame(lua_state, 38.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_attack03"));
    }
    frame(lua_state, 40.0);
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
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        AREA_WIND_2ND_arg10(agent, 0, 2, 80, 300, 0.8, 4, 8, 44, 16, 50);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
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
        PLAY_SE(agent, Hash40::new("se_miifighter_final06"));
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_special_c1_l01"));
    }
}

unsafe extern "C" fn game_speciallw1loop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        agent.clear_lua_stack();
        lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        app::sv_kinetic_energy::clear_speed(lua_state);
        KineticModule::clear_speed_all(boma);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        SET_SPEED_EX(agent, 2.75, -2.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 16.0, 361, 66, 0, 60, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 12.0); //f9
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 13.0, 361, 66, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
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
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 60, 105, 0, 70, 4.0, 0.0, 4.0, 5.0, Some(0.0), Some(4.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}


unsafe extern "C" fn sound_speciallw1landing(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_miifighter_final06"));
        PLAY_SE(agent, Hash40::new("se_miifighter_special_l03"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw1", game_speciallw1, Priority::Low);
    agent.acmd("effect_speciallw1", effect_speciallw1, Priority::Low);
    agent.acmd("sound_speciallw1", sound_speciallw1, Priority::Low);
    agent.acmd("expression_speciallw1", expression_speciallw1, Priority::Low);
    agent.acmd("effect_specialairlw1", effect_specialairlw1, Priority::Low);
    agent.acmd("sound_specialairlw1", sound_specialairlw1, Priority::Low);

    agent.acmd("game_speciallw1loop", game_speciallw1loop, Priority::Low);
    agent.acmd("effect_speciallw1loop", effect_speciallw1loop, Priority::Low);

    agent.acmd("game_speciallw1landing", game_speciallw1landing, Priority::Low);
    agent.acmd("sound_speciallw1landing", sound_speciallw1landing, Priority::Low);
}