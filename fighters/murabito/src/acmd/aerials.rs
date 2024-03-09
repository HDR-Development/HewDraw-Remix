
use super::*;

unsafe extern "C" fn murabito_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.0, 361, 90, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("handr"), 12.0, 361, 90, 0, 30, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("handl"), 12.0, 361, 90, 0, 30, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.0, 361, 90, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("handr"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("handl"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

unsafe extern "C" fn murabito_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 12.0, 9.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SLINGSHOT, true, 0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 365, 45, 0, 35, 3.5, 0.0, 5.5, -0.5, Some(0.0), Some(5.5), Some(-3.5),   0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 365, 45, 0, 35, 5.5, 0.0, 5.5, -10.0, None, None, None,                  0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 80, 45, 0, 35, 3.5, 0.0, 5.5, -0.5, Some(0.0), Some(5.5), Some(-3.5),    0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 80, 45, 0, 35, 5.5, 0.0, 5.5, -10.0, None, None, None,                   0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 135, 0, 40, 3.5, 0.0, 5.5, -0.5, Some(0.0), Some(5.5), Some(-3.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 361, 135, 0, 40, 6.0, 0.0, 5.5, -10.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 28.0);
    FT_MOTION_RATE(fighter, 19.0/(41.0-28.0));
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

//Not an ideal solution as hitbox will appear a frame before effects begin. Will have to revisit this in the future for a better solution

unsafe extern "C" fn murabito_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0.0, -4.0, 0.0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.6);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("murabito_firework"), Hash40::new("top"), 0, 4.5, -10.0, 0, -90, 0, 0.8, false);
        LAST_EFFECT_SET_RATE(fighter, 1.25);
        EFFECT(fighter, Hash40::new("murabito_erase_smoke"), Hash40::new("top"), 0.0, 4.5, -5.0, 0, -90, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("murabito_erase_smoke"), Hash40::new("top"), 0.0, 4.5, -10.0, 0, -90, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(fighter, 1.25);
    }

}

unsafe extern "C" fn murabito_attack_air_b_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_murabito_attackair_b01"));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_murabito_smash_h02"));
    }

}

unsafe extern "C" fn murabito_attack_air_b_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn murabito_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        if WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 85, 105, 0, 30, 5.2, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
        if WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 2 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 85, 105, 0, 30, 6.5, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
        if !(WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 2 || WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 1) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 85, 105, 0, 30, 7.5, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        if WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 90, 60, 0, 50, 4.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
        if WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 2 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 90, 60, 0, 50, 5.5, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
        if !(WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 2 || WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 1) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 90, 60, 0, 50, 6.5, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE(fighter, 0.7);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

unsafe extern "C" fn murabito_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.769);
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 1.000);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        if WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 270, 80, 0, 0, 5.2, 0.0, -6.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 270, 80, 0, 0, 5.2, 0.0, -6.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
        if WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 2 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 270, 80, 0, 0, 6.5, 0.0, -5.5, 0.5, Some(0.0), Some(-2.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 270, 86, 0, 0, 6.5, 0.0, -5.5, 0.5, Some(0.0), Some(-2.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
        if !(WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 2 || WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 1) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 270, 80, 0, 0, 7.5, 0.0, -5.5, 0.5, Some(0.0), Some(-2.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 270, 80, 0, 0, 7.5, 0.0, -5.5, 0.5, Some(0.0), Some(-2.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        if WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 1 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 69, 60, 0, 50, 4.2, 0.0, -5.0, 0.5, Some(0.0), Some(-2.5), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            AttackModule::clear(boma, 1, false);
        }
        if WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 2 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 69, 60, 0, 50, 5.5, 0.0, -5.0, 0.5, Some(0.0), Some(-2.5), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            AttackModule::clear(boma, 1, false);
        }
        if !(WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 2 || WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 1) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 69, 60, 0, 50, 6.5, 0.0, -5.0, 0.5, Some(0.0), Some(-2.5), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            AttackModule::clear(boma, 1, false);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

pub fn install() {
    smashline::Agent::new("murabito")
        .acmd("game_attackairn", murabito_attack_air_n_game)
        .acmd("game_attackairb", murabito_attack_air_b_game)
        .acmd("effect_attackairb", murabito_attack_air_b_effect)
        .acmd("sound_attackairb", murabito_attack_air_b_sound)
        .acmd("expression_attackairb", murabito_attack_air_b_expression)
        .acmd("game_attackairhi", murabito_attack_air_hi_game)
        .acmd("game_attackairlw", murabito_attack_air_lw_game)
        .install();
}
