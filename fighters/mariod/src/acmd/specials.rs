use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 15.0, 8.0);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, 0);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.46, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_n01"));
    }
}

unsafe extern "C" fn game_specialnchill(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 69, 84, 0, 42, 3.5, 0.0, 6.5, 4.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 69, 84, 0, 42, 4.75, 0.0, 4.0, 7.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE_RANGE(agent, 19.0, 43.0, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 43.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialnchill(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 8, 11, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FLASH(agent, 0.5, 0.25, 1, 0.35);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 4, 7, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_ice_landing"), Hash40::new("top"), 0, 4, 7, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(agent, 0.75);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FLIP(agent, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), 0, 4, 7, 0, 0, 0, 0.46, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 4, 7, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_specialnchill(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_frieze_l"));
        PLAY_SE(agent, Hash40::new("vc_mariod_012"));
    }
}

unsafe extern "C" fn expression_specialnchill(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_55_smash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, false, 0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    /*frame(lua_state, 6.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIOD_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }*/
    frame(lua_state, 6.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        //search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 110, 100, 80, 0, 4.5, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 110, 100, 80, 0, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 35, 100, 45, 0, 10, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(13.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE_RANGE(agent, 23.0, 46.0, 13.0);
    if is_excute(agent) {
        let motion_rate = 13.0/(46.0-23.0);
        ArticleModule::set_rate(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, 1.0/motion_rate);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 46.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.2);
        if PostureModule::lr(boma) > 0.0{
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), 1, 7.5, 5, 0, 35, 0, 1.2, true, *EF_FLIP_NONE);
        }
        else{
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), -1, 7.5, 5, 0, -35, 0, 1.2, true, *EF_FLIP_NONE);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("mariod_supermant_flash"), Hash40::new("top"), 0, 5.5, 7.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_impact"), true, true);
    }
}

unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, false, 0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    /*frame(lua_state, 6.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIOD_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }*/
    frame(lua_state, 6.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        //search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::mariod::instance::SPECIAL_S_DISABLE_STALL) {
            WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
            VarModule::on_flag(agent.battle_object, vars::mariod::instance::SPECIAL_S_DISABLE_STALL);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 110, 100, 80, 0, 4.5, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 110, 100, 80, 0, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 35, 100, 45, 0, 10, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(13.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE_RANGE(agent, 23.0, 46.0, 13.0);
    if is_excute(agent) {
        let motion_rate = 13.0/(46.0-23.0);
        ArticleModule::set_rate(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, 1.0/motion_rate);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 46.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), 1, 7.5, 5, 0, 35, 0, 1.2, true, *EF_FLIP_NONE);
        }
        else{
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), -1, 7.5, 5, 0, -35, 0, 1.2, true, *EF_FLIP_NONE);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("mariod_supermant_flash"), Hash40::new("top"), 0, 5.5, 7.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_supermant_wind_r"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_supermant_wind_l"), -1);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_impact"), true, true);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
        VarModule::off_flag(agent.object(), vars::mariod::instance::UP_SPECIAL_CANCEL);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 70, 100, 125, 0, 5.0, 0.0, 6.0, 9.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if boma.is_stick_backward() {
            VarModule::on_flag(agent.object(), vars::mariod::instance::UP_SPECIAL_CANCEL);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
        }
        else {
            ATTACK(agent, 0, 1, Hash40::new("top"), 14.0, 50, 89, 0, 33, 6.0, 0.0, 6.0, 9.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);    
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 7.0, 74, 66, 0, 64, 4.0, 0.0, 6.5, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_aura"), Hash40::new("mariod_smash_aura"), Hash40::new("havel"), -1.5, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0 {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 0, 1, 0, 0, 0, 1.05, true);
        }
        else{
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 1, 0, 0, 0, 0, 1.05, true);
        }
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("havel"), 1.0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 0.75);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_power"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1.45, true);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_superjump_fnish"), -1);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_smash_impact"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_smash_aura"), -1);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_superjump_power"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_aura"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_impact"), false, true);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        ATTACK(agent, 0, 1, Hash40::new("top"), 14.0, 50, 89, 0, 33, 6.0, 0.0, 6.0, 9.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if boma.is_stick_backward() {
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        ATTACK(agent, 0, 1, Hash40::new("top"), 7.0, 74, 66, 0, 64, 4.0, 0.0, 6.5, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
    }
}

unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_aura"), Hash40::new("mariod_smash_aura"), Hash40::new("havel"), -1.5, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 0, 1, 0, 0, 0, 1.05, true);
        }
        else{
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 1, 0, 0, 0, 0, 1.05, true);
        }
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("havel"), 1.0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 0.75);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_power"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1.45, true);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_superjump_fnish"), -1);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_smash_impact"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_smash_aura"), -1);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_superjump_power"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_aura"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_impact"), false, true);
    }
}

unsafe extern "C" fn game_landingfallspecial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if VarModule::is_flag(agent.object(), vars::mariod::instance::UP_SPECIAL_CANCEL) {
        FT_MOTION_RATE_RANGE(agent, 1.0, 29.0, 24.0);
    }
    frame(lua_state, 29.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 7.0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.6, 90, 100, 80, 0, 4.0, 0.0, 3.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 4, 0, Hash40::new("top"), 1.6, 105, 100, 45, 0, 4.5, 0.0, 9.0, -6.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 366, 100, 15, 0, 4.5, 0.0, 9.0, -6.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.6, 105, 100, 45, 0, 4.5, 0.0, 9.0, 6.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.6, 366, 100, 15, 0, 4.5, 0.0, 9.0, 6.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 46, 154, 0, 80, 6.0, 0.0, 12.0, 6.0, Some(0.0), Some(12.0), Some(-6.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 46, 154, 0, 80, 5.5, 0.0, 4.0, 2.5, Some(0.0), Some(4.0), Some(-2.5), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    wait(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_r"), Hash40::new("mariod_cyclone_l"), Hash40::new("top"), 0, 1, 0, 0, 180, 0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_l"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 1.25, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_impact"), Hash40::new("mariod_cyclone_impact"), Hash40::new("top"), 3, 9.5, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_cyclone_impact"), -1);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_impact"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_impact"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_aura"), false, true);
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.6, 90, 100, 80, 0, 4.0, 0.0, 3.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 4, 0, Hash40::new("top"), 1.6, 105, 100, 45, 0, 4.5, 0.0, 9.0, -6.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 366, 100, 15, 0, 4.5, 0.0, 9.0, -6.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.6, 105, 100, 45, 0, 4.5, 0.0, 9.0, 6.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.6, 366, 100, 15, 0, 4.5, 0.0, 9.0, 6.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 46, 154, 0, 80, 6.0, 0.0, 12.0, 6.0, Some(0.0), Some(12.0), Some(-6.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 46, 154, 0, 80, 5.5, 0.0, 4.0, 2.5, Some(0.0), Some(4.0), Some(-2.5), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    wait(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_r"), Hash40::new("mariod_cyclone_l"), Hash40::new("top"), 0, -1.5, 0, 0, 180, 0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_l"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 1.25, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_impact"), Hash40::new("mariod_cyclone_impact"), Hash40::new("top"), 3, 9.5, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_cyclone_impact"), -1);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_impact"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_impact"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_smash_aura"), false, true);
    }

}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn);
    agent.acmd("game_specialairn", game_specialn);
    agent.acmd("effect_specialn", effect_specialn);
    agent.acmd("effect_specialairn", effect_specialn);
    agent.acmd("sound_specialn", sound_specialn);
    agent.acmd("sound_specialairn", sound_specialn);

    agent.acmd("game_specialnchill", game_specialnchill);
    agent.acmd("game_specialairnchill", game_specialnchill);
    agent.acmd("effect_specialnchill", effect_specialnchill);
    agent.acmd("effect_specialairnchill", effect_specialnchill);
    agent.acmd("sound_specialnchill", sound_specialnchill);
    agent.acmd("sound_specialairnchill", sound_specialnchill);
    agent.acmd("expression_specialnchill", expression_specialnchill);
    agent.acmd("expression_specialairnchill", expression_specialnchill);

    agent.acmd("game_specials", game_specials);
    agent.acmd("effect_specials", effect_specials);
    agent.acmd("game_specialairs", game_specialairs);
    agent.acmd("effect_specialairs", effect_specialairs);

    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("effect_specialhi", effect_specialhi);
    agent.acmd("game_specialairhi", game_specialairhi);
    agent.acmd("effect_specialairhi", effect_specialairhi);

    agent.acmd("game_landingfallspecial", game_landingfallspecial);
    
    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("effect_speciallw", effect_speciallw);
    agent.acmd("game_specialairlw", game_specialairlw);
    agent.acmd("effect_specialairlw", effect_specialairlw);
}
