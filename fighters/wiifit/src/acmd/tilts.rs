use super::*;

#[acmd_script( agent = "wiifit", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 11.0, 105, 65, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 4, 0, Hash40::new("footl"), 11.0, 105, 65, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 11.0, 361, 85, 0, 50, 3.5, 3.0, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 11.0, 361, 85, 0, 50, 3.5, 1.5, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 11.0, 105, 65, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "wiifit", script = "effect_attacks3" , category = ACMD_EFFECT , low_priority)]
unsafe fn wiifit_attack_s3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    // if is_excute(fighter) && WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
    //     crate::opff::start_ring(utils::util::get_fighter_common_from_accessor(boma), 14.0, 1.0, 1.5, Hash40::new("kneel"), Vector3f::new(1.5, 2.5, 0.0), Vector3f::new(3000.0, 0.7, 0.7), Vector3f::new(0.7, 1000.0, 0.7), false);
    // }

}

#[acmd_script( agent = "wiifit", script = "expression_attacks3" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn wiifit_attack_s3_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

#[acmd_script( agent = "wiifit", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 90, 70, 0, 80, 4.3, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 70, 0, 80, 3.0, 0.0, 10.0, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 10.0, 90, 70, 0, 80, 4.0, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0, 90, 70, 0, 80, 2.5, 0.0, 11.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("footl"), 10.0, 75, 70, 0, 80, 3.0, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "wiifit", script = "effect_attackhi3" , category = ACMD_EFFECT , low_priority)]
unsafe fn wiifit_attack_hi3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -5, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    // frame(lua_state, 5.0);
    // if is_excute(fighter) && WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
    //     crate::opff::start_ring(utils::util::get_fighter_common_from_accessor(boma), 14.0, 0.75, 1.5, Hash40::new("handr"), Vector3f::new(-0.25, -1.0, 0.0), Vector3f::new(3000.0, 0.7, 0.7), Vector3f::new(0.7, 1000.0, 0.7), false);
    // }
}

#[acmd_script( agent = "wiifit", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.75);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 0.0, 5.3);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 35, 86, 0, 30, 4.0, 0.0, 3.0, 10.5, Some(0.0), Some(4.8), Some(13.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 35, 86, 0, 30, 3.5, 0.0, 3.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 5.8, 5.8);
    }
    
}

#[acmd_script( agent = "wiifit", script = "effect_attacklw3" , category = ACMD_EFFECT , low_priority)]
unsafe fn wiifit_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("wiifit_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        // if WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
        //     crate::opff::start_ring(utils::util::get_fighter_common_from_accessor(boma), 14.0, 1.0, 1.5, Hash40::new("top"), Vector3f::new(11.0, 4.5, 0.0), Vector3f::new(3000.0, 0.7, 0.7), Vector3f::new(0.7, 1000.0, 0.7), false);
        // }
    }

}

pub fn install() {
    install_acmd_scripts!(
        wiifit_attack_s3_s_game,
        wiifit_attack_s3_effect,
        wiifit_attack_s3_expression,
        wiifit_attack_hi3_game,
        wiifit_attack_hi3_effect,
        wiifit_attack_lw3_game,
        wiifit_attack_lw3_effect,
    );
}