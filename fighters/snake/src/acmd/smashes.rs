
use super::*;


//first hit
#[acmd_script( agent = "snake", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn snake_side_smash_game(fighter : &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, false, 0);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 6.0, 361, 25, 0, 20, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.0, 170, 20, 0, 25, 3.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 361, 15, 0, 30, 2.0, 0.0, 5.0, 5.0, Some(0.0), Some(9.0), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_ENABLE); 
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_ENABLE); 
    }
    frame(fighter.lua_state_agent, 54.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, ArticleOperationTarget(0));
    }
}
#[acmd_script( agent = "snake", script = "expression_attacks4", category = ACMD_EXPRESSION, low_priority )]
unsafe fn snake_side_smash_expr(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}
#[acmd_script( agent = "snake", script = "sound_attacks4", category = ACMD_SOUND, low_priority )]
unsafe fn snake_side_smash_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_squat_gear"));
    }
}
#[acmd_script( agent = "snake", script = "effect_attacks4", category = ACMD_EFFECT, low_priority )]
unsafe fn snake_side_smash_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 4, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 5.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3, 0.11, 0, 0, 0, 0.34, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3.5, 0.05, 0, 180, 0, 0.15, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light2"), Hash40::new("haver"), 0, -0.1, 0, 0, 0, 0, 0.5, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light1"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light2"), false, true);
    }
}
//charge
#[acmd_script( agent = "snake", script = "effect_attacks4charge", category = ACMD_EFFECT, low_priority )]
unsafe fn snake_side_smash_charge_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    for _ in 0..34 {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, false);
        }
    }
}

//second hit
#[acmd_script( agent = "snake", script = "game_attacks4s2", category = ACMD_GAME, low_priority )]
unsafe fn snake_side_smash_2_game(fighter : &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        //used extra stun to better link into third hit
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.0, 65, 95, 0, 85, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze_ghost"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.0, 65, 95, 0, 85, 3.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze_ghost"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_paralyze_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 25, false);
        AttackModule::set_paralyze_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 25, false);
        //fake/empty hitboxes to create particle effects
        ATTACK(fighter, 2, 1, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 1, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 3.0, 0.0, 4.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_ENABLE); 
        // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_ENABLE); 
        // WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, ArticleOperationTarget(0));
    }
}
#[acmd_script( agent = "snake", script = "expression_attacks4s2", category = ACMD_EXPRESSION, low_priority )]
unsafe fn snake_side_smash_2_expr(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}
#[acmd_script( agent = "snake", script = "sound_attacks4s2", category = ACMD_SOUND, low_priority )]
unsafe fn snake_side_smash_2_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_squat_gear"));
    }
}
#[acmd_script( agent = "snake", script = "effect_attacks4s2", category = ACMD_EFFECT, low_priority )]
unsafe fn snake_side_smash_2_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 4, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 5.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3, 0.11, 0, 0, 0, 0.34, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3.5, 0.05, 0, 180, 0, 0.15, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light2"), Hash40::new("haver"), 0, -0.1, 0, 0, 0, 0, 0.5, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light1"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light2"), false, true);
    }
}

//third hit
#[acmd_script( agent = "snake", script = "game_attacks4s3", category = ACMD_GAME, low_priority )]
unsafe fn snake_side_smash_3_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.5, 361, 75, 0, 85, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.5, 361, 75, 0, 85, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, ArticleOperationTarget(0));
    }
}
#[acmd_script( agent = "snake", script = "expression_attacks4s3", category = ACMD_EXPRESSION, low_priority )]
unsafe fn snake_side_smash_3_expr(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}
#[acmd_script( agent = "snake", script = "sound_attacks4s3", category = ACMD_SOUND, low_priority )]
unsafe fn snake_side_smash_3_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_squat_gear"));
    }
}
#[acmd_script( agent = "snake", script = "effect_attacks4s3", category = ACMD_EFFECT, low_priority )]
unsafe fn snake_side_smash_3_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 4, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 5.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3, 0.11, 0, 0, 0, 0.34, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3.5, 0.05, 0, 180, 0, 0.15, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light2"), Hash40::new("haver"), 0, -0.1, 0, 0, 0, 0, 0.5, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light1"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light2"), false, true);
    }
}
////

////changed down-smash to spinning double kick
//attack
#[acmd_script( agent = "snake", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn snake_down_smash_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 12.0, 25, 87, 0, 30, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("legr"), 12.0, 25, 87, 0, 30, 3.3, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 12.0, 25, 87, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 14.0, 20, 88, 0, 30, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("legl"), 14.0, 20, 88, 0, 30, 3.3, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 14.0, 20, 88, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "snake", script = "sound_attacklw4", category = ACMD_SOUND, low_priority )]
unsafe fn snake_down_smash_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack_smash_l"));
        PLAY_SE(fighter, Hash40::new("se_snake_smash_l01"));
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack"));
        // PLAY_SE(fighter, Hash40::new("vc_snake_attack04"));
        // PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack_smash_s"));
        PLAY_SE(fighter, Hash40::new("se_snake_smash_l02"));
    }
}
#[acmd_script( agent = "snake", script = "effect_attacklw4", category = ACMD_EFFECT, low_priority )]
unsafe fn snake_down_smash_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        // EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 13, -1, 35, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 5, 7, 9, 10, -70, 6, 1, true);
    }
    // frame(fighter.lua_state_agent, 13.0);
    // if is_excute(fighter) {
    //     EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, 17, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
    // }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 9, -2, 14, -180, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, -17, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
    }
}
//charge
#[acmd_script( agent = "snake", script = "expession_attacklw4charge", category = ACMD_EXPRESSION, low_priority )]
unsafe fn snake_down_smash_charge_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.2, 0.2, -1, 0.7, 0.5, -1, Hash40::new("invalid"));
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, 0);
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, 0);
    }
}
#[acmd_script( agent = "snake", script = "effect_attacklw4charge", category = ACMD_EFFECT, low_priority )]
unsafe fn snake_down_smash_charge_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    for _ in 0..34 {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, false);
        }
    }
}
////

pub fn install() {
    install_acmd_scripts!(
        snake_side_smash_game,
        snake_side_smash_expr,
        snake_side_smash_snd,
        snake_side_smash_eff,
        snake_side_smash_charge_eff,
        snake_side_smash_2_game,
        snake_side_smash_2_expr,
        snake_side_smash_2_snd,
        snake_side_smash_2_eff,
        snake_side_smash_3_game,
        snake_side_smash_3_expr,
        snake_side_smash_3_snd,
        snake_side_smash_3_eff,
        snake_down_smash_game,
        snake_down_smash_snd,
        snake_down_smash_eff,
        snake_down_smash_charge_exp,
        snake_down_smash_charge_eff,
    );
}

