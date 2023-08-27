use super::*;

#[acmd_script( agent = "kirby", script = "game_ganonfloatstart" , category = ACMD_GAME , low_priority)]
unsafe fn ganon_float_start_game(fighter: &mut L2CAgentBase) {
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
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::ganon::status::FLOAT_GROUND_CHANGE_KINETIC);
        VarModule::on_flag(fighter.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "kirby", script = "effect_ganonfloatstart", category = ACMD_EFFECT , low_priority)]
unsafe fn ganon_float_start_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "expression_ganonfloatstart", category = ACMD_EXPRESSION , low_priority)]
unsafe fn ganon_float_start_expression(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "sound_ganonfloatstart", category = ACMD_SOUND , low_priority)]
unsafe fn ganon_float_start_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "game_ganonfloatairstart" , category = ACMD_GAME , low_priority)]
unsafe fn ganon_float_air_start_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "effect_ganonfloatairstart", category = ACMD_EFFECT , low_priority)]
unsafe fn ganon_float_air_start_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "expression_ganonfloatairstart", category = ACMD_EXPRESSION , low_priority)]
unsafe fn ganon_float_air_start_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "kirby", script = "sound_ganonfloatairstart", category = ACMD_SOUND , low_priority)]
unsafe fn ganon_float_air_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ganon_appear01"));
    }
}

#[acmd_script( agent = "kirby", script = "game_ganonfloat" , category = ACMD_GAME , low_priority)]
unsafe fn ganon_float_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "effect_ganonfloat", category = ACMD_EFFECT , low_priority)]
unsafe fn ganon_float_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "expression_ganonfloat", category = ACMD_EXPRESSION , low_priority)]
unsafe fn ganon_float_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "kirby", scripts = ["effect_koopaspecialnstart","effect_koopaspecialairnstart"], category = ACMD_EFFECT, low_priority )]
unsafe fn koopa_special_n_start_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0.961, 0.569, 0.569, 0.392);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 1, 0.537, 0.537, 0.588);
        macros::FLASH_FRM(fighter, 20, 0, 0, 0, 0);
    }
    wait(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::COL_NORMAL(fighter);
    }
}

#[acmd_script( agent = "kirby", script = "sound_koopaspecialnstart", category = ACMD_SOUND, low_priority )]
unsafe fn koopa_special_n_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n01"));
    }
    wait(lua_state, 19.0);
    if is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_kirby_copy_koopa_01"));
    }
}

#[acmd_script( agent = "kirby", scripts = ["game_koopaspecialnend","game_koopaspecialairnend"], category = ACMD_EFFECT, low_priority )]
unsafe fn koopa_special_n_end_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(fighter,1.0,31.0,16.0);
}


#[acmd_script( agent = "kirby", scripts = ["game_koopaspecialnmax","game_koopaspecialairnmax"], category = ACMD_GAME, low_priority )]
unsafe fn koopa_special_n_max_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,KOOPA_MAX_COOLDOWN);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_START);
    }
}

#[acmd_script( agent = "kirby", scripts = ["effect_koopaspecialnmax","effect_koopaspecialairnmax"], category = ACMD_EFFECT, low_priority )]
unsafe fn koopa_special_n_max_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_explosion_sign"), Hash40::new("jaw"), 0, 1.0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(fighter,1.5);

        if fighter.is_motion(Hash40::new("koopa_special_n_max")){
            macros::LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        macros::FLASH(fighter, 0.961, 0.569, 0.569, 0.392);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 1, 0.537, 0.537, 0.588);
        macros::FLASH_FRM(fighter, 20, 0, 0, 0, 0);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_explosion_sign"),false,false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("koopa_breath_m_fire"),false,false);

        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire_fly"), Hash40::new("jaw"), 0, 0, 0, 180, 0, 50, 0.5, true);
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        macros::COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("koopa_appeal_s"), Hash40::new("mouth2"), 0, -1.3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter,2.0,0.5,0);
    }
}

#[acmd_script( agent = "kirby", scripts = ["sound_koopaspecialnmax","sound_koopaspecialairnmax"], category = ACMD_SOUND, low_priority )]
unsafe fn koopa_special_n_max_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n01"));
    }
    wait(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        if fighter.is_motion(Hash40::new("koopa_special_n_max")){
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_koopa_step_left_m"));
        }
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_fire_m_damage"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_kirby_attack05"));
    }
}

#[acmd_script( agent = "kirby", scripts = ["expression_koopaspecialnmax","expression_koopaspecialairnmax"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn koopa_special_n_max_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "kirby", scripts = ["effect_kroolspecialnfire", "effect_kroolspecialairnfire"], category = ACMD_EFFECT, low_priority )]
unsafe fn krool_special_n_fire_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", scripts = ["sound_kroolspecialnfire", "sound_kroolspecialairnfire"], category = ACMD_SOUND, low_priority )]
unsafe fn krool_special_n_fire_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", scripts = ["expression_kroolspecialnfire", "expression_kroolspecialairnfire"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn krool_special_n_fire_expression(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", scripts = ["effect_kroolspecialnloop", "effect_kroolspecialairnloop"], category = ACMD_EFFECT, low_priority )]
unsafe fn krool_special_n_loop_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "game_littlemacspecialnstart" , category = ACMD_GAME , low_priority)]
unsafe fn littlemac_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_CHECK_DASH);
    }
}

#[acmd_script( agent = "kirby", script = "game_littlemacspecialairnstart" , category = ACMD_GAME , low_priority)]
unsafe fn littlemac_special_air_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15.0);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_CHECK_DASH);
    }
}

#[acmd_script( agent = "kirby", script = "game_littlemacspecialncancel" , category = ACMD_GAME , low_priority)]
unsafe fn littlemac_special_n_cancel_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 8.0/(39.0 - 1.0));
    }
}

#[acmd_script( agent = "kirby", script = "effect_littlemacspecialncancel" , category = ACMD_EFFECT , low_priority)]
unsafe fn littlemac_special_n_cancel_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "kirby", script = "sound_littlemacspecialncancel" , category = ACMD_SOUND , low_priority)]
unsafe fn littlemac_special_n_cancel_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("vc_kirby_copy_littlemac_01"));
    }
}

#[acmd_script( agent = "kirby", script = "expression_littlemacspecialncancel" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn littlemac_special_n_cancel_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "kirby", script = "game_littlemacspecialairncancel" , category = ACMD_GAME , low_priority)]
unsafe fn littlemac_special_air_n_cancel_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 8.0/(39.0 - 1.0));
    }
}

#[acmd_script( agent = "kirby", script = "effect_littlemacspecialairncancel" , category = ACMD_EFFECT , low_priority)]
unsafe fn littlemac_special_air_n_cancel_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "kirby", script = "sound_littlemacspecialairncancel" , category = ACMD_SOUND , low_priority)]
unsafe fn littlemac_special_air_n_cancel_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("vc_kirby_copy_littlemac_01"));
    }
}

#[acmd_script( agent = "kirby", script = "expression_littlemacspecialairncancel" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn littlemac_special_air_n_cancel_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "kirby", script = "game_littlemacspecialn2" , category = ACMD_GAME , low_priority)]
unsafe fn littlemac_special_n2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::set_damage_shake_scale(boma, 0.67);
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 25.0, 80, 103, 0, 25, 5.0, 3.0, 0.0, 0.0, None, None, None, 1.5, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 25.0, 80, 103, 0, 25, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.5, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 25.0, 80, 103, 0, 25, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("bust"), 25.0, 80, 103, 0, 25, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
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

#[acmd_script( agent = "kirby", script = "sound_littlemacspecialn2" , category = ACMD_SOUND , low_priority)]
unsafe fn littlemac_special_n2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_littlemac_rnd_special_n2"));
        PLAY_SE(fighter, Hash40::new("se_littlemac_special_n03"));
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

#[acmd_script( agent = "kirby", script = "game_littlemacspecialairn2" , category = ACMD_GAME , low_priority)]
unsafe fn littlemac_special_air_n2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::set_damage_shake_scale(boma, 0.67);
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 20.0, 75, 125, 0, 25, 5.0, 3.0, 0.0, 0.0, None, None, None, 1.5, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 20.0, 75, 125, 0, 25, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.5, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 20.0, 75, 125, 0, 25, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("bust"), 20.0, 75, 125, 0, 25, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 11.0);
    SA_SET(fighter, *SITUATION_KIND_AIR);
    wait(lua_state, 3.0);
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

#[acmd_script( agent = "kirby", script = "sound_littlemacspecialairn2" , category = ACMD_SOUND , low_priority)]
unsafe fn littlemac_special_air_n2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_littlemac_rnd_special_n2"));
        PLAY_SE(fighter, Hash40::new("se_littlemac_special_n03"));
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

// SPECIAL N START //

#[acmd_script ( agent = "kirby", scripts = ["game_lucasspecialnstart", "game_lucasspecialairnstart"], category = ACMD_GAME, low_priority)]
unsafe fn lucas_special_n_start_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script (agent = "kirby", scripts = ["sound_lucasspecialnstart", "sound_lucasspecialairnstart"], category = ACMD_SOUND, low_priority)]
unsafe fn lucas_special_n_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_smash_l03"));
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_kirby_010"));
    }

}

// SPECIAL N HOLD //

#[acmd_script ( agent = "kirby", scripts = ["game_lucasspecialnhold", "game_lucasspecialairnhold"], category = ACMD_GAME, low_priority)]
unsafe fn lucas_special_n_hold_game(fighter: &mut L2CAgentBase) {
    // INTENTIONALLY LEFT BLANK
    /* if fighter.kind() == *FIGHTER_KIND_KIRBY {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    } */
}

#[acmd_script (agent = "kirby", scripts = ["effect_lucasspecialnhold", "effect_lucasspecialairnhold"], category = ACMD_EFFECT, low_priority)]
unsafe fn lucas_special_n_hold_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script (agent = "kirby", scripts = ["sound_lucasspecialairnhold", "sound_lucasspecialnhold"], category = ACMD_SOUND, low_priority)]
unsafe fn lucas_special_n_hold_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_special_h02"));
        PLAY_STATUS(fighter, Hash40::new("se_lucas_pk_charge"));
    }
}

// SPECIAL N FIRE //

#[acmd_script ( agent = "kirby", scripts = ["game_lucasspecialnfire", "game_lucasspecialairnfire"], category = ACMD_GAME, low_priority)]
unsafe fn lucas_special_n_fire_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script ( agent = "kirby", scripts = ["effect_lucasspecialnfire", "effect_lucasspecialairnfire"], category = ACMD_EFFECT, low_priority)]
unsafe fn lucas_special_n_fire_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script (agent = "kirby", scripts = ["sound_lucasspecialairnfire", "sound_lucasspecialnfire"], category = ACMD_SOUND, low_priority)]
unsafe fn lucas_special_n_fire_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_kirby_attack04"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_special_n04_l"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_electric_hit_m"));
    }
}

#[acmd_script( agent = "kirby", script = "game_luigispecialn" , category = ACMD_GAME , low_priority)]
unsafe fn luigi_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
        }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            VarModule::on_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 9.0, 69, 55, 0, 65, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 9.0, 69, 55, 0, 65, 4.5, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 9.0, 69, 55, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            //HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            //HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
            //HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 2.0);
        }
        else{
            ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            FT_MOTION_RATE(fighter, 0.5);
        }
    }
}

#[acmd_script( agent = "kirby", script = "effect_luigispecialn" , category = ACMD_EFFECT , low_priority)]
unsafe fn luigi_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("havel"), 3.0, 0.0, 0.0, 0, 90, 90, 0.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 90, 90, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 1.25);
            FLASH(fighter, 0, 0.25, 1.0, 0.7);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("luigi_fb_shoot"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
            FLASH(fighter, 0, 1, 0, 0.353);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_thunder"), false, false);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), true, true);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("luigi_fb_shoot"), false, false);
    }
}

#[acmd_script( agent = "kirby", script = "sound_luigispecialn" , category = ACMD_SOUND , low_priority)]
unsafe fn luigi_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            PLAY_SE(fighter, Hash40::new("se_common_elec_s_damage"));
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            PLAY_SE(fighter, Hash40::new("vc_kirby_attack02"));
            PLAY_SE(fighter, Hash40::new("se_common_elec_m_damage"));
        }
        else{
            PLAY_SE(fighter, Hash40::new("se_luigi_special_n01"));
        }
    }
}

#[acmd_script( agent = "kirby", script = "game_luigispecialairn" , category = ACMD_GAME , low_priority)]
unsafe fn luigi_special_air_n_game(fighter: &mut L2CAgentBase)  {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
        }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            VarModule::on_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 9.0, 69, 55, 0, 65, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 9.0, 69, 55, 0, 65, 4.5, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 9.0, 69, 55, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            //HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            //HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
            //HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 2.0);
        }
        else{
            ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            FT_MOTION_RATE(fighter, 0.5);
        }
    }
}

#[acmd_script( agent = "kirby", script = "effect_luigispecialairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn luigi_special_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("havel"), 3.0, 0.0, 0.0, 0, 90, 90, 0.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 90, 90, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 1.25);
            FLASH(fighter, 0, 0.25, 1.0, 0.7);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("luigi_fb_shoot"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
            FLASH(fighter, 0, 1, 0, 0.353);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_thunder"), false, false);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), true, true);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("luigi_fb_shoot"), false, false);
    }
}

#[acmd_script( agent = "kirby", script = "sound_luigispecialairn" , category = ACMD_SOUND , low_priority)]
unsafe fn luigi_special_air_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            PLAY_SE(fighter, Hash40::new("se_common_elec_s_damage"));
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND){
            PLAY_SE(fighter, Hash40::new("vc_kirby_attack02"));
            PLAY_SE(fighter, Hash40::new("se_common_elec_m_damage"));
        }
        else{
            PLAY_SE(fighter, Hash40::new("se_luigi_special_n01"));
        }
    }
}

#[acmd_script( agent = "kirby", script = "game_mariospecialn" , category = ACMD_GAME , low_priority)]
unsafe fn mario_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 16.0/(14.0-1.0));
        VarModule::off_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            VarModule::on_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
            FT_MOTION_RATE(fighter, 3.0/(14.0-12.0));
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 10.0, 45, 110, 0, 50, 3.0, 0.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 50, 115, 0, 50, 5.5, 5.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            FT_MOTION_RATE(fighter, 1.0);
        }
        else {
            ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 8.0, 40, 100, 0, 50, 3.0, 0.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 40, 100, 0, 50, 5.5, 5.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            AttackModule::clear_all(boma);
            FT_MOTION_RATE(fighter, 32.0/(49.0 - 21.0));
        }
        else {
            FT_MOTION_RATE(fighter, 23.0/(49.0 - 21.0));
        }
    }
}

#[acmd_script( agent = "kirby", script = "effect_mariospecialn" , category = ACMD_EFFECT , low_priority)]
unsafe fn mario_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 0.2);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 1.0, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flame"), Hash40::new("handl"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_bomb_a"), Hash40::new("havel"), 1.0, 0, 0, 0, 0, 0, 0.23, true);
            LAST_EFFECT_SET_RATE(fighter, 1.2);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.35);
        }
        else{
            FLASH(fighter, 1, 0, 0, 0.353);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.75);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_bomb_a"), false, false);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.35);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.35);
        }
        else{
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.75);
        }
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.35);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_flame"), false, false);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.35);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            COL_NORMAL(fighter);
        }
    }
}

#[acmd_script( agent = "kirby", script = "sound_mariospecialn" , category = ACMD_SOUND , low_priority)]
unsafe fn mario_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            PLAY_SE(fighter, Hash40::new("se_mario_smash_s01"));
            PLAY_SE(fighter, Hash40::new("vc_kirby_attack02"));
        }
    }
}

#[acmd_script( agent = "kirby", script = "game_mariospecialairn" , category = ACMD_GAME , low_priority)]
unsafe fn mario_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 16.0/(14.0-1.0));
        VarModule::off_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){
            VarModule::on_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND);
            FT_MOTION_RATE(fighter, 3.0/(14.0-12.0));
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 10.0, 45, 110, 0, 50, 3.0, 0.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 50, 115, 0, 50, 5.5, 5.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            FT_MOTION_RATE(fighter, 1.0);
        }
        else {
            ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 8.0, 40, 100, 0, 50, 3.0, 0.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 40, 100, 0, 50, 5.5, 5.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            AttackModule::clear_all(boma);
            FT_MOTION_RATE(fighter, 32.0/(49.0 - 21.0));
        }
        else {
            FT_MOTION_RATE(fighter, 23.0/(49.0 - 21.0));
        }
    }
}

#[acmd_script( agent = "kirby", script = "effect_mariospecialairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn mario_special_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 0.2);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 1.0, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flame"), Hash40::new("havel"), 1.0, 0, 0, 0, 0, 0, 0.3, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_bomb_a"), Hash40::new("handl"), 1.0, 0, 0, 0, 0, 0, 0.23, true);
            LAST_EFFECT_SET_RATE(fighter, 1.2);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.5);
        }
        else{
            FLASH(fighter, 1, 0, 0, 0.35);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 1.0);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_bomb_a"), false, false);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.5);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.5);
        }
        else{
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 0.95, 0.522, 0.051, 1.0);
        }
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 0.95, 0.522, 0.051, 0.5);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_flame"), false, false);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            FLASH(fighter, 1, 0, 0, 0.5);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            COL_NORMAL(fighter);
        }
    }
}

#[acmd_script( agent = "kirby", script = "sound_mariospecialairn" , category = ACMD_SOUND , low_priority)]
unsafe fn mario_special_air_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
        if VarModule::is_flag(fighter.battle_object, vars::kirby::status::IS_SPECIAL_N_FIREBRAND) {
            PLAY_SE(fighter, Hash40::new("se_mario_smash_s01"));
            PLAY_SE(fighter, Hash40::new("vc_kirby_attack02"));
        }
    }
}

#[acmd_script( agent = "kirby", scripts = ["game_mariodspecialnchill", "game_mariodspecialairnchill"] , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_n_chill_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(fighter, 10.0, 14.0, 5.0);
    frame(lua_state, 14.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 10.0, 69, 90, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 69, 90, 0, 50, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 10.0, 69, 90, 0, 50, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE_RANGE(fighter, 18.0, 43.0, 37.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 43.0);
    FT_MOTION_RATE(fighter, 1.0);
}


#[acmd_script( agent = "kirby", scripts = ["effect_mariodspecialnchill", "effect_mariodspecialairnchill"] , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_special_n_chill_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_ice"), Hash40::new("arml"), 7.5, 0, 0, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_ice_landing"), Hash40::new("arml"), 7.5, 0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FLIP(fighter, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.46, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.5, 0.25, 1, 0.35);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.5, 0.25, 1, 0.75);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.5, 0.25, 1, 0.35);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.75, 0.75, 1.0, 0.75);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_ice"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice_landing"), false, true);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

#[acmd_script( agent = "kirby", scripts = ["sound_mariodspecialnchill", "sound_mariodspecialairnchill"], category = ACMD_SOUND, low_priority )]
unsafe fn mariod_special_n_chill_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_frieze_l"));
        PLAY_SE(fighter, Hash40::new("vc_kirby_attack03"));
    }
}

#[acmd_script( agent = "kirby", scripts = ["expression_mariodspecialnchill", "expression_mariodspecialairnchill"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn mariod_special_n_chill_expression(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", scripts = ["game_mariodspecialn", "game_mariodspecialairn"] , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_n_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", scripts = ["effect_mariodspecialn", "effect_mariodspecialairn"] , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_special_n_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", scripts = ["sound_mariodspecialn", "sound_mariodspecialairn"], category = ACMD_SOUND, low_priority )]
unsafe fn mariod_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mariod_special_n01"));
    }
}

#[acmd_script( agent = "kirby", scripts = ["effect_palutenaspecialn", "effect_palutenaspecialairn"], category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_special_n_effect(agent: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", scripts = ["sound_palutenaspecialn", "sound_palutenaspecialairn"], category = ACMD_SOUND, low_priority )]
unsafe fn palutena_special_n_sound(agent: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", scripts = ["effect_miigunnerspecialn1firemax", "effect_miigunnerspecialairn1firemax"] , category = ACMD_EFFECT , low_priority)]
unsafe fn miigunner_special_n1_fire_max_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", scripts = ["sound_miigunnerspecialn1firemax", "sound_miigunnerspecialairn1firemax"] , category = ACMD_SOUND , low_priority)]
unsafe fn miigunner_special_n1_fire_max_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "game_ridleyspecialnexplode", category = ACMD_GAME )]
unsafe fn ridley_special_n_explode_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "effect_ridleyspecialnexplode", category = ACMD_EFFECT )]
unsafe fn ridley_special_n_explode_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "sound_ridleyspecialnexplode", category = ACMD_SOUND )]
unsafe fn ridley_special_n_explode_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "expression_ridleyspecialnexplode", category = ACMD_EXPRESSION )]
unsafe fn ridley_special_n_explode_expression(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "game_ridleyspecialairnexplode", category = ACMD_GAME )]
unsafe fn ridley_special_air_n_explode_game(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "effect_ridleyspecialairnexplode", category = ACMD_EFFECT )]
unsafe fn ridley_special_air_n_explode_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "sound_ridleyspecialairnexplode", category = ACMD_SOUND )]
unsafe fn ridley_special_air_n_explode_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "expression_ridleyspecialairnexplode", category = ACMD_EXPRESSION )]
unsafe fn ridley_special_air_n_explode_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}

#[acmd_script( agent = "kirby", scripts = ["effect_royspecialnend", "effect_royspecialairnend"] , category = ACMD_EFFECT , low_priority)]
unsafe fn roy_special_n_end_effect(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "roy", scripts = ["sound_royspecialnend", "sound_royspecialairnend"] , category = ACMD_SOUND , low_priority)]
unsafe fn roy_special_n_end_sound(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", scripts = ["effect_shizuespecialnfailure", "effect_shizuespecialairnfailure"] , category = ACMD_EFFECT , low_priority)]
unsafe fn shizue_special_n_failure_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("shizue_cracker"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "kirby", scripts = ["expression_shizuespecialnfailure", "expression_shizuespecialairnfailure"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn shizue_special_n_failure_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 0);
    }
}

#[acmd_script( agent = "kirby", script = "game_sonicspecialnhit" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_special_n_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        let temp = Vector3f { x: -0.3, y: 1.0, z: 0.0 };
		KineticModule::add_speed(boma, &temp);
    }
    FT_MOTION_RATE(fighter, 0.5);

}

#[acmd_script( agent = "kirby", script = "effect_sonicspecialnhit" , category = ACMD_EFFECT , low_priority)]
unsafe fn sonic_special_n_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

#[acmd_script( agent = "kirby", script = "sound_sonicspecialnhit" , category = ACMD_SOUND , low_priority)]
unsafe fn sonic_special_n_hit_sound(fighter: &mut L2CAgentBase) {
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
    install_acmd_scripts!(
        ganon_float_start_game,
        ganon_float_start_effect,
        ganon_float_start_expression,
        ganon_float_start_sound,
        ganon_float_air_start_game,
        ganon_float_air_start_effect,
        ganon_float_air_start_expression,
        ganon_float_air_start_sound,
        ganon_float_game,
        ganon_float_effect,
        ganon_float_expression,
        koopa_special_n_start_effect,
        koopa_special_n_start_sound,
        koopa_special_n_end_game,
        koopa_special_n_max_game,
        koopa_special_n_max_effect,
        koopa_special_n_max_sound,
        koopa_special_n_max_expression,
        krool_special_n_fire_effect,
        krool_special_n_fire_sound,
        krool_special_n_fire_expression,
        krool_special_n_loop_effect,
        littlemac_special_n_start_game,
        littlemac_special_air_n_start_game,
        littlemac_special_n_cancel_game,
        littlemac_special_n_cancel_effect,
        littlemac_special_n_cancel_sound,
        littlemac_special_n_cancel_expression,
        littlemac_special_air_n_cancel_game,
        littlemac_special_air_n_cancel_effect,
        littlemac_special_air_n_cancel_sound,
        littlemac_special_air_n_cancel_expression,
        littlemac_special_n2_game,
        littlemac_special_n2_sound,
        littlemac_special_air_n2_game,
        littlemac_special_air_n2_sound,
        lucas_special_n_start_game,
        lucas_special_n_hold_game,
        lucas_special_n_hold_sound,
        lucas_special_n_fire_sound,
        lucas_special_n_start_sound,
        lucas_special_n_hold_effect,
        lucas_special_n_fire_game,
        lucas_special_n_fire_effect,
        luigi_special_n_game,
        luigi_special_n_effect,
        luigi_special_n_sound,
        luigi_special_air_n_game,
        luigi_special_air_n_effect,
        luigi_special_air_n_sound,
        mario_special_n_game,
        mario_special_n_effect,
        mario_special_n_sound,
        mario_special_air_n_game,
        mario_special_air_n_effect,
        mario_special_air_n_sound,
        mariod_special_n_game,
        mariod_special_n_effect,
        mariod_special_n_sound,
        mariod_special_n_chill_game,
        mariod_special_n_chill_effect,
        mariod_special_n_chill_sound,
        mariod_special_n_chill_expression,
        palutena_special_n_effect,
        palutena_special_n_sound,
		miigunner_special_n1_fire_max_effect,
		miigunner_special_n1_fire_max_sound,
        ridley_special_n_explode_game,
        ridley_special_n_explode_effect,
        ridley_special_n_explode_sound,
        ridley_special_n_explode_expression,
        ridley_special_air_n_explode_game,
        ridley_special_air_n_explode_effect,
        ridley_special_air_n_explode_sound,
        ridley_special_air_n_explode_expression,
        roy_special_n_end_effect,
        roy_special_n_end_sound,
        shizue_special_n_failure_effect,
        shizue_special_n_failure_expression,
        sonic_special_n_hit_game,
        sonic_special_n_hit_effect,
        sonic_special_n_hit_sound,
    );
}

