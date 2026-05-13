use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_PEACH_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 10.0, 7.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 29.0, 18.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::peach::instance::SPECIAL_N_AUTOFIRE);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, *FIGHTER_PEACH_SHIELD_KIND_KINOPIO_GUARD, *FIGHTER_PEACH_SHIELD_GROUP_KIND_KINOPIO_GUARD);
    }
    frame(lua_state, 29.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, *FIGHTER_PEACH_SHIELD_KIND_KINOPIO_GUARD, *FIGHTER_PEACH_SHIELD_GROUP_KIND_KINOPIO_GUARD);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        agent.set_float(boma.lr(), *FIGHTER_PEACH_STATUS_SPECIAL_N_WORK_FLOAT_SHIELD_LR);
        VarModule::on_flag(agent.battle_object, vars::peach::instance::SPECIAL_N_AUTOFIRE);
        agent.change_status_req(*FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT, true);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6, 7, 0, 0, 0, 1.6, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0.75);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 2.5*boma.lr(), 6.5, 6, 0, 0, 0, 0.825, true);
        LAST_EFFECT_SET_RATE(agent, 10.0/17.0);
    }
    for _ in 0..4 {
        if is_excute(agent) {
            FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(agent) {
        FLASH(agent, 0.7, 0.7, 0.7, 0.5);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 6, 5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_specialnhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::peach::instance::SPECIAL_N_AUTOFIRE) {
            WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        }
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 3.0, 4.0);
    if is_excute(agent) {
        ArticleModule::set_rate(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, (3.0-1.0)/4.0);
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ArticleModule::set_rate(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, 1.0);
        ArticleModule::generate_article(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        ArticleModule::generate_article(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    if !VarModule::is_flag(agent.battle_object, vars::peach::instance::SPECIAL_N_AUTOFIRE) {
        frame(lua_state, 9.0);
        if is_excute(agent) {
            ArticleModule::generate_article(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
        }
        frame(lua_state, 11.0);
        if is_excute(agent) {
            ArticleModule::generate_article(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
        }
        frame(lua_state, 13.0);
        if is_excute(agent) {
            ArticleModule::generate_article(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
        }
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(agent, 40.0, 55.0, 14.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 55.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialnhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("peach_kinopio_hit"), Hash40::new("top"), 0, 7.5, 11.8, 0, 0, 0, 1.02, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::peach::instance::SPECIAL_N_AUTOFIRE) {
            EffectModule::kill_kind(boma, Hash40::new("peach_kinopio_hit"), true, true);
            COL_NORMAL(agent);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("peach_kinopio_hit"), true, true);
        COL_NORMAL(agent);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 10, 3, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, -0.85, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_specialsjump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 0.0, 0, 0, 0, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::enable_transition_term(boma, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT);
    }
}

//unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    // starts f1 instead of 0
//}

unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 1.25, 0, 0, 0, 0.6, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_landing_smoke"), -1);
    }
}

unsafe extern "C" fn game_specialairsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        // hit shield
        let hit_status = VarModule::get_int(agent.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
        if hit_status & *COLLISION_KIND_MASK_SHIELD != 0
        && hit_status & *COLLISION_KIND_MASK_PARRY != 1 {
            ATTACK(agent, 0, 0, Hash40::new("hip"), 4.0, 361, 20, 0, 20, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        agent.on_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_FLAG_START_CONTROLLER_MOVE);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_FLAG_DONE_CONTROLLER_MOVE);
    } // cancel into special landing frame
    frame(lua_state, 30.0);
    if is_excute(agent) {
        // uncap max speeds
        agent.on_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_FLAG_START_CONTROLLER_MOVE);
    }
}

unsafe extern "C" fn expression_specialairsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn game_specialshitend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 55, 80, 0, 60, 7.7, 0.0, 5.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HIP);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        let special_air_s_end_control_accel_x = agent.get_param_float("param_special_s", "special_air_s_end_control_accel_x");
        sv_kinetic_energy!(controller_set_accel_x_mul, agent, special_air_s_end_control_accel_x);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 6.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_start"), false, 1.0);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS);
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(agent, 0, 0, Hash40::new("head"), 3.0, 99, 100, 150, 0, 5.0, -1.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(agent, 1, 0, Hash40::new("havel"), 3.0, 118, 100, 150, 0, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::set_vector(boma, 0, 90, false);
        AttackModule::set_offset(boma, 0, &Vector3f{ x: 0.0, y: 0.0, z: 0.0});
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::set_vector(boma, 1, 90, false);
        AttackModule::set_offset(boma, 1, &Vector3f{ x: 0.0, y: 3.95, z: 0.0});
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("havel"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(agent, 0, 0, Hash40::new("havel"), 1.0, 367, 100, 70, 0, 3.6, 0.0, 3.75, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 97, 100, 125, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        let speed = KineticModule::get_sum_speed(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let pos_diff = speed*1.75; // see if angled links
        ATTACK(agent, 0, 0, Hash40::new("havel"), 1.0, 368, 100, 70, 0, 3.6, 0.0, 3.75, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        AttackModule::set_vector(boma, 1, 90, false);
        let hit1 = Vector2f { x: 2.0 + pos_diff, y: 16.5 };
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 9, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("havel"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(agent, 0, 0, Hash40::new("havel"), 4.0, 74, 120, 0, 60, 5.0, 0.0, 4.0, -2.1, Some(0.0), Some(4.0), Some(2.1), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 4.0, 74, 120, 0, 60, 3.0, 0.0, -1.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PARASOL);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn sound_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_peach_wear02"));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_peach_special_h01"));
    }
}

unsafe extern "C" fn expression_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 2);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialhiopen(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_open"), false, -1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("havel"), 3.0, 80, 40, 0, 30, 2.0, -3.1, 6.0, 0.0, Some(3.1), Some(6.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 18, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PEACH_PARASOL, *ATTACK_REGION_PARASOL);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let item_kind = agent.get_int(*FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND); 
    if is_excute(agent) {
        if item_kind == *ITEM_KIND_NONE {
            ArticleModule::generate_article(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_DAIKON, false, -1);
            let have_item = ItemModule::get_have_item_id(boma, 0) as u32;
            let have_item_boma = sv_battle_object::module_accessor(have_item);
            StatusModule::change_status_request_from_script(have_item_boma, *ITEM_STATUS_KIND_HAVE, true); // fix invisibility
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2508b59a2b), FIGHTER_ITEM_HOLD_KIND_GRIP);
        } else if item_kind == *ITEM_KIND_BOMBHEI {
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_BOMBHEI), 0, 0, false, false);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2508b59a2b), FIGHTER_ITEM_HOLD_KIND_GRIP); // make look better
        } else if item_kind == *ITEM_KIND_DOSEISAN {
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_DOSEISAN), 0, 0, false, false);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2508b59a2b), FIGHTER_ITEM_HOLD_KIND_GRIP); // make look better
        } else if item_kind == *ITEM_KIND_BEAMSWORD {
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_BEAMSWORD), 0, 0, false, false);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2508b59a2b), FIGHTER_ITEM_HOLD_KIND_PICKUP);
        }
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 40.0, 35.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if item_kind == *ITEM_KIND_BEAMSWORD {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2508b59a2b), FIGHTER_ITEM_HOLD_KIND_GRIP);
        }
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        if item_kind == *ITEM_KIND_BEAMSWORD {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2508b59a2b), FIGHTER_ITEM_HOLD_KIND_PICKUP); // prevent face clipping
        }
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("peach_hikkonuki"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("peach_hikkonuki"), -1);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_peach_special_l02"));
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_peach_special_l01"));
        if ItemModule::is_have_item(boma, 0) {
            let have_item = ItemModule::get_have_item_id(boma, 0) as u32;
            let have_item_boma = sv_battle_object::module_accessor(have_item);
            let turnip_power=WorkModule::get_int64(have_item_boma,*ITEM_PEACHDAIKON_INSTANCE_WORK_INT_ATTACK_POWER);
            if turnip_power >= 24 || turnip_power == 0 {
                PLAY_SE(agent, Hash40::new("vc_peach_appeal01"));
            }
        }
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        for _ in 0..5 {
            if !ItemModule::is_have_item(boma, 0) {
                ItemModule::pickup_item(boma, ItemSize{_address: *ITEM_SIZE_KIND_SMALL as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_IGNORE_GRASS as u8});
            }
            wait(lua_state, 1.0);
        }
    }
}

unsafe extern "C" fn effect_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        let facing = agent.lr();
        EffectModule::req_follow(boma, Hash40::new("sys_item_get"), Hash40::new("top"), &Vector3f::new(-3.0 * facing, 3.0, 3.0), &Vector3f::zero(), 0.5, false, 0, 0, 0, 0, 0, false, false);   
    }
}

unsafe extern "C" fn game_speciallwthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        let have_item = ItemModule::get_have_item_id(boma, 0) as u32;
        let have_item_boma = sv_battle_object::module_accessor(have_item);
        ItemModule::throw_item(boma, 63.0, 2.3, 1.0, 0, true, agent.get_float(*ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER));
        PostureModule::add_pos_2d(have_item_boma, &Vector2f {x: (0.5 * agent.lr()), y: 1.5});
    }
}

unsafe extern "C" fn effect_speciallwthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_speciallwthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_item_throw"));
    }
}

unsafe extern "C" fn expression_speciallwthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn, Priority::Low);
    agent.acmd("game_specialairn", game_specialn, Priority::Low);
    agent.acmd("effect_specialn", effect_specialn, Priority::Low);
    agent.acmd("effect_specialairn", effect_specialn, Priority::Low);

    agent.acmd("game_specialnhit", game_specialnhit, Priority::Low);
    agent.acmd("game_specialairnhit", game_specialnhit, Priority::Low);

    agent.acmd("effect_specialnhit", effect_specialnhit, Priority::Low);
    agent.acmd("effect_specialairnhit", effect_specialnhit, Priority::Low);


    agent.acmd("effect_specialsstart", effect_specialsstart, Priority::Low);

    agent.acmd("game_specialsjump", game_specialsjump, Priority::Low);

    agent.acmd("game_specialsend", acmd_stub, Priority::Low);
    agent.acmd("game_specialairsend", game_specialairsend, Priority::Low);
    agent.acmd("expression_specialairsend", expression_specialairsend, Priority::Low);

    agent.acmd("game_specialshitend", game_specialshitend, Priority::Low);


    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialairhistart", game_specialhistart, Priority::Low);
    agent.acmd("sound_specialhistart", sound_specialhistart, Priority::Low);
    agent.acmd("sound_specialairhistart", sound_specialhistart, Priority::Low);
    agent.acmd("expression_specialhistart", expression_specialhistart, Priority::Low);
    agent.acmd("expression_specialairhistart", expression_specialhistart, Priority::Low);

    agent.acmd("game_specialhiopen", game_specialhiopen, Priority::Low);
    

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("sound_speciallw", sound_speciallw, Priority::Low);

    agent.acmd("game_specialairlw", game_specialairlw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_specialairlw, Priority::Low);
    agent.acmd("sound_specialairlw", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairlw", expression_speciallwthrow, Priority::Low);

    agent.acmd("game_speciallwthrow", game_speciallwthrow, Priority::Low);
    agent.acmd("effect_speciallwthrow", effect_speciallwthrow, Priority::Low);
    agent.acmd("sound_speciallwthrow", sound_speciallwthrow, Priority::Low);
    agent.acmd("expression_speciallwthrow", expression_speciallwthrow, Priority::Low);
}