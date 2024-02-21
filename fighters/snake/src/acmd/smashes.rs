use super::*;

//first hit

unsafe extern "C" fn snake_side_smash_game(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    FT_DESIRED_RATE(fighter, 14.0, 10.0);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, false, 0);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 6.0, 361, 25, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.0, 170, 20, 0, 25, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 4.0, false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 361, 15, 0, 30, 3.0, 0.0, 5.0, 5.0, Some(0.0), Some(9.0), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 2, 4.0, false);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        VarModule::on_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_ENABLE);
    }

    frame(lua_state, 38.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_ENABLE);
    }
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, ArticleOperationTarget(0));
    }
}


unsafe extern "C" fn snake_side_smash_expr(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashs"), 0);
    }
}


unsafe extern "C" fn snake_side_smash_snd(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack"));
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_squat_gear"));
    }
}


unsafe extern "C" fn snake_side_smash_eff(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 4, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 5.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3, 0.11, 0, 0, 0, 0.34, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3.5, 0.05, 0, 180, 0, 0.15, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light2"), Hash40::new("haver"), 0, -0.1, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light1"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light2"), false, true);
    }
}

//charge

unsafe extern "C" fn snake_side_smash_charge_eff(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    frame(lua_state, 5.0);
    for _ in 0..34 {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, true);
        }
        wait(lua_state, 5.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, false);
        }
    }
}

//second hit

unsafe extern "C" fn snake_side_smash_2_game(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 9.0, 6.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.0, 4.5);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.0, 361, 25, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze_ghost"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.0, 361, 25, 0, 30, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze_ghost"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //fake/empty hitboxes to create particle effects
        ATTACK(fighter, 2, 1, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 1, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 3.5, 0.0, 4.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 4.0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        VarModule::on_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_ENABLE);
        // WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_ENABLE);
        // WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, ArticleOperationTarget(0));
    }
}


unsafe extern "C" fn snake_side_smash_2_expr(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashs"), 0);
    }
}


unsafe extern "C" fn snake_side_smash_2_snd(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack"));
    }
    frame(lua_state, 49.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_squat_gear"));
    }
}


unsafe extern "C" fn snake_side_smash_2_eff(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 4, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 5.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3, 0.11, 0, 0, 0, 0.34, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3.5, 0.05, 0, 180, 0, 0.15, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light2"), Hash40::new("haver"), 0, -0.1, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light1"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light2"), false, true);
    }
}

//third hit

unsafe extern "C" fn snake_side_smash_3_game(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 9.0, 6.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.0, 4.5);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 10.0, 361, 80, 0, 85, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 10.0, 361, 80, 0, 85, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, ArticleOperationTarget(0));
    }
}


unsafe extern "C" fn snake_side_smash_3_expr(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
    }
}


unsafe extern "C" fn snake_side_smash_3_snd(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack"));
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_squat_gear"));
    }
}


unsafe extern "C" fn snake_side_smash_3_eff(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 4, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 5.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3, 0.11, 0, 0, 0, 0.34, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3.5, 0.05, 0, 180, 0, 0.15, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light2"), Hash40::new("haver"), 0, -0.1, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light1"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light2"), false, true);
    }
}

////changed down-smash to spinning double kick

unsafe extern "C" fn snake_down_smash_game(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 12.0, 25, 87, 0, 30, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("legr"), 12.0, 25, 87, 0, 30, 3.3, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 12.0, 25, 87, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    FT_DESIRED_RATE(fighter, 19.0, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 14.0, 20, 88, 0, 30, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("legl"), 14.0, 20, 88, 0, 30, 3.3, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 14.0, 20, 88, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


unsafe extern "C" fn snake_down_smash_snd(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack_smash_l"));
        PLAY_SE(fighter, Hash40::new("se_snake_smash_l01"));
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack"));
        // PLAY_SE(fighter, Hash40::new("vc_snake_attack04"));
        // PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack_smash_s"));
        PLAY_SE(fighter, Hash40::new("se_snake_smash_l02"));
    }
}


unsafe extern "C" fn snake_down_smash_eff(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        // EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 13, -1, 35, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 5, 7, 9, 10, -70, 6, 1, true);
    }
    // frame(lua_state, 13.0);
    // if is_excute(fighter) {
    //     EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, 17, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
    // }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 9, -2, 14, -180, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, -17, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
    }
}

//charge

unsafe extern "C" fn snake_down_smash_charge_exp(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.2, 0.2, -1, 0.7, 0.5, -1, Hash40::new("invalid"));
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, 0);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold2"), 0, true, 0);
    }
}


unsafe extern "C" fn snake_down_smash_charge_eff(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    frame(lua_state, 5.0);
    for _ in 0..34 {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, true);
        }
        wait(lua_state, 5.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, false);
        }
    }
}


pub fn install() {
    smashline::Agent::new("snake")
        .acmd("game_attacks4", snake_side_smash_game)
        .acmd("expression_attacks4", snake_side_smash_expr)
        .acmd("sound_attacks4", snake_side_smash_snd)
        .acmd("effect_attacks4", snake_side_smash_eff)
        .acmd("effect_attacks4charge", snake_side_smash_charge_eff)
        .acmd("game_attacks4s2", snake_side_smash_2_game)
        .acmd("expression_attacks4s2", snake_side_smash_2_expr)
        .acmd("sound_attacks4s2", snake_side_smash_2_snd)
        .acmd("effect_attacks4s2", snake_side_smash_2_eff)
        .acmd("game_attacks4s3", snake_side_smash_3_game)
        .acmd("expression_attacks4s3", snake_side_smash_3_expr)
        .acmd("sound_attacks4s3", snake_side_smash_3_snd)
        .acmd("effect_attacks4s3", snake_side_smash_3_eff)
        .acmd("game_attacklw4", snake_down_smash_game)
        .acmd("sound_attacklw4", snake_down_smash_snd)
        .acmd("effect_attacklw4", snake_down_smash_eff)
        .acmd("expression_attacklw4charge", snake_down_smash_charge_exp)
        .acmd("effect_attacklw4charge", snake_down_smash_charge_eff)
        .install();
}
