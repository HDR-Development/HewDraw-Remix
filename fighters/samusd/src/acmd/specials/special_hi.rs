use super::*;

unsafe extern "C" fn game_specialhihold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 41.0, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 40, 24, 0, 39, 6.5, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_electric_hit_m"));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 41.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialhihold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_AURA(agent);
    }
    frame(lua_state, 1.0);
    loop {
        if is_excute(agent) {
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.73, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(agent, 1.2);
            }
            BURN_COLOR(agent, 0.26, 0.71, 1.5, 0.7);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            BURN_COLOR_FRAME(agent, 20, 1, 1, 1, 0);
        }
        wait(lua_state, 3.0);
        if is_excute(agent) {
            BURN_COLOR_NORMAL(agent);
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, -3, -90, 0, 0, 1.1, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, 3, -90, 0, 0, 1.1, true);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn sound_specialhihold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        let sound = SoundModule::play_status_se(boma, Hash40::new("se_samusd_special_h01"), false, false, false);
        SoundModule::set_se_pitch_ratio(boma, Hash40::new("se_samusd_special_h01"), 0.8);
    }
}

unsafe extern "C" fn expression_specialhihold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        //RUMBLE_HIT_STATUS(agent, Hash40::new("rbkind_spinattack"), 0);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("rbkind_spinattack"), 0);
        sv_animcmd::RUMBLE_HIT_STATUS(lua_state);
        agent.clear_lua_stack();

        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("rot"), 16.0, 70, 60, 0, 75, 5.0, 0.0, -4.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 9, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SAMUSD_SCREW_FINISH, *ATTACK_REGION_BODY);
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_kick_hit_l"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("rot"), 10.0, 60, 50, 0, 85, 5.0, 0.0, -4.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUSD_SCREW_FINISH, *ATTACK_REGION_BODY);
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_kick_hit_m"));
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_AURA(agent);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_dash_attack"), Hash40::new("rot"), 0, -4, 3.5, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_SCALE_W(agent, 0.9, 1.1, 0.9);
        LAST_EFFECT_SET_ALPHA(agent, 0.75);
        let offset = if agent.lr() < 0.0 { -4 } else { 4 };
        EFFECT_FOLLOW(agent, Hash40::new("samusd_cshot_bullet"), Hash40::new("rot"), offset, -4, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_COLOR(agent, 10.0, 10.0, 10.0);
        LAST_EFFECT_SET_ALPHA(agent, 0.75);
        EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("rot"), 0.0, -4.0, -6.0, -90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    loop {
        for i in 0..2 {
            if is_excute(agent) {
                BURN_COLOR(agent, 0.26, 0.71, 1.5, 0.7);
            }
            wait(lua_state, 2.0);
            if is_excute(agent) {
                BURN_COLOR_FRAME(agent, 20, 1, 1, 1, 0);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                BURN_COLOR_NORMAL(agent);
            }
            wait(lua_state, 1.0);
        }
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("samusd_dash_attack"), Hash40::new("rot"), 0, -4, 3.5, 0, 0, 0, 1.1, true);
            LAST_EFFECT_SET_SCALE_W(agent, 0.9, 1.1, 0.9);
        }
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samusd_special_n04"));
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_fly"), 20, true, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

unsafe extern "C" fn game_specialhifall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
        JostleModule::set_status(boma, true);
    }
}

unsafe extern "C" fn effect_specialhifall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_AURA(agent);
        BURN_COLOR(agent, 0.26, 0.71, 1.5, 0.7);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("samusd_win3_aura"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        BURN_COLOR_FRAME(agent, 20, 1, 1, 1, 0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
    }
}

unsafe extern "C" fn effect_specialhilandingf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_AURA(agent);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("samusd_win3_aura"), false, true);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialhilandingf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        //STOP_SE(agent, Hash40::new("se_samusd_special_n04"));
        PLAY_SE(agent, Hash40::new("se_samusd_dash_stop"));
    }
}

unsafe extern "C" fn expression_specialhilandingf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
unsafe extern "C" fn game_specialhilandinglw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 80, 0, 30, 3.0, 0.0, 3.0, -5.0, Some(0.0), Some(3.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialhilandinglw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("samusd_dash_attack"), 0);
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 3, 0, 2, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 3, 0, 2, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_AURA(agent);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("samusd_win3_aura"), false, true);
    }
}

unsafe extern "C" fn sound_specialhilandinglw(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        //STOP_SE(agent, Hash40::new("se_samusd_special_n04"));
        PLAY_SE(agent, Hash40::new("se_samusd_landing02"));
    }
}

unsafe extern "C" fn expression_specialhilandinglw(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialairhiwallf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

unsafe extern "C" fn sound_specialairhiwallf(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_samusd_special_n04"));
        PLAY_SE(agent, Hash40::new("se_common_down_m_01"));
    }
}

unsafe extern "C" fn expression_specialairhiwallf(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialairhiceil(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.2);
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}
unsafe extern "C" fn sound_specialairhiceil(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        //STOP_SE(agent, Hash40::new("se_samusd_special_n04"));
        PLAY_SE(agent, Hash40::new("se_common_down_m_01"));
    }
}

unsafe extern "C" fn expression_specialairhiceil(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhihold", game_specialhihold, Priority::Low);
    agent.acmd("game_specialhiholdair", game_specialhihold, Priority::Low);
    agent.acmd("effect_specialhihold", effect_specialhihold, Priority::Low);
    agent.acmd("effect_specialhiholdair", effect_specialhihold, Priority::Low);
    agent.acmd("sound_specialhihold", sound_specialhihold, Priority::Low);
    agent.acmd("sound_specialhiholdair", sound_specialhihold, Priority::Low);
    agent.acmd("expression_specialhihold", expression_specialhihold, Priority::Low);
    agent.acmd("expression_specialhiholdair", expression_specialhihold, Priority::Low);
    
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi, Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi, Priority::Low);
    agent.acmd("expression_specialhi", expression_specialhi, Priority::Low);
    
    agent.acmd("game_specialhifall", game_specialhifall, Priority::Low);
    agent.acmd("effect_specialhifall", effect_specialhifall, Priority::Low);
    
    agent.acmd("game_specialhilandingf", acmd_stub, Priority::Low);
    agent.acmd("effect_specialhilandingf", effect_specialhilandingf, Priority::Low);
    agent.acmd("sound_specialhilandingf", sound_specialhilandingf, Priority::Low);
    agent.acmd("expression_specialhilandingf", expression_specialhilandingf, Priority::Low);

    agent.acmd("game_specialhilandinglw", game_specialhilandinglw, Priority::Low);
    agent.acmd("effect_specialhilandinglw", effect_specialhilandinglw, Priority::Low);
    agent.acmd("sound_specialhilandinglw", sound_specialhilandinglw, Priority::Low);
    agent.acmd("expression_specialhilandinglw", expression_specialhilandinglw, Priority::Low);
}