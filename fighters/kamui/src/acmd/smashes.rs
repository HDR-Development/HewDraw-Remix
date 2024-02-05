use super::*;


unsafe extern "C" fn kamui_attack_s4_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    FT_MOTION_RATE_RANGE(fighter, 9.0, 19.0, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
            ArticleModule::generate_article(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, -1);
            ArticleModule::change_motion(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("attack_s4_s"), false, -1.0);
            ArticleModule::set_frame(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, 10.0);
        }
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 103, 0, 40, 2.0, 0.0, 12.25, 16.0, Some(0.0), Some(14.0), Some(27.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 50, 103, 0, 40, 1.7, 0.0, 11.0, 7.5, Some(0.0), Some(13.0), Some(21.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 17.0, 50, 107, 0, 45, 1.1, 0.0, 14.0, 27.0, Some(0.0), Some(16.0), Some(41.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.6);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE_RANGE(fighter, 35.0, 61.0, 29.0);
}


unsafe extern "C" fn kamui_attack_s4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    FT_MOTION_RATE_RANGE(fighter, 9.0, 19.0, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
            ArticleModule::generate_article(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, -1);
            ArticleModule::change_motion(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("attack_s4_s"), false, -1.0);
            ArticleModule::set_frame(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, 10.0);
        }
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 103, 0, 40, 2.0, 0.0, 10.25, 16.0, Some(0.0), Some(10.0), Some(27.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 50, 103, 0, 40, 1.7, 0.0, 10.5, 7.5, Some(0.0), Some(10.0), Some(21.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 17.0, 50, 107, 0, 45, 1.1, 0.0, 10.0, 27.0, Some(0.0), Some(9.5), Some(42.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.6);
        
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE_RANGE(fighter, 35.0, 61.0, 29.0);
}


unsafe extern "C" fn kamui_attack_s4_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    FT_MOTION_RATE_RANGE(fighter, 9.0, 19.0, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
            ArticleModule::generate_article(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, -1);
            ArticleModule::change_motion(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("attack_s4_s"), false, -1.0);
            ArticleModule::set_frame(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, 10.0);
        }
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 103, 0, 40, 2.0, 0.0, 7.25, 16.0, Some(0.0), Some(3.75), Some(27.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 50, 103, 0, 40, 1.7, 0.0, 10.0, 7.5, Some(0.0), Some(5.5), Some(21.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 17.0, 50, 107, 0, 45, 1.1, 0.0, 3.5, 27.0, Some(0.0), Some(-1.0), Some(40.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.6);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE_RANGE(fighter, 35.0, 61.0, 29.0);
}


unsafe extern "C" fn kamui_attack_s4_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("dragon") as i64, hash40("dragon_horn") as i64);
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
        VisibilityModule::set_int64(boma, hash40("hair") as i64, hash40("hair_hide") as i64);
    }
    frame(lua_state, 9.0);
    app::sv_animcmd::execute(lua_state, 9.0);
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(fighter) {
            ItemModule::set_have_item_visibility(boma, false, 0);
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("rbkind_attackm"), 0, 0);
        sv_animcmd::RUMBLE_HIT(lua_state);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("rbkind_attackm"), 0, 1);
        sv_animcmd::RUMBLE_HIT(lua_state);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("rbkind_piercel"), 0, 2);
        sv_animcmd::RUMBLE_HIT(lua_state);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("hair") as i64, hash40("hair_normal") as i64);
    }
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("dragon") as i64, hash40("dragon_none") as i64);
    }
}


unsafe extern "C" fn kamui_spearhand_attack_s4_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.8);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("drag1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE(agent, 1.2);
}


unsafe extern "C" fn kamui_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 9.0, 6.0);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 93, 90, 0, 50, 2.0, 0.0, 8.0, 3.0, Some(0.0), Some(19.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 93, 90, 0, 50, 2.0, 0.0, 8.0, -2.2, Some(0.0), Some(19.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 90, 95, 0, 50, 3.3, 0.0, 30.0, 0.0, Some(0.0), Some(26.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0, 95, 80, 0, 75, 2.8, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}


unsafe extern "C" fn kamui_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 11.0, 30, 100, 0, 45, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 11.0, 30, 100, 0, 45, 3.0, -1.0, 4.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 12.0, 32, 100, 0, 40, 2.25, -1.0, 11.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 11.0, 30, 100, 0, 45, 3.0, -1.0, 7.5, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("haver"), 11.0, 30, 100, 0, 45, 3.0, -1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 6, 0, Hash40::new("top"), 10.0, 35, 100, 0, 50, 3.0, 0.0, 9.0, -13.0, Some(0.0), Some(9.0), Some(-6.5), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 7, 0, Hash40::new("top"), 14.0, 35, 95, 0, 50, 2.5, 0.0, 9.0, -22.0, Some(0.0), Some(9.0), Some(-15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}


pub fn install() {
    smashline::Agent::new("kamui_spearhand")
        .acmd("game_attacks4", kamui_spearhand_attack_s4_game)
        .install();
    smashline::Agent::new("kamui")
        .acmd("game_attacks4hi", kamui_attack_s4_hi_game)
        .acmd("game_attacks4", kamui_attack_s4_game)
        .acmd("game_attacks4lw", kamui_attack_s4_lw_game)
        .acmd("expression_attacks4hi", kamui_attack_s4_expression)
        .acmd("expression_attacks4", kamui_attack_s4_expression)
        .acmd("expression_attacks4lw", kamui_attack_s4_expression)
        .acmd("game_attackhi4", kamui_attack_hi4_game)
        .acmd("game_attacklw4", kamui_attack_lw4_game)
        .install();
}
