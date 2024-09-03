use super::*;

mod special_n;
mod special_hi;

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

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::krool::status::GUT_CHECK_CHARGED);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if WorkModule::get_float(boma, 0x4d) >= 1.0 {
                WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
            }
            HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
            HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
            HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        }
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        if VarModule::is_flag(agent.battle_object, vars::krool::status::GUT_CHECK_CHARGED) {
            let damage = VarModule::get_float(agent.battle_object, vars::krool::instance::STORED_DAMAGE) / 5.0;
            ATTACK(agent, 0, 0, Hash40::new("top"), 16.0 + damage, 50, 105, 0, 50, 10.5, 0.0, 10.75, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 69, 86, 0, 70, 9.5, 0.0, 10.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
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
        EFFECT(agent, Hash40::new("krool_counter_success"), Hash40::new("top"), 0, 3.25, 1.75, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
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
    special_n::install(agent);

    agent.acmd("game_specialsthrow", game_specials, Priority::Low);
    agent.acmd("game_specialairsthrow", game_specials, Priority::Low);

    special_hi::install(agent);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw, Priority::Low);
    agent.acmd("sound_speciallw", sound_speciallw, Priority::Low);
    agent.acmd("sound_specialairlw", sound_speciallw, Priority::Low);
    agent.acmd("expression_speciallw", expression_speciallw, Priority::Low);
    agent.acmd("expression_specialairlw", expression_speciallw, Priority::Low);
}