use super::*;


unsafe extern "C" fn gamewatch_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 18.0, 9.0);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_SHOOT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 10, 30, 0, 60, 4.2, 0.0, 8.5, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 10, 30, 0, 60, 3.2, 0.0, 6.5, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute (fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_COUNT_ENABLE);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_RAPID_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_LOOP_CHECK);
    }
    frame(lua_state, 43.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 48.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_COUNT_CHECK);
    }
}


unsafe extern "C" fn gamewatch_special_s1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL);
        VisibilityModule::set_default_int64(boma, hash40("panel") as i64);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 270, 10, 0, 90, 6.0, 0.0, 10.6, 8.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 270, 10, 0, 90, 4.0, 0.0, 6.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 6.0, 0.0, 10.6, 8.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 4.0, 0.0, 6.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
        WorkModule::set_float(boma, 9.0, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_WORK_FLOAT_DAMAGE);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}


unsafe extern "C" fn gamewatch_special_s2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL);
        VisibilityModule::set_default_int64(boma, hash40("panel") as i64);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 40, 0, 10, 6.0, 0.0, 10.6, 8.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 40, 0, 10, 4.0, 0.0, 6.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(boma, 0, 361, 45, 1.0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}


unsafe extern "C" fn gamewatch_special_s3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL);
        VisibilityModule::set_default_int64(boma, hash40("panel") as i64);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 140, 50, 0, 45, 6.0, 0.0, 10.6, 8.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 140, 50, 0, 45, 4.0, 0.0, 6.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}


unsafe extern "C" fn gamewatch_special_s4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL);
        VisibilityModule::set_default_int64(boma, hash40("panel") as i64);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 69, 40, 0, 50, 6.0, 0.0, 10.6, 8.9, None, None, None, 2.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 69, 40, 0, 50, 4.0, 0.0, 6.5, 7.0, None, None, None, 2.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}


unsafe extern "C" fn gamewatch_special_s5_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL);
        VisibilityModule::set_default_int64(boma, hash40("panel") as i64);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 367, 60, 0, 15, 6.0, 0.0, 10.6, 8.9, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 70, 0, 25, 4.0, 0.0, 6.5, 7.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 25.0);
    if(is_excute(fighter)) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 75, 80, 0, 60, 6.0, 0.0, 10.6, 8.9, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 75, 80, 0, 60, 4.0, 0.0, 6.5, 7.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}


unsafe extern "C" fn gamewatch_special_s6_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL);
        VisibilityModule::set_default_int64(boma, hash40("panel") as i64);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 20, 66, 0, 32, 6.0, 0.0, 10.6, 8.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 20, 66, 0, 32, 4.0, 0.0, 6.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


unsafe extern "C" fn gamewatch_special_s7_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL);
        VisibilityModule::set_default_int64(boma, hash40("panel") as i64);
        FT_ADD_DAMAGE(fighter, -9.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 45, 50, 0, 45, 6.0, 0.0, 10.6, 8.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 45, 50, 0, 45, 4.0, 0.0, 6.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}


unsafe extern "C" fn gamewatch_special_s8_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL);
        VisibilityModule::set_default_int64(boma, hash40("panel") as i64);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 100, 60, 0, 6.0, 0.0, 10.6, 8.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 80, 100, 60, 0, 4.0, 0.0, 6.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


unsafe extern "C" fn gamewatch_special_s9_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL);
        VisibilityModule::set_default_int64(boma, hash40("panel") as i64);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 25.0, 361, 80, 0, 100, 6.0, 0.0, 10.6, 8.9, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 25.0, 361, 80, 0, 100, 4.0, 0.0, 6.5, 7.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}


unsafe extern "C" fn gamewatch_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 3.0, 5.0);
    frame(lua_state, 3.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 85, 23, 0, 134, 5.0, 0.0, 5.5, 6.0, Some(0.0), Some(5.5), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            ATTACK(fighter, 0, 0, Hash40::new("waist"), 6.0, 80, 80, 0, 60, 5.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_DECIDE_DIRECTION);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 0.8);
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            ATTACK(fighter, 0, 0, Hash40::new("waist"), 4.0, 80, 80, 0, 60, 5.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 33.0);
    FT_MOTION_RATE_RANGE(fighter, 33.0, 43.0, 12.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, 43.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}


unsafe extern "C" fn gamewatch_special_hi_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_attach_item_visibility(boma, false, 0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_bounce"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("head") as i64, hash40("head_close") as i64);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


unsafe extern "C" fn gamewatch_special_hi_open_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_PARACHUTE, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_PARACHUTE, Hash40::new("special_hi_open"), false, -1.0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 361, 104, 0, 20, 11.22, 0.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE_RANGE(fighter, 30.0, 45.0, 10.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_PARACHUTE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 45.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


unsafe extern "C" fn gamewatch_special_hi_open_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_wave10_hi"));
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_gamewatch_wave10_hi"));
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se(boma, Hash40::new("se_gamewatch_wave03_mi"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 1.25, 0);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_gamewatch_wave03_mi"));
    }
}


unsafe extern "C" fn gamewatch_special_hi_open_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
}


unsafe extern "C" fn gamewatch_parachute_special_hi_open_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("para3") as i64, hash40("on") as i64);
        VisibilityModule::set_int64(boma, hash40("para4") as i64, hash40("on") as i64);
    }
}


unsafe extern "C" fn gamewatch_rescue_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("up") as i64, hash40("off") as i64);
        VisibilityModule::set_int64(boma, hash40("down") as i64, hash40("off") as i64);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 8.0, 11.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("down") as i64, hash40("on") as i64);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("up") as i64, hash40("on") as i64);
        VisibilityModule::set_int64(boma, hash40("down") as i64, hash40("off") as i64);
    }
}


unsafe extern "C" fn gamewatch_rescue_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("up") as i64, hash40("off") as i64);
        VisibilityModule::set_int64(boma, hash40("down") as i64, hash40("off") as i64);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 8.0, 9.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("down") as i64, hash40("on") as i64);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("up") as i64, hash40("on") as i64);
        VisibilityModule::set_int64(boma, hash40("down") as i64, hash40("off") as i64);
    }
}


pub fn install() {
    smashline::Agent::new("gamewatch_rescue")
        .acmd("game_specialhi", gamewatch_rescue_special_hi_game)
        .acmd("game_specialairhi", gamewatch_rescue_special_air_hi_game)
        .install();
    smashline::Agent::new("gamewatch")
        .acmd("game_specialn", gamewatch_special_n_game)
        .acmd("game_specialairn", gamewatch_special_n_game)
        .acmd("game_specials1", gamewatch_special_s1_game)
        .acmd("game_specialairs1", gamewatch_special_s1_game)
        .acmd("game_specials2", gamewatch_special_s2_game)
        .acmd("game_specialairs2", gamewatch_special_s2_game)
        .acmd("game_specials3", gamewatch_special_s3_game)
        .acmd("game_specialairs3", gamewatch_special_s3_game)
        .acmd("game_specials4", gamewatch_special_s4_game)
        .acmd("game_specialairs4", gamewatch_special_s4_game)
        .acmd("game_specials5", gamewatch_special_s5_game)
        .acmd("game_specialairs5", gamewatch_special_s5_game)
        .acmd("game_specials6", gamewatch_special_s6_game)
        .acmd("specialairs6", gamewatch_special_s6_game)
        .acmd("game_specials7", gamewatch_special_s7_game)
        .acmd("game_specialairs7", gamewatch_special_s7_game)
        .acmd("game_specials8", gamewatch_special_s8_game)
        .acmd("game_specialairs8", gamewatch_special_s8_game)
        .acmd("game_specials9", gamewatch_special_s9_game)
        .acmd("game_specialairs9", gamewatch_special_s9_game)
        .acmd("game_specialhi", gamewatch_special_hi_game)
        .acmd("game_specialairhi", gamewatch_special_hi_game)
        .acmd("expression_specialhi", gamewatch_special_hi_expression)
        .acmd("expression_specialairhi", gamewatch_special_hi_expression)
        .acmd("game_specialhiopen", gamewatch_special_hi_open_game)
        .acmd("sound_specialhiopen", gamewatch_special_hi_open_sound)
        .acmd(
            "expression_specialhiopen",
            gamewatch_special_hi_open_expression,
        )
        .install();
    smashline::Agent::new("gamewatch_parachute")
        .acmd(
            "game_specialhiopen",
            gamewatch_parachute_special_hi_open_game,
        )
        .install();
}
