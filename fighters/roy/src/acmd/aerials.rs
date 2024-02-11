
use super::*;


#[acmd_script( agent = "roy", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn roy_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        // Air-only hitboxes
        ATTACK(fighter, 0, 0, Hash40::new("sword1"), 4.0, 100, 30, 0, 30, 4.8, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 4.0, 100, 30, 0, 30, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 4.0, 100, 30, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("sword1"), 4.0, 90, 30, 0, 30, 3.5, 0.0, 0.0, 8.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        // Ground-only hitboxes
        ATTACK(fighter, 4, 0, Hash40::new("sword1"), 4.0, 100, 30, 0, 45, 4.8, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("armr"), 4.0, 100, 30, 0, 45, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 6, 0, Hash40::new("hip"), 4.0, 100, 30, 0, 45, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 7, 0, Hash40::new("sword1"), 4.0, 90, 30, 0, 45, 3.5, 0.0, 0.0, 8.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 3, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 4, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 5, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 6, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 7, 3.0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("sword1"), 10.0, 361, 90, 0, 45, 4.9, 2.0, 0.0, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 90, 0, 45, 4.5, -1.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 10.0, 361, 90, 0, 45, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("sword1"), 10.0, 361, 90, 0, 45, 3.75, 2.0, 0.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        //ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.1);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "roy", script = "effect_attackairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn roy_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), 7, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(fighter, 1.8);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), 7, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);

        EFFECT_FOLLOW(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("roy_fire"), false, false);
        AFTER_IMAGE_OFF(fighter, 2);
    }
}

#[acmd_script( agent = "roy", script = "sound_attackairn" , category = ACMD_SOUND , low_priority)]
unsafe fn roy_attack_air_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_roy_attackair_n01"));
    }
}

#[acmd_script( agent = "roy", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn roy_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        FT_MOTION_RATE(fighter, 7.0/(13.75-1.0));
    }
    frame(lua_state, 13.75);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0/(17.0-13.75));
        ATTACK(fighter, 0, 0, Hash40::new("sword1"), 11.0, 50, 80, 0, 45, 4.25, 0.0, 0.0, 2.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 11.0, 50, 80, 0, 45, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("colonells"), 11.0, 50, 80, 0, 45, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("sword1"), 9.0, 45, 65, 0, 30, 3.5, 0.0, 0.0, 9.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "roy", script = "expression_attackairf", category = ACMD_EXPRESSION, low_priority )]
unsafe fn roy_attack_air_f_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
}

#[acmd_script( agent = "roy", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn roy_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("sword1"), 14.0, 50, 111, 0, 25, 4.5, 0.0, 0.0, 2.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 14.0, 50, 111, 0, 25, 4.25, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("shoulderl"), 14.0, 50, 111, 0, 25, 3.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("sword1"), 11.0, 40, 95, 0, 20, 3.5, 0.0, 0.0, 9.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 111, 0, 25, 4.5, 5.0, 16.0, -4.0, Some(-5.0), Some(16.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
    }    
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        PostureModule::reverse_lr(boma);
    }
    
}

#[acmd_script( agent = "roy", script = "effect_attackairb" , category = ACMD_EFFECT , low_priority)]
unsafe fn roy_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), 7, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);

        EFFECT_FOLLOW(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("roy_fire"), false, false);
    }
}

#[acmd_script( agent = "roy", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn roy_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 3, 0, Hash40::new("sword1"), 10.0, 77, 55, 0, 45, 4.25, 0.0, 0.0, 2.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 77, 55, 0, 45, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("colonells"), 10.0, 77, 55, 0, 45, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("sword1"), 7.0, 80, 55, 0, 30, 3.5, 0.0, 0.0, 9.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE_RANGE(fighter, 12.0, 42.0, 33.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "roy", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn roy_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        // Air-only spike hitboxes
        ATTACK(fighter, 0, 0, Hash40::new("colonells"), 15.0, 290, 60, 0, 18, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("armr"), 15.0, 290, 60, 0, 18, 3.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		// Ground-only spike hitboxes
        ATTACK(fighter, 1, 0, Hash40::new("colonells"), 15.0, 281, 71, 0, 40, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("armr"), 15.0, 281, 71, 0, 40, 3.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		// Midspot/sourspot
        ATTACK(fighter, 2, 0, Hash40::new("sword1"), 11.0, 90, 60, 0, 50, 4.4, 1.0, -3.0, 2.2, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("sword1"), 9.0, 37, 60, 0, 37, 3.5, 1.0, -3.0, 8.25, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    //frame(lua_state, 57.0);
    //if is_excute(fighter) {
    //    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
    //}
    
}

#[acmd_script( agent = "roy", script = "effect_attackairlw" , category = ACMD_EFFECT , low_priority)]
unsafe fn roy_attack_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12.5, 12.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), 20, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
        EFFECT_FOLLOW(fighter, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 10.55, 0, 0, 0, 1.05, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("roy_sword_light"), false, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
}

#[acmd_script( agent = "roy", script = "sound_attackairlw" , category = ACMD_SOUND , low_priority)]
unsafe fn roy_attack_air_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_roy_rnd_attack_air"));
        PLAY_SE(fighter, Hash40::new("se_roy_attackair_h01"));
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_roy_attackair_l01"));
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        roy_attack_air_n_game,
        roy_attack_air_n_effect,
        roy_attack_air_n_sound,
        roy_attack_air_f_game,
        roy_attack_air_f_expression,
        roy_attack_air_b_game,
        roy_attack_air_b_effect,
        roy_attack_air_hi_game,
        roy_attack_air_lw_game,
        roy_attack_air_lw_effect,
        roy_attack_air_lw_sound,
    );
}

