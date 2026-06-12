use super::*;

// ===============================================================================================
// ======================================= REVERSE IMPACT ========================================
// ===============================================================================================

unsafe extern "C" fn game_speciallw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 5.0, 75, 23, 0, 54, 3.5, -3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 5.0, 75, 23, 0, 54, 4.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 5.0, 75, 23, 0, 54, 3.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        ATTACK(agent, 3, 0, Hash40::new("armr"), 5.0, 361, 20, 0, 20, 3.5, -3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        ATTACK(agent, 4, 0, Hash40::new("armr"), 5.0, 361, 20, 0, 20, 4.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        ATTACK(agent, 5, 0, Hash40::new("armr"), 5.0, 361, 20, 0, 20, 3.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::on_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_CHECK_INPUT);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_CHECK_INPUT);
        VarModule::on_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_CHANGE_KINETIC);
    }
}

unsafe extern "C" fn effect_speciallw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 3.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 4, 6.5, -7, 0, 0, 0, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6.5, 10.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_speciallw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miiswordsman_dash_stop"));
        PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
        PLAY_SE(agent, Hash40::new("se_miiswordsman_jump03"));
    }
}

unsafe extern "C" fn expression_speciallw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn game_speciallw1flourish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 77, 75, 0, 40, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("haver"), 6.0, 77, 75, 0, 40, 3.0, 0.0, 3.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 82, 75, 0, 40, 3.0, 0.0, 9.75, -2.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 8.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.4);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.0);
    frame(lua_state, 35.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_CHANGE_KINETIC);
    }
}

unsafe extern "C" fn effect_speciallw1flourish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let lr = PostureModule::lr(boma);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 5.0 * lr, 12, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        let tex_sword = agent.get_int64(*FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
        let sword_add = agent.get_int64(*FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
        let sword_flare = agent.get_int64(*FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 5, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), 0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_speciallw1flourish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_m"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack02"));
    }
}

unsafe extern "C" fn expression_speciallw1flourish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

unsafe extern "C" fn game_speciallw1mordschlag(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 2.0, 3.0);
    frame(lua_state, 2.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        let mut damage = 16.0;
        if agent.is_situation(*SITUATION_KIND_AIR) {
            damage = 14.0;
            let lr = PostureModule::lr(boma);
            sv_kinetic_energy!(set_speed, agent, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.5 * lr, 0.0);
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        ATTACK(agent, 0, 0, Hash40::new("armr"), damage, 49, 78, 0, 80, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 7.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
		ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 49, 78, 0, 80, 3.0, 0.0, 0.0, -2.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 7.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
		ATTACK(agent, 2, 0, Hash40::new("haver"), damage, 49, 78, 0, 80, 3.0, 0.0, 3.0, -2.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 7.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        sv_kinetic_energy!(set_speed, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.8);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW1_CHANGE_KINETIC);
    }
}

unsafe extern "C" fn effect_speciallw1mordschlag(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let lr = PostureModule::lr(boma);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT(agent, Hash40::new("sys_unblockable_flash"), Hash40::new("top"), 5.0 * lr, 12, 5, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("sys_unblockable_flash"), Hash40::new("top"), 5.0 * lr, 12, 8.5, 0, 0, 0, 0.65, true);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 3, 8, 1, 180, 200, 70, 0.8, true, *EF_FLIP_AXIS_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.8);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_speciallw1mordschlag(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_l"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack03"));
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_09"));
    }
}

unsafe extern "C" fn expression_speciallw1mordschlag(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

// ================================================================================================================
// ================================================ SHOCK SPELL ===================================================
// ================================================================================================================

unsafe extern "C" fn game_speciallw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 6.0);  // f9
    FT_MOTION_RATE_RANGE(agent, 6.0, 12.0, 16.0);
    frame(lua_state, 7.0);  // f12
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        VarModule::on_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW2_CHANGE_ARTICLE);
        ArticleModule::generate_article(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_TORNADOSHOT, false, 0);
    }
    frame(lua_state, 12.0); // f25
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(agent, 16.0, 23.0, 10.0);
    frame(lua_state, 23.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_speciallw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -1.5, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_reflect_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0.0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.65);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("arml"), 4, 0, 0, 0, 0, 0, 0.4, true);
        if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SPECIAL_LW2_HOLD) {
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15.0, 8.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 10.5);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0.5, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_paralysis"), Hash40::new("arml"), 4, 0, 0, 0, 0, 0, 0.35, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_paralysis"), true, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_reflect_sword"), false, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("miiswordsman_reflect1"), -1);
    }
}

unsafe extern "C" fn sound_speciallw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miiswordsman_special_c2_l01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_special_c2_l01"));
    }
    frame(lua_state, 10.5);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_m"));
    }
}

// ================================================================================================
// ======================================== BLURRING BLADE ========================================
// ================================================================================================

unsafe extern "C" fn game_speciallw3start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 11.0, 8.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_speciallw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::common::instance::SPECIAL_STALL_USED) {
            VarModule::on_flag(agent.battle_object, vars::common::instance::SPECIAL_STALL_USED);
            SET_SPEED_EX(agent, 0.5, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else {
            let speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            SET_SPEED_EX(agent, 0.5, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    frame(lua_state, 6.0);
    let shield_damage = if agent.is_motion_one_of(&[Hash40::new("special_n3_max"), Hash40::new("special_air_n3_max")]) { 3.5 } else { 1.0 };
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        for _ in 0..4 {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("haver"), 1.0, 180, 100, 7, 0, 3.5, 0.0, 7.0, 0.0, Some(0.0), Some(9.0), Some(0.0), 0.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 1, 0, Hash40::new("haver"), 1.0, 0, 100, 5, 0, 3.5, 0.0, -4.0, 0.0, Some(0.0), Some(9.0), Some(0.0), 0.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
            wait(lua_state, 2.0);
            if is_excute(agent) {
                AttackModule::clear_all(boma);
            }
            wait(lua_state, 2.0);
        }
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 85, 55, 0, 30, 3.5, 0.0, 9.5, 0.0, Some(0.0), Some(10.0), Some(0.0), 1.0, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 70, 55, 0, 30, 3.5, 0.0, -3.0, 0.0, Some(0.0), Some(10.0), Some(0.0), 1.0, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        for _ in 0..4 {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("haver"), 1.0, 366, 100, 10, 0, 3.5, 0.0, -4.0, 0.0, Some(0.0), Some(9.0), Some(0.0), 0.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
            wait(lua_state, 2.0);
            if is_excute(agent) {
                AttackModule::clear_all(boma);
            }
            wait(lua_state, 2.0);
        }
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 85, 55, 0, 30, 3.5, 0.0, 9.5, 0.0, Some(0.0), Some(10.0), Some(0.0), 1.0, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 70, 55, 0, 30, 3.5, 0.0, -2.2, 0.0, Some(0.0), Some(10.0), Some(0.0), 1.0, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW3_CHECK_INPUT);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW3_CHECK_INPUT);
    }
}

unsafe extern "C" fn effect_speciallw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 4, 0, -5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        if agent.is_motion_one_of(&[Hash40::new("special_n3_end_max"), Hash40::new("special_air_n3_max")]) {
            LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        let tex_sword = agent.get_int64(*FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
        let sword_add = agent.get_int64(*FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
        let sword_flare = agent.get_int64(*FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_rapid_slash_wind_s"), Hash40::new("top"), -0.0, 5.5, 12, 0, 0, 0, 1, true);
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_wind_s"), false, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn sound_speciallw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_miiswordsman_special_s01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_special_c3_n01"));
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miiswordsman_special_c3_n01"));
    }
}

unsafe extern "C" fn expression_speciallw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_motion_one_of(&[Hash40::new("special_n3_end_max"), Hash40::new("special_air_n3_max")]) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x26769bd1de), 0, 30, 3);
        }
        else {
            AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_NONE), AttackDirectionAxis(*ATTACK_DIRECTION_NONE));
        }
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashss"), 4);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallw3end2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::add_speed(boma, &Vector3f::new(0.0, 3.5, 0.0));
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        let (shield_damage, sfx) = if agent.is_motion_one_of(&[Hash40::new("special_n3_end2_max"), Hash40::new("special_air_n3_end2_max")])
            { (10, *COLLISION_SOUND_ATTR_FIRE) } else { (2, *COLLISION_SOUND_ATTR_KICK) };
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 80, 85, 0, 64, 10.0, 0.0, 10.0, 9.5, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, sfx, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE_RANGE(agent, 15.0, 55.0, 26.0);
    frame(lua_state, 55.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallw3end2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_rapid_slash_arc"), Hash40::new("top"), 0.75, 11, 1, -20, -70, 90, 1.2, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        }
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("haver"), 0, 9, 4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
}

unsafe extern "C" fn sound_speciallw3end2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miiswordsman_special_c3_n02"));
    }
}

unsafe extern "C" fn expression_speciallw3end2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if agent.is_motion_one_of(&[Hash40::new("special_n3_end2_max"), Hash40::new("special_air_n3_end2_max")]) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        }
        else {
            RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x26769bd1de), 0, 30, 10);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw1", game_speciallw1, Priority::Low);
    agent.acmd("game_specialairlw1", game_speciallw1, Priority::Low);
    agent.acmd("effect_speciallw1", effect_speciallw1, Priority::Low);
    agent.acmd("effect_specialairlw1", effect_speciallw1, Priority::Low);
    agent.acmd("sound_speciallw1", sound_speciallw1, Priority::Low);
    agent.acmd("sound_specialairlw1", sound_speciallw1, Priority::Low);
    agent.acmd("expression_speciallw1", expression_speciallw1, Priority::Low);
    agent.acmd("expression_specialairlw1", expression_speciallw1, Priority::Low);

    agent.acmd("game_speciallw1flourish", game_speciallw1flourish, Priority::Low);
    agent.acmd("game_specialairlw1flourish", game_speciallw1flourish, Priority::Low);
    agent.acmd("effect_speciallw1flourish", effect_speciallw1flourish, Priority::Low);
    agent.acmd("effect_specialairlw1flourish", effect_speciallw1flourish, Priority::Low);
    agent.acmd("sound_speciallw1flourish", sound_speciallw1flourish, Priority::Low);
    agent.acmd("sound_specialairlw1flourish", sound_speciallw1flourish, Priority::Low);
    agent.acmd("expression_speciallw1flourish", expression_speciallw1flourish, Priority::Low);
    agent.acmd("expression_specialairlw1flourish", expression_speciallw1flourish, Priority::Low);

    agent.acmd("game_speciallw1mordschlag", game_speciallw1mordschlag, Priority::Low);
    agent.acmd("game_specialairlw1mordschlag", game_speciallw1mordschlag, Priority::Low);
    agent.acmd("effect_speciallw1mordschlag", effect_speciallw1mordschlag, Priority::Low);
    agent.acmd("effect_specialairlw1mordschlag", effect_speciallw1mordschlag, Priority::Low);
    agent.acmd("sound_speciallw1mordschlag", sound_speciallw1mordschlag, Priority::Low);
    agent.acmd("sound_specialairlw1mordschlag", sound_speciallw1mordschlag, Priority::Low);
    agent.acmd("expression_speciallw1mordschlag", expression_speciallw1mordschlag, Priority::Low);
    agent.acmd("expression_specialairlw1mordschlag", expression_speciallw1mordschlag, Priority::Low);

    agent.acmd("game_speciallw2", game_speciallw2, Priority::Low);
    agent.acmd("game_specialairlw2", game_speciallw2, Priority::Low);
    agent.acmd("effect_speciallw2", effect_speciallw2, Priority::Low);
    agent.acmd("effect_specialairlw2", effect_speciallw2, Priority::Low);
    agent.acmd("sound_speciallw2", sound_speciallw2, Priority::Low);
    agent.acmd("sound_specialairlw2", sound_speciallw2, Priority::Low);

    agent.acmd("game_specialn3start", game_speciallw3start, Priority::Low);
    agent.acmd("game_specialairn3start", game_speciallw3start, Priority::Low);

    agent.acmd("game_specialn3end", game_speciallw3end, Priority::Low);
    agent.acmd("game_specialairn3end", game_speciallw3end, Priority::Low);
    agent.acmd("effect_specialn3end", effect_speciallw3end, Priority::Low);
    agent.acmd("effect_specialairn3end", effect_speciallw3end, Priority::Low);
    agent.acmd("sound_specialn3end", sound_speciallw3end, Priority::Low);
    agent.acmd("sound_specialairn3end", sound_speciallw3end, Priority::Low);
    agent.acmd("expression_specialn3end", expression_speciallw3end, Priority::Low);
    agent.acmd("expression_specialairn3end", expression_speciallw3end, Priority::Low);
    
    agent.acmd("game_specialn3endmax", game_speciallw3end, Priority::Low);
    agent.acmd("game_specialairn3endmax", game_speciallw3end, Priority::Low);
    agent.acmd("effect_specialn3endmax", effect_speciallw3end, Priority::Low);
    agent.acmd("effect_specialairn3endmax", effect_speciallw3end, Priority::Low);
    agent.acmd("sound_specialn3endmax", sound_speciallw3end, Priority::Low);
    agent.acmd("sound_specialairn3endmax", sound_speciallw3end, Priority::Low);
    agent.acmd("expression_specialn3endmax", expression_speciallw3end, Priority::Low);
    agent.acmd("expression_specialairn3endmax", expression_speciallw3end, Priority::Low);

    agent.acmd("game_speciallw3end2", game_speciallw3end2, Priority::Low);
    agent.acmd("game_specialairlw3end2", game_speciallw3end2, Priority::Low);
    agent.acmd("effect_speciallw3end2", effect_speciallw3end2, Priority::Low);
    agent.acmd("effect_specialairlw3end2", effect_speciallw3end2, Priority::Low);
    agent.acmd("sound_speciallw3end2", sound_speciallw3end2, Priority::Low);
    agent.acmd("sound_specialairlw3end2", sound_speciallw3end2, Priority::Low);
    agent.acmd("expression_speciallw3end2", expression_speciallw3end2, Priority::Low);
    agent.acmd("expression_specialairlw3end2", expression_speciallw3end2, Priority::Low);

    agent.acmd("game_speciallw3end2max", game_speciallw3end2, Priority::Low);
    agent.acmd("game_specialairlw3end2max", game_speciallw3end2, Priority::Low);
    agent.acmd("effect_speciallw3end2max", effect_speciallw3end2, Priority::Low);
    agent.acmd("effect_specialairlw3end2max", effect_speciallw3end2, Priority::Low);
    agent.acmd("sound_speciallw3end2max", sound_speciallw3end2, Priority::Low);
    agent.acmd("sound_specialairlw3end2max", sound_speciallw3end2, Priority::Low);
    agent.acmd("expression_speciallw3end2max", expression_speciallw3end2, Priority::Low);
    agent.acmd("expression_specialairlw3end2max", expression_speciallw3end2, Priority::Low);
}