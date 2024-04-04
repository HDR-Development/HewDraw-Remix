use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, 0);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_fb_shoot"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
        FLASH(agent, 0, 1, 0, 0.353);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("luigi_fb_shoot"), false, false);
    }
}

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_special_n01"));
    }
}

unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialnthunder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 68, 55, 0, 65, 5.0, 0.0, 9.0, 8.0, None, None, None, 0.45, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 68, 55, 0, 65, 3.0, 0.0, 9.0, 2.0, None, None, None, 0.45, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialnthunder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 5, 15, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        let mut rand = &Vector3f::new(
            app::sv_math::rand(hash40("fighter"), 50) as f32,
            app::sv_math::rand(hash40("stage"), 50) as f32,
            app::sv_math::rand(hash40("luigi"), 50) as f32
        );
        let mut flip = &Vector3f::new(
            if app::sv_math::rand(hash40("fighter"), 1) == 0 { -1 } else { 1 } as f32,
            if app::sv_math::rand(hash40("stage"), 1) == 0 { -1 } else { 1 } as f32,
            if app::sv_math::rand(hash40("luigi"), 1) == 0 { -1 } else { 1 } as f32
        );
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 9.0, 8.0, 0.0 + (rand.x * flip.x), 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 9.0, 8.0, 120.0 + (rand.y * flip.y), 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 9.0, 8.0, 240.0 + (rand.z * flip.z), 0, 0, 0.5, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0.25, 1.0, 0.7);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 9.0, 8.0, 0, 90, 90, 0.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0.0, 9.0, 8.0, 0, 90, 90, 0.5, true);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_specialnthunder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_luigi_attack07"));
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_l"));
    }
}

unsafe extern "C" fn expression_specialnthunder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_55_smash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialairsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 {
            KineticModule::mul_speed(boma, &Vector3f{x: 1.0, y: 0.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        //this runs here instead of the status script because it would break misfire's code otherwise
        if agent.is_situation(*SITUATION_KIND_AIR) {
            VarModule::on_flag(agent.battle_object, vars::luigi::instance::DISABLE_SPECIAL_S);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_FLAG_REVERSE_LR);
    }
}

unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 2, 15, 5, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        if VarModule::get_int(agent.object(), vars::luigi::instance::REMAINING_SPECIAL_S_UNTIL_MISFIRE) <= 0 {
            LAST_EFFECT_SET_SCALE_W(agent, 0.8, 0.8, 0.8);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
    }
}

unsafe extern "C" fn effect_specialshold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 6, 0,  0, 1, 0, 1, true);
        let handle = EffectModule::get_last_handle(boma) as u32;
        VarModule::set_int(agent.battle_object, vars::luigi::instance::CHARGE_PULSE_EFFECT_HANDLE, handle as i32);
        if WorkModule::is_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.8, 0.0);
        }
    }
    for _ in 0..100 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 10, 00, 4, 0, 0, 0, false);
            let handle = EffectModule::get_last_handle(boma) as u32;
            VarModule::set_int(agent.battle_object, vars::luigi::instance::CHARGE_SMOKE_EFFECT_HANDLE, handle as i32);
            if WorkModule::is_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
                LAST_EFFECT_SET_COLOR(agent, 0.0, 0.7, 0.3);
            }
        }
        wait(lua_state, 10.0);
    }
}

unsafe extern "C" fn effect_specialairshold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 6, 0,  0, 1, 0, 1, true);
        let handle = EffectModule::get_last_handle(boma) as u32;
        VarModule::set_int(agent.battle_object, vars::luigi::instance::CHARGE_PULSE_EFFECT_HANDLE, handle as i32);
        if WorkModule::is_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.8, 0.0);
        }
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, 1.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, 1.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        //WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
}

unsafe extern "C" fn game_specialsdischarge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let misfire_multiplier = VarModule::get_float(agent.battle_object, vars::luigi::instance::MISFIRE_DAMAGE_MULTIPLIER);
    VarModule::set_float(agent.battle_object, vars::luigi::instance::MISFIRE_DAMAGE_MULTIPLIER, 1.0);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 25.0 * misfire_multiplier, 361, 100, 0, 20, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("neck"), 25.0 * misfire_multiplier, 361, 100, 0, 20, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
}

unsafe extern "C" fn effect_specialsdischarge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("luigi_rocket_bomb"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.8, 0.0);
    }

    frame(lua_state, 3.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("luigi_rocket_jet2"), Hash40::new("top"), 0, 4, 4, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.7, 0.3);
        EffectModule::enable_sync_init_pos_last(boma);
    }
}

unsafe extern "C" fn game_specialairsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 90, 80, 0, 50, 2.2, 1.2, 6.0, 7.0, Some(-1.2), Some(6.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 90, 70, 0, 50, 2.7, 0.0, 6.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        //WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 70, 80, 0, 50, 5.5, 0.0, 10.0, -5.5, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 70, 80, 0, 50, 5.5, 0.0, 10.0, 5.5, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 80, 0, 50, 6.0, 0.0, 5.5, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.6);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 70, 100, 0, 50, 7.5, 0.0, 11.0, -9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 70, 100, 0, 50, 7.5, 0.0, 11.0, 9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 70, 100, 0, 50, 6.5, 0.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        //KineticModule::mul_speed(boma, &Vector3f{x: 0.0, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        //KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        //WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.4, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.4, 0, 0, 180, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.4, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 11.4, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.4, 0, 0, 180, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 11.4, 0, 0, 180, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.4, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.4, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.4, 0, 0, 180, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.4, 0, 0, 180, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.4, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 7.4, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.4, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 7.4, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);

        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, 9.0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, -9.0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, 4.5, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, -4.5, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("luigi_cyclone_finish"), Hash40::new("top"), 0, 7, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);

        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.5, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 3.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.5, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 3.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 {
            KineticModule::mul_speed(boma, &Vector3f{x: 1.0, y: 0.1, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        //WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 9.5, 5.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 9.5, -5.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 2.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        //ATTACK(fighter, 4, 1, Hash40::new("top"), 0.0, 180, 100, 45, 0, 15.5, 0.0, 8.5, 0.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        let hitVec = Vector2f{ x: 0.0, y: 8.0 };
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hitVec, 8, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &hitVec, 8, false);
        AttackModule::set_vec_target_pos(boma, 2, Hash40::new("top"), &hitVec, 8, false);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.6);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 70, 130, 0, 85, 7.5, 0.0, 11.0, -9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 70, 130, 0, 85, 7.5, 0.0, 11.0, 9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 70, 130, 0, 85, 6.5, 0.0, 2.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        //WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.4, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.4, 0, 0, 180, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.4, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 11.4, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.4, 0, 0, 180, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 11.4, 0, 0, 180, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.4, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.4, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.4, 0, 0, 180, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.4, 0, 0, 180, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.4, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 7.4, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.4, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 7.4, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);

        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, 9.0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, -9.0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, 4.5, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, -4.5, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("luigi_cyclone_finish"), Hash40::new("top"), 0, 7, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);

        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.5, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 3.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.5, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 3.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn);
    agent.acmd("game_specialairn", game_specialn);
    agent.acmd("effect_specialn", effect_specialn);
    agent.acmd("effect_specialairn", effect_specialn);
    agent.acmd("sound_specialn", sound_specialn);
    agent.acmd("sound_specialairn", sound_specialn);
    agent.acmd("expression_specialn", expression_specialn);
    agent.acmd("expression_specialairn", expression_specialn);

    agent.acmd("game_specialnthunder", game_specialnthunder);
    agent.acmd("game_specialairnthunder", game_specialnthunder);
    agent.acmd("effect_specialnthunder", effect_specialnthunder);
    agent.acmd("effect_specialairnthunder", effect_specialnthunder);
    agent.acmd("sound_specialnthunder", sound_specialnthunder);
    agent.acmd("sound_specialairnthunder", sound_specialnthunder);
    agent.acmd("expression_specialnthunder",expression_specialnthunder);
    agent.acmd("expression_specialairnthunder",expression_specialnthunder);

    agent.acmd("game_specialairsstart", game_specialairsstart);
    agent.acmd("effect_specialsstart", effect_specialsstart);
    agent.acmd("effect_specialairsstart", effect_specialsstart);
    agent.acmd("effect_specialshold", effect_specialshold);
    agent.acmd("effect_specialairshold", effect_specialairshold);
    agent.acmd("game_specials", game_specials);
    agent.acmd("game_specialsdischarge", game_specialsdischarge);
    agent.acmd("effect_specialsdischarge", effect_specialsdischarge);
    agent.acmd("game_specialairsend", game_specialairsend);

    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("game_specialairhi", game_specialairhi);

    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("effect_speciallw", effect_speciallw);
    agent.acmd("game_specialairlw", game_specialairlw);
    agent.acmd("effect_specialairlw", effect_specialairlw);
}
