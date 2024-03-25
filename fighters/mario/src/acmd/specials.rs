use smash::app::sv_animcmd::QUAKE;

use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_special_n01"));
    }
}

unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 11.0, 7.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f::new(-0.5, 0.0, 0.0));
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 101, 0, 52, 3.0, 0.0, 6.5, 4.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 50, 101, 0, 52, 5.0, 0.0, 7.5, 10.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 40, 100, 0, 50, 3.0, 0.0, 6.5, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 40, 100, 0, 50, 5.0, 0.0, 7.5, 10.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.5);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 6, 11, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 0.55, true);
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0, 0, 0, 0, 45, 0, 0.55, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 0.55, true);
            EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0, 0, 0, 0, -45, 0, 0.55, true);
        }
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0, 0.35);
        EFFECT_FOLLOW(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 7.5, 10.5, 0, 0, 0, 0.26, true);
        LAST_EFFECT_SET_COLOR(agent, 0.65, 0.2, 0.08);
        LAST_EFFECT_SET_RATE(agent, 0.75);
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 6.0, 10.5, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 0.55);
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 5.0, 10.5, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 0.55);
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

unsafe extern "C" fn sound_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_special_n01"));
        PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
        PLAY_SE(agent, Hash40::new("vc_mario_attack07"));
    }
}

unsafe extern "C" fn expression_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_MANTLE, false, 0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    /*frame(lua_state, 6.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }*/
    frame(lua_state, 6.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        //search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 110, 100, 80, 0, 7.5, 0.0, 6.7, 9.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 110, 100, 80, 0, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::off_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}

unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_MANTLE, false, 0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    /*frame(lua_state, 6.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }*/
    frame(lua_state, 6.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        //search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::mario::instance::SPECIAL_S_DISABLE_STALL) {
            WorkModule::on_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
            VarModule::on_flag(agent.battle_object, vars::mario::instance::SPECIAL_S_DISABLE_STALL);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 110, 100, 80, 0, 7.5, 0.0, 6.7, 9.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 110, 100, 80, 0, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::off_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if (WorkModule::is_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY)) {
        if is_excute(agent) {
            boma.select_cliff_hangdata_from_name("special_hi");
        }
        frame(lua_state, 3.0);
        if is_excute(agent) {
            SA_SET(agent, *SITUATION_KIND_AIR);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 60, 100, 160, 0, 2.5, 0.0, 4.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 86, 100, 150, 0, 4.0, 0.0, 4.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 100, 100, 150, 0, 4.0, 0.0, 4.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        wait(lua_state, 3.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            SA_SET(agent, *SITUATION_KIND_AIR);
            AttackModule::clear_all(boma);
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 4.5, 2.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 4.5, 7.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 8.0, 2.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.8, 0.0, 8.0, 8.0, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
            AttackModule::set_no_finish_camera(boma, 1, true, false);
            AttackModule::set_no_finish_camera(boma, 2, true, false);
            AttackModule::set_no_finish_camera(boma, 3, true, false);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        frame(lua_state, 13.0);
        if is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 17.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 125, 0, 50, 6.5, 0.0, 9.5, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 60, 125, 0, 50, 6.5, 0.0, 9.5, 2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 23.0);
        if is_excute(agent) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
        }
    }
    else {
        if is_excute(agent) {
            boma.select_cliff_hangdata_from_name("special_hi");
        }
        frame(lua_state, 3.0);
        if is_excute(agent) {
            SA_SET(agent, *SITUATION_KIND_AIR);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 60, 100, 160, 0, 2.5, 0.0, 4.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 86, 100, 150, 0, 4.0, 0.0, 4.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 100, 100, 150, 0, 4.0, 0.0, 4.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        wait(lua_state, 3.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            SA_SET(agent, *SITUATION_KIND_AIR);
            AttackModule::clear_all(boma);
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 4.5, 2.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 4.5, 7.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 8.0, 2.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.8, 0.0, 8.0, 8.0, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
            AttackModule::set_no_finish_camera(boma, 1, true, false);
            AttackModule::set_no_finish_camera(boma, 2, true, false);
            AttackModule::set_no_finish_camera(boma, 3, true, false);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        frame(lua_state, 13.0);
        if is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 17.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 125, 0, 50, 6.5, 0.0, 9.5, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_COIN_LAST, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 60, 125, 0, 50, 6.5, 0.0, 9.5, 2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_COIN_LAST, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 23.0);
        if is_excute(agent) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
        }
    }

}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mario_superjump_power"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, -0.4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mario_superjump_fnish"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_superjump_power"), false, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mario_superjump_fnish"), -1);
    }
}

unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mario_superjump_power"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, -0.4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mario_superjump_fnish"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_superjump_power"), false, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mario_superjump_fnish"), -1);
    }
}

unsafe extern "C" fn game_speciallwlight(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE_RANGE(agent, 0.0, 10.0, 5.0);
    if is_excute(agent){
        ArticleModule::remove_exist(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 40.0, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 45, 0, 88, 3.2, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(-6.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 70, 45, 0, 88, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(5.0), Some(-0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(agent, 40.0, 45.0, 7.0);
    if is_excute(agent){
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 45.0);
    FT_MOTION_RATE_RANGE(agent, 45.0, 48.0, 3.0);
    frame(lua_state, 48.0);
    FT_MOTION_RATE_RANGE(agent, 48.0, 52.0, 7.0); 
    frame(lua_state, 52.0);
    FT_MOTION_RATE_RANGE(agent, 52.0, 95.0, 35.0);

}

unsafe extern "C" fn effect_speciallwlight(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.4, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.65);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.4, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.65);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.4, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.65);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.4, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.65);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, 9.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, -9.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, 4.5, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, -4.5, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
}

unsafe extern "C" fn sound_speciallwlight(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
	if is_excute(agent) {
        let handle = SoundModule::play_se(boma, Hash40::new("se_mario_special_l01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 0.7, 0);
		PLAY_SE(agent, Hash40::new("vc_mario_attack05"));
		PLAY_SE(agent, Hash40::new("se_mario_attackair_l01"));
    }

}

unsafe extern "C" fn expression_speciallwlight(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent){
        if VarModule::is_flag(agent.battle_object, vars::mario::instance::DISABLE_DSPECIAL_STALL) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        } else {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        }
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }

}

unsafe extern "C" fn game_specialairlwlight(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent){
        ArticleModule::remove_exist(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    FT_MOTION_RATE_RANGE(agent, 0.0, 10.0, 5.0);
    if !VarModule::is_flag(agent.battle_object, vars::mario::instance::DISABLE_DSPECIAL_STALL) {
        frame(lua_state, 10.0);
        FT_MOTION_RATE_RANGE(agent, 10.0, 40.0, 17.0);
        if is_excute(agent) { 
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 45, 0, 88, 3.2, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(-6.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 70, 45, 0, 88, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(5.0), Some(-0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
           
        }
        frame(lua_state, 40.0);
        FT_MOTION_RATE_RANGE(agent, 40.0, 45.0, 7.0);
        if is_excute(agent){
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 45.0);
        FT_MOTION_RATE_RANGE(agent, 45.0, 48.0, 3.0);
        frame(lua_state, 48.0);
        FT_MOTION_RATE_RANGE(agent, 48.0, 52.0, 7.0);
        if is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 52.0);
        FT_MOTION_RATE_RANGE(agent, 52.0, 95.0, 35.0);
    }
    else {
        frame(lua_state, 10.0);
        FT_MOTION_RATE_RANGE(agent, 10.0, 40.0, 16.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 70, 40, 0, 70, 2.7, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(-6.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 70, 40, 0, 70, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(5.0), Some(-0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        }
        frame(lua_state, 40.0);
        FT_MOTION_RATE_RANGE(agent, 40.0, 45.0, 7.0);
        if is_excute(agent){
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 45.0);
        FT_MOTION_RATE_RANGE(agent, 45.0, 48.0, 3.0);
        frame(lua_state, 48.0);
        FT_MOTION_RATE_RANGE(agent, 48.0, 52.0, 7.0); 
        frame(lua_state, 52.0);
        FT_MOTION_RATE_RANGE(agent, 52.0, 95.0, 35.0);
        
    }
}

unsafe extern "C" fn effect_specialairlwlight(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::mario::instance::DISABLE_DSPECIAL_STALL) { // Effects will disappear if you used galaxy spin in the air
            EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.5, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
            LAST_EFFECT_SET_ALPHA(agent, 0.55);
            LAST_EFFECT_SET_RATE(agent, 0.45);
            EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.45, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
            LAST_EFFECT_SET_ALPHA(agent, 0.55);
            LAST_EFFECT_SET_RATE(agent, 0.45);
            EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.5, 0, 0, 180, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
            LAST_EFFECT_SET_ALPHA(agent, 0.55);
            LAST_EFFECT_SET_RATE(agent, 0.45);
            EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.45, 0, 0, 180, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
            LAST_EFFECT_SET_ALPHA(agent, 0.55);
            LAST_EFFECT_SET_RATE(agent, 0.45);

            EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, 9.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, -9.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, 4.5, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, -4.5, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.5);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.5, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.1);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.5, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.1);
            LAST_EFFECT_SET_RATE(agent, 0.8);

            EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.2);
            EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.2);

        }
    }
}

unsafe extern "C" fn sound_specialairlwlight(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
	if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::mario::instance::DISABLE_DSPECIAL_STALL) { // Effects will change if you used galaxy spin in the air
			let handle = SoundModule::play_se(boma, Hash40::new("se_mario_special_l01"), true, false, false, false, app::enSEType(0));
            SoundModule::set_se_vol(boma, handle as i32, 0.7, 0);
            PLAY_SE(agent, Hash40::new("vc_mario_attack05"));
			PLAY_SE(agent, Hash40::new("se_mario_attackair_l01"));
        }
        else {
            PLAY_SE(agent, Hash40::new("se_mario_attackair_l01"));
            PLAY_SE(agent, Hash40::new("vc_mario_attack05"));
        }
	}
}

pub fn install(agent: &mut Agent) {
    agent.game_acmd("game_specialn", game_specialn);
    agent.game_acmd("game_specialairn", game_specialn);
    agent.effect_acmd("effect_specialn", effect_specialn);
    agent.effect_acmd("effect_specialairn", effect_specialn);
    agent.sound_acmd("sound_specialn", sound_specialn);
    agent.sound_acmd("sound_specialairn", sound_specialn);
    agent.expression_acmd("expression_specialn", expression_specialn);
    agent.expression_acmd("expression_specialairn", expression_specialn);
    agent.game_acmd("game_specialnfire", game_specialnfire);
    agent.game_acmd("game_specialairnfire", game_specialnfire);
    agent.effect_acmd("effect_specialnfire", effect_specialnfire);
    agent.effect_acmd("effect_specialairnfire", effect_specialnfire);
    agent.sound_acmd("sound_specialnfire", sound_specialnfire);
    agent.sound_acmd("sound_specialairnfire", sound_specialnfire);
    agent.expression_acmd("expression_specialnfire", expression_specialnfire);
    agent.expression_acmd("expression_specialairnfire", expression_specialnfire);
    agent.game_acmd("game_specials", game_specials);
    agent.game_acmd("game_specialairs", game_specialairs);
    agent.game_acmd("game_specialhi", game_specialhi);
    agent.game_acmd("game_specialairhi", game_specialhi);
    agent.effect_acmd("effect_specialhi", effect_specialhi);
    agent.effect_acmd("effect_specialairhi", effect_specialairhi);
    agent.game_acmd("game_speciallwlight", game_speciallwlight);
    agent.effect_acmd("effect_speciallwlight", effect_speciallwlight);
    agent.sound_acmd("sound_speciallwlight", sound_speciallwlight);
    agent.expression_acmd("expression_speciallwlight", expression_speciallwlight);
    agent.expression_acmd("expression_specialairlwlight", expression_speciallwlight);
    agent.game_acmd("game_specialairlwlight", game_specialairlwlight);
    agent.effect_acmd("effect_specialairlwlight", effect_specialairlwlight);
    agent.sound_acmd("sound_specialairlwlight", sound_specialairlwlight);
}
