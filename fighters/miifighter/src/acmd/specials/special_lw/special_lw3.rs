use super::*;

// ================================================================================================
// ======================================== BOILING PUNT ==========================================
// ================================================================================================

unsafe extern "C" fn game_speciallw31g(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 17.0, 12.0);
    frame(lua_state, 17.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 7.0, 70, 112, 0, 40, 4.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 7.0, 70, 112, 0, 40, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallw31g(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 3.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -4, 180, 0, 0, 0.7, true);
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6.5, 10.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_speciallw31g(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_swing_m"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_attack01"));
    }
}

unsafe extern "C" fn expression_speciallw31g(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_speciallw31a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 7.0, 65, 112, 0, 40, 4.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 7.0, 65, 112, 0, 40, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_LW3_ENABLE_BOUNCE);
    }
    frame(lua_state, 13.0);
    if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
        FT_MOTION_RATE_RANGE(agent, 13.0, 29.0, 25.0);
    }
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_LW3_ENABLE_LANDING);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_speciallw31a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_c"), Hash40::new("sys_attack_arc_c"), Hash40::new("top"), 2, 7, -2, 75, -151, 70, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, -1, 2.5, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn sound_speciallw31a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_swing_m"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_attack01"));
    }
}

unsafe extern "C" fn expression_speciallw31a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn game_speciallw32g(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 17.0, 12.0);
    frame(lua_state, 17.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 12.0, 60, 80, 0, 56, 4.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 12.0, 60, 80, 0, 56, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 18.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 2.25);
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallw32g(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 3.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("miifighter_boiling2"), Hash40::new("miifighter_boiling2"), Hash40::new("top"), -2, 4.7, 1.5, -16, -43, 15, 1, true, *EF_FLIP_AXIS_YZ);
        LAST_EFFECT_SET_COLOR(agent, 3.0, 1.0, 0.0);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -4, 180, 0, 0, 0.7, true);
        LAST_EFFECT_SET_COLOR(agent, 2.0, 0.5, 0.0);
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6.5, 10.5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 3.0, 0.0, 0.0);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_attack_impact"), 0);
    }
}

unsafe extern "C" fn sound_speciallw32g(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_swing_l"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_attack02"));
    }
}

unsafe extern "C" fn expression_speciallw32g(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_speciallw32a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 12.0, 60, 80, 0, 56, 4.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 12.0, 60, 80, 0, 56, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_LW3_ENABLE_BOUNCE);
    }
    frame(lua_state, 13.0);
    if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
        FT_MOTION_RATE_RANGE(agent, 13.0, 29.0, 25.0);
    }
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_LW3_ENABLE_LANDING);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_speciallw32a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("miifighter_boiling_air2"), Hash40::new("miifighter_boiling_air2"), Hash40::new("top"), 2, 7, -2, 75, -151, 70, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(agent, 2.0, 1.0, 0.0);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, -1, 2.5, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(agent, 3.0, 0.0, 0.0);
    }
}

unsafe extern "C" fn sound_speciallw32a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_swing_l"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_attack02"));
    }
}

unsafe extern "C" fn expression_speciallw32a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallw33g(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 43.0, 9.0);
    frame(lua_state, 43.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 50.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_STALL);
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 24.0, 45, 78, 0, 45, 4.5, 3.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 24.0, 45, 78, 0, 45, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_kick_hit_l"));
        AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_kick_hit_l"));
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 51.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 3.3);
    frame(lua_state, 54.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::miifighter::instance::SPECIAL_LW3_STALL);
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 20.4, 50, 60, 0, 30, 3.5, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 20.4, 50, 60, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 61.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallw33g(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut handle = 0;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_hyakuretsukick"), Hash40::new("top"), -4, 12, 7, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), -4, 12, 7, 0, 0, 0, 0.5, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_sidekick_hold"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.8, true);
        handle = EffectModule::req_follow(boma, Hash40::new("miifighter_sidekick_flash"), Hash40::new("toel"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, false, false);
        EffectModule::set_rate(boma, handle as u32, (43.0 - 1.0)/12.0);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miifighter_hyakuretsukick"), true, true);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_sp_flash"), true, true);
    }
    frame(lua_state, 33.0);
    for _ in 0..4 {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("miifighter_sidekick"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EffectModule::set_rate(boma, handle as u32, 1.0);
    }
    frame(lua_state, 49.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 3.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miifighter_sidekick_hold"), true, true);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miifighter_sidekick_flash"), true, true);
    }
}

unsafe extern "C" fn sound_speciallw33g(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let handle = SoundModule::play_se(boma, Hash40::new("se_miifighter_special_c3_n01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_speed(boma, handle as i32, 3.0);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_special_c3_n02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_special_c3_n01"));
    }
}

unsafe extern "C" fn expression_speciallw33g(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(lua_state, 77.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_speciallw33a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 19.2, 284, 36, 0, 42, 4.5, 3.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 19.2, 284, 36, 0, 42, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_kick_hit_l"));
        AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_kick_hit_l"));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 19.2, 284, 36, 0, 42, 4.5, 3.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 19.2, 284, 36, 0, 42, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_kick_hit_l"));
        AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_kick_hit_l"));
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        //VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_LW3_ENABLE_BOUNCE);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 20.0, 40.0, 25.0);
    frame(lua_state, 34.0); // f38
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_LW3_ENABLE_LANDING);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallw33a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_hyakuretsukick"), Hash40::new("top"), -4, 12, 7, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), -4, 12, 7, 0, 0, 0, 0.5, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miifighter_hyakuretsukick"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_sp_flash"), true, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("miifighter_boiling_air3"), Hash40::new("miifighter_boiling_air3"), Hash40::new("top"), -1, 5, 0, 90, 90, 0, 0.72, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_boiling_firearc"), Hash40::new("top"), -1, 5, 0, 90, 0, 0, 0.72, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_boiling_firearc"), Hash40::new("top"), -1, 5, 0, 90, 0, 0, 0.72, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}

unsafe extern "C" fn sound_speciallw33a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_special_c3_n01"));
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_swing_ll"));
        PLAY_SE(agent, Hash40::new("se_common_fire_m"));
    }
}

unsafe extern "C" fn expression_speciallw33a(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw31g", game_speciallw31g, Priority::Low);
    agent.acmd("effect_speciallw31g", effect_speciallw31g, Priority::Low);
    agent.acmd("sound_speciallw31g", sound_speciallw31g, Priority::Low);
    agent.acmd("expression_speciallw31g", expression_speciallw31g, Priority::Low);

    agent.acmd("game_speciallw31a", game_speciallw31a, Priority::Low);
    agent.acmd("effect_speciallw31a", effect_speciallw31a, Priority::Low);
    agent.acmd("sound_speciallw31a", sound_speciallw31a, Priority::Low);
    agent.acmd("expression_speciallw31a", expression_speciallw31a, Priority::Low);

    agent.acmd("game_speciallw32g", game_speciallw32g, Priority::Low);
    agent.acmd("effect_speciallw32g", effect_speciallw32g, Priority::Low);
    agent.acmd("sound_speciallw32g", sound_speciallw32g, Priority::Low);
    agent.acmd("expression_speciallw32g", expression_speciallw32g, Priority::Low);

    agent.acmd("game_speciallw32a", game_speciallw32a, Priority::Low);
    agent.acmd("effect_speciallw32a", effect_speciallw32a, Priority::Low);
    agent.acmd("sound_speciallw32a", sound_speciallw32a, Priority::Low);
    agent.acmd("expression_speciallw32a", expression_speciallw32a, Priority::Low);

    agent.acmd("game_speciallw33g", game_speciallw33g, Priority::Low);
    agent.acmd("effect_speciallw33g", effect_speciallw33g, Priority::Low);
    agent.acmd("sound_speciallw33g", sound_speciallw33g, Priority::Low);
    agent.acmd("expression_speciallw33g", expression_speciallw33g, Priority::Low);

    agent.acmd("game_speciallw33a", game_speciallw33a, Priority::Low);
    agent.acmd("effect_speciallw33a", effect_speciallw33a, Priority::Low);
    agent.acmd("sound_speciallw33a", sound_speciallw33a, Priority::Low);
    agent.acmd("expression_speciallw33a", expression_speciallw33a, Priority::Low);
}