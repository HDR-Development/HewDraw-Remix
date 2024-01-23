use super::*;

#[acmd_script( agent = "samusd", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn samusd_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 6.0, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 1.0);
    for _ in 0..7 {
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 215, 48, 0, 34, 4.0, 0.0, 15.0, 4.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 215, 48, 0, 34, 4.0, 0.0, 15.0, -4.1, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 110, 48, 0, 34, 4.0, 0.0, 6.0, 4.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 110, 48, 0, 34, 4.0, 0.0, 6.0, -4.1, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.4, 48, 78, 0, 52, 10.5, 0.0, 9.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 3.0);
    FT_MOTION_RATE(fighter, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "samusd", script = "effect_attackairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn samusd_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2.0, 0.0, 0.0, 0, 0, 0, 2.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x09aee445d1), 2.0, 0.0, 0.5, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0.0, 0.0, -0.5, 0, 0, 0, 1.70000005, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0999999, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x0954eb78b2), 2.0, 0.0, -0.5, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0.0, 0.0, 0.0, 0, 0, 0, 1.70000005, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0999999, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
    }
    frame(lua_state, 6.0);
    for _ in 0..3 {
        if is_excute(fighter) {
            BURN_COLOR(fighter, 0.699999988, 0.200000003, 1.0, 0.699999988);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 14.7, 4.3, 0, 0, 0, 0.23, true);
            LAST_EFFECT_SET_RATE(fighter, 3.0);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0, 1.0, 0, 0, 0, 1.8, true);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            BURN_COLOR_FRAME(fighter, 1, 0.699999988, 0.200000003, 1.0, 0);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            BURN_COLOR_NORMAL(fighter);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 3.5, -6.1, 0, 0, 0, 0.17, true);
            LAST_EFFECT_SET_RATE(fighter, 3.0);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0, 1.0, 0, 0, 0, 1.8, true);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            FLASH(fighter, 1, 0.699999988, 1.0, 0.5);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            FLASH_FRM(fighter, 1, 1, 0.699999988, 1.0, 0);
            COL_NORMAL(fighter);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 8.4, 0.2, 0, 0, 0, 0.32, true);
            LAST_EFFECT_SET_RATE(fighter, 3.0);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0, 1.0, 0, 0, 0, 1.8, true);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            BURN_COLOR(fighter, 0.699999988, 0.200000003, 1.0, 0.699999988);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
            LAST_EFFECT_SET_RATE(fighter, 1);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            BURN_COLOR_FRAME(fighter, 1, 0.699999988, 0.200000003, 1.0, 0);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 6.2, 5.6, 0, 0, 0, 0.2, true);
            LAST_EFFECT_SET_RATE(fighter, 3.0);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0, 1.0, 0, 0, 0, 1.8, true);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            BURN_COLOR_NORMAL(fighter);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            FLASH(fighter, 1, 0.699999988, 1.0, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 16.3, -6.1, 0, 0, 0, 0.15, true);
            LAST_EFFECT_SET_RATE(fighter, 3.0);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0, 1.0, 0, 0, 0, 1.8, true);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            FLASH_FRM(fighter, 1, 1, 0.699999988, 1.0, 0);
            COL_NORMAL(fighter);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 14.7, 4.3, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
    }

}

#[acmd_script( agent = "samusd", script = "sound_attackairn" , category = ACMD_SOUND , low_priority)]
unsafe fn samusd_attack_air_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    for _ in 0..2 {
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_elec_spark1"));
        }
        wait(lua_state, 5.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_elec_spark1"));
        }
        wait(lua_state, 5.0);
    }
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }

}

#[acmd_script( agent = "samusd", script = "expression_attackairn", category = ACMD_EXPRESSION, low_priority )]
unsafe fn samusd_attack_air_n_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "samusd", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn samusd_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    FT_MOTION_RATE_RANGE(fighter, 1.0, 11.0, 8.0);
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.0, 50, 85, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.0, 50, 85, 0, 30, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    FT_MOTION_RATE_RANGE(fighter, 17.0, 38.0, 24.0);
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "samusd", script = "effect_attackairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn samusd_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 11, 0, -1, 16, -107, 1.5, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.7, 3.0);
    }
}

#[acmd_script( agent = "samusd", script = "sound_attackairf" , category = ACMD_SOUND , low_priority)]
unsafe fn samusd_attack_air_f_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_m"));
    }

}

#[acmd_script( agent = "samusd", script = "expression_attackairf", category = ACMD_EXPRESSION, low_priority )]
unsafe fn samusd_attack_air_f_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }

    frame(lua_state, 11.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "samusd", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn samusd_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 9.0, 7.0);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.0, 38, 90, 0, 44, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("legr"), 12.0, 38, 90, 0, 44, 4.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 15.0, 38, 90, 0, 44, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 9.0, 361, 90, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("legr"), 9.0, 361, 90, 0, 20, 4.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 9.0, 361, 90, 0, 20, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE_RANGE(fighter, 18.0, 32.0, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "samusd", script = "effect_attackairb", category = ACMD_EFFECT, low_priority)]
unsafe fn samusd_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -1.5, 5, -6, -171, 47, 29, 1.1, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.7, 3.0);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 8, -15, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, true);
    }
}

#[acmd_script( agent = "samusd", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn samusd_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 0.66);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 6.0/(18.0-6.0));
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.0, 85, 93, 0, 60, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 85, 93, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 10.0, 85, 93, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "samusd", script = "effect_attackairhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn samusd_attack_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 15.5, -2, 180, 90, -90, 1.1, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.7, 3.0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {  }
}

#[acmd_script( agent = "samusd", script = "sound_attackairhi" , category = ACMD_SOUND , low_priority)]
unsafe fn samusd_attack_air_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_s"));
    }

}

#[acmd_script( agent = "samusd", script = "expression_attackairhi", category = ACMD_EXPRESSION, low_priority )]
unsafe fn samusd_attack_air_hi_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "samusd", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn samusd_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.562);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 14.0, 270, 75, 0, 26, 4.3, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 14.0, 270, 75, 0, 26, 5.0, 5.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        /* Air-only */
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 14.0, 270, 51, 0, 26, 4.3, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("armr"), 14.0, 270, 51, 0, 26, 5.0, 5.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 11.0, 67, 60, 0, 46, 4.3, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 11.0, 67, 60, 0, 46, 5.0, 4.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "samusd", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority)]
unsafe fn samusd_attack_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_atk_air_lw"), Hash40::new("top"), 0, 12.5, 0, 0, -200, 90, 0.85, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 3.0);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }

}

#[acmd_script( agent = "samusd", script = "game_aircatch", category = ACMD_GAME, low_priority)]
unsafe fn samusd_air_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 4.0, 45, 30, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("throw"), 4.0, 45, 100, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "samusd", script = "game_aircatchlanding" , category = ACMD_GAME , low_priority)]
unsafe fn samusd_landing_air_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 16.0/27.0);
    }

}

pub fn install() {
    install_acmd_scripts!(
        samusd_attack_air_n_game,
        samusd_attack_air_n_effect,
        samusd_attack_air_n_sound,
        samusd_attack_air_n_expression,
        samusd_attack_air_f_game,
        samusd_attack_air_f_effect,
        samusd_attack_air_f_sound,
        samusd_attack_air_f_expression,
        samusd_attack_air_b_game,
        samusd_attack_air_b_effect,
        samusd_attack_air_hi_game,
        samusd_attack_air_hi_effect,
        samusd_attack_air_hi_sound,
        samusd_attack_air_hi_expression,
        samusd_attack_air_lw_game,
        samusd_attack_air_lw_effect,
        samusd_air_catch_game,
        samusd_landing_air_catch_game,
    );
}

