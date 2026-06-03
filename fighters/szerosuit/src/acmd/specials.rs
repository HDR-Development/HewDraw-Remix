use super::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 15.0/(21.0-1.0));
}

unsafe extern "C" fn game_specialnshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 26.0/(28.0-1.0));
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_N_FLAG_SHOOT);
        ArticleModule::generate_article(boma, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_PARALYZER_BULLET, false, -1);
    }
}

unsafe extern "C" fn game_specialnlanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 9.0/(14.0-1.0));
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_SZEROSUIT_STATUS_SPECIAL_S_FLAG_S2_CHECK);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 28.0, 14.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_SZEROSUIT_STATUS_SPECIAL_S_FLAG_S2_CHECK);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 29.0);
    FT_MOTION_RATE(agent, 4.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 80, 0, 60, 4.0, 0.0, 8.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_WHIP);
            ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 15, 100, 65, 0, 3.5, 0.0, 8.0, 19.0, Some(0.0), Some(8.0), Some(17.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
            ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 25, 100, 35, 0, 4.0, 0.0, 8.0, 30.0, Some(0.0), Some(8.0), Some(26.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 80, 0, 60, 4.0, 0.0, 8.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_WHIP);
            ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 15, 100, 70, 0, 3.5, 0.0, 8.0, 19.0, Some(0.0), Some(8.0), Some(17.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
            ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 25, 100, 40, 0, 4.0, 0.0, 8.0, 30.0, Some(0.0), Some(8.0), Some(26.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        }
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_S_FLAG_S2);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            ATTACK(agent, 0, 1, Hash40::new("top"), 9.0, 50, 115, 0, 50, 5.0, 0.0, 8.5, 30.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
            ATTACK(agent, 1, 1, Hash40::new("top"), 9.0, 50, 115, 0, 50, 6.0, 0.0, 8.5, 36.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
        }
        else {
            ATTACK(agent, 0, 1, Hash40::new("top"), 9.0, 50, 115, 0, 50, 7.0, 0.0, 8.5, 28.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
            ATTACK(agent, 1, 1, Hash40::new("top"), 9.0, 50, 115, 0, 50, 8.0, 0.0, 8.5, 34.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if !agent.is_flag(*FIGHTER_SZEROSUIT_STATUS_SPECIAL_S_FLAG_S2_UNABLE) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15, 10, 0, 0, 0, 0.75, true);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("szero_whip_c"), Hash40::new("top"), 0, 8.5, 25, 0, -90, 0, 1, true);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("szero_pwhip_hit"), Hash40::new("top"), 0, 8.5, 36, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("szero_whip_vanish"), Hash40::new("haver"), 0, 0, 0, 0, 0, 90, 0.6, true);
    }
}

unsafe extern "C" fn game_specials2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 32.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 5, Hash40::new("top"), 7.0, 98, 75, 0, 50, 6.0, 0.0, 8.0, 27.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        ATTACK(agent, 1, 5, Hash40::new("top"), 7.0, 105, 75, 0, 50, 7.0, 0.0, 8.0, 33.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialairs2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("szero_whip_c"), Hash40::new("top"), 0, 8.5, 25, 0, -90, 0, 1, true);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("szero_whip_vanish"), Hash40::new("haver"), 0, 0, 0, 0, 0, 90, 0.6, true);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 5.0);
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 2.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 90, 100, 80, 0, 6.0, 0.0, 8.0, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 367, 100, 50, 0, 5.0, 0.0, 17.0, 5.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 1.3, 86, 100, 12, 0, 5.0, 0.0, 16.0, 5.5, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 48, 196, 0, 35, 7.0, 0.0, 12.5, 11.0, Some(0.0), Some(12.0), Some(9.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE_RANGE(agent, 1.0, 3.0, 6.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::szerosuit::status::SPECIAL_LW_ENABLE_MANUAL_FOOTSTOOL);
    }
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_START_WAIT_INPUT);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_KICK_ENABLE);
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_REVERSE);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        VarModule::on_flag(boma.object(), vars::szerosuit::status::SPECIAL_LW_ENABLE_MANUAL_FOOTSTOOL);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_WALL_JUMP_ENABLE);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
        VarModule::off_flag(boma.object(), vars::szerosuit::status::SPECIAL_LW_ENABLE_MANUAL_FOOTSTOOL);
    }
}

unsafe extern "C" fn game_specialairlwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE_RANGE(agent, 1.0, 3.0, 6.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::szerosuit::status::SPECIAL_LW_ENABLE_MANUAL_FOOTSTOOL);
    }
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_START_WAIT_INPUT);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_KICK_ENABLE);
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_REVERSE);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_WALL_JUMP_ENABLE);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
    }
}

unsafe extern "C" fn game_specialairlwkick(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_KICK_REVERSE_ENABLE);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 12.0, 48, 115, 0, 45, 4.8, -4.0, 0.0, 0.0, Some(-1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // Air-only
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 14.0, 290, 52, 0, 30, 4.8, 10.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // Ground-only
        ATTACK(agent, 2, 0, Hash40::new("kneer"), 14.0, 45, 100, 0, 45, 4.8, 10.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_INSTANCE_WORK_ID_FLAG_TO_SPECIAL_LANDING);
    }
}

unsafe extern "C" fn game_specialairlwflip(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if VarModule::is_flag(boma.object(), vars::szerosuit::status::SPECIAL_LW_ENABLE_MANUAL_FOOTSTOOL) {
        FT_MOTION_RATE_RANGE(agent, 1.0, 15.0, 19.0);
    }
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::szerosuit::status::ATTACK_AIR_LW_REBOUND) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 270, 82, 0, 11, 3.0, 0.0, -3.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 82, 0, 11, 4.0, 0.0, -3.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 270, 82, 0, 11, 3.0, 0.0, -3.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialairnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialnshoot", game_specialnshoot, Priority::Low);
    agent.acmd("game_specialairnshoot", game_specialnshoot, Priority::Low);
    agent.acmd("game_specialnlanding", game_specialnlanding, Priority::Low);

    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialairs", game_specials, Priority::Low);
    agent.acmd("effect_specials", effect_specials, Priority::Low);
    agent.acmd("effect_specialairs", effect_specials, Priority::Low);

    agent.acmd("game_specials2", game_specials2, Priority::Low);
    agent.acmd("game_specialairs2", game_specials2, Priority::Low);
    agent.acmd("effect_specialairs2", effect_specialairs2, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);

    agent.acmd("game_speciallwstart", game_speciallwstart, Priority::Low);
    agent.acmd("game_specialairlwstart", game_specialairlwstart, Priority::Low);
    agent.acmd("game_specialairlwkick", game_specialairlwkick, Priority::Low);
    agent.acmd("game_specialairlwflip", game_specialairlwflip, Priority::Low);
}