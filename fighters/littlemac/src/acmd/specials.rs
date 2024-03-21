use super::*;

unsafe extern "C" fn littlemac_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_CHECK_DASH);
    }
}

unsafe extern "C" fn littlemac_special_n_cancel_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 8.0/(39.0 - 1.0));
    frame(lua_state, 39.0);
    FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn littlemac_special_n_cancel_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

unsafe extern "C" fn littlemac_special_n_cancel_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

unsafe extern "C" fn littlemac_special_n_cancel_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

unsafe extern "C" fn littlemac_special_n2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) && (WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE) == 100.0) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        let mut meter = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        let meter_lvl = if meter < 40.0 { 0 } else if meter >= 40.0 && meter < 100.0 { 1 } else { 2 };
        let damage = (if meter_lvl == 2 { 25.0 } else { 10.0 + meter / 8.0 }) * if fighter.is_situation(*SITUATION_KIND_GROUND) { 1.0 } else { 0.8 };
        let angle = if fighter.is_situation(*SITUATION_KIND_GROUND) { 80 } else { 75 };
        let bkb = if fighter.is_situation(*SITUATION_KIND_GROUND) { 40 } else { 30 };
        let kbg = if fighter.is_situation(*SITUATION_KIND_GROUND) { 104 } else { 124 };
        let hitlag = if meter_lvl == 0 { 1.0 } else if meter_lvl == 1 { 1.2 } else { 1.5 };
        let shield_damage = if fighter.is_situation(*SITUATION_KIND_GROUND) { 2 } else { 0 };
        let sfx_lvl = if meter_lvl == 0 { *ATTACK_SOUND_LEVEL_M } else { *ATTACK_SOUND_LEVEL_L };
        let sound_attr = if meter_lvl == 0 { *COLLISION_SOUND_ATTR_PUNCH } else if meter_lvl == 1 { *COLLISION_SOUND_ATTR_HEAVY } else { *COLLISION_SOUND_ATTR_KICK };
        ATTACK(fighter, 0, 0, Hash40::new("armr"), damage, angle, kbg, 0, bkb, 5.0, 3.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx_lvl, sound_attr, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), damage, angle, kbg, 0, bkb, 3.0, -1.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx_lvl, sound_attr, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), damage, angle, kbg, 0, bkb, 3.0, 0.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx_lvl, sound_attr, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("bust"), damage, angle, kbg, 0, bkb, 3.0, 0.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sfx_lvl, sound_attr, *ATTACK_REGION_PUNCH);
        if meter_lvl == 2 { AttackModule::set_damage_shake_scale(boma, 0.67); }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_RESET_KO_GAUGE);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_KO_GRAVITY);
    }
    
}

unsafe extern "C" fn littlemac_special_n2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let meter = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    if is_excute(fighter) {
        let size = if meter < 40.0 { 0.5 } else if meter >= 40.0 && meter < 100.0 { 0.7 } else { 1.0 };
        EFFECT_FLW_POS(fighter, Hash40::new("littlemac_ko_uppercut_start"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, size, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("littlemac_ko_uppercut"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, size, true);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("littlemac_ko_uppercut_start"), -1);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if meter >= 40.0 {
            if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
                EFFECT_FOLLOW(fighter, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), 0.5, 1, -3, 0, -60, 70, 1, true);
                EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), 0.5, 1, -3, 0, -60, 70, 1, false);
            }
            else {
                EFFECT_FOLLOW(fighter, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, true);
                EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, false);
            }
        }
    }
    frame(lua_state, 8.0);
    let mut handle = EffectModule::req_follow(boma, Hash40::new("sys_starrod_bullet"), Hash40::new("handr"), &Vector3f::new(3.0, 0.0, 0.0), &Vector3f::new(0.0, 90.0, 0.0), if meter == 100.0 { 0.3 } else { 0.0 }, false, 0, 0, 0, 0, 0, false, false);
    if is_excute(fighter) {
        EffectModule::set_rate(boma, handle as u32, 1.5);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if meter == 100.0 { EffectModule::set_scale(boma, handle as u32, &Vector3f::new(0.8, 0.8, 0.8)); }
        if meter < 40.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("littlemac_attack_arc"), Hash40::new("top"), -1, 10.5, 1.0, 4, 30, 110, 0.8, true);
            LAST_EFFECT_SET_SCALE_W(fighter, 0.7, 0.7, 0.9);
            LAST_EFFECT_SET_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("littlemac_ko_uppercut"), false, false);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_starrod_bullet"), false, false);
    }

}

unsafe extern "C" fn littlemac_special_n2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if (WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE) == 100.0) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_littlemac_rnd_special_n2"));
            PLAY_SE(fighter, Hash40::new("se_littlemac_special_n03"));
        }
        else {
            if (WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE) < 40.0) {
                match smash::app::sv_math::rand(smash::hash40("fighter"), 3) {
                    0 => PLAY_SE(fighter, Hash40::new("vc_littlemac_attack06")),
                    1 => PLAY_SE(fighter, Hash40::new("vc_littlemac_attack07")),
                    2 => PLAY_SE(fighter, Hash40::new("vc_littlemac_special_l01")),
                    _ => PLAY_SE(fighter, Hash40::new("vc_littlemac_special_l01"))
                };
            }
            else {
                match smash::app::sv_math::rand(smash::hash40("fighter"), 3) {
                    0 => PLAY_SE(fighter, Hash40::new("vc_littlemac_special_l02")),
                    1 => PLAY_SE(fighter, Hash40::new("vc_littlemac_special_n02")),
                    2 => PLAY_SE(fighter, Hash40::new("vc_littlemac_special_n03")),
                    _ => PLAY_SE(fighter, Hash40::new("vc_littlemac_special_l02"))
                };
            }
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal04"));
        }
        else if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)) {
            PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal05"));
        }
        else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal06"));
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_swing_ll"));
    }

}

unsafe extern "C" fn littlemac_special_n2_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        let meter = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        QUAKE(fighter, if meter < 40.0 {*CAMERA_QUAKE_KIND_M} else { *CAMERA_QUAKE_KIND_L });
        let rumble = if meter < 100.0 { Hash40::new("rbkind_nohitl") } else { Hash40::new("rbkind_nohitll") };
        ControlModule::set_rumble(boma, rumble, 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_elecattack"), 0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AREA_WIND_2ND_arg10(fighter, 0, 4, 45, 200, 1, 17, 15, 38, 30, 50);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(boma, 0);
    }

}

unsafe extern "C" fn littlemac_special_s_jump_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 5.1, 0.0, 7.4, 3.0, 0.0, 7.4, 3.2, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 23, 0, 3.0, 0.0, 0.0, 10.0, Some(0.0), Some(0.0), Some(2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 5.2, 0.0, 8.6, -3.0, 0.0, 9.0, 3.4, 0.0, 0.0, 1, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn littlemac_special_air_s_blow_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 14.0, 361, 68, 0, 75, 5.0, 3.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 14.0, 361, 68, 0, 75, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 361, 68, 0, 75, 3.5, 0.0, 4.5, 5.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 361, 68, 0, 75, 5.0, 0.0, 18.0, 5.0, Some(0.0), Some(10.0), Some(10.5), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 3, false);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

unsafe extern "C" fn littlemac_special_air_s_blow_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("littlemac_attack_arc2_aura"), Hash40::new("top"), 1, 10, -1.5, 0, -20, -110, 1, false);
        EFFECT_FOLLOW(fighter, Hash40::new("littlemac_attack_arc2"), Hash40::new("top"), 1, 8.5, -1.5, 0, -20, -110, 1, false);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("littlemac_attack_arc2_splash"), Hash40::new("top"), 1, 10, -1.5, 0, -20, -110, 1, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("littlemac_joltblow"), false, false);
        EffectModule::enable_sync_init_pos_last(boma);
    }
}

unsafe extern "C" fn littlemac_special_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 103, 100, 155, 0, 2.0, 0.0, 6.0, 10.0, Some(0.0), Some(12.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 95, 100, 160, 0, 3.0, 0.0, 5.0, 5.0, Some(0.0), Some(12.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn littlemac_special_air_hi_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 103, 100, 185, 0, 2.0, 0.0, 6.0, 10.0, Some(0.0), Some(12.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 95, 100, 180, 0, 3.0, 0.0, 5.0, 5.0, Some(0.0), Some(12.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 103, 100, 155, 0, 2.0, 0.0, 6.0, 10.0, Some(0.0), Some(12.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 95, 100, 160, 0, 3.0, 0.0, 5.0, 5.0, Some(0.0), Some(12.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn littlemac_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 50, 0, 40, 6.5, 0.0, 26.0, 0.0, None, None, None, 0.7, 0.9, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 90, 0, 180, 0, 3.0, 0.0, 18.0, 0.0, None, None, None, 0.7, 0.9, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE_RANGE(fighter, 19.0, 23.0, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 197, 0, 45, 6.5, 0.0, 26.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        let air_speed_x_stable = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0);
        let fall_x_mul = WorkModule::get_param_float(
            boma,
            hash40("param_special_hi"),
            hash40("special_hi_fall_x_mul")
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * fall_x_mul,
            0.0
        );
    }
}

pub fn install() {
    smashline::Agent::new("littlemac")
        .acmd("game_specialnstart", littlemac_special_n_start_game)
        .acmd("game_specialairnstart", littlemac_special_n_start_game)
        .acmd("game_specialncancel", littlemac_special_n_cancel_game)
        .acmd("game_specialairncancel", littlemac_special_n_cancel_game)
        .acmd("effect_specialncancel", littlemac_special_n_cancel_effect)
        .acmd("effect_specialairncancel", littlemac_special_n_cancel_effect)
        .acmd("sound_specialncancel", littlemac_special_n_cancel_sound)
        .acmd("sound_specialairncancel", littlemac_special_n_cancel_sound)
        .acmd("expression_specialncancel", littlemac_special_n_cancel_expression)
        .acmd("expression_specialairncancel", littlemac_special_n_cancel_expression)
        .acmd("game_specialn2", littlemac_special_n2_game)
        .acmd("game_specialairn2", littlemac_special_n2_game)
        .acmd("effect_specialn2", littlemac_special_n2_effect)
        .acmd("effect_specialairn2", littlemac_special_n2_effect)
        .acmd("sound_specialn2", littlemac_special_n2_sound)
        .acmd("sound_specialairn2", littlemac_special_n2_sound)
        .acmd("expression_specialn2", littlemac_special_n2_expression)
        .acmd("expression_specialairn2", littlemac_special_n2_expression)
        .acmd("game_specialsjump", littlemac_special_s_jump_game)
        .acmd("game_specialairsblow", littlemac_special_air_s_blow_game)
        .acmd("effect_specialairsblow", littlemac_special_air_s_blow_effect)
        .acmd("game_specialhistart", littlemac_special_hi_start_game)
        .acmd("game_specialairhistart", littlemac_special_air_hi_start_game)
        .acmd("game_specialhi", littlemac_special_hi_game)
        .install();
}