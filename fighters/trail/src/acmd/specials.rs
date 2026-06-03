use super::*;

unsafe extern "C" fn game_specialn3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, 0, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_FLAG_CHANGE_MAGIC);
    }
    wait(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, 1, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, 0);
    }
    wait(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, 2, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, 0);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
}

unsafe extern "C" fn game_specialairn3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma,  *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, 0, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_FLAG_CHANGE_MAGIC); 
    }
    wait(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, 1, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, 0);
    }
    wait(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, 2, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, 0);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn game_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma,  *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
        ArticleModule::remove_exist(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER, app::ArticleOperationTarget(0));
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER,  Hash40::new("special_n2"), false, 0.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.7);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 78, 60, 0, 70, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 78, 60, 0, 70, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 90.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
    }
}

unsafe extern "C" fn effect_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_ice_hold"), Hash40::new("haver"), 0, 10, -1, -90, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("trail_ice_sword_flare"), Hash40::new("haver"), 0, 10, -1, -90, 0, 0, 1, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), 0, 3.7, 0, 0, 0, 180, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 0.25);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), 0, 3.7, 0, 0, 180, 180, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 0.25);
    }
}

unsafe extern "C" fn sound_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_trail_special_n_b01"));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_trail_special_n02"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_trail_special_n_b02"));
    }
}

unsafe extern "C" fn expression_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_81_special_n2"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 43.0);
    if ItemModule::is_have_item(boma, 0) {
        if is_excute(agent) {
            //FT_MOTION_INTP_WAIT_ITEM(agent, true, 0.05);
        }
    }
    frame(lua_state, 49.0);
    if is_excute(agent) {
        //FT_MOTION_INTP_WAIT_ITEM(agent, true, 0.08);
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        //FT_MOTION_INTP_WAIT_ITEM(agent, true, 0);
    }
}

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 72, 10, 0, 120, 3.2, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 72, 10, 0, 120, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        KineticModule::add_speed(boma, &Vector3f::new(-0.5, 0.0, 0.0));
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

unsafe extern "C" fn game_specialairs1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 72, 10, 0, 120, 3.2, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 72, 10, 0, 120, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        KineticModule::add_speed(boma, &Vector3f::new(-1.0, 0.0, 0.0));
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

unsafe extern "C" fn sound_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        let rng = sv_math::rand(hash40("fighter"), 3) as i32;
        if rng == 2 {
            PLAY_SE(agent, Hash40::new("vc_trail_special_s01"));
        } else {
            PLAY_SEQUENCE(agent, Hash40::new("seq_trail_rnd_attack03"));
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_trail_special_s02"));
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_HI_FLAG_JUMP_START);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.8, 86, 100, 116, 0, 4.2, 0.0, 4.2, 7.6, Some(0.0), Some(4.2), Some(7.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.8, 88, 100, 82, 0, 4.2, 0.0, 12.6, 7.6, Some(0.0), Some(12.6), Some(7.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.8, 110, 100, 120, 0, 4.2, 0.0, 4.2, 16.8, Some(0.0), Some(4.2), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.8, 108, 100, 86, 0, 4.2, 0.0, 12.6, 16.8, Some(0.0), Some(12.6), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 3.8, 94, 100, 130, 0, 4.2, 0.0, 4.2, 7.6, Some(0.0), Some(4.2), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("top"), 3.8, 92, 100, 76, 0, 4.2, 0.0, 12.6, 7.6, Some(0.0), Some(12.6), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_finish_camera(boma, 4, true, false);
        AttackModule::set_no_finish_camera(boma, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 80, 100, 124, 0, 2.4, 0.0, 7.6, -6.6, Some(0.0), Some(7.6), Some(-6.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 82, 100, 70, 0, 2.4, 0.0, 12.2, -6.6, Some(0.0), Some(12.2), Some(-6.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.1, 106, 100, 122, 0, 2.4, 0.0, 7.6, -12.2, Some(0.0), Some(7.6), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 2.1, 108, 100, 78, 0, 2.4, 0.0, 12.2, -12.2, Some(0.0), Some(12.2), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 2.1, 94, 100, 114, 0, 2.4, 0.0, 7.6, -6.6, Some(0.0), Some(7.6), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("top"), 2.1, 92, 100, 68, 0, 2.4, 0.0, 12.2, -6.6, Some(0.0), Some(12.2), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_finish_camera(boma, 4, true, false);
        AttackModule::set_no_finish_camera(boma, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 76, 100, 92, 0, 2.8, 0.0, 4.6, 6.4, Some(0.0), Some(4.6), Some(6.4), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 78, 100, 62, 0, 2.8, 0.0, 10.0, 6.4, Some(0.0), Some(10.0), Some(6.4), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.1, 108, 100, 100, 0, 2.8, 0.0, 4.6, 14.6, Some(0.0), Some(4.6), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 2.1, 106, 100, 72, 0, 2.8, 0.0, 10.0, 14.6, Some(0.0), Some(10.0), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 2.1, 92, 100, 110, 0, 2.8, 0.0, 4.6, 6.4, Some(0.0), Some(4.6), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("top"), 2.1, 90, 100, 58, 0, 2.8, 0.0, 10.0, 6.4, Some(0.0), Some(10.0), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_finish_camera(boma, 4, true, false);
        AttackModule::set_no_finish_camera(boma, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 88, 100, 92, 0, 2.4, 0.0, 8.8, -6.6, Some(0.0), Some(8.8), Some(-6.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 84, 100, 46, 0, 2.4, 0.0, 13.4, -6.6, Some(0.0), Some(13.4), Some(-6.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.1, 102, 100, 88, 0, 2.4, 0.0, 8.8, -12.2, Some(0.0), Some(8.8), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 2.1, 106, 100, 48, 0, 2.4, 0.0, 13.4, -12.2, Some(0.0), Some(13.4), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 2.1, 92, 100, 90, 0, 2.4, 0.0, 8.8, -6.6, Some(0.0), Some(8.8), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("top"), 2.1, 90, 100, 48, 0, 2.4, 0.0, 13.4, -6.6, Some(0.0), Some(13.4), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_finish_camera(boma, 4, true, false);
        AttackModule::set_no_finish_camera(boma, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 74, 100, 90, 0, 2.8, 0.0, 7.2, 6.4, Some(0.0), Some(7.2), Some(6.4), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 72, 100, 72, 0, 2.8, 0.0, 12.6, 6.4, Some(0.0), Some(12.6), Some(6.4), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.1, 98, 100, 92, 0, 2.8, 0.0, 7.2, 14.6, Some(0.0), Some(7.2), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 2.1, 96, 100, 72, 0, 2.8, 0.0, 12.6, 14.6, Some(0.0), Some(12.6), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 2.1, 94, 100, 102, 0, 2.8, 0.0, 7.2, 6.4, Some(0.0), Some(7.2), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("top"), 2.1, 92, 100, 62, 0, 2.8, 0.0, 12.6, 6.4, Some(0.0), Some(12.6), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_finish_camera(boma, 4, true, false);
        AttackModule::set_no_finish_camera(boma, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 125, 100, 130, 0, 2.4, 0.0, 8.2, -6.6, Some(0.0), Some(8.2), Some(-6.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 150, 100, 110, 0, 2.4, 0.0, 12.8, -6.6, Some(0.0), Some(12.8), Some(-6.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.1, 130, 100, 150, 0, 2.4, 0.0, 8.2, -12.2, Some(0.0), Some(8.2), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 2.1, 150, 100, 150, 0, 2.4, 0.0, 12.8, -12.2, Some(0.0), Some(12.8), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 2.1, 130, 100, 130, 0, 2.4, 0.0, 8.2, -6.6, Some(0.0), Some(8.2), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("top"), 2.1, 150, 100, 130, 0, 2.4, 0.0, 12.8, -6.6, Some(0.0), Some(12.8), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_finish_camera(boma, 4, true, false);
        AttackModule::set_no_finish_camera(boma, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 4.6, 62, 180, 0, 44, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 4.6, 62, 180, 0, 44, 3.6, 0.0, 4.6, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 4.6, 62, 180, 0, 44, 3.6, 0.0, 9.2, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 4.6, 62, 180, 0, 44, 5.2, 0.0, 7.4, 8.4, Some(0.0), Some(9.6), Some(8.4), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 4.6, 62, 180, 0, 44, 4.8, 0.0, 12.6, 15.6, Some(0.0), Some(14.8), Some(15.6), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 48.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_HI_FLAG_COMMAND_ACCEPT);
    }
    frame(lua_state, 75.0);
    FT_MOTION_RATE(agent, 2.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, 79.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("trail_as_flash_start"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, -3, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_spin"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("trail_as_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_as_wind"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_spin"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("trail_as_flash_finish"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_as_swing"), Hash40::new("top"), -3, 14, 5, 0, 0, -23, 1, true);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, false);
    }
}

unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("trail_as_flash_start"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_spin"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("trail_as_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_as_wind"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_spin"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("trail_as_flash_finish"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_as_swing"), Hash40::new("top"), -3, 14, 5, 0, 0, -23, 1, true);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, false);
    }
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 21.0, 13.0);
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 22.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::trail::status::SPECIAL_LW_ENABLE_CONTROL);
        ATTACK(agent, 0, 0, Hash40::new("rot"), 9.0, 75, 100, 150, 0, 4.5, 0.0, 1.0, 0.25, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("rot"), 7.0, 75, 100, 100, 0, 4.5, 0.0, 1.0, 0.25, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        CancelModule::enable_cancel(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(agent, 0, 0, Hash40::new("rot"), 5.0, 80, 100, 80, 0, 4.5, 0.0, 1.0, 0.25, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 61.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    FT_MOTION_RATE(agent, 0.875);
    frame(lua_state, 80.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::trail::status::SPECIAL_LW_ENABLE_GRAVITY);
    }
}

unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("trail_flow_launch"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_aura"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_aura"), Hash40::new("clavicler"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_aura"), Hash40::new("claviclel"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_aura"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        //EFFECT_OFF_KIND(agent, Hash40::new("sys_shield_smoke"),false,false);
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_spark"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("trail_super_jump"), Hash40::new("rot"), 0.0,0.0,1.0, 30.0,0,0, 1.0, true);
        //LAST_EFFECT_SET_SCALE_W(agent, 1.0, 1.5, 1.0);
        //LAST_EFFECT_SET_COLOR(agent,1.0,0.0,1.0);
        //LAST_PARTICLE_SET_COLOR(agent, 1.0,0.0,1.0);
    }
    frame(lua_state, 60.0);
    for i in 0..11 {
        if is_excute(agent) {
            let alpha = (11.0-i as f32)/11.0;
            LAST_EFFECT_SET_ALPHA(agent, alpha);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("trail_aura"), false, false);
    }
}

unsafe extern "C" fn sound_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_trail_special_l01"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_trail_special_l01_counter01"));
    }
}

unsafe extern "C" fn expression_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialairlwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 20.0);
    for f in (20..34).step_by(3) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 105, 20, 0, 20, 7.0, 0.0, 6.0, 0.0, None, None, None, 0.8, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 65, 65, 0, 80, 4.6, 0.0, 0.0, -1.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 65, 65, 0, 80, 4.6, 0.0, 6.2, -1.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 65, 65, 0, 80, 4.6, 0.0, 10.6, -1.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::trail::status::SPECIAL_LW_AIR_FALL);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 9.0, 65, 75, 0, 75, 4.6, 0.0, 0.0, -1.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 65, 75, 0, 75, 4.6, 0.0, 2.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialairlwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_aura"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_aura"), Hash40::new("clavicler"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_aura"), Hash40::new("claviclel"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_aura"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 5.0, 0.0, 3.0);
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_spiral"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_spiral2"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 10.0/13.0);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_flow_spark"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_flow"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("trail_aura"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
        AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn sound_specialairlwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_trail_special_l02_01"));
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
    }
}

unsafe extern "C" fn expression_specialairlwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialairlwrebound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 38, 82, 0, 54, 4.7, 0.0, 3.6, -9.2, Some(0.0), Some(3.6), Some(13.2), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 3.0);
    FT_MOTION_RATE_RANGE(agent, 3.0, 38.0, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 38.0); // 27 frames of landing lag
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialairlwrebound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("trail_flow_shockwave"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("trail_flow_shockwave2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialairlwrebound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_trail_special_l02_02"));
    }
}

unsafe extern "C" fn expression_specialairlwrebound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn3", game_specialn3, Priority::Low);
    agent.acmd("game_specialairn3", game_specialairn3, Priority::Low);

    agent.acmd("game_specialn2", game_specialn2, Priority::Low);
    agent.acmd("effect_specialn2", effect_specialn2, Priority::Low);
    agent.acmd("sound_specialn2", sound_specialn2, Priority::Low);   
    agent.acmd("expression_specialn2", expression_specialn2, Priority::Low);
    agent.acmd("game_specialairn2", game_specialn2, Priority::Low);
    agent.acmd("effect_specialairn2", effect_specialn2, Priority::Low);
    agent.acmd("sound_specialairn2", sound_specialn2, Priority::Low);
    agent.acmd("expression_specialairn2", expression_specialn2, Priority::Low);

    agent.acmd("game_specials1", game_specials1, Priority::Low);
    agent.acmd("sound_specials1", sound_specials1, Priority::Low);
    agent.acmd("game_specialairs1", game_specialairs1, Priority::Low);
    agent.acmd("sound_specialairs1", sound_specials1, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialairhi", effect_specialairhi, Priority::Low);
    
	agent.acmd("game_speciallwstart", game_speciallwstart, Priority::Low);
	agent.acmd("effect_speciallwstart", effect_speciallwstart, Priority::Low);
	agent.acmd("sound_speciallwstart", sound_speciallwstart, Priority::Low);
	agent.acmd("expression_speciallwstart", expression_speciallwstart, Priority::Low);
    
	agent.acmd("game_specialairlwstart", game_specialairlwstart, Priority::Low);
	agent.acmd("effect_specialairlwstart", effect_specialairlwstart, Priority::Low);
	agent.acmd("sound_specialairlwstart", sound_specialairlwstart, Priority::Low);
	agent.acmd("expression_specialairlwstart", expression_specialairlwstart, Priority::Low);

	agent.acmd("game_specialairlwrebound", game_specialairlwrebound, Priority::Low);
	agent.acmd("effect_specialairlwrebound", effect_specialairlwrebound, Priority::Low);
	agent.acmd("sound_specialairlwrebound", sound_specialairlwrebound, Priority::Low);
	agent.acmd("expression_specialairlwrebound", expression_specialairlwrebound, Priority::Low);
}