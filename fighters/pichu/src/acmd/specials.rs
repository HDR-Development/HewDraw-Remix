use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = if agent.kind() == *FIGHTER_KIND_KIRBY {false} else {VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1};
    let charge_state_time = if agent.kind() == *FIGHTER_KIND_KIRBY {1} else {ParamModule::get_int(boma.object(), ParamType::Agent, "charge_state_time")};
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
        if !charged {
            FT_MOTION_RATE(agent, (14.0/18.0));
        }
        else if charged {
            VarModule::on_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
            VarModule::sub_int(agent.battle_object, vars::common::instance::GIMMICK_TIMER, 180);
            MeterModule::drain_direct(boma.object(), (50.0/(charge_state_time as f32)) * 180.0);
        }
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PICHU_GENERATE_ARTICLE_DENGEKIDAMA, false, -1);
        if !VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            MeterModule::add(agent.battle_object, 2.0);
            FT_ADD_DAMAGE(agent, 1.0);
        }
        else {
            FT_ADD_DAMAGE(agent, 3.0);
        }
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    let charge_state_time = ParamModule::get_int(boma.object(), ParamType::Agent, "charge_state_time");
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
        if charged {
            VarModule::on_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
            VarModule::sub_int(agent.battle_object, vars::common::instance::GIMMICK_TIMER, 120);
            MeterModule::drain_direct(boma.object(), (50.0/(charge_state_time as f32)) * 120.0);
        }
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_ATTACK_TRIGGER);
        if !VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            FT_ADD_DAMAGE(agent, 1.0);
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 68, 55, 0, 70, 3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_CALC_ATTACK_POWER);
        }
        else {
            FT_ADD_DAMAGE(agent, 3.0);
            FT_DESIRED_RATE(agent, 18.0, 12.0);
            ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 40, 65, 0, 60, 3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            wait(lua_state, 9.0);
            ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 40, 65, 0, 60, 3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        }
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
    }
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("pichu_rocket_bomb"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 5.0);        
    if !VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
        if is_excute(agent) {
            EFFECT_FLW_POS(agent, Hash40::new("pichu_rocket_aura"), Hash40::new("head"), 6.5, 0, 0, 90, 0, 0, 0.9, true);
            EffectModule::enable_sync_init_pos_last(boma);
            LAST_EFFECT_SET_COLOR(agent, 0.85, 0.9, 1);
            EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("pichu_rocket_jet"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 0.8 as u64, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FLW_POS(agent, Hash40::new("pichu_rocket_aura_max"), Hash40::new("head"), 6.5, 0, 0, 90, 0, 0, 1.3, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("pichu_rocket_jet"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1.25 as u64, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_final_sphere_start"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.3, true);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FLASH(agent, 0.08, 0.661, 1, 0.471);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0.392);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    wait(lua_state, 1.0);
}

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            FT_DESIRED_RATE(agent, 26.0, 33.0);
        }
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_specialairsmissend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            FT_DESIRED_RATE(agent, 26.0, 33.0);
        }
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_MISS_END_RUMBLE_2);
    }
}

unsafe extern "C" fn game_specialhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    let charge_state_time = ParamModule::get_int(boma.object(), ParamType::Agent, "charge_state_time");
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
        if charged {
            VarModule::on_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
            VarModule::sub_int(agent.battle_object, vars::common::instance::GIMMICK_TIMER, 120);
            MeterModule::drain_direct(boma.object(), (50.0/(charge_state_time as f32)) * 120.0);
        }
        if VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            ATTACK(agent, 0, 0, Hash40::new("neck"), 2.0, 70, 50, 0, 20, 1.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            FT_ADD_DAMAGE(agent, 1.0);
        }
        else {
            FT_ADD_DAMAGE(agent, 0.5);
        }
        JostleModule::set_status(boma, false);
    }
}

unsafe extern "C" fn expression_specialhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), false);
        MotionModule::set_helper_calculation(boma, false);
        if VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        MotionModule::set_helper_calculation(boma, true);
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            ATTACK(agent, 0, 0, Hash40::new("neck"), 3.0, 70, 150, 0, 20, 1.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            FT_ADD_DAMAGE(agent, 2.0);
        }
        else {
            FT_ADD_DAMAGE(agent, 1.0);
        }
        JostleModule::set_status(boma, false);
    }
}

unsafe extern "C" fn expression_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), false);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        MotionModule::set_helper_calculation(boma, false);
        if VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        MotionModule::set_helper_calculation(boma, true);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = VarModule::get_int(agent.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
        if !charged {
            VarModule::off_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_KAMINARI_GENERATE);
        }
        else {
            let charge_state_time = ParamModule::get_int(boma.object(), ParamType::Agent, "charge_state_time") as f32;
            let charge_state_remaining = VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) as f32;
            // 5 seconds to use full strength Discharge before it starts decreasing in power
            let discharge_decrease_power_frame = charge_state_time - 300.0;
            // 50% damage at minimum
            let discharge_min_power_mul = 0.5;
            let discharge_power_mul = 1.0 - ((1.0 - (charge_state_remaining.min(discharge_decrease_power_frame)/discharge_decrease_power_frame)) * (1.0 - discharge_min_power_mul));
            VarModule::set_float(boma.object(), vars::pichu::instance::DISCHARGE_POWER_MUL, discharge_power_mul);
            // 75% size at minimum
            let discharge_min_size_mul = 0.75;
            let discharge_size_mul = 1.0 - ((1.0 - (charge_state_remaining.min(discharge_decrease_power_frame)/discharge_decrease_power_frame)) * (1.0 - discharge_min_size_mul));
            VarModule::set_float(boma.object(), vars::pichu::instance::DISCHARGE_SIZE_MUL, discharge_size_mul);
            VarModule::on_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK);
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
            MeterModule::drain_direct(boma.object(), 999.0);
        }
    }
    if VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
        frame(lua_state, 7.0);
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
        frame(lua_state, 15.0);
        FT_DESIRED_RATE(agent, 6.0, 3.0);
        if is_excute(agent) {
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        }
        frame(lua_state, 21.0);
        if is_excute(agent) {
            boma.change_status_req(*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT, true);
        }
    }
}

unsafe extern "C" fn game_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let discharge_power_mul = VarModule::get_float(boma.object(), vars::pichu::instance::DISCHARGE_POWER_MUL);
    let discharge_size_mul = VarModule::get_float(boma.object(), vars::pichu::instance::DISCHARGE_SIZE_MUL);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            MeterModule::watch_damage(agent.battle_object, true);
            MeterModule::add(agent.battle_object, 2.0);
            FT_ADD_DAMAGE(agent, 3.5);
            ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 71, 0, 90, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        else {
            FT_ADD_DAMAGE(agent, 8.0 * discharge_power_mul);
            ATTACK(agent, 0, 0, Hash40::new("top"), 20.0 * discharge_power_mul, 361, 80, 0, 70, 18.0 * discharge_size_mul, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        if VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            FT_MOTION_RATE(agent, 1.5 * discharge_power_mul);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) && agent.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
}

unsafe extern "C" fn effect_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 1.0);
    WorkModule::is_flag(boma, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_KAMINARI_ATTACK_HIT);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            let discharge_effect_mul = VarModule::get_float(boma.object(), vars::pichu::instance::DISCHARGE_SIZE_MUL);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_kaminari_hit2"), Hash40::new("hip"), 0, 0, 0, 0, 90, 0, 1.15 * discharge_effect_mul, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_final_explosion"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.3 * discharge_effect_mul, false);
        }
        else {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_kaminari_hit2"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 1.15, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_kaminari_hit"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 0.85, true);
        }
    }
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0);
        BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pichu_kaminari_hit2"), false, true);
    }
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0);
        BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
    }
    wait(lua_state, 1.0);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pichu_kaminari_hit"), false, true);
    }
    for _ in 0..3{
        if is_excute(agent) {
            FLASH(agent, 0, 0, 0, 0);
            BURN_COLOR(agent, 2, 2, 0.5, 0.9);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            FLASH_FRM(agent, 2, 0, 0, 0, 0);
            BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            FLASH_FRM(agent, 2, 0, 0, 0, 0);
            BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            BURN_COLOR_NORMAL(agent);
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0);
        BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek"), false, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairlwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
    }
    frame(lua_state, 1.0);
    WorkModule::is_flag(boma, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_KAMINARI_ATTACK_HIT);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
            let discharge_effect_mul = VarModule::get_float(boma.object(), vars::pichu::instance::DISCHARGE_SIZE_MUL);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_kaminari_hit2"), Hash40::new("hip"), 0, 0, 0, 0, 90, 0, 1.15 * discharge_effect_mul, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_final_explosion"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.3 * discharge_effect_mul, false);
        }
        else {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_kaminari_hit2"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 1.15, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_kaminari_hit"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 0.85, true);
        }
    }
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0);
        BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pichu_kaminari_hit2"), false, true);
    }
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0);
        BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
    }
    wait(lua_state, 1.0);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pichu_kaminari_hit"), false, true);
    }
    for _ in 0..3{
        if is_excute(agent) {
            FLASH(agent, 0, 0, 0, 0);
            BURN_COLOR(agent, 2, 2, 0.5, 0.9);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            FLASH_FRM(agent, 2, 0, 0, 0, 0);
            BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            FLASH_FRM(agent, 2, 0, 0, 0, 0);
            BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            BURN_COLOR_NORMAL(agent);
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0);
        BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek"), false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn);
    agent.acmd("game_specialairn", game_specialn);

    agent.acmd("game_specials", game_specials);
    agent.acmd("effect_specials", effect_specials);
    agent.acmd("game_specialsend", game_specialsend);
    agent.acmd("game_specialairsend", game_specialsend);
    agent.acmd("game_specialairsmissend", game_specialairsmissend);

    agent.acmd("game_specialhi1", game_specialhi1);
    agent.acmd("game_specialairhi1", game_specialhi1);
    agent.acmd("expression_specialhi1", expression_specialhi1);
    agent.acmd("expression_specialairhi1", expression_specialhi1);
    agent.acmd("game_specialhi2", game_specialhi2);
    agent.acmd("game_specialairhi2", game_specialhi2);
    agent.acmd("expression_specialhi2", expression_specialhi2);
    agent.acmd("expression_specialairhi2", expression_specialhi2);

    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("game_specialairlw", game_speciallw);
    agent.acmd("game_speciallwhit", game_speciallwhit);
    agent.acmd("game_specialairlwhit", game_speciallwhit);
    agent.acmd("effect_speciallwhit", effect_speciallwhit);
    agent.acmd("effect_specialairlwhit", effect_specialairlwhit);
}
