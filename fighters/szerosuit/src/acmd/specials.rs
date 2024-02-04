
use super::*;


unsafe extern "C" fn szerosuit_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 15.0/(21.0-1.0));
}


unsafe extern "C" fn szerosuit_special_air_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 15.0/(21.0-1.0));
}


unsafe extern "C" fn szerosuit_special_n_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 26.0/(28.0-1.0));
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_N_FLAG_SHOOT);
        ArticleModule::generate_article(boma, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_PARALYZER_BULLET, false, -1);
    }
}



unsafe extern "C" fn szerosuit_special_air_n_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 26.0/(28.0-1.0));
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_N_FLAG_SHOOT);
        ArticleModule::generate_article(boma, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_PARALYZER_BULLET, false, -1);
    }
}



unsafe extern "C" fn szerosuit_special_n_landing_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 9.0/(14.0-1.0));
}


unsafe extern "C" fn szerosuit_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 23.0/(28.0-1.0));
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 15, 100, 90, 0, 4.0, 0.0, 8.0, 9.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 1, Hash40::new("top"), 4.0, 20, 100, 80, 0, 3.5, 0.0, 8.0, 19.0, Some(0.0), Some(8.0), Some(17.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 1, Hash40::new("top"), 4.0, 25, 100, 50, 0, 4.0, 0.0, 8.0, 30.0, Some(0.0), Some(8.0), Some(26.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_S_FLAG_S2);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("top"), 9.0, 50, 115, 0, 50, 6.0, 0.0, 8.5, 36.0, Some(0.0), Some(8.5), Some(30.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


unsafe extern "C" fn szerosuit_whip_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, (28.0-1.0)/23.0);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(boma, false);
    }
}


unsafe extern "C" fn szerosuit_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.2, 365, 100, 85, 0, 3.5, 0.0, 8.0, 19.0, Some(0.0), Some(8.0), Some(17.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 0, 100, 95, 0, 3.0, 0.0, 8.0, 9.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 0, 100, 100, 0, 3.0, 0.0, 8.0, 9.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 1, Hash40::new("top"), 1.2, 345, 100, 85, 0, 3.5, 0.0, 8.0, 25.0, Some(0.0), Some(8.0), Some(17.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        ATTACK(fighter, 1, 2, Hash40::new("top"), 1.2, 10, 100, 85, 0, 3.7, 0.0, 8.0, 28.0, Some(0.0), Some(8.0), Some(25.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 3, Hash40::new("top"), 1.2, 345, 100, 85, 0, 4.0, 0.0, 8.0, 30.0, Some(0.0), Some(8.0), Some(28.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_S_FLAG_S2);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 4, Hash40::new("top"), 9.0, 50, 115, 0, 50, 6.0, 0.0, 8.5, 36.0, Some(0.0), Some(8.5), Some(30.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
        //ATTACK(fighter, 0, 4, Hash40::new("top"), 8.0, 50, 105, 0, 45, 6.6, 0.0, 8.5, 38.0, Some(0.0), Some(8.5), Some(32.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_WHIP);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


unsafe extern "C" fn szerosuit_whip_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(boma, false);
    }
}


unsafe extern "C" fn szerosuit_special_s_2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 5, Hash40::new("top"), 7.0, 110, 60, 0, 90, 6.0, 0.0, 8.0, 34.0, Some(0.0), Some(8.0), Some(30.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        //AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


unsafe extern "C" fn szerosuit_whip_special_s2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, (28.0-1.0)/23.0);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 57.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(boma, false);
    }
}


unsafe extern "C" fn szerosuit_whip_special_s2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("szero_whip_flash"), Hash40::new("plasmawhip1"), 1, 0, 0, 0, 0, 0, 1, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("szero_pwhip"), true, true);
        EFFECT_FLW_POS(fighter, Hash40::new("szero_whip"), Hash40::new("attach"), 0, 0, 0, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip2"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip3"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip4"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip5"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip6"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip7"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("szero_gbeam_lightning"), Hash40::new("plasmawhip8"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("szero_gbeam_lightning"), false, true);
    }
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("szero_whip"), false, true);
    }
    frame(lua_state, 56.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("szero_whip_flash"), false, true);
    }
}


unsafe extern "C" fn szerosuit_special_air_s_2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 5, Hash40::new("top"), 7.0, 110, 60, 0, 90, 6.0, 0.0, 8.0, 34.0, Some(0.0), Some(8.0), Some(30.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_WHIP);
        //AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}


unsafe extern "C" fn szerosuit_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 5.0/(8.0-1.0));
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 90, 100, 150, 0, 6.0, 0.0, 8.0, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.3, 367, 100, 50, 0, 7.0, 0.0, 19.0, 5.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("top"), 1.3, 90, 100, 12, 0, 7.0, 0.0, 12.0, 10.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 1, Hash40::new("top"), 1.3, 290, 100, 10, 0, 4.0, 0.0, 20.0, 5.5, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 48, 208, 0, 40, 8.0, 0.0, 12.5, 11.0, Some(0.0), Some(12.0), Some(9.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}


unsafe extern "C" fn szerosuit_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 5.0/(8.0-1.0));
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE (fighter, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 90, 100, 150, 0, 6.0, 0.0, 8.0, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.3, 367, 100, 50, 0, 7.0, 0.0, 19.0, 5.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("top"), 1.3, 90, 100, 12, 0, 7.0, 0.0, 12.0, 10.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 1, Hash40::new("top"), 1.3, 290, 100, 10, 0, 4.0, 0.0, 20.0, 5.5, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 48, 196, 0, 35, 8.0, 0.0, 12.5, 11.0, Some(0.0), Some(12.0), Some(9.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}


unsafe extern "C" fn szerosuit_special_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(boma.object(), vars::szerosuit::status::SPECIAL_LW_MANUAL_FLIPSTOOL_ENABLE);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_START_WAIT_INPUT);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_KICK_ENABLE);
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_REVERSE);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        //WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
        VarModule::on_flag(boma.object(), vars::szerosuit::status::SPECIAL_LW_MANUAL_FLIPSTOOL_ENABLE);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_WALL_JUMP_ENABLE);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
        VarModule::off_flag(boma.object(), vars::szerosuit::status::SPECIAL_LW_MANUAL_FLIPSTOOL_ENABLE);
    }
}


unsafe extern "C" fn szerosuit_special_air_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(boma.object(), vars::szerosuit::status::SPECIAL_LW_MANUAL_FLIPSTOOL_ENABLE);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_START_WAIT_INPUT);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_KICK_ENABLE);
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_REVERSE);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        //WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_WALL_JUMP_ENABLE);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
    }
}


unsafe extern "C" fn szerosuit_special_air_lw_kick_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_KICK_REVERSE_ENABLE);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 12.0, 48, 115, 0, 45, 4.8, -4.0, 0.0, 0.0, Some(-1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // Air-only
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 14.0, 290, 52, 0, 30, 4.8, 10.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // Ground-only
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 14.0, 45, 100, 0, 45, 4.8, 10.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SZEROSUIT_INSTANCE_WORK_ID_FLAG_TO_SPECIAL_LANDING);
    }
    
}


unsafe extern "C" fn szerosuit_special_air_lw_flip_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if !VarModule::is_flag(boma.object(), vars::szerosuit::status::ATTACK_AIR_LW_REBOUND){
            ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 270, 80, 0, 10, 3.0, 0.0, -3.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 90, 0, 10, 4.0, 0.0, -3.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 270, 80, 0, 10, 3.0, 0.0, -3.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}




pub fn install() {
    smashline::Agent::new("szerosuit")
        .acmd("game_specialnstart", szerosuit_special_n_start_game)
        .acmd("game_specialairnstart", szerosuit_special_air_n_start_game)
        .acmd("game_specialnshoot", szerosuit_special_n_shoot_game)
        .acmd("game_specialairnshoot", szerosuit_special_air_n_shoot_game)
        .acmd("game_specialnlanding", szerosuit_special_n_landing_game)
        .acmd("game_specials", szerosuit_special_s_game)
        .acmd("game_specialairs", szerosuit_special_air_s_game)
        .acmd("game_specials2", szerosuit_special_s_2_game)
        .acmd("game_specialairs2", szerosuit_special_air_s_2_game)
        .acmd("game_specialhi", szerosuit_special_hi_game)
        .acmd("game_specialairhi", szerosuit_special_air_hi_game)
        .acmd("game_speciallwstart", szerosuit_special_lw_start_game)
        .acmd(
            "game_specialairlwstart",
            szerosuit_special_air_lw_start_game,
        )
        .acmd("game_specialairlwkick", szerosuit_special_air_lw_kick_game)
        .acmd("game_specialairlwflip", szerosuit_special_air_lw_flip_game)
        .install();
    smashline::Agent::new("szerosuit_whip")
        .acmd("game_specials", szerosuit_whip_special_s_game)
        .acmd("game_specialairs", szerosuit_whip_special_air_s_game)
        .acmd("game_specials2", szerosuit_whip_special_s2_game)
        .acmd("effect_specials2", szerosuit_whip_special_s2_effect)
        .install();
}
