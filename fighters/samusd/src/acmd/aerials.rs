use super::*;

unsafe extern "C" fn samusd_attack_air_n_game(fighter: &mut L2CAgentBase) {
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
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 365, 48, 0, 34, 4.0, 0.0, 15.0, 4.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 365, 48, 0, 34, 4.0, 0.0, 15.0, -4.1, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 110, 48, 0, 34, 4.0, 0.0, 6.0, 4.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 110, 48, 0, 34, 4.0, 0.0, 6.0, -4.1, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
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

unsafe extern "C" fn samusd_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2.0, 0.0, 0.0, 0, 0, 0, 2.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("clavicler"), 2.0, 0.0, 0.5, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0.0, 0.0, -0.5, 0, 0, 0, 1.70000005, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0999999, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("claviclel"), 2.0, 0.0, -0.5, 0, 0, 0, 2.0, true);
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

unsafe extern "C" fn samusd_attack_air_n_sound(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn samusd_attack_air_n_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn samusd_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 5.0, 3.0);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE_RANGE(fighter, 11.0, 15.0, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.0, 361, 85, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.0, 361, 85, 0, 30, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 1.0);//34 faf
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn samusd_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 9, 3, 15, -18 , -85, 1.15, true);
        let color_vec = match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => Vector3f::new(0.1, 0.7, 3.0),//nor
            1 => Vector3f::new(0.55, 0.88, 0.0004),//g
            2 => Vector3f::new(1.25, 0.55, 1.5),//pur
            3 => Vector3f::new(0.84, 0.7, 0.03),//r
            4 => Vector3f::new(0.1, 1.0, 2.0),//y
            5 => Vector3f::new(0.9, 0.03, 0.03),//w
            6 => Vector3f::new(1.15, 0.65, 0.03),//blac
            7 => Vector3f::new(0.78, 0.5, 2.5),//pi
            _ => Vector3f::new(0.1, 0.7, 3.0)
        }; //matches glow color
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

unsafe extern "C" fn samusd_attack_air_f_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_m"));
    }

}

unsafe extern "C" fn samusd_attack_air_f_expression(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn samusd_attack_air_b_game(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn samusd_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -1.5, 5, -6, -171, 47, 29, 1.1, true);
        let color_vec = match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => Vector3f::new(0.1, 0.7, 3.0),//nor
            1 => Vector3f::new(0.55, 0.88, 0.0004),//g
            2 => Vector3f::new(1.25, 0.55, 1.5),//pur
            3 => Vector3f::new(0.84, 0.7, 0.03),//r
            4 => Vector3f::new(0.1, 1.0, 2.0),//y
            5 => Vector3f::new(0.9, 0.03, 0.03),//w
            6 => Vector3f::new(1.15, 0.65, 0.03),//blac
            7 => Vector3f::new(0.78, 0.5, 2.5),//pi
            _ => Vector3f::new(0.1, 0.7, 3.0)
        }; //matches glow color
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 8, -15, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, true);
    }
}

unsafe extern "C" fn samusd_attack_air_hi_game(fighter: &mut L2CAgentBase) {
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
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.0, 75, 101, 0, 50, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 75, 101, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 10.0, 75, 101, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
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

unsafe extern "C" fn samusd_attack_air_hi_effect(fighter: &mut L2CAgentBase) {
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
        let color_vec = match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => Vector3f::new(0.1, 0.7, 3.0),//nor
            1 => Vector3f::new(0.55, 0.88, 0.0004),//g
            2 => Vector3f::new(1.25, 0.55, 1.5),//pur
            3 => Vector3f::new(0.84, 0.7, 0.03),//r
            4 => Vector3f::new(0.1, 1.0, 2.0),//y
            5 => Vector3f::new(0.9, 0.03, 0.03),//w
            6 => Vector3f::new(1.15, 0.65, 0.03),//blac
            7 => Vector3f::new(0.78, 0.5, 2.5),//pi
            _ => Vector3f::new(0.1, 0.7, 3.0)
        }; //matches glow color
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
    }
}

unsafe extern "C" fn samusd_attack_air_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_s"));
    }
}

unsafe extern "C" fn samusd_attack_air_hi_expression(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn samusd_attack_air_lw_game(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn samusd_attack_air_lw_effect(fighter: &mut L2CAgentBase) {
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
        let color_vec = match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
            0 => Vector3f::new(0.5, 0.5, 3.0),//nor
            1 => Vector3f::new(0.8, 0.5, 0.0),//g
            2 => Vector3f::new(2.0, 0.3, 2.5),//pur
            3 => Vector3f::new(2.5, 0.85, 0.0),//r
            4 => Vector3f::new(0.5, 0.5, 2.5),//y
            5 => Vector3f::new(3.0, 0.07, 0.15),//w
            6 => Vector3f::new(2.0, 0.42, 0.0), //blac
            7 => Vector3f::new(0.8, 0.2, 2.5),//pi
            _ => Vector3f::new(0.5, 0.5, 3.0)
        }; //matches glow color
        LAST_EFFECT_SET_COLOR(fighter, color_vec.x, color_vec.y, color_vec.z);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }

}

unsafe extern "C" fn samusd_air_catch_game(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn samusd_landing_air_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 16.0/27.0);
    }

}

pub fn install() {
    smashline::Agent::new("samusd")
        .acmd("game_attackairn", samusd_attack_air_n_game)
        .acmd("effect_attackairn", samusd_attack_air_n_effect)
        .acmd("sound_attackairn", samusd_attack_air_n_sound)
        .acmd("expression_attackairn", samusd_attack_air_n_expression)
        .acmd("game_attackairf", samusd_attack_air_f_game)
        .acmd("effect_attackairf", samusd_attack_air_f_effect)
        .acmd("sound_attackairf", samusd_attack_air_f_sound)
        .acmd("expression_attackairf", samusd_attack_air_f_expression)
        .acmd("game_attackairb", samusd_attack_air_b_game)
        .acmd("effect_attackairb", samusd_attack_air_b_effect)
        .acmd("game_attackairhi", samusd_attack_air_hi_game)
        .acmd("effect_attackairhi", samusd_attack_air_hi_effect)
        .acmd("sound_attackairhi", samusd_attack_air_hi_sound)
        .acmd("expression_attackairhi", samusd_attack_air_hi_expression)
        .acmd("game_attackairlw", samusd_attack_air_lw_game)
        .acmd("effect_attackairlw", samusd_attack_air_lw_effect)
        .acmd("game_aircatch", samusd_air_catch_game)
        .acmd("game_aircatchlanding", samusd_landing_air_catch_game)
        .install();
}
