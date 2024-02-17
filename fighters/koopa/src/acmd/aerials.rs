use super::*;

#[acmd_script( agent = "koopa", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn koopa_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.5, 6.5);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 61, 80, 0, 45, 6.0, 0.0, 7.6, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 61, 80, 0, 45, 6.0, 0.0, 7.6, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 61, 80, 0, 45, 6.0, 0.0, 7.6, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 61, 80, 0, 45, 6.0, 0.0, 7.6, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 8.0, 8.0);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "koopa", script = "effect_attackairn", category = ACMD_EFFECT, low_priority )]
unsafe fn koopa_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    for _ in 0..2 {
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopa_shell_b"), Hash40::new("koopa_shell_a"), Hash40::new("rot"), 0, 03, 0, 0, 0, 0, 1.45, true, *EF_FLIP_NONE);
        }
        wait(lua_state, 5.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopa_shell_b"), Hash40::new("koopa_shell_a"), Hash40::new("rot"), 0, 3, 0, 0, 0, 0, 1.45, true, *EF_FLIP_NONE);
        }
        wait(lua_state, 5.0);
    }
    if is_excute(fighter) {
        LAST_EFFECT_SET_RATE(fighter,1.2);
    }
}

#[acmd_script( agent = "koopa", script = "sound_attackairn", category = ACMD_SOUND, low_priority )]
unsafe fn koopa_attack_air_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_koopa_attackair_l01"));
    }
}

#[acmd_script( agent = "koopa", script = "expression_attackairn", category = ACMD_EXPRESSION, low_priority )]
unsafe fn koopa_attack_air_n_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("top"), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ItemModule::set_attach_item_visibility(boma, false, *ATTACH_ITEM_GROUP_ALL as u8);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 7);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, true, 0);
        ItemModule::set_attach_item_visibility(boma, true, *ATTACH_ITEM_GROUP_ALL as u8);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

#[acmd_script( agent = "koopa", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn koopa_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    FT_MOTION_RATE_RANGE(fighter, 4.0, 11.0, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE_RANGE(fighter, 11.0, 14.25, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("colonells"), 12.0, 361, 85, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 361, 85, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 15.0, 361, 75, 0, 50, 6.5, 6.0, 0.0, -0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 14.25);
    FT_MOTION_RATE_RANGE(fighter, 14.25, 16.0, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "koopa", script = "effect_attackairf", category = ACMD_EFFECT, low_priority )]
unsafe fn koopa_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), 0, 16, 4, -1, 16, -107, 1.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script( agent = "koopa", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn koopa_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 40, 75, 0, 25, 8.5, 0.0, 8.5, -14.0, Some(0.0), Some(8.5), Some(-9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 40, 75, 0, 25, 3.5, 0.0, 2.5, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 81, 0, 21, 8.5, 0.0, 8.5, -11.0, Some(0.0), Some(8.5), Some(-10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 361, 81, 0, 21, 3.5, 0.0, 2.5, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "koopa", script = "expression_attackairb", category = ACMD_EXPRESSION, low_priority )]
unsafe fn koopa_attack_air_b_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
}

#[acmd_script( agent = "koopa", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn koopa_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 6.0, 9.0);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mouth2"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(fighter, 8.0, 17.0, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("head"), 16.0, 80, 90, 0, 45, 7.5, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 17.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "koopa", script = "effect_attackairhi", category = ACMD_EFFECT, low_priority )]
unsafe fn koopa_attack_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1.5, 14.5, 2, -153, 90, -49, 1.5, true, *EF_FLIP_YZ, 0.7);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), true, true);
    }
}

#[acmd_script( agent = "koopa", script = "expression_attackairhi", category = ACMD_EXPRESSION, low_priority )]
unsafe fn koopa_attack_air_hi_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "koopa", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn koopa_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 16.0, 9.0);
    frame(lua_state, 16.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 17.0);
    FT_MOTION_RATE_RANGE(fighter, 17.0, 49.0, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 366, 100, 0, 30, 9.0, 0.0, 1.0, -1.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 49.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 55, 90, 0, 60, 9.75, 0.0, 1.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 67.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "koopa", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn koopa_attack_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -5, -5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("koopa_atk_fall"), Hash40::new("top"), 0, 2.5, 0, 0, 0, 0, 0.65, true);
    }
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("koopa_atk_fall"), false, false);
    }
}

#[acmd_script( agent = "koopa", script = "game_landingairlw", category = ACMD_GAME, low_priority )]
unsafe fn koopa_landing_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 20, 30, 0, 60, 7.0, 0.0, 5.0, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 20, 30, 0, 60, 7.0, 0.0, 5.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mouth2"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        koopa_attack_air_n_game,
        koopa_attack_air_n_effect,
        koopa_attack_air_n_sound,
        koopa_attack_air_n_expression,

        koopa_attack_air_f_game,
        koopa_attack_air_f_effect,

        koopa_attack_air_b_game,
        koopa_attack_air_b_expression,

        koopa_attack_air_hi_game,
        koopa_attack_air_hi_effect,
        koopa_attack_air_hi_expression,
        
        koopa_attack_air_lw_game,
        koopa_attack_air_lw_effect,
        koopa_landing_air_lw_game,
    );
}