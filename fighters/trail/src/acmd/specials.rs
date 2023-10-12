use super::*;

#[acmd_script( agent = "trail", script = "game_specialn3" , category = ACMD_GAME , low_priority)]
unsafe fn game_specialn3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, 0, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_FLAG_CHANGE_MAGIC);
    }
    wait(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, 1, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, 0);
    }
    wait(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, 2, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, 0);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
}

#[acmd_script( agent = "trail", script = "game_specialairn3" , category = ACMD_GAME , low_priority)]
unsafe fn game_specialairn3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma,  *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, 0, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_FLAG_CHANGE_MAGIC); 
    }
    wait(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, 1, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, 0);
    }
    wait(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, 2, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, 0);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(lua_state, 70.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

#[acmd_script( agent = "trail", scripts = ["game_specialn2", "game_specialairn2"] , category = ACMD_GAME , low_priority)]
unsafe fn game_specialn2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if fighter.kind() == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
        }
        frame(lua_state, 1.0);
        FT_MOTION_RATE(fighter, 0.5);
        frame(lua_state, 19.0);
        FT_MOTION_RATE(fighter, 1.0);
        frame(lua_state, 22.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 3.2, 0.0, 6.4, 2.2, Some(0.0), Some(6.4), Some(5.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_MAGIC);
        }
        frame(lua_state, 24.0);
        FT_MOTION_RATE(fighter, 1.25);
        if is_excute(fighter) {
            AttackModule::clear(boma, 0, false);
            WorkModule::set_float(boma, 4.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
            ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
            WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            WorkModule::set_float(boma, 16.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
            ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            WorkModule::set_float(boma, -8.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
            ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            WorkModule::set_float(boma, 24.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
            ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            WorkModule::set_float(boma, -2.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
            ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            WorkModule::set_float(boma, 12.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
            ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            WorkModule::set_float(boma, -14.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
            ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_LAST_SHOOT);
            WorkModule::set_float(boma, 0.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
            ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
        }
        wait(lua_state, 2.0);
        FT_MOTION_RATE(fighter, 1.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 42, 64, 0, 52, 3.2, 0.0, 6.4, 2.2, Some(0.0), Some(6.4), Some(6.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
        wait(lua_state, 2.0);
        FT_MOTION_RATE(fighter, 0.8);
        if is_excute(fighter) {
            AttackModule::clear(boma, 0, false);
            WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
        }
        frame(lua_state, 57.0);
        FT_START_ADJUST_MOTION_FRAME_arg1(fighter, 1.0);
    } else {
        if is_excute(fighter) {
            WorkModule::on_flag(boma,  *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
            ArticleModule::remove_exist(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER, app::ArticleOperationTarget(0));
            ArticleModule::generate_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER, false, 0);
            ArticleModule::change_motion(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER,  Hash40::new("special_n2"), false, 0.0);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 78, 60, 0, 70, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 78, 60, 0, 70, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 30.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 40.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 50.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 6.0, 0.0, 6.0, 1.0, Some(0.0), Some(6.0), Some(-1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 361, 60, 0, 30, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(-12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 90.0);
        if is_excute(fighter) {
            WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
            WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
        }
    }
}

#[acmd_script( agent = "trail", scripts = ["effect_specialn2", "effect_specialairn2"] , category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialn2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("trail_ice_hold"), Hash40::new("haver"), 0, 10, -1, -90, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(fighter, Hash40::new("trail_ice_sword_flare"), Hash40::new("haver"), 0, 10, -1, -90, 0, 0, 1, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), 0, 3.7, 0, 0, 0, 180, 0.9, true);
        LAST_EFFECT_SET_RATE(fighter, 0.25);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), 0, 3.7, 0, 0, 180, 180, 0.9, true);
        LAST_EFFECT_SET_RATE(fighter, 0.25);
    }
}

#[acmd_script( agent = "trail", scripts = ["sound_specialn2", "sound_specialairn2"] , category = ACMD_SOUND, low_priority )]
unsafe fn sound_specialn2(agent: &mut L2CAgentBase) {
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

// Now unused
#[acmd_script( agent = "trail", script = "game_specials1" , category = ACMD_GAME , low_priority)]
unsafe fn game_specials1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.2, 72, 10, 0, 85, 3.2, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.2, 72, 10, 0, 85, 3.2, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.2, 72, 10, 0, 85, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        KineticModule::add_speed(boma, &Vector3f::new(-1.0, 0.0, 0.0));
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
    }
}

#[acmd_script( agent = "trail", script = "game_specialairs1" , category = ACMD_GAME , low_priority)]
unsafe fn game_specialairs1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.2, 72, 10, 0, 85, 3.2, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.2, 72, 10, 0, 85, 3.2, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.2, 72, 10, 0, 85, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        //KineticModule::add_speed(boma, &Vector3f::new(-1.0, 0, 0.0));
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
    }
}



// Is now effectively Sonic Blade 1
#[acmd_script( agent = "trail", script = "game_specials2" , category = ACMD_GAME , low_priority)]
unsafe fn game_specials2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.2, 72, 10, 0, 85, 3.2, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.2, 72, 10, 0, 85, 3.2, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.2, 72, 10, 0, 85, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
        let angle = WorkModule::get_float(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
        if angle == 0.0 || angle >= 180.0 {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        KineticModule::add_speed(boma, &Vector3f::new(-0.5, 0.0, 0.0));
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
    }
}

#[acmd_script( agent = "trail", script = "game_specialairs2" , category = ACMD_GAME , low_priority)]
unsafe fn game_specialairs2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.2, 72, 10, 0, 85, 3.2, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.2, 72, 10, 0, 85, 3.2, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.2, 72, 10, 0, 85, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
        let angle = WorkModule::get_float(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
        if angle == 0.0 || angle >= 180.0 {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        // KineticModule::add_speed(boma, &Vector3f::new(-0.5, 0, 0.0));
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
    }
}

#[acmd_script( agent = "trail", scripts = [ "sound_specials2", "sound_specialairs2" ] , category = ACMD_SOUND , low_priority)]
unsafe fn sound_specials2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_trail_special_s01"));
    }
    frame(lua_state, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_trail_special_s02"));
    }
}

#[acmd_script( agent = "trail", script = "game_specials3" , category = ACMD_GAME , low_priority)]
unsafe fn game_specials3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        let attack_num = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("attack_num"));
        // New check since this ACMD script gets used for both the second and third Sonic Blades now.
        if WorkModule::get_int(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT) < attack_num - 1 {
            // For when you're *not* on the last rep.
            if WorkModule::is_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_ATTACK_BUTTON){
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.2, 108, 8, 0, 80, 3.2, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.2, 108, 8, 0, 80, 3.2, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.2, 108, 8, 0, 80, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);    
            }
            else{
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.0, 108, 8, 0, 80, 3.2, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.0, 108, 8, 0, 80, 3.2, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.0, 108, 8, 0, 80, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
            }
        }
        else {
            // For when you *are* on the last rep.
            if WorkModule::is_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_ATTACK_BUTTON){
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.2, 46, 85, 0, 80, 3.2, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.2, 46, 85, 0, 80, 3.2, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.2, 46, 85, 0, 80, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            }
            else{
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.0, 46, 88, 0, 80, 3.2, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.0, 46, 88, 0, 80, 3.2, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.0, 46, 88, 0, 80, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            }
        }
        let angle = WorkModule::get_float(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
        if angle == 0.0 || angle >= 180.0 {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        KineticModule::add_speed(boma, &Vector3f::new(-0.5, 0.0, 0.0));
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
    }
}

#[acmd_script( agent = "trail", script = "game_specialairs3" , category = ACMD_GAME , low_priority)]
unsafe fn game_specialairs3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        let attack_num = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("attack_num"));
        // New check since this ACMD script gets used for both the second and third Sonic Blades now.
        if WorkModule::get_int(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT) < attack_num - 1 {
            // For when you're *not* on the last rep.
            if WorkModule::is_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_ATTACK_BUTTON){
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.2, 108, 8, 0, 80, 3.2, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.2, 108, 8, 0, 80, 3.2, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.2, 108, 8, 0, 80, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);    
            }
            else{
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.0, 108, 8, 0, 80, 3.2, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.0, 108, 8, 0, 80, 3.2, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.0, 108, 8, 0, 80, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_STAB, *ATTACK_REGION_SWORD);
            }
        }
        else {
            // For when you *are* on the last rep.
            if WorkModule::is_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_ATTACK_BUTTON){
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.2, 46, 85, 0, 80, 3.2, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.2, 46, 85, 0, 80, 3.2, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.2, 46, 85, 0, 80, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            }
            else{
                ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.0, 46, 88, 0, 80, 3.2, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.0, 46, 88, 0, 80, 3.2, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
                ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.0, 46, 88, 0, 80, 3.8, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
            }
        }
        let angle = WorkModule::get_float(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
        if angle == 0.0 || angle >= 180.0 {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        //KineticModule::add_speed(boma, &Vector3f::new(-0.5, 0, 0.0));
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
    }
}

#[acmd_script( agent = "trail", scripts = [ "sound_specials3", "sound_specialairs3" ] , category = ACMD_SOUND , low_priority)]
unsafe fn sound_specials3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    let attack_num = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("attack_num"));
    // New check since this ACMD script gets used for both the second and third Sonic Blades now.
    if WorkModule::get_int(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT) < attack_num - 1 {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_trail_special_s03"));
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_trail_special_s04"));
        }
    }
}

#[acmd_script( agent = "trail", scripts = [ "game_specialssearch", "game_specialairssearch" ], category = ACMD_GAME, low_priority )]
unsafe fn game_specialssearch(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "trail", scripts = [ "effect_specialssearch", "effect_specialairssearch" ] , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialssearch(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "trail", script = 0x15bfed9702 , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialsturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("trail_sonic_turn"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "trail", script = 0x1152d69c4b , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialsup(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("trail_sonic_turn"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "trail", script = "effect_specialsdown" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialsdown(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("trail_sonic_turn"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "trail", script = 0x18f6f0b024 , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialairsturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("trail_sonic_turn"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "trail", script = 0x1424373288 , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialairsup(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("trail_sonic_turn"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "trail", script = "effect_specialairsdown" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_specialairsdown(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("trail_sonic_turn"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "trail", scripts = ["game_specialhi", "game_specialairhi"] , category = ACMD_GAME , low_priority)]
unsafe fn game_specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(boma.object(), vars::trail::status::UP_SPECIAL_HIT);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 2.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_HI_FLAG_JUMP_START);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.8, 86, 100, 116, 0, 4.2, 0.0, 4.2, 7.6, Some(0.0), Some(4.2), Some(7.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.8, 88, 100, 82, 0, 4.2, 0.0, 12.6, 7.6, Some(0.0), Some(12.6), Some(7.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.8, 110, 100, 120, 0, 4.2, 0.0, 4.2, 16.8, Some(0.0), Some(4.2), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.8, 108, 100, 86, 0, 4.2, 0.0, 12.6, 16.8, Some(0.0), Some(12.6), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 3.8, 94, 100, 130, 0, 4.2, 0.0, 4.2, 7.6, Some(0.0), Some(4.2), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 3.8, 92, 100, 76, 0, 4.2, 0.0, 12.6, 7.6, Some(0.0), Some(12.6), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_finish_camera(boma, 4, true, false);
        AttackModule::set_no_finish_camera(boma, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.1, 80, 100, 124, 0, 2.4, 0.0, 7.6, -6.6, Some(0.0), Some(7.6), Some(-6.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.1, 82, 100, 70, 0, 2.4, 0.0, 12.2, -6.6, Some(0.0), Some(12.2), Some(-6.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.1, 106, 100, 122, 0, 2.4, 0.0, 7.6, -12.2, Some(0.0), Some(7.6), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.1, 108, 100, 78, 0, 2.4, 0.0, 12.2, -12.2, Some(0.0), Some(12.2), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 2.1, 94, 100, 114, 0, 2.4, 0.0, 7.6, -6.6, Some(0.0), Some(7.6), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 2.1, 92, 100, 68, 0, 2.4, 0.0, 12.2, -6.6, Some(0.0), Some(12.2), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_finish_camera(boma, 4, true, false);
        AttackModule::set_no_finish_camera(boma, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.1, 76, 100, 92, 0, 2.8, 0.0, 4.6, 6.4, Some(0.0), Some(4.6), Some(6.4), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.1, 78, 100, 62, 0, 2.8, 0.0, 10.0, 6.4, Some(0.0), Some(10.0), Some(6.4), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.1, 108, 100, 100, 0, 2.8, 0.0, 4.6, 14.6, Some(0.0), Some(4.6), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.1, 106, 100, 72, 0, 2.8, 0.0, 10.0, 14.6, Some(0.0), Some(10.0), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 2.1, 92, 100, 110, 0, 2.8, 0.0, 4.6, 6.4, Some(0.0), Some(4.6), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 2.1, 90, 100, 58, 0, 2.8, 0.0, 10.0, 6.4, Some(0.0), Some(10.0), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_finish_camera(boma, 4, true, false);
        AttackModule::set_no_finish_camera(boma, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.1, 88, 100, 92, 0, 2.4, 0.0, 8.8, -6.6, Some(0.0), Some(8.8), Some(-6.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.1, 84, 100, 46, 0, 2.4, 0.0, 13.4, -6.6, Some(0.0), Some(13.4), Some(-6.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.1, 102, 100, 88, 0, 2.4, 0.0, 8.8, -12.2, Some(0.0), Some(8.8), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.1, 106, 100, 48, 0, 2.4, 0.0, 13.4, -12.2, Some(0.0), Some(13.4), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 2.1, 92, 100, 90, 0, 2.4, 0.0, 8.8, -6.6, Some(0.0), Some(8.8), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 2.1, 90, 100, 48, 0, 2.4, 0.0, 13.4, -6.6, Some(0.0), Some(13.4), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_finish_camera(boma, 4, true, false);
        AttackModule::set_no_finish_camera(boma, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.1, 74, 100, 90, 0, 2.8, 0.0, 7.2, 6.4, Some(0.0), Some(7.2), Some(6.4), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.1, 72, 100, 72, 0, 2.8, 0.0, 12.6, 6.4, Some(0.0), Some(12.6), Some(6.4), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.1, 98, 100, 92, 0, 2.8, 0.0, 7.2, 14.6, Some(0.0), Some(7.2), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.1, 96, 100, 72, 0, 2.8, 0.0, 12.6, 14.6, Some(0.0), Some(12.6), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 2.1, 94, 100, 102, 0, 2.8, 0.0, 7.2, 6.4, Some(0.0), Some(7.2), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 2.1, 92, 100, 62, 0, 2.8, 0.0, 12.6, 6.4, Some(0.0), Some(12.6), Some(14.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_finish_camera(boma, 4, true, false);
        AttackModule::set_no_finish_camera(boma, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.1, 125, 100, 130, 0, 2.4, 0.0, 8.2, -6.6, Some(0.0), Some(8.2), Some(-6.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.1, 150, 100, 110, 0, 2.4, 0.0, 12.8, -6.6, Some(0.0), Some(12.8), Some(-6.6), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.1, 130, 100, 150, 0, 2.4, 0.0, 8.2, -12.2, Some(0.0), Some(8.2), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.1, 150, 100, 150, 0, 2.4, 0.0, 12.8, -12.2, Some(0.0), Some(12.8), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 2.1, 130, 100, 130, 0, 2.4, 0.0, 8.2, -6.6, Some(0.0), Some(8.2), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 2.1, 150, 100, 130, 0, 2.4, 0.0, 12.8, -6.6, Some(0.0), Some(12.8), Some(-12.2), 0.3, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_finish_camera(boma, 4, true, false);
        AttackModule::set_no_finish_camera(boma, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.6, 62, 176, 0, 44, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.6, 62, 176, 0, 44, 3.6, 0.0, 4.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.6, 62, 176, 0, 44, 3.6, 0.0, 9.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 4.6, 62, 176, 0, 44, 5.2, 0.0, 7.4, 8.4, Some(0.0), Some(9.6), Some(8.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 4.6, 62, 176, 0, 44, 4.8, 0.0, 12.6, 15.6, Some(0.0), Some(14.8), Some(15.6), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        if VarModule::is_flag(boma.object(), vars::trail::status::UP_SPECIAL_HIT){
            WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_HI_FLAG_COMMAND_ACCEPT);
        }
        FT_MOTION_RATE(fighter, 0.5);
    }
    frame(lua_state, 70.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_HI_FLAG_COMMAND_ACCEPT);
    }
    frame(lua_state, 75.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::trail::status::UP_SPECIAL_HIT){
            FT_MOTION_RATE(fighter, 2.0);
        }
    }
    frame(lua_state, 79.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::trail::status::UP_SPECIAL_HIT){
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
}

#[acmd_script( agent = "trail_fire", scripts = ["game_fly", "game_fly2"] , category = ACMD_GAME , low_priority)]
unsafe fn game_trail_fire_fly(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.6, 361, 24, 0, 42, 3.8, 0.0, 7.5, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1.2, -1.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::enable_safe_pos(boma);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 0.0, false);
        ATK_SET_SHIELD_SETOFF_MUL2(fighter, 0, 1.05);
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.2, 361, 22, 0, 36, 3.8, 0.0, 7.5, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -0.8, -1.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 0.0, false);
        ATK_SET_SHIELD_SETOFF_MUL2(fighter, 0, 1.05);
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.8, 361, 20, 0, 32, 3.8, 0.0, 7.5, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -0.8, -1.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 0.0, false);
        ATK_SET_SHIELD_SETOFF_MUL2(fighter, 0, 1.05);
    }
}

#[acmd_script( agent = "trail_fire", scripts = ["effect_fly", "effect_fly2"], category = ACMD_EFFECT, low_priority )]
unsafe fn effect_trail_fire_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("trail_fire_bullet"), Hash40::new("top"), 0, 7.5, 14.0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_trail_fire_fly,
        effect_trail_fire_fly,
        game_specialairn3,
        game_specialn3,
        game_specialn2,
        effect_specialn2,
        sound_specialn2,
        game_specials1,
        game_specialairs1,
        game_specials2,
        game_specialairs2,
        sound_specials2,
        game_specials3,
        game_specialairs3,
        sound_specials3,
        game_specialssearch, effect_specialssearch,
        effect_specialsturn,
        effect_specialsup,
        effect_specialsdown,
        effect_specialairsturn,
        effect_specialairsup,
        effect_specialairsdown,
        game_specialhi,
    );
}
