use super::*;

unsafe extern "C" fn kirby_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_INHALE);
        CATCH(fighter, 0, Hash40::new("top"), 6.0, 0.0, 6.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
        SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 7.0, 8.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn kirby_special_n_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_INHALE);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 160, 100, 30, 0, 8.5, 0.0, 7.0, 15.5, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 160, 100, 25, 0, 7.8, 0.0, 7.0, 9.7, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        CATCH(fighter, 0, Hash40::new("top"), 5.3, 0.0, 6.4, 10.2, None, None, None, *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, 1, Hash40::new("top"), 5.8, 0.0, 6.4, 6.2, None, None, None, *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
        SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 7.0, 8.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn kirby_special_n_eat_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 52.0, 24.0);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_ITEM_REMOVE);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_ITEM_USE);
    }
}

unsafe extern "C" fn kirby_special_n_drink_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 361, 0, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_SPIT);
    }
}

unsafe extern "C" fn kirby_special_n_large_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 86.0, 54.0);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_ITEM_REMOVE);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_ITEM_USE);
    }
}

unsafe extern "C" fn kirby_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE_RANGE(fighter, 0.0, 15.0, 6.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, -1);
    }
}

unsafe extern "C" fn kirby_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 19.0, 48, 78, 0, 60, 5.4, 0.0, 4.5, 11.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 19.0, 48, 78, 0, 60, 3.5, 0.0, 4.5, 5.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE_RANGE(fighter, 13.0, 54.0, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 54.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn kirby_special_ss_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 19.0, 48, 78, 0, 60, 5.4, 0.0, 4.5, 11.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 19.0, 48, 78, 0, 60, 3.5, 0.0, 4.5, 5.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE_RANGE(fighter, 13.0, 54.0, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 54.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn kirby_special_s_max_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 35.0, 361, 78, 0, 60, 5.8, 0.0, 4.2, 11.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 35.0, 361, 78, 0, 60, 3.9, 0.0, 4.2, 5.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HAMMER);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE_RANGE(fighter, 13.0, 54.0, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 54.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 56.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn kirby_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE_RANGE(fighter, 0.0, 18.0, 11.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, -1);
    }

}

unsafe extern "C" fn kirby_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 11.0, 5.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 106, 0, 67, 5.4, 0.0, 4.3, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 50, 106, 0, 67, 3.2, 0.0, 4.3, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.05);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 35, 93, 0, 67, 5.4, 0.0, 4.3, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 35, 93, 0, 67, 3.2, 0.0, 4.3, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.05);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }

}

unsafe extern "C" fn kirby_special_air_ss_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 11.0, 5.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 106, 0, 67, 5.4, 0.0, 4.3, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 50, 106, 0, 67, 3.2, 0.0, 4.3, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.05);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 35, 93, 0, 67, 5.4, 0.0, 4.3, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 35, 93, 0, 67, 3.2, 0.0, 4.3, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.05);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }

}

unsafe extern "C" fn kirby_special_air_hi2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi2"), false, -1.0);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 85, 100, 117, 0, 3.0, 0.0, 3.5, 7.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 93, 100, 117, 0, 3.0, 0.0, 3.5, 16.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 90, 100, 102, 0, 3.0, 0.0, 13.5, 7.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 93, 100, 102, 0, 3.0, 0.0, 13.5, 16.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 85, 100, 60, 0, 3.0, 0.0, 3.5, 7.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 93, 100, 60, 0, 3.0, 0.0, 3.5, 16.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 90, 100, 50, 0, 3.0, 0.0, 13.5, 7.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 93, 100, 50, 0, 3.0, 0.0, 13.5, 16.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.0, 275, 102, 0, 40, 6.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.0, 361, 180, 0, 30, 5.5, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

unsafe extern "C" fn kirby_special_air_hi2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        effect!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0, 3, 0.25, Hash40::new("haver"), 0, 20, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);
        EFFECT_FOLLOW(fighter, Hash40::new("kirby_fcut_rise"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        effect!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0, 2.5, 0.5, Hash40::new("haver"), 0, 16, 0.3, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -4, 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn kirby_special_hi_h_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE_RANGE(fighter, 0.0, 23.0, 7.0);  // startup
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi1"), false, -1.0);
    }
    frame(lua_state, 22.0); // rush
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 10.0, 72, 48, 0, 60, 5.5, 0.0, 2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 10.0, 72, 48, 0, 60, 5.5, 0.0, 10.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 72, 48, 0, 50, 5.5, 0.0, 7.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 72, 48, 0, 50, 5.5, 0.0, 10.0, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 42.0);
    FT_MOTION_RATE_RANGE(fighter, 42.0, 74.0, 15.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn kirby_special_hi_h_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 1, 6, 85, 120, -90, 1, false, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.66);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        effect!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0, 2, 0.25, Hash40::new("haver"), 0, 14, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 4, 6, -65, -100, -90, 1, false, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.66);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 0);
        effect!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0, 2, 0.25, Hash40::new("haver"), 0, 14, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 0);
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 6, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 6, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }

}

unsafe extern "C" fn kirby_special_hi_h_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h02"));
        match smash::app::sv_math::rand(smash::hash40("fighter"), 2) {
            0 => PLAY_SE(fighter, Hash40::new("vc_kirby_002")),
            1 => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
            _ => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
        };
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h04"));
    }
}

unsafe extern "C" fn kirby_special_hi_h_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
}

unsafe extern "C" fn kirby_special_air_hi_h_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE_RANGE(fighter, 0.0, 18.0, 6.0);  // startup
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi1"), false, -1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 10.0, 78, 48, 0, 80, 5.0, 0.0, 2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 10.0, 78, 48, 0, 80, 5.0, 0.0, 10.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0); // rush
    FT_MOTION_RATE_RANGE(fighter, 18.0, 29.0, 7.0);
    frame(lua_state, 29.0); // flip
    FT_MOTION_RATE_RANGE(fighter, 29.0, 48.0, 12.0);
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 48.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, vars::kirby::instance::DISABLE_SPECIAL_HI);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
    frame(lua_state, 56.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

unsafe extern "C" fn kirby_special_air_hi_h_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 2, 0, 0, 90, -90, 1, false, *EF_FLIP_XY);
        LAST_EFFECT_SET_SCALE_W(fighter, 0.65, 1.0, 0.55);
        LAST_EFFECT_SET_RATE(fighter, 0.66);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        effect!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0, 2, 0.25, Hash40::new("haver"), 0, 14, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 0);
    }

}

unsafe extern "C" fn kirby_special_air_hi_h_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h02"));
        match smash::app::sv_math::rand(smash::hash40("fighter"), 2) {
            0 => PLAY_SE(fighter, Hash40::new("vc_kirby_002")),
            1 => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
            _ => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
        };
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h04"));
    }
}

unsafe extern "C" fn kirby_special_air_hi_h_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
}

unsafe extern "C" fn kirby_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw"), false, -1.0);
    }
    FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_BLINK_ONOFF);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw_to_ground"), false, -1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 25, 30, 0, 86, 6.0, 0.0, 4.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 25, 30, 0, 86, 6.0, 0.0, 4.0, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::init_attack_pos(boma, 0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn kirby_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 30.0, 24.0);
    if is_excute(fighter) {
        //KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_air_lw"), false, -1.0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 100);
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_BLINK_ONOFF);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15.0);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        //KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        ArticleModule::change_motion(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw_to_air"), false, -1.0);
        AttackModule::clear(boma, 1, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 70, 76, 0, 69, 6.5, 0.0, 2.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::init_attack_pos(boma, 0);
    }
}

unsafe extern "C" fn kirby_special_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 3, 2, 7, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_XY);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("kirby_stone_s"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        FLASH(fighter, 0.706, 0.502, 0.392, 0.157);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.314, 0.235, 0.157, 0.235);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.706, 0.502, 0.392, 0.314);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.314, 0.235, 0.157, 0.392);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.706, 0.502, 0.392, 0.471);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.314, 0.235, 0.157, 0.549);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.706, 0.502, 0.392, 0.627);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.314, 0.235, 0.157, 0.706);
    }
}

pub fn install() {
    smashline::Agent::new("kirby")
        .acmd("game_specialnstart", kirby_special_n_start_game)
        .acmd("game_specialairnstart", kirby_special_n_start_game)
        .acmd("game_specialnloop", kirby_special_n_loop_game)
        .acmd("game_specialairnloop", kirby_special_n_loop_game)
        .acmd("game_specialneat", kirby_special_n_eat_game)
        .acmd("game_specialairneat", kirby_special_n_eat_game)
        .acmd("game_specialndrink", kirby_special_n_drink_game)
        .acmd("game_specialairndrink", kirby_special_n_drink_game)
        .acmd("game_specialnlarge", kirby_special_n_large_game)
        .acmd("game_specialairnlarge", kirby_special_n_large_game)
        .acmd("game_specialsstart", kirby_special_s_start_game)
        .acmd("game_specials", kirby_special_s_game)
        .acmd("game_specialss", kirby_special_ss_game)
        .acmd("game_specialsmax", kirby_special_s_max_game)
        .acmd("game_specialairsstart", kirby_special_air_s_start_game)
        .acmd("game_specialairs", kirby_special_air_s_game)
        .acmd("game_specialairss", kirby_special_air_ss_game)
        .acmd("game_specialairhi2", kirby_special_air_hi2_game)
        .acmd("effect_specialairhi2", kirby_special_air_hi2_effect)
        .acmd("game_specialhih", kirby_special_hi_h_game)
        .acmd("effect_specialhih", kirby_special_hi_h_effect)
        .acmd("sound_specialhih", kirby_special_hi_h_sound)
        .acmd("expression_specialhih", kirby_special_hi_h_expression)
        .acmd("game_specialairhih", kirby_special_air_hi_h_game)
        .acmd("effect_specialairhih", kirby_special_air_hi_h_effect)
        .acmd("sound_specialairhih", kirby_special_air_hi_h_sound)
        .acmd(
            "expression_specialairhih",
            kirby_special_air_hi_h_expression,
        )
        .acmd("game_speciallw", kirby_special_lw_game)
        .acmd("game_specialairlw", kirby_special_air_lw_game)
        .acmd("effect_specialairlw", kirby_special_air_lw_effect)
        .install();
}
