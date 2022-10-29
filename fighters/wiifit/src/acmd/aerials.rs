
use super::*;


#[acmd_script( agent = "wiifit", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let is_zen_mode = WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if is_zen_mode {
            FT_MOTION_RATE(fighter, 7.0/8.0);
        }
        else {
            FT_MOTION_RATE(fighter, 5.0/8.0);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        let zen_mul = if is_zen_mode { 1.0 } else { 0.0 };
        let rate = if is_zen_mode { 12.0/(12.0 - 8.0) } else { 1.0 };
        let angle = if is_zen_mode { 90 } else { 110 };
        let bkb = if is_zen_mode { 0 } else { 50 };
        let fkb = if is_zen_mode { 50 } else { 0 };
        let kbg = if is_zen_mode { 100 } else { 72 };
        FT_MOTION_RATE(fighter, rate);
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 5.0, angle, kbg, fkb, bkb, 4.0 + 0.5 * zen_mul, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("head"), 5.0, angle, kbg, fkb, bkb, 4.0 + 0.5 * zen_mul, -1.0, 1.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 5.0, 90, kbg, fkb, bkb, 4.0 + 0.5 * zen_mul, 0.0, 0.0, -0.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 5.0, angle, kbg, fkb, bkb, 4.0 + 0.5 * zen_mul, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 4, 0, Hash40::new("footl"), 5.0, angle, kbg, fkb, bkb, 4.0 + 0.5 * zen_mul, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if is_zen_mode {
            AttackModule::clear_all(boma);
        }
        else {
            AttackModule::clear(boma, 2, false);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        let zen_mul = if is_zen_mode { 1.0 } else { 0.0 };
        let angle = if is_zen_mode { 361 } else { 50 };
        FT_MOTION_RATE(fighter, 1.0);
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 361, 80, 0, 45, 8.0 + 0.5 * zen_mul, 0.0, 5.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "wiifit", script = "effect_attackairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn wiifit_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) && WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
        crate::opff::start_ring(utils::util::get_fighter_common_from_accessor(boma), 17.0, 1.0, 2.0, Hash40::new("top"), Vector3f::new(0.0, 9.0, 1.5), Vector3f::new(3000.0, 0.7, 0.7), Vector3f::new(0.7, 1000.0, 0.7), true);
    }
}

#[acmd_script( agent = "wiifit", script = "game_landingairn" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_landing_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let landing_lag = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_n"), 0);
    if is_excute(fighter) && WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
        FT_MOTION_RATE(fighter, (landing_lag + 2.0)/landing_lag);
    }
}

#[acmd_script( agent = "wiifit", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let is_zen_mode = WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !is_zen_mode {
            FT_MOTION_RATE(fighter, 0.75);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        let zen_mul = if is_zen_mode { 1.0 } else { 0.0 };
        let bkb = if is_zen_mode { 40 } else { 20 };
        let kbg = if is_zen_mode { 77 } else { 102 };
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0, 50, kbg, 0, bkb, 4.5 + 1.0 * zen_mul, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 50, kbg, 0, bkb, 3.5 + 0.5 * zen_mul, -2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 50, kbg, 0, bkb, 3.5 + 0.5 * zen_mul, 0.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        /* Ground-only */
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 10.0, 300, 68, 0, 15, 3.5 + 0.5 * zen_mul, 4.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        /* Air-only */
        ATTACK(fighter, 4, 0, Hash40::new("kneer"), 10.0, 300, 68, 0, 15, 3.5 + 0.5 * zen_mul, 4.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        let zen_mul = if is_zen_mode { 1.0 } else { 0.0 };
        AttackModule::clear(boma, 4, false);
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.5, 55, 102, 0, 20, 4.0 + 0.5 * zen_mul, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 7.5, 55, 102, 0, 20, 3.5 + 0.5 * zen_mul, -2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 6.0, 45, 102, 0, 20, 3.5 + 0.5 * zen_mul, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 7.5, 55, 102, 0, 20, 3.5 + 0.5 * zen_mul, 0.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.640);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
}

#[acmd_script( agent = "wiifit", script = "effect_attackairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn wiifit_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) && WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
        crate::opff::start_ring(utils::util::get_fighter_common_from_accessor(boma), 17.0, 0.75, 1.0, Hash40::new("handr"), Vector3f::new(0.0, 0.0, 0.0), Vector3f::new(3000.0, 0.7, 0.7), Vector3f::new(0.7, 1000.0, 0.7), true);
    }
}

#[acmd_script( agent = "wiifit", script = "game_landingairf" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_landing_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let landing_lag = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_f"), 0);
    if is_excute(fighter) && WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
        FT_MOTION_RATE(fighter, (landing_lag + 1.0)/landing_lag);
    }
}

#[acmd_script( agent = "wiifit", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let is_zen_mode = WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if is_zen_mode {
            FT_MOTION_RATE(fighter, 6.0/(5.0 - 1.0));
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        let zen_mul = if is_zen_mode { 1.0 } else { 0.0 };
        let kbg = if is_zen_mode { 82 } else { 102 };
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 40, kbg, 0, 15, 3.5 + 0.5 * zen_mul, 0.0, 4.3, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 40, kbg, 0, 15, 4.0 + 0.5 * zen_mul, 0.0, 4.3, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 40, kbg, 0, 40, 4.5 + 0.5 * zen_mul, 0.0, 4.3, -10.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        let zen_mul = if is_zen_mode { 1.0 } else { 0.0 };
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 102, 0, 25, 2.5 + 0.5 * zen_mul, 0.0, 4.3, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 102, 0, 25, 3.0 + 0.5 * zen_mul, 0.0, 4.3, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 361, 102, 0, 25, 3.5 + 0.5 * zen_mul, 0.0, 4.3, -10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "wiifit", script = "effect_attackairb" , category = ACMD_EFFECT , low_priority)]
unsafe fn wiifit_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) && WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
        crate::opff::start_ring(utils::util::get_fighter_common_from_accessor(boma), 10.0, 1.0, 1.25, Hash40::new("top"), Vector3f::new(0.0, 4.0, -8.0), Vector3f::new(3000.0, 0.7, 0.7), Vector3f::new(0.7, 1000.0, 0.7), true);
    }
}

#[acmd_script( agent = "wiifit", script = "game_landingairb" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_landing_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let landing_lag = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_b"), 0);
    if is_excute(fighter) && WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
        FT_MOTION_RATE(fighter, (landing_lag + 2.0)/landing_lag);
    }
}

#[acmd_script( agent = "wiifit", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let is_zen_mode = WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if is_zen_mode {
            FT_MOTION_RATE(fighter, 7.0/(6.0 - 1.0))
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        let zen_mul = if is_zen_mode { 1.0 } else { 0.0 };
        let bkb = if is_zen_mode { 80 } else { 60 };
        let kbg = if is_zen_mode { 62 } else { 82 };
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("bust"), 10.0, 72, kbg, 0, bkb, 4.5 + 0.5 * zen_mul, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("bust"), 10.0, 72, kbg, 0, bkb, 4.0 + 0.5 * zen_mul, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("bust"), 10.0, 72, kbg, 0, bkb, 3.5 + 0.5 * zen_mul, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "wiifit", script = "effect_attackairhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn wiifit_attack_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) && WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
        crate::opff::start_ring(utils::util::get_fighter_common_from_accessor(boma), 10.0, 1.0, 1.25, Hash40::new("head"), Vector3f::new(3.0, 0.0, 0.0), Vector3f::new(3000.0, 0.7, 0.7), Vector3f::new(0.7, 1000.0, 0.7), true);
    }
}

#[acmd_script( agent = "wiifit", script = "game_landingairhi" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_landing_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let landing_lag = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_hi"), 0);
    if is_excute(fighter) && WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
        FT_MOTION_RATE(fighter, (landing_lag + 2.0)/landing_lag);
    }
}

#[acmd_script( agent = "wiifit", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let is_zen_mode = WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        let rate = if is_zen_mode { 15.0 } else { 13.0 };
        FT_MOTION_RATE(fighter, rate/(20.0 - 1.0));
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        let zen_mul = if is_zen_mode { 1.0 } else { 0.0 };
        /* Ground-only */
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 270, 100, 0, 40, 4.0 + 1.0 * zen_mul, 0.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 14.0, 270, 100, 0, 40, 4.0 + 1.0 * zen_mul, 0.0, 6.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        /* Air-only */
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 270, 54, 0, 20, 4.0 + 1.0 * zen_mul, 0.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 270, 54, 0, 20, 4.0 + 1.0 * zen_mul, 0.0, 6.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        let zen_mul = if is_zen_mode { 1.0 } else { 0.0 };
        AttackModule::clear(boma, 1, false);
        AttackModule::clear(boma, 3, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 102, 0, 30, 4.0 + 1.0 * zen_mul, 0.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 361, 102, 0, 30, 4.0 + 1.0 * zen_mul, 0.0, 6.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "wiifit", script = "effect_attackairlw" , category = ACMD_EFFECT , low_priority)]
unsafe fn wiifit_attack_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 20.0);
    if is_excute(fighter) && WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
        crate::opff::start_ring(utils::util::get_fighter_common_from_accessor(boma), 27.0, 1.25, 1.75, Hash40::new("kneer"), Vector3f::new(1.5, -1.0, 0.0), Vector3f::new(3000.0, 0.7, 0.7), Vector3f::new(0.7, 1000.0, 0.7), true);
    }
}

#[acmd_script( agent = "wiifit", script = "game_landingairlw" , category = ACMD_GAME , low_priority)]
unsafe fn wiifit_landing_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let landing_lag = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_lw"), 0);
    if is_excute(fighter) && WorkModule::is_flag(boma, vars::wiifit::instance::IS_ZEN_MODE) {
        FT_MOTION_RATE(fighter, (landing_lag + 2.0)/landing_lag);
    }
}

pub fn install() {
    install_acmd_scripts!(
        wiifit_attack_air_n_game,
        wiifit_attack_air_n_effect,
        wiifit_landing_air_n_game,
        wiifit_attack_air_f_game,
        wiifit_attack_air_f_effect,
        wiifit_landing_air_f_game,
        wiifit_attack_air_b_game,
        wiifit_attack_air_b_effect,
        wiifit_landing_air_b_game,
        wiifit_attack_air_hi_game,
        wiifit_attack_air_hi_effect,
        wiifit_landing_air_hi_game,
        wiifit_attack_air_lw_game,
        wiifit_attack_air_lw_effect,
        wiifit_landing_air_lw_game,
    );
}

