
use super::*;

#[acmd_script( agent = "nana", script = "game_throwlw_nana" , category = ACMD_GAME , low_priority)]
unsafe fn nana_throw_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 80, 120, 0, 45, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        // scale attack startup based on opponent weight
        let weight =  WorkModule::get_param_float(boma.get_grabbed_opponent_boma(), hash40("weight"), 0);
        FT_MOTION_RATE(fighter, ((weight/100.00) * 32.0) / 36.0);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        CHECK_FINISH_CAMERA(fighter, 10, 4);
        // FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        // FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        FT_MOTION_RATE(fighter, 17.0/(49.0 - 37.0));
    }
}

#[acmd_script( agent = "nana", script = "effect_throwlw_nana" , category = ACMD_EFFECT , low_priority)]
unsafe fn nana_throw_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -4.5, 5.4, 8, -90, 0, 0, 0.9, true);
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_crown"), Hash40::new("sys_crown"), Hash40::new("top"), 9, 0, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
}

#[acmd_script( agent = "nana", script = "sound_throwlw_nana" , category = ACMD_SOUND , low_priority)]
unsafe fn nana_throw_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_nana_attack04"));
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_s_01"));
        PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
    }
}

#[acmd_script( agent = "nana", script = "expression_throwlw_nana" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn nana_throw_lw_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 4, false, 0x50000000 /* default value */);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, 0x50000000 /* default value */);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 4);
    }
}

#[acmd_script( agent = "nana", script = "game_throwf_nana" , category = ACMD_GAME , low_priority)]
unsafe fn nana_throw_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 45, 60, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 60, 0, 50, 5.0, 0.0, 6.5, 10.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(fighter, 16, 9);
        // FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        // FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "nana", script = "effect_throwf_nana" , category = ACMD_EFFECT , low_priority)]
unsafe fn nana_throw_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("popo_attack_arc_b"), Hash40::new("popo_attack_arc_b"), Hash40::new("top"), 0.5, 8, 1, 163.714, 220.144, -140.09, 1.35, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "nana", script = "sound_throwf_nana" , category = ACMD_SOUND , low_priority)]
unsafe fn nana_throw_f_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
        PLAY_SE(fighter, Hash40::new("vc_nana_attack01"));
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_popo_swing_l"));
    }
}

#[acmd_script( agent = "nana", script = "expression_throwf_nana" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn nana_throw_f_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, 0x50000000 /* default value */);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "nana", script = "game_throwhi_nana" , category = ACMD_GAME , low_priority)]
unsafe fn nana_throw_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 88, 26, 0, 120, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        FT_MOTION_RATE(fighter, 28.0/20.0);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("havel"), 3.0, 70, 60, 0, 75, 5.0, 0.0, 6.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("havel"), 3.0, 70, 60, 0, 75, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(fighter, 2, 19);
        // FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        // FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "nana", script = "effect_throwhi_nana" , category = ACMD_EFFECT , low_priority)]
unsafe fn nana_throw_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("popo_attack_arc_b"), Hash40::new("popo_attack_arc_b"), Hash40::new("top"), -1.5, 10, -3, 180, -90, -270, 1.3, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "nana", script = "sound_throwhi_nana" , category = ACMD_SOUND , low_priority)]
unsafe fn nana_throw_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_nana_attack03"));
        PLAY_SE(fighter, Hash40::new("se_popo_swing_ll"));
    }
}

#[acmd_script( agent = "nana", script = "expression_throwhi_nana" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn nana_throw_hi_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, 0x50000000 /* default value */);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
}

#[acmd_script( agent = "nana", script = "game_throwb_nana" , category = ACMD_GAME , low_priority)]
unsafe fn nana_throw_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 60, 30, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 18, 10);
        // FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
        // FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "nana", script = "effect_throwb_nana" , category = ACMD_EFFECT , low_priority)]
unsafe fn nana_throw_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), -5, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_crown"), Hash40::new("sys_crown"), Hash40::new("top"), 16.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
}

#[acmd_script( agent = "nana", script = "sound_throwb_nana" , category = ACMD_SOUND , low_priority)]
unsafe fn nana_throw_b_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_nana_attack02"));
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

#[acmd_script( agent = "nana", script = "expression_throwb_nana" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn nana_throw_b_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 14);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, 0x50000000 /* default value */);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 10);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
}

pub fn install() {
    install_acmd_scripts!(
        nana_throw_lw_game,
        nana_throw_lw_effect,
        nana_throw_lw_sound,
        nana_throw_lw_expression,
        nana_throw_f_game,
        nana_throw_f_effect,
        nana_throw_f_sound,
        nana_throw_f_expression,
        nana_throw_hi_game,
        nana_throw_hi_effect,
        nana_throw_hi_sound,
        nana_throw_hi_expression,
        nana_throw_b_game,
        nana_throw_b_effect,
        nana_throw_b_sound,
        nana_throw_b_expression
    );
}

