use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            VarModule::on_flag(agent.battle_object, vars::richter::instance::SPECIAL_N_LAND_CANCEL);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, false, -1);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ArticleModule::shoot(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::richter::instance::SPECIAL_N_LAND_CANCEL);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 5, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
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

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_richter_special_l01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_richter_rnd_special_s"));
    }
}

unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            app::sv_kinetic_energy::reset_energy(agent.lua_state_agent);
            boma.select_cliff_hangdata_from_name("special_s1");
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_NONE);
        }
        VarModule::on_flag(agent.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_FALL);
        VarModule::on_flag(agent.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_DRIFT);
        VarModule::on_flag(agent.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_CONTINUE);
        VarModule::on_flag(agent.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 60, 65, 0, 60, 4.0, 0.0, 8.5, 1.0, Some(0.0), Some(8.5), Some(8.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 60, 65, 0, 60, 4.0, 0.0, 12.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 60, 65, 0, 60, 4.0, 0.0, 5.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 60, 0, 55, 4.0, 0.0, 8.5, 1.0, Some(0.0), Some(8.5), Some(8.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 60, 60, 0, 55, 4.0, 0.0, 12.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 60, 60, 0, 55, 4.0, 0.0, 5.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
}

unsafe extern "C" fn effect_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 3, 8, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("richter_whip_dash"), Hash40::new("top"), -2.4, 9, 1, 0, 0, 0, 0.86, true);
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("richter_whip_dash"), true, true);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_richter_whip_holding"));
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_richter_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_richter_attackdash"));
    }
}

unsafe extern "C" fn expression_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 3.0);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handl"), 13.0, 71, 78, 0, 67, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        ATTACK(agent, 1, 0, Hash40::new("havel"), 13.0, 71, 78, 0, 67, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handl"), 7.0, 71, 90, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        ATTACK(agent, 1, 0, Hash40::new("havel"), 7.0, 71, 90, 0, 65, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("havel"), 7.0, 71, 90, 0, 65, 4.0, 5.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 71, 90, 0, 65, 2.5, 0.0, 17.0, 6.0, Some(0.0), Some(17.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
    EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FLASH(agent, 1.0, 1.0, 1.0, 0.1);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FLASH(agent, 1.0, 1.0, 1.0, 0.1);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_FLW_POS(agent, Hash40::new("richter_upper"), Hash40::new("havel"), -2, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("richter_upper"), true, true);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        let play_vc = app::sv_math::rand(hash40("fighter"), 3);
        if play_vc == 0 {
            PLAY_SE(agent, Hash40::new("vc_richter_special_h01"));
        }
        if play_vc == 1 {
            PLAY_SE(agent, Hash40::new("vc_richter_special_n01"));
        }
        if play_vc == 2 {
            PLAY_SE(agent, Hash40::new("vc_richter_attack07"));
        }
    }
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_richter_special_h01"));
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 14.0, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_GENERATE_HOLYWATER);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_SHOOT_HOLYWATER);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("richter_bottle_release"), Hash40::new("haver"), 0, 6, -2, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_richter_special_l01"));
        PLAY_SE(agent, Hash40::new("se_richter_special_l01"));
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrowitem"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn effect_speciallwblank(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("richter_bottle_blank"), Hash40::new("haver"), 0, 6, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_speciallwblank(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_richter_special_l01"));
        PLAY_SE(agent, Hash40::new("se_richter_special_l01"));
    }
}

unsafe extern "C" fn expression_speciallwblank(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrowitem"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn);
    agent.acmd("effect_specialn", effect_specialn);
    agent.acmd("sound_specialn", sound_specialn);
    agent.acmd("expression_specialn", expression_specialn);
    agent.acmd("game_specialairn", game_specialn);
    agent.acmd("effect_specialairn", effect_specialn);
    agent.acmd("sound_specialairn", sound_specialn);
    agent.acmd("expression_specialairn", expression_specialn);

    agent.acmd("game_specials1", game_specials1);
    agent.acmd("effect_specials1", effect_specials1);
    agent.acmd("sound_specials1", sound_specials1);
    agent.acmd("expression_specials1", expression_specials1);
    agent.acmd("game_specialairs1", game_specials1);
    agent.acmd("effect_specialairs1", effect_specials1);
    agent.acmd("sound_specialairs1", sound_specials1);
    agent.acmd("expression_specialairs1", expression_specials1);

    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("effect_specialhi", effect_specialhi);
    agent.acmd("sound_specialhi", sound_specialhi);
    agent.acmd("game_specialairhi", game_specialhi);
    agent.acmd("effect_specialairhi", effect_specialhi);
    agent.acmd("sound_specialairhi", sound_specialhi);

    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("effect_speciallw", effect_speciallw);
    agent.acmd("sound_speciallw", sound_speciallw);
    agent.acmd("expression_speciallw", expression_speciallw);
    agent.acmd("game_specialairlw", game_speciallw);
    agent.acmd("effect_specialairlw", effect_speciallw);
    agent.acmd("sound_specialairlw", sound_speciallw);
    agent.acmd("expression_specialairlw", expression_speciallw);

    agent.acmd("effect_speciallwblank", effect_speciallwblank);
    agent.acmd("sound_speciallwblank", sound_speciallwblank);
    agent.acmd("expression_speciallwblank", expression_speciallwblank);
    agent.acmd("effect_specialairlwblank", effect_speciallwblank);
    agent.acmd("sound_specialairlwblank", sound_speciallwblank);
    agent.acmd("expression_specialairlwblank", expression_speciallwblank);
}