use super::*;

unsafe extern "C" fn effect_attacks4charge(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    frame(lua_state, 5.0);
    for _ in 0..34 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, true);
        }
        wait(lua_state, 5.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn game_attacks4(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    FT_DESIRED_RATE(agent, 14.0, 10.0);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, false, 0);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 361, 25, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 6.0, 170, 20, 0, 25, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 4.0, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 361, 15, 0, 30, 3.0, 0.0, 5.0, 5.0, Some(0.0), Some(9.0), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 2, 4.0, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::on_flag(agent.object(), vars::snake::instance::KNIFE_COMBO_ENABLE);
    }

    frame(lua_state, 38.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.object(), vars::snake::instance::KNIFE_COMBO_ENABLE);
    }
    frame(lua_state, 54.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn effect_attacks4(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 4, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 5.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3, 0.11, 0, 0, 0, 0.34, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3.5, 0.05, 0, 180, 0, 0.15, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_killsword_light2"), Hash40::new("haver"), 0, -0.1, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_killsword_light1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_killsword_light2"), false, true);
    }
}

unsafe extern "C" fn sound_attacks4(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_sword_swing_s"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_snake_rnd_attack"));
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_snake_squat_gear"));
    }
}

unsafe extern "C" fn expression_attacks4(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

unsafe extern "C" fn game_attacks4s2(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 9.0, 6.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.0, 4.5);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 7.0, 361, 25, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze_ghost"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 7.0, 361, 25, 0, 30, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze_ghost"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //fake/empty hitboxes to create particle effects
        ATTACK(agent, 2, 1, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 1, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 3.5, 0.0, 4.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 4.0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::on_flag(agent.object(), vars::snake::instance::KNIFE_COMBO_ENABLE);
        // WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.object(), vars::snake::instance::KNIFE_COMBO_ENABLE);
        // WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn effect_attacks4s2(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 4, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 5.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3, 0.11, 0, 0, 0, 0.34, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3.5, 0.05, 0, 180, 0, 0.15, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_killsword_light2"), Hash40::new("haver"), 0, -0.1, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_killsword_light1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_killsword_light2"), false, true);
    }
}

unsafe extern "C" fn sound_attacks4s2(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_sword_swing_s"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_snake_rnd_attack"));
    }
    frame(lua_state, 49.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_snake_squat_gear"));
    }
}

unsafe extern "C" fn expression_attacks4s2(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

unsafe extern "C" fn game_attacks4s3(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 9.0, 6.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.0, 4.5);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 10.0, 361, 80, 0, 85, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 361, 80, 0, 85, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn effect_attacks4s3(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 4, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 5.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3, 0.11, 0, 0, 0, 0.34, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3.5, 0.05, 0, 180, 0, 0.15, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_killsword_light2"), Hash40::new("haver"), 0, -0.1, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_killsword_light1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_killsword_light2"), false, true);
    }
}

unsafe extern "C" fn sound_attacks4s3(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_sword_swing_s"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_snake_rnd_attack"));
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_snake_squat_gear"));
    }
}

unsafe extern "C" fn expression_attacks4s3(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

unsafe extern "C" fn effect_attacklw4charge(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    frame(lua_state, 5.0);
    for _ in 0..34 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, true);
        }
        wait(lua_state, 5.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn expression_attacklw4charge(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        physics!(agent, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.2, 0.2, -1, 0.7, 0.5, -1, Hash40::new("invalid"));
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, 0);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold2"), 0, true, 0);
    }
}

unsafe extern "C" fn game_attacklw4(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 12.0, 25, 87, 0, 30, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legr"), 12.0, 25, 87, 0, 30, 3.3, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("hip"), 12.0, 25, 87, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    FT_DESIRED_RATE(agent, 19.0, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 14.0, 20, 88, 0, 30, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 14.0, 20, 88, 0, 30, 3.3, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("hip"), 14.0, 20, 88, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacklw4(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        // EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 13, -1, 35, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 5, 7, 9, 10, -70, 6, 1, true);
    }
    // frame(lua_state, 13.0);
    // if is_excute(agent) {
    //     EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, 17, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
    // }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 9, -2, 14, -180, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, -17, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
    }
}

unsafe extern "C" fn sound_attacklw4(agent : &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_snake_rnd_attack_smash_l"));
        PLAY_SE(agent, Hash40::new("se_snake_smash_l01"));
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_snake_rnd_attack"));
        // PLAY_SE(agent, Hash40::new("vc_snake_attack04"));
        // PLAY_SEQUENCE(agent, Hash40::new("seq_snake_rnd_attack_smash_s"));
        PLAY_SE(agent, Hash40::new("se_snake_smash_l02"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_attacks4charge", effect_attacks4charge);
    agent.acmd("game_attacks4", game_attacks4);
    agent.acmd("effect_attacks4", effect_attacks4);
    agent.acmd("sound_attacks4", sound_attacks4);
    agent.acmd("expression_attacks4", expression_attacks4);

    agent.acmd("game_attacks4s2", game_attacks4s2);
    agent.acmd("effect_attacks4s2", effect_attacks4s2);
    agent.acmd("sound_attacks4s2", sound_attacks4s2);
    agent.acmd("expression_attacks4s2", expression_attacks4s2);

    agent.acmd("game_attacks4s3", game_attacks4s3);
    agent.acmd("effect_attacks4s3", effect_attacks4s3);
    agent.acmd("sound_attacks4s3", sound_attacks4s3);
    agent.acmd("expression_attacks4s3", expression_attacks4s3);

    agent.acmd("effect_attacklw4charge", effect_attacklw4charge);
    agent.acmd("expression_attacklw4charge", expression_attacklw4charge);

    agent.acmd("game_attacklw4", game_attacklw4);
    agent.acmd("effect_attacklw4", effect_attacklw4);
    agent.acmd("sound_attacklw4", sound_attacklw4);
}