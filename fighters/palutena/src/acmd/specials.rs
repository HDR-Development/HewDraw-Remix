use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 72, 40, 0, 70, 8.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.15);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.15);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.15);
        EFFECT(agent, Hash40::new("palutena_wand_finish"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.15);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_palutena_special_n01"));
    }
    wait(lua_state, 22.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialnr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = if VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED) {4.0} else {0.0};
    let sound_lvl = if VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED) {*ATTACK_SOUND_LEVEL_L} else {*ATTACK_SOUND_LEVEL_M};
    let size = if VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED) {2.0} else {0.0};
    let kbg = if VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED) {10} else {0};
    FT_DESIRED_RATE(agent, 18.0, 12.0);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !MeterModule::drain(boma.object(), 2) {
            MeterModule::drain(boma.object(), 1);
        }
        else {
            MeterModule::drain(boma.object(), 2);
        }
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0 + power, 361, 95 - kbg, 0, 40, 5.6 + (size / 2.0), 1.0, 11.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sound_lvl, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0 + (power / 2.0), 361, 97 - kbg, 0, 40, 8.6 + size, 1.0, 11.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sound_lvl, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialnr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED);
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 2.0, 0.03, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 2.0, 0.03, 0.01);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 1, 21, 2.5, 0, -50, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 2.0, 0.03, 0.01);
    }
    frame(lua_state, 18.0);
    if power {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 12.0, 10.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
    
}

unsafe extern "C" fn sound_specialnr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED);
    let sound_lvl = if power {Hash40::new("se_common_bomb_l")} else {Hash40::new("se_common_bomb_s")};
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_smash_s"));
        if power {
            PLAY_SE(agent, Hash40::new("se_palutena_smash_s01"));
        }
        PLAY_SE(agent, sound_lvl);
    }
}

unsafe extern "C" fn expression_specialnr(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 4);
        AREA_WIND_2ND_arg10(agent, 0, 1, 80, 300, 0.8, 0, 12, 24, 24, 40);
    }
    frame(lua_state, 11.0);
    app::sv_animcmd::execute(lua_state, 11.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AREA_WIND_2ND_arg10(agent, 0, 2, 30, 300, 0.8, 25, 12, 50, 24, 80);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

unsafe extern "C" fn game_specialnb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let powered = VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED);
    let power = if powered {5.0} else {0.0};
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !MeterModule::drain(boma.object(), 2) {
            MeterModule::drain(boma.object(), 1);
        }
        else {
            MeterModule::drain(boma.object(), 2);
        }
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    FT_DESIRED_RATE(agent, 14.0, 8.0);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0 + power, 88, 91, 0, 53, 4.5, 0.0, 15.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0 + power, 88, 91, 0, 58, 2.8, 0.0, 30.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        if powered {
            ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 88, 91, 0, 62, 2.8, 0.0, 45.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            FT_DESIRED_RATE(agent, 17.0, 7.0);
        }
    }
    frame(lua_state, 43.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialnb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let powered = VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED);
    let length = if powered { 2.7 } else { 1.8 };
    let length2 = if powered { 0.69 } else { 0.5 };
    let y_pos = if powered {25} else {16};
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.05, 0.90);
        EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 3, 10, 0, 0, 0, 0.75, true);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, length2, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.3);
        LAST_EFFECT_SET_RATE(agent, (16.0/10.0));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        LAST_EFFECT_SET_COLOR(agent, 0.35, 0.35, 0.90);
        EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, y_pos, 10, 0, 250, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.35, 0.35, 0.90);
        EffectModule::set_scale_last(boma, &Vector3f::new(0.5, length, 0.5));
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_ice"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_pressure"), false, false);
    }
    if is_excute(agent) {
        let size = if powered { 0.6 } else { 0.5 };
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 3, 10, 0, 0, 0, 0.6, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 24, 10, 0, 0, 0, size, true);
        if powered {
            EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 40, 10, 0, 0, 0, 0.3, true);
        }
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

unsafe extern "C" fn sound_specialnb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED);
    let sound_lvl = if power {Hash40::new("se_common_frieze_l")} else {Hash40::new("se_common_frieze_m")};
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_palutena_attack07"));
        if power {
            PLAY_SE(agent, Hash40::new("se_palutena_smash_h01"));
        }
        PLAY_SE(agent, sound_lvl);
    }
}

unsafe extern "C" fn expression_specialnb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let powered = VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED);

    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 13.0);
    app::sv_animcmd::execute(lua_state, 13.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        AREA_WIND_2ND_arg10(agent, 0, 2, 90, 300, 1, 9, 35, 18, 70, 80);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if powered {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beamm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        } else {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

unsafe extern "C" fn game_specialny(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let hitlag = if VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED) {0.5} else {0.75};
    let paralyze = if VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED) {0.6} else {0.3};
    let power = if VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED) {2} else {4};
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !MeterModule::drain(boma.object(), 2) {
            MeterModule::drain(boma.object(), 1);
        }
        else {
            MeterModule::drain(boma.object(), 2);
        }
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            WorkModule::set_float(boma, PostureModule::pos_x(boma) - 100.0, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_WORK_FLOAT_TARGET_POS_X);
        }
        else {
            WorkModule::set_float(boma, PostureModule::pos_x(boma) + 100.0, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_WORK_FLOAT_TARGET_POS_X);
        }
        WorkModule::set_float(boma, PostureModule::pos_y(boma) + 5.0, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_WORK_FLOAT_TARGET_POS_Y);
        ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTOAIMBULLET, false, -1);
    }
}

unsafe extern "C" fn effect_specialny(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.85, 0.40, 0.001);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.85, 0.40, 0.001);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.85, 0.40, 0.001);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

unsafe extern "C" fn sound_specialny(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let power = VarModule::is_flag(agent.object(), vars::palutena::instance::POWERED);
    let sound_lvl = if power {Hash40::new("se_common_electric_hit_l")} else {Hash40::new("se_common_electric_hit_m")};
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_palutena_special_n01"));
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_s"));
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, sound_lvl);
    }
    wait(lua_state, 29.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

unsafe extern "C" fn expression_specialny(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_beams"), 0);
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_beamm"), 0);
    }
}

unsafe extern "C" fn game_specialnp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("attack_lw4_charge"), false, -1.0);
    }
    frame(lua_state, 1.0);
    FT_DESIRED_RATE(agent, 14.0, 16.0);
    if is_excute(agent) {
        MeterModule::drain(boma.object(), 2);
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("attack_lw4"), false, -1.0);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 258, 90, 105, 0, 6.0, 0.0, 4.0, 14.0, Some(0.0), Some(4.0), Some(-14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 258, 60, 0, 30, 6.0, 0.0, 4.0, 14.0, Some(0.0), Some(4.0), Some(-14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_MAGIC);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        AttackModule::set_add_reaction_frame(boma, 0, 20.0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            FT_DESIRED_RATE(agent, 45.0, 25.0);
        }
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 67.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn effect_specialnp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_smash_lw_trace"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 3, 13.5, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if !agent.is_situation(*SITUATION_KIND_AIR){
            EFFECT_FOLLOW(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, false);
            LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
        }
        EFFECT(agent, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 3, 8, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 3, -8, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

unsafe extern "C" fn sound_specialnp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_03"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_smash_l"));
        PLAY_SE(agent, Hash40::new("se_palutena_smash_l01"));
        PLAY_SE(agent, Hash40::new("se_common_down_soil_l"));
    }
}

unsafe extern "C" fn expression_specialnp(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 4);
    }
    frame(lua_state, 14.0);
    app::sv_animcmd::execute(lua_state, 14.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        }
        else {
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        }
        AREA_WIND_2ND_arg10(agent, 0, 0.75, 110, 300, 0.8, 0, 15, 24, 30, 40);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 0.75, 70, 300, 0.8, 0, 12, 24, 24, 40);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

unsafe extern "C" fn game_specialno(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MeterModule::drain(boma.object(), 2);
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
        PostureModule::set_lr(boma, PostureModule::lr(agent.module_accessor));
        PostureModule::update_rot_y_lr(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, 0);
    }
}

unsafe extern "C" fn effect_specialno(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LAST_EFFECT_SET_COLOR(agent, 0.8, 0.2, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.8, 0.2, 0.01);
        EFFECT(agent, Hash40::new("palutena_throw_twinkle"), Hash40::new("top"), 0.0, 16.0, -8.0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_mirror_break"), Hash40::new("top"), 0.0, 16.0, -8.0, 0, 0, 0, 0.225, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -1, 21, 1, 0, 90, 0, 1, true, 0.7);
        LAST_EFFECT_SET_COLOR(agent, 0.8, 0.2, 0.01);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

unsafe extern "C" fn sound_specialno(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_special_l02"));
    }
}

unsafe extern "C" fn expression_specialno(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialng(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MeterModule::drain(boma.object(), 2);
        VarModule::on_flag(boma.object(), vars::palutena::instance::FLUSH);
    }
    FT_DESIRED_RATE(agent, 20.0, 10.0);
    frame(lua_state, 20.0);
    FT_DESIRED_RATE(agent, 25.0, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 366, 40, 70, 0, 6.0, 0.0, 19.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 25.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let angle_mod = ((sv_math::rand(hash40("fighter"), 51) as i32) - 25) as u64;
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 90 + angle_mod, 116, 0, 75, 8.0, 0.0, 21.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialng(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
        EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.01);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 1.0, 0.05);
        EFFECT_FOLLOW(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 0.95, true);
        LAST_EFFECT_SET_COLOR(agent, 0.05, 0.50, 0.05);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_club_tornado"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 0.70, 0.25);
        LAST_EFFECT_SET_ALPHA(agent, 0.3);
    }
    wait(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_club_tornado"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_club_tornado"), -1);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_club_tornado"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_club_tornado"), false, false);
    }
}

unsafe extern "C" fn sound_specialng(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_palutena_special_n01"));
        PLAY_SE(agent, Hash40::new("se_common_slip_01"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        PLAY_SE(agent, Hash40::new("se_palutena_throw"));
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_item_club_wind"));
    }
    wait(lua_state, 25.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_item_club_wind"));
    }
}

unsafe extern "C" fn expression_specialng(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_27_spinslash"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashss"), 10);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 10);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let current_damage = DamageModule::damage(boma, 0);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 20.0);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        VarModule::set_float(boma.object(), vars::palutena::status::ADD_DAMAGE, DamageModule::damage(boma, 0) - current_damage);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0 + (VarModule::get_float(boma.object(), vars::palutena::status::ADD_DAMAGE) * 0.75), 361, 50, 0, 70, 7.0, 0.0, 10.5, 13.0, Some(0.0), Some(10.5), Some(14.5), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0 + (VarModule::get_float(boma.object(), vars::palutena::status::ADD_DAMAGE) * 0.75), 361, 30, 0, 65, 9.0, 0.0, 10.5, 9.0, Some(0.0), Some(10.5), Some(20.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("palutena_counter_flash"), Hash40::new("shield"), -1, 0, -3, 0, 0, 0, 0.7, true);
        EFFECT(agent, Hash40::new("palutena_counter_success"), Hash40::new("top"), 0, 14.8, -1, 0, 90, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 14.8, -1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("palutena_backlight"), Hash40::new("waist"), 10, -3, -1, 0, 90, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 2.75);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
        EFFECT_FLW_POS(agent, Hash40::new("palutena_counter_attack"), Hash40::new("stick"), 0, 8.5, 0, 0, 0, 0, 0.9, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("palutena_backlight"), -1);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_palutena_special_l01"));
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_palutena_special_l02"));
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_palutena_special_l03"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_special_l01"));
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_counter"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn);
    agent.acmd("game_specialairn", game_specialn);
    agent.acmd("effect_specialn", effect_specialn);
    agent.acmd("effect_specialairn", effect_specialn);
    agent.acmd("sound_specialn", sound_specialn);
    agent.acmd("sound_specialairn", sound_specialn);
    agent.acmd("expression_specialn", expression_specialn);
    agent.acmd("expression_specialairn", expression_specialn);

    agent.acmd("game_specialnr", game_specialnr);
    agent.acmd("game_specialairnr", game_specialnr);
    agent.acmd("effect_specialnr", effect_specialnr);
    agent.acmd("effect_specialairnr", effect_specialnr);
    agent.acmd("sound_specialnr", sound_specialnr);
    agent.acmd("sound_specialairnr", sound_specialnr);
    agent.acmd("expression_specialnr", expression_specialnr);
    agent.acmd("expression_specialairnr", expression_specialnr);

    agent.acmd("game_specialnb", game_specialnb);
    agent.acmd("game_specialairnb", game_specialnb);
    agent.acmd("effect_specialnb", effect_specialnb);
    agent.acmd("effect_specialairnb", effect_specialnb);
    agent.acmd("sound_specialnb", sound_specialnb);
    agent.acmd("sound_specialairnb", sound_specialnb);
    agent.acmd("expression_specialnb", expression_specialnb);
    agent.acmd("expression_specialairnb", expression_specialnb);

    agent.acmd("game_specialny", game_specialny);
    agent.acmd("game_specialairny", game_specialny);
    agent.acmd("effect_specialny", effect_specialny);
    agent.acmd("effect_specialairny", effect_specialny);
    agent.acmd("sound_specialny", sound_specialny);
    agent.acmd("sound_specialairny", sound_specialny);
    agent.acmd("expression_specialny", expression_specialny);
    agent.acmd("expression_specialairny", expression_specialny);

    agent.acmd("game_specialnp", game_specialnp);
    agent.acmd("game_specialairnp", game_specialnp);
    agent.acmd("effect_specialnp", effect_specialnp);
    agent.acmd("effect_specialairnp", effect_specialnp);
    agent.acmd("sound_specialnp", sound_specialnp);
    agent.acmd("sound_specialairnp", sound_specialnp);
    agent.acmd("expression_specialnp", expression_specialnp);
    agent.acmd("expression_specialairnp", expression_specialnp);

    agent.acmd("game_specialno", game_specialno);
    agent.acmd("game_specialairno", game_specialno);
    agent.acmd("effect_specialno", effect_specialno);
    agent.acmd("effect_specialairno", effect_specialno);
    agent.acmd("sound_specialno", sound_specialno);
    agent.acmd("sound_specialairno", sound_specialno);
    agent.acmd("expression_specialno", expression_specialno);
    agent.acmd("expression_specialairno", expression_specialno);

    agent.acmd("game_specialng", game_specialng);
    agent.acmd("game_specialairng", game_specialng);
    agent.acmd("effect_specialng", effect_specialng);
    agent.acmd("effect_specialairng", effect_specialng);
    agent.acmd("sound_specialng", sound_specialng);
    agent.acmd("sound_specialairng", sound_specialng);
    agent.acmd("expression_specialng", expression_specialng);
    agent.acmd("expression_specialairng", expression_specialng);

    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("game_specialairlw", game_speciallw);
    agent.acmd("effect_speciallw", effect_speciallw);
    agent.acmd("effect_specialairlw", effect_speciallw);
    agent.acmd("sound_speciallw", sound_speciallw);
    agent.acmd("sound_specialairlw", sound_speciallw);
    agent.acmd("expression_speciallw", expression_speciallw);
    agent.acmd("expression_specialairlw", expression_speciallw);
}