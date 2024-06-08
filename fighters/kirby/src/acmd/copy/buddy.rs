use super::*;

unsafe fn will_bayonet(agent: &mut L2CAgentBase) -> bool {
    let boma = agent.boma();
    let is_csticking = ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0;
    if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_BUDDY
    && (is_csticking) {
        return true;
    }
    return false;
}

// Normals
unsafe extern "C" fn game_buddyattacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER,  Hash40::new("attack_s3_s"), false, 0.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 45, 67, 0, 46, 3.2, 0.0, 7.2, 13.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 45, 71, 0, 46, 2.6, 0.0, 7.2, 18.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 45, 67, 0, 46, 3.6, 0.0, 7.2, 7.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
    AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
    ArticleModule::remove_exist(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_buddyattacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6.5, 9.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, 0.5);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("buddy_attack_line"), Hash40::new("buddy_attack_line"), Hash40::new("top"), 0, 6.5, 7, 0, 0, 0, 0.75, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.6);
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -0.5, 7, 19, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_bag_bottom"), -3, 4, 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
    }
}

unsafe extern "C" fn sound_buddyattacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attackhard_s01"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attackhard_s02"));
    }
    if IS_RANDOM(agent, 2) {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_buddy_attackhard_s01"));
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_buddy_attackhard_s02"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attackhard_s03"));
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_n04_01"));
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_n04_02"));
    }
}

unsafe extern "C" fn expression_buddyattacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_buddyattacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER,  Hash40::new("attack_s3_hi"), false, 0.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 50, 67, 0, 46, 3.2, 0.0, 14.2, 12.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 50, 71, 0, 46, 2.6, 0.0, 16.2, 17.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 50, 67, 0, 46, 3.6, 0.0, 12.4, 6.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_buddyattacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11, 9.5, -26, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, 0.5);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("buddy_attack_line"), Hash40::new("buddy_attack_line"), Hash40::new("top"), 0, 10, 6.5, -26, 0, 0, 0.75, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.6);
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -0.5, 15, 18, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_bag_bottom"), -3, 4, 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
    }
}

unsafe extern "C" fn sound_buddyattacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attackhard_s01"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attackhard_s02"));
    }
    if IS_RANDOM(agent, 2) {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_buddy_attackhard_s01"));
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_buddy_attackhard_s02"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attackhard_s03"));
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_n04_01"));
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_n04_02"));
    }
}

unsafe extern "C" fn expression_buddyattacks3hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_buddyattacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER,  Hash40::new("attack_s3_lw"), false, 0.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 40, 67, 0, 46, 3.2, 0.0, 2.8, 12.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 40, 71, 0, 46, 2.6, 0.0, 1.2, 17.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 40, 67, 0, 46, 3.6, 0.0, 4.2, 6.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
    AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
    ArticleModule::remove_exist(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_buddyattacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3.5, 9.5, 15, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, 0.5);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("buddy_attack_line"), Hash40::new("buddy_attack_line"), Hash40::new("top"), 0, 3.5, 6, 15, 0, 0, 0.75, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.6);
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 0.8, 17.5, 0, 0, 0, 0.9, true);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_bag_bottom"), -3, 4, 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
    }
}

unsafe extern "C" fn sound_buddyattacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attackhard_s01"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attackhard_s02"));
    }
    if IS_RANDOM(agent, 2) {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_buddy_attackhard_s01"));
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_buddy_attackhard_s02"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_attackhard_s03"));
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_n04_01"));
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_n04_02"));
    }
}

unsafe extern "C" fn expression_buddyattacks3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

// Specials
unsafe extern "C" fn effect_buddyspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 
    && !VarModule::is_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL) {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("buddy_missile_shot_l"), Hash40::new("top"), 0, 10, 9, 0, 0, 0, 1, false);
        }
        else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("buddy_missile_shot_r"), Hash40::new("top"), 0, 10, 9, 0, 0, 0, 1, false);
        }
    }
}
if is_excute(agent) {
    EFFECT_FOLLOW_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_all"), 0, -6, 0, 0, 0, 0, 0.8, true);
}
frame(lua_state, 16.0);
if is_excute(agent) {
    EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
}
}

unsafe extern "C" fn sound_buddyspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_n01"));
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL) {
            PLAY_SE(agent, Hash40::new("se_buddy_special_n04_01"));
        }
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_step_left_m"));
    }
    frame(lua_state, 61.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_step_right_m"));
    }
    frame(lua_state, 73.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_n04_02"));
    }
}

unsafe extern "C" fn expression_buddyspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

pub fn install(agent: &mut Agent) {
    // Normals
    //agent.acmd("game_buddyattacks3", game_buddyattacks3, Priority::Low);
    //agent.acmd("effect_buddyattacks3", effect_buddyattacks3, Priority::Low);
    //agent.acmd("sound_buddyattacks3", sound_buddyattacks3, Priority::Low);
    //agent.acmd("expression_buddyattacks3", expression_buddyattacks3, Priority::Low);

    //agent.acmd("game_buddyattacks3hi", game_buddyattacks3hi, Priority::Low);
    //agent.acmd("effect_buddyattacks3hi", effect_buddyattacks3hi, Priority::Low);
    //agent.acmd("sound_buddyattacks3hi", sound_buddyattacks3hi, Priority::Low);
    //agent.acmd("expression_buddyattacks3hi", expression_buddyattacks3hi, Priority::Low);

    //agent.acmd("game_buddyattacks3lw", game_buddyattacks3lw, Priority::Low);
    //agent.acmd("effect_buddyattacks3lw", effect_buddyattacks3lw, Priority::Low);
    //agent.acmd("sound_buddyattacks3lw", sound_buddyattacks3lw, Priority::Low);
    //agent.acmd("expression_buddyattacks3lw", expression_buddyattacks3lw, Priority::Low);

    // Specials
    agent.acmd("effect_buddyspecialnfire", effect_buddyspecialnfire, Priority::Low);
    agent.acmd("sound_buddyspecialnfire", sound_buddyspecialnfire, Priority::Low);
    agent.acmd("expression_buddyspecialnfire", expression_buddyspecialnfire, Priority::Low);
}
