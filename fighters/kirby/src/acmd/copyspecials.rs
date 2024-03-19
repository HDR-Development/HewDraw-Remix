use super::*;

unsafe extern "C" fn diddy_special_n_cancel_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, -1);
        FT_MOTION_RATE(fighter, 8.0/(31.0 - 1.0));
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn diddy_special_n_cancel_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

unsafe extern "C" fn diddy_special_n_cancel_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

unsafe extern "C" fn diddy_special_n_cancel_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

unsafe extern "C" fn diddy_special_air_n_cancel_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, -1);
        FT_MOTION_RATE(fighter, 8.0/(35.0 - 1.0));
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn diddy_special_air_n_cancel_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

unsafe extern "C" fn diddy_special_air_n_cancel_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

unsafe extern "C" fn diddy_special_air_n_cancel_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

unsafe extern "C" fn edge_special_n_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(fighter, 20.0, 32.0, 8.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_S, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 32.0);
    FT_MOTION_RATE_RANGE(fighter, 32.0, 79.0, 51.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_M, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 79.0);
    FT_MOTION_RATE(fighter, 1.2);
    frame(lua_state, 99.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 100.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_L, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 105.0);
    FT_MOTION_RATE(fighter, 1.6);
    frame(lua_state, 115.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 120.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, -1);
    }
    frame(lua_state, 140.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
}

unsafe extern "C" fn edge_special_n1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 11.0, 13.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, 0);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE_RANGE(fighter, 15.0, 35.0, 5.0);
    frame(lua_state, 35.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }

}

unsafe extern "C" fn edge_special_n2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 11.0, 13.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, 0);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE(fighter, 0.4);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
    frame(lua_state, 60.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}

unsafe extern "C" fn ganon_float_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::ganon::status::FLOAT_GROUND_DECIDE_ANGLE);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::ganon::status::FLOAT_GROUND_CHANGE_KINETIC);
        VarModule::on_flag(fighter.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn ganon_float_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 6.0);
    for _ in 0..5 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
}

unsafe extern "C" fn ganon_float_start_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn ganon_float_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ganon_appear01"));
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_jump01"));
    }
}

unsafe extern "C" fn ganon_float_air_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 1.0 / 10.0);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn ganon_float_air_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
    }
    frame(lua_state, 2.0);
    for _ in 0..5 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
}

unsafe extern "C" fn ganon_float_air_start_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn ganon_float_air_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ganon_appear01"));
    }
}

unsafe extern "C" fn ganon_float_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::ganon::status::FLOAT_FALL_SPEED_Y_INCREASE);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn ganon_float_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
    }
    for _ in 0..5 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ganon_final_hand_triforce"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("ganon_entry_aura"), false, false);
    }
}

unsafe extern "C" fn ganon_float_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn koopa_special_n_start_effect(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 0.961, 0.569, 0.569, 0.392);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(fighter, 20, 0, 0, 0, 0);
    }
    wait(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

unsafe extern "C" fn koopa_special_n_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_special_n01"));
    }
    wait(lua_state, 19.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_kirby_copy_koopa_01"));
    }
}

unsafe extern "C" fn koopa_special_n_end_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(fighter,1.0,31.0,16.0);
}

unsafe extern "C" fn koopa_special_n_max_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,KOOPA_MAX_COOLDOWN);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_START);
    }
}

unsafe extern "C" fn koopa_special_n_max_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_explosion_sign"), Hash40::new("jaw"), 0, 1.0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(fighter,1.5);

        if fighter.is_motion(Hash40::new("koopa_special_n_max")){
            LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        FLASH(fighter, 0.961, 0.569, 0.569, 0.392);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(fighter, 20, 0, 0, 0, 0);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_explosion_sign"),false,false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("koopa_breath_m_fire"),false,false);

        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire_fly"), Hash40::new("jaw"), 0, 0, 0, 180, 0, 50, 0.5, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("koopa_appeal_s"), Hash40::new("mouth2"), 0, -1.3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter,2.0,0.5,0);
    }
}

unsafe extern "C" fn koopa_special_n_max_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_special_n01"));
    }
    wait(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        if fighter.is_motion(Hash40::new("koopa_special_n_max")){
            PLAY_SE_REMAIN(fighter, Hash40::new("se_koopa_step_left_m"));
        }
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_fire_m_damage"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_kirby_attack05"));
    }
}

unsafe extern "C" fn koopa_special_n_max_expression(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn koopajr_special_n_shoot_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_KOOPAJR_STATUS_SPECIAL_N_FLAG_FAIL) {
            EFFECT(fighter, Hash40::new("koopajr_cannon_miss"), Hash40::new("clowntongue2"), 3, 0, 0, 0, 0, -90, 0.5, 0, 0, 0, 0, 0, 0, true);
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        else {
            let offset = if fighter.is_situation(*SITUATION_KIND_GROUND) { 0 } else { 2 };
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
    }
}

unsafe extern "C" fn krool_special_n_fire_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
            EFFECT(fighter, Hash40::new("krool_cannon_shot"), Hash40::new("top"), 16, 10, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn krool_special_n_fire_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_krool_special_n08"));
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL) {
            PLAY_SE(fighter, Hash40::new("se_krool_special_n07"));
        }
        else if !VarModule::is_flag(fighter.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
            PLAY_SE(fighter, Hash40::new("se_krool_special_n01"));
        }
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_krool_special_n09"));
    }
}

unsafe extern "C" fn krool_special_n_fire_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
        VisibilityModule::set_int64(boma, hash40("crown") as i64, hash40("crown_hide") as i64);
    }
    if IS_EXIST_ARTICLE(fighter, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT) {
        if is_excute(fighter) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) && IS_GENERATABLE_ARTICLE(fighter, *FIGHTER_KROOL_GENERATE_ARTICLE_IRONBALL) {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 76.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("crown") as i64, hash40("crown_normal") as i64);
    }
    if is_excute(fighter) {
        if IS_EXIST_ARTICLE(fighter, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 89.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn krool_special_n_loop_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
            EFFECT_FOLLOW(fighter, Hash40::new("krool_cannon_vacuum"), Hash40::new("top"), 0, 10, 17, 0, 0, 0, 0.8, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("krool_cannon_vacuum"), Hash40::new("top"), 0, 10, 17, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_SCALE_W(fighter, 0.6, 2.0, 1.0);
            LAST_EFFECT_SET_ALPHA(fighter, 0.9);
        }
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.6, 10, 0, 4, 0, 0, 0, false);
        }
    }
    wait(lua_state, 10.0);
}

unsafe extern "C" fn littlemac_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        let damage =  22.0 * if fighter.is_situation(*SITUATION_KIND_GROUND) { 1.0 } else { 0.8 };
        let angle = if fighter.is_situation(*SITUATION_KIND_GROUND) { 80 } else { 75 };
        let bkb = if fighter.is_situation(*SITUATION_KIND_GROUND) { 40 } else { 30 };
        let kbg = if fighter.is_situation(*SITUATION_KIND_GROUND) { 104 } else { 124 };
        let shield_damage = if fighter.is_situation(*SITUATION_KIND_GROUND) { 2 } else { 0 };
        ATTACK(fighter, 0, 0, Hash40::new("armr"), damage, angle, kbg, 0, bkb, 5.0, 3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), damage, angle, kbg, 0, bkb, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), damage, angle, kbg, 0, bkb, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("bust"), damage, angle, kbg, 0, bkb, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_damage_shake_scale(boma, 0.67);
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
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.object(), vars::kirby::status::KO_PUNCH_GRAVITY);
    }
    
}

unsafe extern "C" fn littlemac_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let size = 1.0;
        EFFECT_FLW_POS(fighter, Hash40::new("littlemac_ko_uppercut_start"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, size, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("littlemac_ko_uppercut"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, size, true);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("littlemac_ko_uppercut_start"), -1);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), 0.5, 1, -3, 0, -60, 70, 1, true);
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), 0.5, 1, -3, 0, -60, 70, 1, false);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, true);
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, false);
        }
    }
    frame(lua_state, 8.0);
    let mut handle = EffectModule::req_follow(boma, Hash40::new("sys_starrod_bullet"), Hash40::new("handr"), &Vector3f::new(3.0, 0.0, 0.0), &Vector3f::new(45.0, 135.0, 45.0), 0.3, false, 0, 0, 0, 0, 0, false, false);
    if is_excute(fighter) {
        EffectModule::set_rate(boma, handle as u32, 1.5);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EffectModule::set_scale(boma, handle as u32, &Vector3f::new(0.8, 0.8, 0.8));
        let facing = PostureModule::lr(boma);
        EffectModule::set_rot(boma, handle as u32, &Vector3f::new(45.0 * facing, 135.0, 45.0 * facing));
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

unsafe extern "C" fn littlemac_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_special_n03"));
        PLAY_SE(fighter, Hash40::new("vc_kirby_hammermax"));
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_swing_ll"));
    }

}

unsafe extern "C" fn littlemac_special_n_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
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

// SPECIAL N START //

unsafe extern "C" fn lucas_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 10, 0, 0, 55, 14.0, 0.0, 10.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

unsafe extern "C" fn lucas_special_n_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_smash_l03"));
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_kirby_010"));
    }

}

// SPECIAL N HOLD //

unsafe extern "C" fn lucas_special_n_hold_game(fighter: &mut L2CAgentBase) {
    // INTENTIONALLY LEFT BLANK
    /* if fighter.kind() == *FIGHTER_KIND_KIRBY {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    } */
}

unsafe extern "C" fn lucas_special_n_hold_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 9, 0, 9, 0, 0, 0, false);
        FLASH(fighter, 0.01, 0.5, 1, 0.4);
    }
    for i in 1..=50 {
        if is_excute(fighter) {
            if i%2==0 {
                EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkfr_hold"), false, false);
                EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkfr_hold"), Hash40::new("top"), 0, sv_math::rand(hash40("fighter"), 4) as i32 + 12, sv_math::rand(hash40("fighter"), 4) as i32 - 2, 0, 0, 0, 0.5, true);
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_defense_up"), false, false);
                EFFECT_FLW_POS(fighter, Hash40::new("sys_status_defense_up"), Hash40::new("top"), 0, sv_math::rand(hash40("fighter"), 4) as i32 + 12, sv_math::rand(hash40("fighter"), 4) as i32 - 2, 0, 0, 0, 0.2, true);
            }
            if i%4==0 {
                EFFECT_FLW_POS(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, false);
            }
            FLASH(fighter, 1, 1, 1, 0.6);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter){
            COL_NORMAL(fighter);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            FLASH(fighter, 1, 1, 1, 0.6);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter){
            COL_NORMAL(fighter);
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn lucas_special_n_hold_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_special_h02"));
        PLAY_STATUS(fighter, Hash40::new("se_lucas_pk_charge"));
    }
}

// SPECIAL N FIRE //

unsafe extern "C" fn lucas_special_n_fire_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT) {
        frame(lua_state, 2.0);
        if is_excute(fighter) {
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 115, 0, 50, 3.0, 0.0, 10.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 100, 0, 50, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
            AttackModule::clear_all(boma);
        }
    } else {
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkfr_hold"), false, false);
        }
        frame(lua_state, 2.0);
        if is_excute(fighter) {
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
        }

    }
}

unsafe extern "C" fn lucas_special_n_fire_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkt_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.9, true);
        EFFECT_FLW_POS(fighter, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.5, true);
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 1..=5 {
        if is_excute(fighter) {
            FLASH(fighter, 0.01, 0.5, 1, 0.4);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            FLASH(fighter, 1, 1, 1, 0.6);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        wait(lua_state, 3.0)
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_pkt_hold"), false, false);
    }
}

unsafe extern "C" fn lucas_special_n_fire_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_kirby_attack04"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_special_n04_l"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_electric_hit_m"));
    }
}

unsafe extern "C" fn luigi_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, 0);
    }

}

unsafe extern "C" fn luigi_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("luigi_fb_shoot"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
        FLASH(fighter, 0, 1, 0, 0.353);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("luigi_fb_shoot"), false, false);
    }

}

unsafe extern "C" fn luigi_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_luigi_special_n01"));
    }

}

unsafe extern "C" fn luigi_special_n_thunder_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 68, 55, 0, 65, 5.0, 0.0, 6.5, 9.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 68, 55, 0, 65, 3.0, 0.0, 6.5, 3.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn luigi_special_n_thunder_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 5, 15, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        let mut rand = &Vector3f::new(
            app::sv_math::rand(hash40("fighter"), 50) as f32,
            app::sv_math::rand(hash40("stage"), 50) as f32,
            app::sv_math::rand(hash40("luigi"), 50) as f32
        );
        let mut flip = &Vector3f::new(
            if app::sv_math::rand(hash40("fighter"), 1) == 0 { -1 } else { 1 } as f32,
            if app::sv_math::rand(hash40("stage"), 1) == 0 { -1 } else { 1 } as f32,
            if app::sv_math::rand(hash40("luigi"), 1) == 0 { -1 } else { 1 } as f32
        );
        EFFECT_FOLLOW(fighter, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 6.5, 9.0, 0.0 + (rand.x * flip.x), 0, 0, 0.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 6.5, 9.0, 120.0 + (rand.y * flip.y), 0, 0, 0.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 6.5, 9.0, 240.0 + (rand.z * flip.z), 0, 0, 0.5, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0.25, 1.0, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 6.5, 9.0, 0, 90, 90, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0.0, 6.5, 9.0, 0, 90, 90, 0.5, true);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 0.2);
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

unsafe extern "C" fn luigi_special_n_thunder_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_kirby_attack02"));
        PLAY_SE(fighter, Hash40::new("se_common_electric_hit_l"));
    }
}

unsafe extern "C" fn luigi_special_n_thunder_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_55_smash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn mario_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(fighter, 10.0, 14.0, 8.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE_RANGE(fighter, 21.0, 49.0, 23.0);
    frame(lua_state, 49.0);
    FT_MOTION_RATE(fighter, 1.0);

}

unsafe extern "C" fn mario_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if PostureModule::lr(boma) > 0.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 1, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 0, 0, 0.353);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }

}

unsafe extern "C" fn mario_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
    }
}

unsafe extern "C" fn mario_special_n_fire_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(fighter, 10.0, 11.0, 7.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(-0.5, 0.0, 0.0));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 101, 0, 52, 3.0, 0.0, 5.5, 4.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 50, 101, 0, 52, 5.0, 0.0, 6.5, 11.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 40, 100, 0, 50, 3.0, 0.0, 5.5, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 40, 100, 0, 50, 5.0, 0.0, 6.5, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn mario_special_n_fire_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.5);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 6, 11, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if PostureModule::lr(boma) > 0.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 0.7, true);
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0, 0, 0, 0, 45, 0, 0.7, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 0.7, true);
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0, 0, 0, 0, -45, 0, 0.7, true);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 0.2);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 0, 0, 0.35);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flame"), Hash40::new("handl"), 0.0, 0, 0, 0, 0, 0, 0.2, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flame"), Hash40::new("handr"), 0.0, 0, 0, 0, 0, 0, 0.2, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 6.5, 11.5, 0, 0, 0, 0.26, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        EffectModule::enable_sync_init_pos_last(boma);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 0.2);
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 0, 0, 0.75);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_bomb_a"), false, false);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 0, 0, 0.35);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_flame"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
}

unsafe extern "C" fn mario_special_n_fire_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
        PLAY_SE(fighter, Hash40::new("vc_kirby_attack02"));
    }
}

unsafe extern "C" fn mario_special_n_fire_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_55_smash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn mariod_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(fighter, 10.0, 15.0, 8.0);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, 0);
    }
}

unsafe extern "C" fn mariod_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.46, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn mariod_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mariod_special_n01"));
    }
}

unsafe extern "C" fn mariod_special_n_chill_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 69, 84, 0, 42, 3.5, 0.0, 6.5, 4.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 69, 84, 0, 42, 4.75, 0.0, 4.0, 7.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE_RANGE(fighter, 19.0, 43.0, 36.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 43.0);
    FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn mariod_special_n_chill_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 8, 11, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.5, 0.25, 1, 0.35);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 4, 7, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_ice_landing"), Hash40::new("top"), 0, 4, 7, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FLIP(fighter, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), 0, 4, 7, 0, 0, 0, 0.46, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 4, 7, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 0.2);
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

unsafe extern "C" fn mariod_special_n_chill_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_frieze_l"));
        PLAY_SE(fighter, Hash40::new("vc_kirby_attack03"));
    }
}

unsafe extern "C" fn mariod_special_n_chill_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_55_smash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn miigunner_special_n1_fire_max_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 0, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(fighter, 2.0);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 100.0, 10.0);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 90, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(fighter, 2.0);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 100.0, 3.0);
		}
	}
	frame(lua_state, 2.6);
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("miigunner_sb_tama"), false, false);
		EFFECT_DETACH_KIND(fighter, Hash40::new("miigunner_sb_tama"), -1);
	}
	frame(lua_state, 2.8);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_air_bullet"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 5.0, 0.55);
			LAST_EFFECT_SET_RATE(fighter, 0.85);
		}
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, false);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_laser"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_SCALE_W(fighter, 1.0, 0.7, 1.0);
			LAST_EFFECT_SET_RATE(fighter, 0.8);
			EFFECT_FOLLOW(fighter, Hash40::new("miigunner_atk_shot_s"), Hash40::new("armr"), 6.3, 0, 0, 0, 0, -90, 1, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 5.0, 5.0);
			LAST_EFFECT_SET_RATE(fighter, 0.5);
			if fighter.is_situation(*SITUATION_KIND_GROUND) {
				LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
				FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
			}
		}
		else {
			EFFECT(fighter, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			if fighter.is_situation(*SITUATION_KIND_GROUND) {
				LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			}
		}
	}

}

unsafe extern "C" fn miigunner_special_n1_fire_max_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_special_c1_n01"));
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			PLAY_SE(fighter, Hash40::new("se_miigunner_final01"));
		}
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			STOP_SE(fighter, Hash40::new("se_miigunner_final01"));
			PLAY_SE(fighter, Hash40::new("se_miigunner_final04"));
		}
	}

}

unsafe extern "C" fn palutena_special_n_effect(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn palutena_special_n_sound(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn palutena_special_n_expression(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn richter_special_n_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.5);
        LAST_EFFECT_SET_RATE(agent, 0.6);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_sp_flash"), false, true);
    }
}

unsafe extern "C" fn richter_special_n_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_richter_special_l01"));
        PLAY_SE(agent, Hash40::new("vc_kirby_copy_richter_01"));
    }
}

unsafe extern "C" fn richter_special_n_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn richter_special_air_n_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.5);
        LAST_EFFECT_SET_RATE(agent, 0.6);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_sp_flash"), false, true);
    }
}

unsafe extern "C" fn richter_special_air_n_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_richter_special_l01"));
        PLAY_SE(agent, Hash40::new("vc_kirby_copy_richter_01"));
    }
}

unsafe extern "C" fn richter_special_air_n_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn ridley_special_n_explode_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 9.0, 0.0, 8.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn ridley_special_n_explode_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn ridley_special_n_explode_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_smash_s01"));
        PLAY_SE(fighter, Hash40::new("vc_kirby_attack05"));
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_smash_s02"));
    }
}

unsafe extern "C" fn ridley_special_n_explode_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}

unsafe extern "C" fn ridley_special_air_n_explode_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 9.0, 0.0, 8.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn ridley_special_air_n_explode_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn ridley_special_air_n_explode_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_smash_s01"));
        PLAY_SE(fighter, Hash40::new("vc_kirby_attack05"));
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_smash_s02"));
    }
}

unsafe extern "C" fn ridley_special_air_n_explode_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}

unsafe extern "C" fn roy_special_n_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("roy_erupution_hold"), false, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("roy_erupution_hold"), Hash40::new("roy_erupution_hold"), Hash40::new("havel"), 0.0, 0.0, 0.0, -90.0, 90.0, 0.0, 1.4, true, *EF_FLIP_NONE);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("roy_sword"), Hash40::new("roy_sword"), Hash40::new("havel"), 0.0, 0.0, 0.0, -90.0, 90.0, 0.0, 1.0, true, *EF_FLIP_NONE);

        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("roy_attack_fire"), Hash40::new("roy_attack_fire"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(fighter, 1.25);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("roy_fire"), Hash40::new("roy_fire"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(fighter, 1.25);
        //AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), 7, Hash40::new("havel"), 0.0, 0.0, -0.8, Hash40::new("havel"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("roy_erupution_hold"), false, false);
        //EFFECT(fighter, Hash40::new("roy_eruption_bomb_main"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        //LAST_EFFECT_SET_RATE(fighter, 1.5);
        //EFFECT(fighter, Hash40::new("roy_eruption_bomb_start"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 1.05, 0, 0, 0, 0, 0, 0, true);
        //LAST_EFFECT_SET_RATE(fighter, 1.5);
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("roy_fire"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("roy_attack_fire"), false, false);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        EFFECT_OFF_KIND(fighter, Hash40::new("roy_sword"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("roy_erupution_hold"), false, false);
    }

}

unsafe extern "C" fn roy_special_n_end_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_roy_special_n01"));
        PLAY_SE(fighter, Hash40::new("se_roy_special_n02"));
        PLAY_SE(fighter, Hash40::new("vc_kirby_copy_roy_02"));
        PLAY_SE(fighter, Hash40::new("se_roy_attackl_s01"));
    }
}

unsafe extern "C" fn shizue_special_n_failure_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("shizue_cracker"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn shizue_special_n_failure_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 0);
    }
}

unsafe extern "C" fn sonic_special_n_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        let temp = Vector3f { x: -0.3, y: 1.0, z: 0.0 };
		KineticModule::add_speed(boma, &temp);
    }
    FT_MOTION_RATE(fighter, 0.5);

}

unsafe extern "C" fn sonic_special_n_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

unsafe extern "C" fn sonic_special_n_hit_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("vc_kirby_copy_sonic_01"));
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_kirby_appeal01"));
    }
}

pub fn install() {
    smashline::Agent::new("roy")
        .acmd("sound_royspecialnend", roy_special_n_end_sound)
        .acmd("sound_royspecialairnend", roy_special_n_end_sound)
        .install();
    smashline::Agent::new("kirby")
        .acmd("game_diddyspecialncancel", diddy_special_n_cancel_game)
        .acmd("effect_diddyspecialncancel", diddy_special_n_cancel_effect)
        .acmd("sound_diddyspecialncancel", diddy_special_n_cancel_sound)
        .acmd(
            "expression_diddyspecialncancel",
            diddy_special_n_cancel_expression,
        )
        .acmd(
            "game_diddyspecialairncancel",
            diddy_special_air_n_cancel_game,
        )
        .acmd(
            "effect_diddyspecialairncancel",
            diddy_special_air_n_cancel_effect,
        )
        .acmd(
            "sound_diddyspecialairncancel",
            diddy_special_air_n_cancel_sound,
        )
        .acmd(
            "expression_diddyspecialairncancel",
            diddy_special_air_n_cancel_expression,
        )
        .acmd("game_edgespecialnstart", edge_special_n_start)
        .acmd("game_edgespecialairnstart", edge_special_n_start)
        .acmd("game_edgespecialn1", edge_special_n1_game)
        .acmd("game_edgespecialairn1", edge_special_n1_game)
        .acmd("game_edgespecialn2", edge_special_n2_game)
        .acmd("game_edgespecialairn2", edge_special_n2_game)
        .acmd("game_ganonfloatstart", ganon_float_start_game)
        .acmd("effect_ganonfloatstart", ganon_float_start_effect)
        .acmd("expression_ganonfloatstart", ganon_float_start_expression)
        .acmd("sound_ganonfloatstart", ganon_float_start_sound)
        .acmd("game_ganonfloatairstart", ganon_float_air_start_game)
        .acmd("effect_ganonfloatairstart", ganon_float_air_start_effect)
        .acmd(
            "expression_ganonfloatairstart",
            ganon_float_air_start_expression,
        )
        .acmd("sound_ganonfloatairstart", ganon_float_air_start_sound)
        .acmd("game_ganonfloat", ganon_float_game)
        .acmd("effect_ganonfloat", ganon_float_effect)
        .acmd("expression_ganonfloat", ganon_float_expression)
        .acmd("effect_koopaspecialnstart", koopa_special_n_start_effect)
        .acmd("effect_koopaspecialairnstart", koopa_special_n_start_effect)
        .acmd("sound_koopaspecialnstart", koopa_special_n_start_sound)
        .acmd("game_koopaspecialnend", koopa_special_n_end_game)
        .acmd("game_koopaspecialairnend", koopa_special_n_end_game)
        .acmd("game_koopaspecialnmax", koopa_special_n_max_game)
        .acmd("game_koopaspecialairnmax", koopa_special_n_max_game)
        .acmd("effect_koopaspecialnmax", koopa_special_n_max_effect)
        .acmd("effect_koopaspecialairnmax", koopa_special_n_max_effect)
        .acmd("sound_koopaspecialnmax", koopa_special_n_max_sound)
        .acmd("sound_koopaspecialairnmax", koopa_special_n_max_sound)
        .acmd(
            "expression_koopaspecialnmax",
            koopa_special_n_max_expression,
        )
        .acmd(
            "expression_koopaspecialairnmax",
            koopa_special_n_max_expression,
        )
        .acmd(
            "effect_koopajrspecialnshoot",
            koopajr_special_n_shoot_effect,
        )
        .acmd("effect_kroolspecialnfire", krool_special_n_fire_effect)
        .acmd("effect_kroolspecialairnfire", krool_special_n_fire_effect)
        .acmd("sound_kroolspecialnfire", krool_special_n_fire_sound)
        .acmd("sound_kroolspecialairnfire", krool_special_n_fire_sound)
        .acmd(
            "expression_kroolspecialnfire",
            krool_special_n_fire_expression,
        )
        .acmd(
            "expression_kroolspecialairnfire",
            krool_special_n_fire_expression,
        )
        .acmd("effect_kroolspecialnloop", krool_special_n_loop_effect)
        .acmd("effect_kroolspecialairnloop", krool_special_n_loop_effect)
        .acmd("game_littlemacspecialn", littlemac_special_n_game)
        .acmd("game_littlemacspecialairn", littlemac_special_n_game)
        .acmd("effect_littlemacspecialn", littlemac_special_n_effect)
        .acmd("effect_littlemacspecialairn", littlemac_special_n_effect)
        .acmd("sound_littlemacspecialn", littlemac_special_n_sound)
        .acmd("sound_littlemacspecialairn", littlemac_special_n_sound)
        .acmd("expression_littlemacspecialn", littlemac_special_n_expression)
        .acmd("expression_littlemacspecialairn", littlemac_special_n_expression)
        .acmd("game_lucasspecialnstart", lucas_special_n_start_game)
        .acmd("game_lucasspecialairnstart", lucas_special_n_start_game)
        .acmd("sound_lucasspecialnstart", lucas_special_n_start_sound)
        .acmd("sound_lucasspecialairnstart", lucas_special_n_start_sound)
        .acmd("game_lucasspecialnhold", lucas_special_n_hold_game)
        .acmd("game_lucasspecialairnhold", lucas_special_n_hold_game)
        .acmd("effect_lucasspecialnhold", lucas_special_n_hold_effect)
        .acmd("effect_lucasspecialairnhold", lucas_special_n_hold_effect)
        .acmd("sound_lucasspecialairnhold", lucas_special_n_hold_sound)
        .acmd("sound_lucasspecialnhold", lucas_special_n_hold_sound)
        .acmd("game_lucasspecialnfire", lucas_special_n_fire_game)
        .acmd("game_lucasspecialairnfire", lucas_special_n_fire_game)
        .acmd("effect_lucasspecialnfire", lucas_special_n_fire_effect)
        .acmd("effect_lucasspecialairnfire", lucas_special_n_fire_effect)
        .acmd("sound_lucasspecialairnfire", lucas_special_n_fire_sound)
        .acmd("sound_lucasspecialnfire", lucas_special_n_fire_sound)
        .acmd("game_luigispecialn", luigi_special_n_game)
        .acmd("game_luigispecialairn", luigi_special_n_game)
        .acmd("effect_luigispecialn", luigi_special_n_effect)
        .acmd("effect_luigispecialairn", luigi_special_n_effect)
        .acmd("sound_luigispecialn", luigi_special_n_sound)
        .acmd("sound_luigispecialairn", luigi_special_n_sound)
        .acmd("game_luigispecialnthunder", luigi_special_n_thunder_game)
        .acmd("game_luigispecialairnthunder", luigi_special_n_thunder_game)
        .acmd(
            "effect_luigispecialnthunder",
            luigi_special_n_thunder_effect,
        )
        .acmd(
            "effect_luigispecialairnthunder",
            luigi_special_n_thunder_effect,
        )
        .acmd("sound_luigispecialnthunder", luigi_special_n_thunder_sound)
        .acmd(
            "sound_luigispecialairnthunder",
            luigi_special_n_thunder_sound,
        )
        .acmd(
            "expression_luigispecialnthunder",
            luigi_special_n_thunder_expression,
        )
        .acmd(
            "expression_luigispecialairnthunder",
            luigi_special_n_thunder_expression,
        )
        .acmd("game_mariospecialn", mario_special_n_game)
        .acmd("game_mariospecialairn", mario_special_n_game)
        .acmd("effect_mariospecialn", mario_special_n_effect)
        .acmd("effect_mariospecialairn", mario_special_n_effect)
        .acmd("sound_mariospecialn", mario_special_n_sound)
        .acmd("sound_mariospecialairn", mario_special_n_sound)
        .acmd("game_mariospecialnfire", mario_special_n_fire_game)
        .acmd("game_mariospecialairnfire", mario_special_n_fire_game)
        .acmd("effect_mariospecialnfire", mario_special_n_fire_effect)
        .acmd("effect_mariospecialairnfire", mario_special_n_fire_effect)
        .acmd("sound_mariospecialnfire", mario_special_n_fire_sound)
        .acmd("sound_mariospecialairnfire", mario_special_n_fire_sound)
        .acmd(
            "expression_mariospecialnfire",
            mario_special_n_fire_expression,
        )
        .acmd(
            "expression_mariospecialairnfire",
            mario_special_n_fire_expression,
        )
        .acmd("game_mariodspecialn", mariod_special_n_game)
        .acmd("game_mariodspecialairn", mariod_special_n_game)
        .acmd("effect_mariospecialn", mariod_special_n_effect)
        .acmd("effect_mariospecialairn", mariod_special_n_effect)
        .acmd("sound_mariodspecialn", mariod_special_n_sound)
        .acmd("sound_mariodspecialairn", mariod_special_n_sound)
        .acmd("game_mariodspecialnchill", mariod_special_n_chill_game)
        .acmd("game_mariodspecialairnchill", mariod_special_n_chill_game)
        .acmd("effect_mariodspecialnchill", mariod_special_n_chill_effect)
        .acmd(
            "effect_mariodspecialairnchill",
            mariod_special_n_chill_effect,
        )
        .acmd("sound_mariodspecialnchill", mariod_special_n_chill_sound)
        .acmd("sound_mariodspecialairnchill", mariod_special_n_chill_sound)
        .acmd(
            "expression_mariodspecialnchill",
            mariod_special_n_chill_expression,
        )
        .acmd(
            "expression_mariodspecialairnchill",
            mariod_special_n_chill_expression,
        )
        .acmd(
            "effect_miigunnerspecialn1firemax",
            miigunner_special_n1_fire_max_effect,
        )
        .acmd(
            "effect_miigunnerspecialairn1firemax",
            miigunner_special_n1_fire_max_effect,
        )
        .acmd(
            "sound_miigunnerspecialn1firemax",
            miigunner_special_n1_fire_max_sound,
        )
        .acmd(
            "sound_miigunnerspecialairn1firemax",
            miigunner_special_n1_fire_max_sound,
        )
        .acmd("effect_palutenaspecialn", palutena_special_n_effect)
        .acmd("effect_palutenaspecialairn", palutena_special_n_effect)
        .acmd("sound_palutenaspecialn", palutena_special_n_sound)
        .acmd("sound_palutenaspecialairn", palutena_special_n_sound)
        .acmd("expression_palutenaspecialn", palutena_special_n_expression)
        .acmd(
            "expression_palutenaspecialairn",
            palutena_special_n_expression,
        )
        .acmd("effect_richterspecialn", richter_special_n_effect)
        .acmd("sound_richterspecialn", richter_special_n_sound)
        .acmd("expression_richterspecialn", richter_special_n_expression)
        .acmd("effect_richterspecialairn", richter_special_air_n_effect)
        .acmd("sound_richterspecialairn", richter_special_air_n_sound)
        .acmd("expression_richterspecialairn", richter_special_air_n_expression)
        .acmd("game_ridleyspecialnexplode", ridley_special_n_explode_game)
        .acmd(
            "effect_ridleyspecialnexplode",
            ridley_special_n_explode_effect,
        )
        .acmd(
            "sound_ridleyspecialnexplode",
            ridley_special_n_explode_sound,
        )
        .acmd(
            "expression_ridleyspecialnexplode",
            ridley_special_n_explode_expression,
        )
        .acmd(
            "game_ridleyspecialairnexplode",
            ridley_special_air_n_explode_game,
        )
        .acmd(
            "effect_ridleyspecialairnexplode",
            ridley_special_air_n_explode_effect,
        )
        .acmd(
            "sound_ridleyspecialairnexplode",
            ridley_special_air_n_explode_sound,
        )
        .acmd(
            "expression_ridleyspecialairnexplode",
            ridley_special_air_n_explode_expression,
        )
        .acmd("effect_royspecialnend", roy_special_n_end_effect)
        .acmd("effect_royspecialairnend", roy_special_n_end_effect)
        .acmd(
            "effect_shizuespecialnfailure",
            shizue_special_n_failure_effect,
        )
        .acmd(
            "effect_shizuespecialairnfailure",
            shizue_special_n_failure_effect,
        )
        .acmd(
            "expression_shizuespecialnfailure",
            shizue_special_n_failure_expression,
        )
        .acmd(
            "expression_shizuespecialairnfailure",
            shizue_special_n_failure_expression,
        )
        .acmd("game_sonicspecialnhit", sonic_special_n_hit_game)
        .acmd("effect_sonicspecialnhit", sonic_special_n_hit_effect)
        .acmd("sound_sonicspecialnhit", sonic_special_n_hit_sound)
        .install();
}
