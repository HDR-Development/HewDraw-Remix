use super::*;

// BAYONETTA
// BRAVE
// BUDDY
// CAPTAIN
// CHROM
// CLOUD
// DAISY
// DEDEDE
// DEMON
// DIDDY
unsafe extern "C" fn game_diddyspecialncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, -1);
        FT_MOTION_RATE(agent, 8.0/(31.0 - 1.0));
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_diddyspecialncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn sound_diddyspecialncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn expression_diddyspecialncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn game_diddyspecialairncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, -1);
        FT_MOTION_RATE(agent, 8.0/(35.0 - 1.0));
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_diddyspecialairncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn sound_diddyspecialairncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn expression_diddyspecialairncancel(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}


// DOLLY
// DONKEY
// DUCKHUNT
// EDGE
unsafe extern "C" fn game_edgespecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 20.0, 32.0, 8.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_S, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 32.0);
    FT_MOTION_RATE_RANGE(agent, 32.0, 79.0, 51.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_M, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 79.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 99.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 100.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_L, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 105.0);
    FT_MOTION_RATE(agent, 1.6);
    frame(lua_state, 115.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 120.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, -1);
    }
    frame(lua_state, 140.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
}

unsafe extern "C" fn game_edgespecialn1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 11.0, 13.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, 0);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE_RANGE(agent, 15.0, 35.0, 5.0);
    frame(lua_state, 35.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }

}

unsafe extern "C" fn game_edgespecialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 11.0, 13.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, 0);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE(agent, 0.4);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
    frame(lua_state, 60.0);
    FT_MOTION_RATE(agent, 1.0);
    
}


// EFLAME
// ELIGHT
// FALCO
// FOX
// GAMEWATCH
// GANON
unsafe extern "C" fn game_ganonfloatstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_GROUND_DECIDE_ANGLE);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_GROUND_CHANGE_KINETIC);
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_ganonfloatstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 6.0);
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
}

unsafe extern "C" fn expression_ganonfloatstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn sound_ganonfloatstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_jump01"));
    }
}

unsafe extern "C" fn game_ganonfloatairstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0 / 10.0);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_ganonfloatairstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
    }
    frame(lua_state, 2.0);
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
}

unsafe extern "C" fn expression_ganonfloatairstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn sound_ganonfloatairstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
}

unsafe extern "C" fn game_ganonfloat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 20.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::ganon::status::FLOAT_FALL_SPEED_Y_INCREASE);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_ganonfloat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_final_hand_triforce"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_entry_aura"), false, false);
    }
}

unsafe extern "C" fn expression_ganonfloat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


// GAOGAEN
// GEKKOUGA
// IKE
// INKLING
// JACK
// KAMUI
// KEN
// KOOPA
unsafe extern "C" fn effect_koopaspecialnstart(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    wait(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_koopaspecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
    wait(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("vc_kirby_copy_koopa_01"));
    }
}

unsafe extern "C" fn game_koopaspecialnend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent,1.0,31.0,16.0);
}

unsafe extern "C" fn game_koopaspecialnmax(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,KOOPA_MAX_COOLDOWN);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_START);
    }
}

unsafe extern "C" fn effect_koopaspecialnmax(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_sign"), Hash40::new("jaw"), 0, 1.0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(agent,1.5);

        if agent.is_motion(Hash40::new("koopa_special_n_max")){
            LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("sys_explosion_sign"),false,false);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("koopa_breath_m_fire"),false,false);

        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("jaw"), 0, 0, 0, 180, 0, 50, 0.5, true);
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 42.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("koopa_appeal_s"), Hash40::new("mouth2"), 0, -1.3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent,2.0,0.5,0);
    }
}

unsafe extern "C" fn sound_koopaspecialnmax(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
    wait(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        if agent.is_motion(Hash40::new("koopa_special_n_max")){
            PLAY_SE_REMAIN(agent, Hash40::new("se_koopa_step_left_m"));
        }
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_fire_m_damage"));
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_attack05"));
    }
}

unsafe extern "C" fn expression_koopaspecialnmax(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


// KOOPAJR
unsafe extern "C" fn effect_koopajrspecialnshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_KOOPAJR_STATUS_SPECIAL_N_FLAG_FAIL) {
            EFFECT(agent, Hash40::new("koopajr_cannon_miss"), Hash40::new("clowntongue2"), 3, 0, 0, 0, 0, -90, 0.5, 0, 0, 0, 0, 0, 0, true);
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        else {
            let offset = if agent.is_situation(*SITUATION_KIND_GROUND) { 0 } else { 2 };
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
    }
}


// KROOL
unsafe extern "C" fn effect_kroolspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 30.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
            EFFECT(agent, Hash40::new("krool_cannon_shot"), Hash40::new("top"), 16, 10, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_kroolspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_n08"));
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL) {
            PLAY_SE(agent, Hash40::new("se_krool_special_n07"));
        }
        else if !VarModule::is_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
            PLAY_SE(agent, Hash40::new("se_krool_special_n01"));
        }
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_n09"));
    }
}

unsafe extern "C" fn expression_kroolspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
        VisibilityModule::set_int64(boma, hash40("crown") as i64, hash40("crown_hide") as i64);
    }
    if IS_EXIST_ARTICLE(agent, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT) {
        if is_excute(agent) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) && IS_GENERATABLE_ARTICLE(agent, *FIGHTER_KROOL_GENERATE_ARTICLE_IRONBALL) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("crown") as i64, hash40("crown_normal") as i64);
    }
    if is_excute(agent) {
        if IS_EXIST_ARTICLE(agent, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 89.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn effect_kroolspecialnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
            EFFECT_FOLLOW(agent, Hash40::new("krool_cannon_vacuum"), Hash40::new("top"), 0, 10, 17, 0, 0, 0, 0.8, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("krool_cannon_vacuum"), Hash40::new("top"), 0, 10, 17, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_SCALE_W(agent, 0.6, 2.0, 1.0);
            LAST_EFFECT_SET_ALPHA(agent, 0.9);
        }
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.6, 10, 0, 4, 0, 0, 0, false);
        }
    }
    wait(lua_state, 10.0);
}


// LINK
// LITTLEMAC
unsafe extern "C" fn game_littlemacspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        let damage =  22.0 * if agent.is_situation(*SITUATION_KIND_GROUND) { 1.0 } else { 0.8 };
        let angle = if agent.is_situation(*SITUATION_KIND_GROUND) { 80 } else { 75 };
        let bkb = if agent.is_situation(*SITUATION_KIND_GROUND) { 40 } else { 30 };
        let kbg = if agent.is_situation(*SITUATION_KIND_GROUND) { 104 } else { 124 };
        let shield_damage = if agent.is_situation(*SITUATION_KIND_GROUND) { 2 } else { 0 };
        ATTACK(agent, 0, 0, Hash40::new("armr"), damage, angle, kbg, 0, bkb, 5.0, 3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), damage, angle, kbg, 0, bkb, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), damage, angle, kbg, 0, bkb, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("bust"), damage, angle, kbg, 0, bkb, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_damage_shake_scale(boma, 0.67);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.object(), vars::kirby::status::KO_PUNCH_GRAVITY);
    }
    
}

unsafe extern "C" fn effect_littlemacspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let size = 1.0;
        EFFECT_FLW_POS(agent, Hash40::new("littlemac_ko_uppercut_start"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, size, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, size, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("littlemac_ko_uppercut_start"), -1);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), 0.5, 1, -3, 0, -60, 70, 1, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), 0.5, 1, -3, 0, -60, 70, 1, false);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, false);
        }
    }
    frame(lua_state, 8.0);
    let mut handle = EffectModule::req_follow(boma, Hash40::new("sys_starrod_bullet"), Hash40::new("handr"), &Vector3f::new(3.0, 0.0, 0.0), &Vector3f::new(45.0, 135.0, 45.0), 0.3, false, 0, 0, 0, 0, 0, false, false);
    if is_excute(agent) {
        EffectModule::set_rate(boma, handle as u32, 1.5);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EffectModule::set_scale(boma, handle as u32, &Vector3f::new(0.8, 0.8, 0.8));
        let facing = PostureModule::lr(boma);
        EffectModule::set_rot(boma, handle as u32, &Vector3f::new(45.0 * facing, 135.0, 45.0 * facing));
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("littlemac_ko_uppercut"), false, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_starrod_bullet"), false, false);
    }

}

unsafe extern "C" fn sound_littlemacspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_littlemac_special_n03"));
        PLAY_SE(agent, Hash40::new("vc_kirby_hammermax"));
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_littlemac_swing_ll"));
    }

}

unsafe extern "C" fn expression_littlemacspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_elecattack"), 0);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 4, 45, 200, 1, 17, 15, 38, 30, 50);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }

}

// LUCARIO
// LUCAS
unsafe extern "C" fn game_lucasspecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 10, 0, 0, 55, 14.0, 0.0, 10.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }

}

unsafe extern "C" fn sound_lucasspecialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_lucas_smash_l03"));
        PLAY_SE_REMAIN(agent, Hash40::new("vc_kirby_010"));
    }

}

unsafe extern "C" fn game_lucasspecialnhold(agent: &mut L2CAgentBase) {
    // INTENTIONALLY LEFT BLANK
    /* if agent.kind() == *FIGHTER_KIND_KIRBY {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    } */
}

unsafe extern "C" fn effect_lucasspecialnhold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 9, 0, 9, 0, 0, 0, false);
        FLASH(agent, 0.01, 0.5, 1, 0.4);
    }
    for i in 1..=50 {
        if is_excute(agent) {
            if i%2==0 {
                EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_hold"), false, false);
                EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_hold"), Hash40::new("top"), 0, sv_math::rand(hash40("fighter"), 4) as i32 + 12, sv_math::rand(hash40("fighter"), 4) as i32 - 2, 0, 0, 0, 0.5, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_status_defense_up"), false, false);
                EFFECT_FLW_POS(agent, Hash40::new("sys_status_defense_up"), Hash40::new("top"), 0, sv_math::rand(hash40("fighter"), 4) as i32 + 12, sv_math::rand(hash40("fighter"), 4) as i32 - 2, 0, 0, 0, 0.2, true);
            }
            if i%4==0 {
                EFFECT_FLW_POS(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, false);
            }
            FLASH(agent, 1, 1, 1, 0.6);
        }
        wait(lua_state, 1.0);
        if is_excute(agent){
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 1, 1, 0.6);
        }
        wait(lua_state, 1.0);
        if is_excute(agent){
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn sound_lucasspecialnhold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_lucas_special_h02"));
        PLAY_STATUS(agent, Hash40::new("se_lucas_pk_charge"));
    }
}

unsafe extern "C" fn game_lucasspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT) {
        frame(lua_state, 2.0);
        if is_excute(agent) {
            VarModule::on_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 115, 0, 50, 3.0, 0.0, 10.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 100, 0, 50, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
                VarModule::off_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
                VarModule::set_float(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
                VarModule::off_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            }
            AttackModule::clear_all(boma);
        }
    } else {
        frame(lua_state, 1.0);
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_hold"), false, false);
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            VarModule::off_flag(agent.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
        }

    }
}

unsafe extern "C" fn effect_lucasspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("lucas_pkt_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.9, true);
        EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.5, true);
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 1..=5 {
        if is_excute(agent) {
            FLASH(agent, 0.01, 0.5, 1, 0.4);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 1, 1, 0.6);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 3.0)
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkt_hold"), false, false);
    }
}

unsafe extern "C" fn sound_lucasspecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("vc_kirby_attack04"));
        PLAY_SE_REMAIN(agent, Hash40::new("se_lucas_special_n04_l"));
        PLAY_SE_REMAIN(agent, Hash40::new("se_common_electric_hit_m"));
    }
}


// LUCINA
// LUIGI
unsafe extern "C" fn game_luigispecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, 0);
    }

}

unsafe extern "C" fn effect_luigispecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_fb_shoot"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
        FLASH(agent, 0, 1, 0, 0.353);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("luigi_fb_shoot"), false, false);
    }

}

unsafe extern "C" fn sound_luigispecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_special_n01"));
    }

}

unsafe extern "C" fn game_luigispecialnthunder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 68, 55, 0, 65, 5.0, 0.0, 6.5, 9.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 68, 55, 0, 65, 3.0, 0.0, 6.5, 3.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_luigispecialnthunder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 5, 15, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
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
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 6.5, 9.0, 0.0 + (rand.x * flip.x), 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 6.5, 9.0, 120.0 + (rand.y * flip.y), 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 6.5, 9.0, 240.0 + (rand.z * flip.z), 0, 0, 0.5, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0.25, 1.0, 0.7);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 6.5, 9.0, 0, 90, 90, 0.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0.0, 6.5, 9.0, 0, 90, 90, 0.5, true);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_luigispecialnthunder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_attack02"));
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_l"));
    }
}

unsafe extern "C" fn expression_luigispecialnthunder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_55_smash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


// MARIO
unsafe extern "C" fn game_mariospecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 14.0, 8.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE_RANGE(agent, 21.0, 49.0, 23.0);
    frame(lua_state, 49.0);
    FT_MOTION_RATE(agent, 1.0);

}

unsafe extern "C" fn effect_mariospecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 1, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0, 0.353);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, false);
    }

}

unsafe extern "C" fn sound_mariospecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_special_n01"));
    }
}

unsafe extern "C" fn game_mariospecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 11.0, 7.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f::new(-0.5, 0.0, 0.0));
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 101, 0, 52, 3.0, 0.0, 5.5, 4.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 50, 101, 0, 52, 5.0, 0.0, 6.5, 11.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 40, 100, 0, 50, 3.0, 0.0, 5.5, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 40, 100, 0, 50, 5.0, 0.0, 6.5, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_mariospecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.5);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 6, 11, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 0.7, true);
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0, 0, 0, 0, 45, 0, 0.7, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 0.7, true);
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0, 0, 0, 0, -45, 0, 0.7, true);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0, 0.35);
        EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("handl"), 0.0, 0, 0, 0, 0, 0, 0.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("handr"), 0.0, 0, 0, 0, 0, 0, 0.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 6.5, 11.5, 0, 0, 0, 0.26, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
        EffectModule::enable_sync_init_pos_last(boma);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0, 0.75);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_bomb_a"), false, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0, 0.35);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_flame"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, false);
    }
}

unsafe extern "C" fn sound_mariospecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_special_n01"));
        PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
        PLAY_SE(agent, Hash40::new("vc_kirby_attack02"));
    }
}

unsafe extern "C" fn expression_mariospecialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_55_smash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


// MARIOD
unsafe extern "C" fn game_mariodspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 15.0, 8.0);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, 0);
    }
}

unsafe extern "C" fn effect_mariodspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.46, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_mariodspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_n01"));
    }
}

unsafe extern "C" fn game_mariodspecialnchill(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 69, 84, 0, 42, 3.5, 0.0, 6.5, 4.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 69, 84, 0, 42, 4.75, 0.0, 4.0, 7.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE_RANGE(agent, 19.0, 43.0, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 43.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_mariodspecialnchill(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 8, 11, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FLASH(agent, 0.5, 0.25, 1, 0.35);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 4, 7, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_ice_landing"), Hash40::new("top"), 0, 4, 7, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(agent, 0.75);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FLIP(agent, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), 0, 4, 7, 0, 0, 0, 0.46, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 4, 7, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_mariodspecialnchill(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_frieze_l"));
        PLAY_SE(agent, Hash40::new("vc_kirby_attack03"));
    }
}

unsafe extern "C" fn expression_mariodspecialnchill(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_55_smash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


// MARTH
// MASTER
// METAKNIGHT
// MEWTWO
// MIIFIGHTER
// MIIGUNNER
unsafe extern "C" fn effect_miigunnerspecialn1firemax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 0, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(agent, 2.0);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 100.0, 10.0);
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_sb_tama"), Hash40::new("armr"), 6.0, 0, 0, 90, 0, 0, 3.5, true);
			LAST_EFFECT_SET_RATE(agent, 2.0);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 100.0, 3.0);
		}
	}
	frame(lua_state, 2.6);
	if is_excute(agent) {
		EFFECT_OFF_KIND(agent, Hash40::new("miigunner_sb_tama"), false, false);
		EFFECT_DETACH_KIND(agent, Hash40::new("miigunner_sb_tama"), -1);
	}
	frame(lua_state, 2.8);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_air_bullet"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 5.0, 0.55);
			LAST_EFFECT_SET_RATE(agent, 0.85);
		}
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, false);
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_laser"), Hash40::new("top"), 0, 6.3, 10.5, 0, 0, 0, 1, false);
			LAST_EFFECT_SET_SCALE_W(agent, 1.0, 0.7, 1.0);
			LAST_EFFECT_SET_RATE(agent, 0.8);
			EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot_s"), Hash40::new("armr"), 6.3, 0, 0, 0, 0, -90, 1, false);
			LAST_EFFECT_SET_COLOR(agent, 0.15, 5.0, 5.0);
			LAST_EFFECT_SET_RATE(agent, 0.5);
			if agent.is_situation(*SITUATION_KIND_GROUND) {
				LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
				FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
			}
		}
		else {
			EFFECT(agent, Hash40::new("miigunner_cshot_shot"), Hash40::new("top"), 6, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			if agent.is_situation(*SITUATION_KIND_GROUND) {
				LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			}
		}
	}

}

unsafe extern "C" fn sound_miigunnerspecialn1firemax(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
	frame(lua_state, 1.0);
	if is_excute(agent) {
		PLAY_SEQUENCE(agent, Hash40::new("seq_miigunner_rnd_special_c1_n01"));
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			PLAY_SE(agent, Hash40::new("se_miigunner_final01"));
		}
	}
	frame(lua_state, 3.0);
	if is_excute(agent) {
		if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
			STOP_SE(agent, Hash40::new("se_miigunner_final01"));
			PLAY_SE(agent, Hash40::new("se_miigunner_final04"));
		}
	}

}


// MIISWORDSMAN
// MURABITO
// NANA
// NESS
// PACKUN
// PACMAN
// PALUTENA
unsafe extern "C" fn effect_palutenaspecialn(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn sound_palutenaspecialn(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn expression_palutenaspecialn(agent: &mut L2CAgentBase) {
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


// PEACH
// PFUSHIGISOU
// PICHU
// PICKEL
// PIKACHU
// PIKMIN
// PIT
// PITB
// PLIZARDON
// POPO
// PURIN
// PZENIGAME
// REFLET
// RICHTER
unsafe extern "C" fn effect_richterspecialn(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn sound_richterspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_richter_special_l01"));
        PLAY_SE(agent, Hash40::new("vc_kirby_copy_richter_01"));
    }
}

unsafe extern "C" fn expression_richterspecialn(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn effect_richterspecialairn(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn sound_richterspecialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_richter_special_l01"));
        PLAY_SE(agent, Hash40::new("vc_kirby_copy_richter_01"));
    }
}

unsafe extern "C" fn expression_richterspecialairn(agent: &mut L2CAgentBase) {
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


// RIDLEY
unsafe extern "C" fn game_ridleyspecialnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 9.0, 0.0, 8.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_ridleyspecialnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn sound_ridleyspecialnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_smash_s01"));
        PLAY_SE(agent, Hash40::new("vc_kirby_attack05"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_smash_s02"));
    }
}

unsafe extern "C" fn expression_ridleyspecialnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}

unsafe extern "C" fn game_ridleyspecialairnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 9.0, 0.0, 8.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_ridleyspecialairnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn sound_ridleyspecialairnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_smash_s01"));
        PLAY_SE(agent, Hash40::new("vc_kirby_attack05"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_smash_s02"));
    }
}

unsafe extern "C" fn expression_ridleyspecialairnexplode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}


// ROBOT
// ROCKMAN
// ROSETTA
// ROY
unsafe extern "C" fn effect_royspecialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_erupution_hold"), Hash40::new("roy_erupution_hold"), Hash40::new("havel"), 0.0, 0.0, 0.0, -90.0, 90.0, 0.0, 1.4, true, *EF_FLIP_NONE);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_sword"), Hash40::new("roy_sword"), Hash40::new("havel"), 0.0, 0.0, 0.0, -90.0, 90.0, 0.0, 1.0, true, *EF_FLIP_NONE);

        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_attack_fire"), Hash40::new("roy_attack_fire"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.25);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("roy_fire"), Hash40::new("roy_fire"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.25);
        //AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), 7, Hash40::new("havel"), 0.0, 0.0, -0.8, Hash40::new("havel"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
        //EFFECT(agent, Hash40::new("roy_eruption_bomb_main"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        //LAST_EFFECT_SET_RATE(agent, 1.5);
        //EFFECT(agent, Hash40::new("roy_eruption_bomb_start"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 1.05, 0, 0, 0, 0, 0, 0, true);
        //LAST_EFFECT_SET_RATE(agent, 1.5);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_fire"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_attack_fire"), false, false);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_sword"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_erupution_hold"), false, false);
    }

}

unsafe extern "C" fn sound_royspecialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_roy_special_n01"));
        PLAY_SE(agent, Hash40::new("se_roy_special_n02"));
        PLAY_SE(agent, Hash40::new("vc_kirby_copy_roy_02"));
        PLAY_SE(agent, Hash40::new("se_roy_attackl_s01"));
    }
}


// RYU
// SAMUS
// SAMUSD
// SHEIK
// SHIZUE
unsafe extern "C" fn effect_shizuespecialnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("shizue_cracker"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn expression_shizuespecialnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
    }
}


// SHULK
// SIMON
// SNAKE
// SONIC
unsafe extern "C" fn game_sonicspecialnhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let temp = Vector3f { x: -0.3, y: 1.0, z: 0.0 };
		KineticModule::add_speed(boma, &temp);
    }
    FT_MOTION_RATE(agent, 0.5);

}

unsafe extern "C" fn effect_sonicspecialnhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

}

unsafe extern "C" fn sound_sonicspecialnhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("vc_kirby_copy_sonic_01"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_appeal01"));
    }
}


// SZEROSUIT
// TANTAN
// TOONLINK
// TRAIL
// WARIO
// WIIFIT
// WOLF
// YOSHI
// YOUNGLINK
// ZELDA

pub fn install(agent: &mut Agent) {
    // BAYONETTA
    // BRAVE
    // CAPTAIN
    // CHROM
    // CLOUD
    // DAISY
    // DEDEDE
    // DEMON
    // DIDDY
        agent.acmd("game_diddyspecialncancel", game_diddyspecialncancel);
        agent.acmd("effect_diddyspecialncancel", effect_diddyspecialncancel);
        agent.acmd("sound_diddyspecialncancel", sound_diddyspecialncancel);
        agent.acmd(
            "expression_diddyspecialncancel",
            expression_diddyspecialncancel,
        );
        agent.acmd(
            "game_diddyspecialairncancel",
            game_diddyspecialairncancel,
        );
        agent.acmd(
            "effect_diddyspecialairncancel",
            effect_diddyspecialairncancel,
        );
        agent.acmd(
            "sound_diddyspecialairncancel",
            sound_diddyspecialairncancel,
        );
        agent.acmd(
            "expression_diddyspecialairncancel",
            expression_diddyspecialairncancel,
        );
    // DOLLY
    // DONKEY
    // DUCKHUNT
    // EDGE
        agent.acmd("game_edgespecialnstart", game_edgespecialnstart);
        agent.acmd("game_edgespecialairnstart", game_edgespecialnstart);
        agent.acmd("game_edgespecialn1", game_edgespecialn1);
        agent.acmd("game_edgespecialairn1", game_edgespecialn1);
        agent.acmd("game_edgespecialn2", game_edgespecialn2);
        agent.acmd("game_edgespecialairn2", game_edgespecialn2);
    // EFLAME
    // ELIGHT
    // FOX
    // FALCO
    // FOX
    // GAMEWATCH
    // GANON
        agent.acmd("game_ganonfloatstart", game_ganonfloatstart);
        agent.acmd("effect_ganonfloatstart", effect_ganonfloatstart);
        agent.acmd("expression_ganonfloatstart", expression_ganonfloatstart);
        agent.acmd("sound_ganonfloatstart", sound_ganonfloatstart);
        agent.acmd("game_ganonfloatairstart", game_ganonfloatairstart);
        agent.acmd("effect_ganonfloatairstart", effect_ganonfloatairstart);
        agent.acmd(
            "expression_ganonfloatairstart",
            expression_ganonfloatairstart,
        );
        agent.acmd("sound_ganonfloatairstart", sound_ganonfloatairstart);
        agent.acmd("game_ganonfloat", game_ganonfloat);
        agent.acmd("effect_ganonfloat", effect_ganonfloat);
        agent.acmd("expression_ganonfloat", expression_ganonfloat);
    // GAOGAEN
    // GEKKOUGA
    // IKE
    // INKLING
    // JACK
    // KAMUI
    // KEN
    // KOOPA
        agent.acmd("effect_koopaspecialnstart", effect_koopaspecialnstart);
        agent.acmd("effect_koopaspecialairnstart", effect_koopaspecialnstart);
        agent.acmd("sound_koopaspecialnstart", sound_koopaspecialnstart);
        agent.acmd("game_koopaspecialnend", game_koopaspecialnend);
        agent.acmd("game_koopaspecialairnend", game_koopaspecialnend);
        agent.acmd("game_koopaspecialnmax", game_koopaspecialnmax);
        agent.acmd("game_koopaspecialairnmax", game_koopaspecialnmax);
        agent.acmd("effect_koopaspecialnmax", effect_koopaspecialnmax);
        agent.acmd("effect_koopaspecialairnmax", effect_koopaspecialnmax);
        agent.acmd("sound_koopaspecialnmax", sound_koopaspecialnmax);
        agent.acmd("sound_koopaspecialairnmax", sound_koopaspecialnmax);
        agent.acmd(
            "expression_koopaspecialnmax",
            expression_koopaspecialnmax,
        );
        agent.acmd(
            "expression_koopaspecialairnmax",
            expression_koopaspecialnmax,
        );
    // KOOPAJR
      agent.acmd(
            "effect_koopajrspecialnshoot",
            effect_koopajrspecialnshoot,
        );
    // KROOL
        agent.acmd("effect_kroolspecialnfire", effect_kroolspecialnfire);
        agent.acmd("effect_kroolspecialairnfire", effect_kroolspecialnfire);
        agent.acmd("sound_kroolspecialnfire", sound_kroolspecialnfire);
        agent.acmd("sound_kroolspecialairnfire", sound_kroolspecialnfire);
        agent.acmd(
            "expression_kroolspecialnfire",
            expression_kroolspecialnfire,
        );
        agent.acmd(
            "expression_kroolspecialairnfire",
            expression_kroolspecialnfire,
        );
        agent.acmd("effect_kroolspecialnloop", effect_kroolspecialnloop);
        agent.acmd("effect_kroolspecialairnloop", effect_kroolspecialnloop);
    // LINK
    // LITTLEMAC
        agent.acmd("game_littlemacspecialn", game_littlemacspecialn);
        agent.acmd("game_littlemacspecialairn", game_littlemacspecialn);
        agent.acmd("effect_littlemacspecialn", effect_littlemacspecialn);
        agent.acmd("effect_littlemacspecialairn", effect_littlemacspecialn);
        agent.acmd("sound_littlemacspecialn", sound_littlemacspecialn);
        agent.acmd("sound_littlemacspecialairn", sound_littlemacspecialn);
        agent.acmd("expression_littlemacspecialn", expression_littlemacspecialn);
        agent.acmd("expression_littlemacspecialairn", expression_littlemacspecialn);
    // LUCARIO
    // LUCAS
        agent.acmd("game_lucasspecialnstart", game_lucasspecialnstart);
        agent.acmd("game_lucasspecialairnstart", game_lucasspecialnstart);
        agent.acmd("sound_lucasspecialnstart", sound_lucasspecialnstart);
        agent.acmd("sound_lucasspecialairnstart", sound_lucasspecialnstart);
        agent.acmd("game_lucasspecialnhold", game_lucasspecialnhold);
        agent.acmd("game_lucasspecialairnhold", game_lucasspecialnhold);
        agent.acmd("effect_lucasspecialnhold", effect_lucasspecialnhold);
        agent.acmd("effect_lucasspecialairnhold", effect_lucasspecialnhold);
        agent.acmd("sound_lucasspecialairnhold", effect_lucasspecialnhold);
        agent.acmd("sound_lucasspecialnhold", effect_lucasspecialnhold);
        agent.acmd("game_lucasspecialnfire", game_lucasspecialnfire);
        agent.acmd("game_lucasspecialairnfire", game_lucasspecialnfire);
        agent.acmd("effect_lucasspecialnfire", effect_lucasspecialnfire);
        agent.acmd("effect_lucasspecialairnfire", effect_lucasspecialnfire);
        agent.acmd("sound_lucasspecialairnfire", sound_lucasspecialnfire);
        agent.acmd("sound_lucasspecialnfire", sound_lucasspecialnfire);
    // LUCINA
    // LUIGI
        agent.acmd("game_luigispecialn", game_luigispecialn);
        agent.acmd("game_luigispecialairn", game_luigispecialn);
        agent.acmd("effect_luigispecialn", effect_luigispecialn);
        agent.acmd("effect_luigispecialairn", effect_luigispecialn);
        agent.acmd("sound_luigispecialn", sound_luigispecialn);
        agent.acmd("sound_luigispecialairn", sound_luigispecialn);
        agent.acmd("game_luigispecialnthunder", game_luigispecialnthunder);
        agent.acmd("game_luigispecialairnthunder", game_luigispecialnthunder);
        agent.acmd(
            "effect_luigispecialnthunder",
            effect_luigispecialnthunder,
        );
        agent.acmd(
            "effect_luigispecialairnthunder",
            effect_luigispecialnthunder,
        );
        agent.acmd("sound_luigispecialnthunder", sound_luigispecialnthunder);
        agent.acmd(
            "sound_luigispecialairnthunder",
            sound_luigispecialnthunder,
        );
        agent.acmd(
            "expression_luigispecialnthunder",
            expression_luigispecialnthunder,
        );
        agent.acmd(
            "expression_luigispecialairnthunder",
            expression_luigispecialnthunder,
        );
    // MARIO
        agent.acmd("game_mariospecialn", game_mariospecialn);
        agent.acmd("game_mariospecialairn", game_mariospecialn);
        agent.acmd("effect_mariospecialn", effect_mariospecialn);
        agent.acmd("effect_mariospecialairn", effect_mariospecialn);
        agent.acmd("sound_mariospecialn", sound_mariospecialn);
        agent.acmd("sound_mariospecialairn", sound_mariospecialn);
        agent.acmd("game_mariospecialnfire", game_mariospecialnfire);
        agent.acmd("game_mariospecialairnfire", game_mariospecialnfire);
        agent.acmd("effect_mariospecialnfire", effect_mariospecialnfire);
        agent.acmd("effect_mariospecialairnfire", effect_mariospecialnfire);
        agent.acmd("sound_mariospecialnfire", sound_mariospecialnfire);
        agent.acmd("sound_mariospecialairnfire", sound_mariospecialnfire);
        agent.acmd(
            "expression_mariospecialnfire",
            expression_mariospecialnfire,
        );
        agent.acmd(
            "expression_mariospecialairnfire",
            expression_mariospecialnfire,
        );
    // MARIOD
        agent.acmd("game_mariodspecialn", game_mariodspecialn);
        agent.acmd("game_mariodspecialairn", game_mariodspecialn);
        agent.acmd("effect_mariodspecialn", effect_mariodspecialn);
        agent.acmd("effect_mariodspecialairn", effect_mariodspecialn);
        agent.acmd("sound_mariodspecialn", sound_mariodspecialn);
        agent.acmd("sound_mariodspecialairn", sound_mariodspecialn);
        agent.acmd("game_mariodspecialnchill", game_mariodspecialnchill);
        agent.acmd("game_mariodspecialairnchill", game_mariodspecialnchill);
        agent.acmd("effect_mariodspecialnchill", effect_mariodspecialnchill);
        agent.acmd(
            "effect_mariodspecialairnchill",
            effect_mariodspecialnchill,
        );
        agent.acmd("sound_mariodspecialnchill", sound_mariodspecialnchill);
        agent.acmd("sound_mariodspecialairnchill", sound_mariodspecialnchill);
        agent.acmd(
            "expression_mariodspecialnchill",
            expression_mariodspecialnchill,
        );
        agent.acmd(
            "expression_mariodspecialairnchill",
            expression_mariodspecialnchill,
        );
    // MARTH
    // MASTER
    // METAKNIGHT
    // MEWTWO
    // MIIFIGHTER
    // MIIGUNNER
        agent.acmd(
            "effect_miigunnerspecialn1firemax",
            effect_miigunnerspecialn1firemax,
        );
        agent.acmd(
            "effect_miigunnerspecialairn1firemax",
            effect_miigunnerspecialn1firemax,
        );
        agent.acmd(
            "sound_miigunnerspecialn1firemax",
            sound_miigunnerspecialn1firemax,
        );
        agent.acmd(
            "sound_miigunnerspecialairn1firemax",
            sound_miigunnerspecialn1firemax,
        );
    // MIISWORDSMAN
    // MURABITO
    // NANA
    // NESS
    // PACKUN
    // PACMAN
    // PALUTENA
        agent.acmd("effect_palutenaspecialn", effect_palutenaspecialn);
        agent.acmd("effect_palutenaspecialairn", effect_palutenaspecialn);
        agent.acmd("sound_palutenaspecialn", sound_palutenaspecialn);
        agent.acmd("sound_palutenaspecialairn", sound_palutenaspecialn);
        agent.acmd("expression_palutenaspecialn", expression_palutenaspecialn);
        agent.acmd(
            "expression_palutenaspecialairn",
            expression_palutenaspecialn,
        );
    // PEACH
    // PFUSHIGISOU
    // PICHU
    // PICKEL
    // PIKACHU
    // PIKMIN
    // PIT
    // PITB
    // PLIZARDON
    // POPO
    // PURIN
    // PZENIGAME
    // REFLET
    // RICHTER
        agent.acmd("effect_richterspecialn", effect_richterspecialn);
        agent.acmd("sound_richterspecialn", sound_richterspecialn);
        agent.acmd("expression_richterspecialn", expression_richterspecialn);
        agent.acmd("effect_richterspecialairn", effect_richterspecialairn);
        agent.acmd("sound_richterspecialairn", sound_richterspecialairn);
        agent.acmd("expression_richterspecialairn", expression_richterspecialairn);
    // RIDLEY
        agent.acmd("game_ridleyspecialnexplode", game_ridleyspecialnexplode);
        agent.acmd(
            "effect_ridleyspecialnexplode",
            effect_ridleyspecialnexplode,
        );
        agent.acmd(
            "sound_ridleyspecialnexplode",
            sound_ridleyspecialnexplode,
        );
        agent.acmd(
            "expression_ridleyspecialnexplode",
            expression_ridleyspecialnexplode,
        );
        agent.acmd(
            "game_ridleyspecialairnexplode",
            game_ridleyspecialairnexplode,
        );
        agent.acmd(
            "effect_ridleyspecialairnexplode",
            effect_ridleyspecialairnexplode,
        );
        agent.acmd(
            "sound_ridleyspecialairnexplode",
            sound_ridleyspecialairnexplode,
        );
        agent.acmd(
            "expression_ridleyspecialairnexplode",
            expression_ridleyspecialairnexplode,
        );
    // ROBOT
    // ROCKMAN
    // ROSETTA
    // ROY
        agent.acmd("sound_royspecialnend", sound_royspecialnend);
        agent.acmd("sound_royspecialairnend", sound_royspecialnend);
        agent.acmd("effect_royspecialnend", effect_royspecialnend);
        agent.acmd("effect_royspecialairnend", effect_royspecialnend);
    // RYU
    // SAMUS
    // SAMUSD
    // SHEIK
    // SHIZUE
        agent.acmd(
            "effect_shizuespecialnfailure",
            effect_shizuespecialnfailure,
        );
        agent.acmd(
            "effect_shizuespecialairnfailure",
            effect_shizuespecialnfailure,
        );
        agent.acmd(
            "expression_shizuespecialnfailure",
            expression_shizuespecialnfailure,
        );
        agent.acmd(
            "expression_shizuespecialairnfailure",
            expression_shizuespecialnfailure,
        );
    // SHULK
    // SIMON
    // SNAKE
    // SONIC
        agent.acmd("game_sonicspecialnhit", game_sonicspecialnhit);
        agent.acmd("effect_sonicspecialnhit", effect_sonicspecialnhit);
        agent.acmd("sound_sonicspecialnhit", sound_sonicspecialnhit);
    // SZEROSUIT
    // TANTAN
    // TOONLINK
    // TRAIL
    // WARIO
    // WIIFIT
    // WOLF
    // YOSHI
    // YOUNGLINK
    // ZELDA
}
