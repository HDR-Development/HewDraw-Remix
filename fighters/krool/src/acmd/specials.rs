use super::*;
use std::convert::TryInto;

unsafe extern "C" fn game_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 25.0, 16.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if boma.is_button_on(Buttons::AttackRaw) || boma.is_button_on(Buttons::Guard) {
            VarModule::on_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB);
        }
    }
    frame(lua_state, 26.0);
    if VarModule::is_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
        FT_MOTION_RATE_RANGE(agent, 26.0, 37.0, 1.0);
    }
    else {
        FT_MOTION_RATE_RANGE(agent, 26.0, 30.0, 5.0);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
            WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL);
            WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL);
        }
    }
    frame(lua_state, 37.0);
    FT_MOTION_RATE_RANGE(agent, 37.0, 70.0, 30.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL);
    }
    frame(lua_state, 70.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL);
    }
}

unsafe extern "C" fn effect_specialnfire(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn sound_specialnfire(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn expression_specialnfire(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn game_specialnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
            CATCH(agent, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(18.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 9.5, 10.7, Some(0.0), Some(9.5), Some(20.5), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_A);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 160, 100, 50, 0, 9.0, 0.0, 9.0, 22.0, Some(0.0), Some(9.0), Some(27.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        else {
            SEARCH(agent, 0, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(60.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
        }
    }
}

unsafe extern "C" fn effect_specialnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB) {
            EFFECT_FOLLOW(agent, Hash40::new("krool_cannon_vacuum"), Hash40::new("top"), 0, 10, 17, 0, 0, 0, 0.8, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("krool_cannon_vacuum"), Hash40::new("top"), 0, 10, 17, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_SCALE_W(agent, 0.6, 2.0, 1.0);
            LAST_EFFECT_SET_ALPHA(agent, 0.75);
        }
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.6, 10, 0, 4, 0, 0, 0, false);
        }
    }
    wait(lua_state, 10.0);
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 180);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 0.0, 361, 100, 30, 0, 4.0, 5.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
    ArticleModule::generate_article(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_CROWN, false, 0);
    VisibilityModule::set_int64(boma, Hash40::new("crown").hash as i64, Hash40::new("crown_hide").hash as i64);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
    AttackModule::clear_all(boma);
    }   
    frame(lua_state, 64.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    for _ in 0..50 {
        wait(lua_state, 12.0);
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 12.0);
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_h02"));
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK) {
            if agent.is_motion(Hash40::new("special_air_hi")) {
                ArticleModule::change_motion(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, Hash40::new("fly_air"), false, 0.0);
            } else {
                ArticleModule::change_motion(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, Hash40::new("fly"), false, 0.0);
            }
        }
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_krool_special_h02"));
        PLAY_STATUS(agent, Hash40::new("se_krool_special_h01"));
        PLAY_SE(agent, Hash40::new("se_common_swing_08"));
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_jet"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        let charge = (VarModule::get_int(boma.object(), vars::krool::instance::SPECIAL_HI_FUEL));
        if charge >= 35 {
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        } else {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::krool::status::GUT_CHECK_CHARGED);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if WorkModule::get_float(agent.module_accessor, 0x4d) >= 1.0
        && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
        }
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        if VarModule::is_flag(agent.battle_object, vars::krool::status::GUT_CHECK_CHARGED) {
            let damage = VarModule::get_float(agent.battle_object, vars::krool::instance::STORED_DAMAGE) / 5.0;
            ATTACK(agent, 0, 0, Hash40::new("top"), 16.0 + damage, 50, 90, 0, 50, 10.5, 0.0, 10.75, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 69, 90, 0, 70, 9.0, 0.0, 10.0, 4.25, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::krool::status::GUT_CHECK_CHARGED) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::krool::status::GUT_CHECK_CHARGED) {
            AttackModule::clear_all(boma);
        }
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("krool_counter_success_body"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("krool_counter_success_body_l"), true, true);
        EFFECT(agent, Hash40::new("krool_counter_success"), Hash40::new("top"), 0, 3.25, 0.75, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        let charged = VarModule::is_flag(agent.battle_object, vars::krool::status::GUT_CHECK_CHARGED);
        if charged {
            EFFECT_FOLLOW(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 12, 6, 0, 0, 0, 0.9, false);
        }
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 0.7);
        }
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("krool_counter_attack_body_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("krool_counter_attack"), Hash40::new("top"), 6, 12, 15, -90, 30, 0, 0.4, false);
            if charged {
                EFFECT_FOLLOW(agent, Hash40::new("krool_counter_attack2"), Hash40::new("top"), 6, 12, 4, 0, 25, 0, 1, false);
            }
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("krool_counter_attack_body"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("krool_counter_attack"), Hash40::new("top"), -6, 12, 15, -90, -30, 0, 0.4, false);
            if charged {
                EFFECT_FOLLOW(agent, Hash40::new("krool_counter_attack2"), Hash40::new("top"), -6, 12, 4, 0, -25, 0, 1, false);
            }
        }
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("krool_counter_attack2"), false, true);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_l01"));
        PLAY_SE(agent, Hash40::new("vc_krool_special_l01"));
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_smash_h01"));
        if VarModule::is_flag(agent.battle_object, vars::krool::status::GUT_CHECK_CHARGED) {
            PLAY_SE(agent, Hash40::new("se_krool_special_l02"));
            PLAY_SE(agent, Hash40::new("se_krool_special_l05"));
        }
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_l03"));
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
        ControlModule::set_rumble(boma, Hash40::new("rbkind_counter"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnfire", game_specialnfire);
    agent.acmd("game_specialairnfire", game_specialnfire);
    agent.acmd("effect_specialnfire", effect_specialnfire);
    agent.acmd("effect_specialairnfire", effect_specialnfire);
    agent.acmd("sound_specialnfire", sound_specialnfire);
    agent.acmd("sound_specialairnfire", sound_specialnfire);
    agent.acmd("expression_specialnfire", expression_specialnfire);
    agent.acmd("expression_specialairnfire", expression_specialnfire);
    agent.acmd("game_specialnloop", game_specialnloop);
    agent.acmd("game_specialairnloop", game_specialnloop);
    agent.acmd("effect_specialnloop", effect_specialnloop);
    agent.acmd("effect_specialairnloop", effect_specialnloop);

    agent.acmd("game_specialsthrow", game_specials);
    agent.acmd("game_specialairsthrow", game_specials);

    agent.acmd("effect_specialhistart", effect_specialhistart);
    agent.acmd("sound_specialhistart", sound_specialhistart);
    agent.acmd("sound_specialairhistart", sound_specialhistart);
    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("effect_specialhi", effect_specialhi);
    agent.acmd("sound_specialhi", sound_specialhi);
    agent.acmd("expression_specialhi", expression_specialhi);
    
    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("game_specialairlw", game_speciallw);
    agent.acmd("effect_speciallw", effect_speciallw);
    agent.acmd("effect_specialairlw", effect_speciallw);
    agent.acmd("sound_speciallw", sound_speciallw);
    agent.acmd("sound_specialairlw", sound_speciallw);
    agent.acmd("expression_speciallw", expression_speciallw);
    agent.acmd("expression_specialairlw", expression_speciallw);
}