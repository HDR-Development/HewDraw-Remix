
use super::*;

unsafe extern "C" fn krool_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if WorkModule::get_float(fighter.module_accessor, 0x4d) >= 1.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        let charge = VarModule::get_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE);
        let damage = 0.15 * (charge as f32);
        let hitlag = 1.0 + 0.025 * (charge as f32);
        let sound = if charge >= 10 { *COLLISION_SOUND_ATTR_HEAVY } else { *COLLISION_SOUND_ATTR_PUNCH };
        let shieldstun = 1.0 + 0.02 * (charge as f32);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0 + damage, 361, 75, 0, 63, 5.0, 0.0, 14.0, 15.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0 + damage, 361, 75, 0, 63, 6.5, 0.0, 11.0, 8.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("havel"), 13.0 + damage, 42, 75, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, shieldstun);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn krool_attack_s3_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), -4, 16.5, 5, -10, -55, 16, 1.4, true, *EF_FLIP_YZ, 0.4);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), 4, 18, 5, -351, -55, 185, 1.4, true, *EF_FLIP_YZ, 0.4);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("krool_attack_s3_impact"), Hash40::new("top"), 0, 20, 22, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 1);
    }
}

unsafe extern "C" fn krool_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if WorkModule::get_float(fighter.module_accessor, 0x4d) >= 1.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        let charge = VarModule::get_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE);
        let damage = 0.15 * (charge as f32);
        let hitlag = 1.0 + 0.025 * (charge as f32);
        let sound = if charge >= 10 { *COLLISION_SOUND_ATTR_HEAVY } else { *COLLISION_SOUND_ATTR_PUNCH };
        let shieldstun = 1.0 + 0.02 * (charge as f32);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0 + damage, 361, 75, 0, 63, 5.0, 0.0, 11.0, 15.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0 + damage, 361, 75, 0, 63, 6.5, 0.0, 11.0, 8.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("havel"), 13.0 + damage, 42, 75, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, shieldstun);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn krool_attack_s3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), -4, 13, 7, 24, -55, -7, 1.4, true, *EF_FLIP_YZ, 0.4);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), 4, 16, 7, -4, -55, -139, 1.4, true, *EF_FLIP_YZ, 0.4);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("krool_attack_s3_impact"), Hash40::new("top"), 0, 12, 24, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 1);
    }
}

unsafe extern "C" fn krool_attack_s3_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if WorkModule::get_float(fighter.module_accessor, 0x4d) >= 1.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        let charge = VarModule::get_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE);
        let damage = 0.15 * (charge as f32);
        let hitlag = 1.0 + 0.025 * (charge as f32);
        let sound = if charge >= 10 { *COLLISION_SOUND_ATTR_HEAVY } else { *COLLISION_SOUND_ATTR_PUNCH };
        let shieldstun = 1.0 + 0.02 * (charge as f32);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0 + damage, 361, 75, 0, 63, 5.0, 0.0, 7.0, 14.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0 + damage, 361, 75, 0, 63, 6.5, 0.0, 11.0, 8.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("havel"), 13.0 + damage, 42, 75, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, shieldstun);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn krool_attack_s3_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), -4, 11, 5, 30, -49, -19, 1.4, true, *EF_FLIP_YZ, 0.4);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), 4, 10, 5, -15, -48, -124, 1.4, true, *EF_FLIP_YZ, 0.4);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("krool_attack_s3_impact"), Hash40::new("top"), 0, 6, 22, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 1);
    }
}

unsafe extern "C" fn krool_attack_s3_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_krool_attackhard_s01"));
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_krool_attackhard_s02"));
        if VarModule::get_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE) == 0 {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_krool_rnd_attack"));
        }
    }
    wait(lua_state, 8.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_krool_attackhard_s02"));
        PLAY_SE(fighter, Hash40::new("se_krool_attackhard_s03"));
    }
}

unsafe extern "C" fn krool_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 2.0, 5.0);
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(fighter, 2.0, 4.0, 2.0);
    if is_excute(fighter) {
        if WorkModule::get_float(fighter.module_accessor, 0x4d) >= 1.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
        }
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        let charge = VarModule::get_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE);
        let damage = 0.15 * (charge as f32);
        let hitlag = 1.0 + 0.025 * (charge as f32);
        let shieldstun = 1.0 + 0.02 * (charge as f32);
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.0 + damage, 84, 63, 0, 76, 4.0, 2.2, 0.5, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 12.0 + damage, 84, 63, 0, 76, 4.0, -1.0, 0.5, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 12.0 + damage, 84, 63, 0, 76, 5.0, 7.5, 2.5, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, shieldstun);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        let charge = VarModule::get_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE);
        let damage = 0.05 * (charge as f32);
        let hitlag = 1.0 + 0.025 * (charge as f32);
        let shieldstun = 1.0 + 0.02 * (charge as f32);
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 7.0 + damage, 84, 49, 0, 60, 4.0, 2.2, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 7.0 + damage, 84, 49, 0, 60, 4.0, -1.0, 0.5, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 9.0 + damage, 84, 49, 0, 60, 5.0, 7.0, 2.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, shieldstun);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE_RANGE(fighter, 14.0, 38.0, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 38.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}

unsafe extern "C" fn krool_attack_hi3_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_krool_attackhard_h01"));
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        if VarModule::get_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE) == 0 {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_krool_rnd_attack"));
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_krool_attackhard_h01"));
        PLAY_SE(fighter, Hash40::new("se_krool_attackhard_h02"));
    }
}

unsafe extern "C" fn krool_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if WorkModule::get_float(fighter.module_accessor, 0x4d) >= 1.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        let charge = VarModule::get_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE);
        let damage = 0.15 * (charge as f32);
        let hitlag = 1.0 + 0.025 * (charge as f32);
        let shieldstun = 1.0 + 0.02 * (charge as f32);
        // ground-only
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 13.0 + damage, 270, 35, 0, 100, 5.0, 2.5, -1.5, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // air-only
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 13.0 + damage, 361, 85, 0, 40, 5.0, 2.5, -1.5, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, shieldstun);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        let charge = VarModule::get_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE);
        let damage = 0.05 * (charge as f32);
        let hitlag = 1.0 + 0.025 * (charge as f32);
        let shieldstun = 1.0 + 0.02 * (charge as f32);
        // ground-only
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0 + damage, 270, 35, 0, 100, 5.0, 0.0, 3.5, 16.5, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0 + damage, 85, 25, 0, 100, 5.0, 0.0, 3.5, 10.0, Some(0.0), Some(3.5), Some(23.0), hitlag * 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // air-only
        ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0 + damage, 270, 35, 0, 30, 5.0, 0.0, 3.5, 16.5, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 2, shieldstun);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn krool_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -3, 24, 12, 70, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.6);
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -3, 24, 12, 70, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.6);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("krool_smash_lw_ground"), Hash40::new("krool_smash_lw_ground"), Hash40::new("top"), 0, 0, 20, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 20, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn krool_attack_lw3_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_krool_attackhard_l01"));
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if VarModule::get_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE) == 0 {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_krool_rnd_attack"));
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_krool_attackhard_l02"));
    }
}

pub fn install() {
    smashline::Agent::new("krool")
        .acmd("game_attacks3hi", krool_attack_s3_hi_game)
        .acmd("effect_attacks3hi", krool_attack_s3_hi_effect)
        .acmd("game_attacks3", krool_attack_s3_s_game)
        .acmd("effect_attacks3", krool_attack_s3_effect)
        .acmd("game_attacks3lw", krool_attack_s3_lw_game)
        .acmd("effect_attacks3lw", krool_attack_s3_lw_effect)
        .acmd("sound_attacks3hi", krool_attack_s3_sound)
        .acmd("sound_attacks3", krool_attack_s3_sound)
        .acmd("sound_attacks3lw", krool_attack_s3_sound)
        .acmd("game_attackhi3", krool_attack_hi3_game)
        .acmd("sound_attackhi3", krool_attack_hi3_sound)
        .acmd("game_attacklw3", krool_attack_lw3_game)
        .acmd("effect_attacklw3", krool_attack_lw3_effect)
        .acmd("sound_attacklw3", krool_attack_lw3_sound)
        .install();
}
