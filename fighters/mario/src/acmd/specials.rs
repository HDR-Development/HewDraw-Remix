use smash::app::sv_animcmd::QUAKE;

use super::*;


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


unsafe extern "C" fn mario_special_n_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
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
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 101, 0, 52, 3.0, 0.0, 6.5, 4.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 50, 101, 0, 52, 5.0, 0.0, 7.5, 10.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 40, 100, 0, 50, 3.0, 0.0, 6.5, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 40, 100, 0, 50, 5.0, 0.0, 7.5, 10.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
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
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flame"), Hash40::new("handl"), 1.0, 0, 0, 0, 0, 0, 0.2, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flame"), Hash40::new("handr"), 1.0, 0, 0, 0, 0, 0, 0.2, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 7.5, 10.5, 0, 0, 0, 0.26, true);
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
        PLAY_SE(fighter, Hash40::new("vc_mario_attack07"));
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


unsafe extern "C" fn mario_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_MANTLE, false, 0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    /*frame(lua_state, 6.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }*/
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        //search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 110, 100, 80, 0, 7.5, 0.0, 6.7, 9.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 110, 100, 80, 0, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::off_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}


unsafe extern "C" fn mario_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_MANTLE, false, 0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    /*frame(lua_state, 6.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }*/
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        //search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::mario::instance::SPECIAL_S_DISABLE_STALL) {
            WorkModule::on_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
            VarModule::on_flag(fighter.battle_object, vars::mario::instance::SPECIAL_S_DISABLE_STALL);
        }
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 110, 100, 80, 0, 7.5, 0.0, 6.7, 9.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 110, 100, 80, 0, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::off_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}


unsafe extern "C" fn mario_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if (WorkModule::is_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY)) {
        if is_excute(fighter) {
            boma.select_cliff_hangdata_from_name("special_hi");
        }
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            SA_SET(fighter, *SITUATION_KIND_AIR);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 60, 100, 160, 0, 2.5, 0.0, 4.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 86, 100, 150, 0, 4.0, 0.0, 4.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 100, 100, 150, 0, 4.0, 0.0, 4.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            SA_SET(fighter, *SITUATION_KIND_AIR);
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 4.5, 2.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 4.5, 7.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 8.0, 2.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.8, 0.0, 8.0, 8.0, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
            AttackModule::set_no_finish_camera(boma, 1, true, false);
            AttackModule::set_no_finish_camera(boma, 2, true, false);
            AttackModule::set_no_finish_camera(boma, 3, true, false);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 17.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 125, 0, 50, 6.5, 0.0, 9.5, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 125, 0, 50, 6.5, 0.0, 9.5, 2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 23.0);
        if is_excute(fighter) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
        }
    }
    else {
        if is_excute(fighter) {
            boma.select_cliff_hangdata_from_name("special_hi");
        }
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            SA_SET(fighter, *SITUATION_KIND_AIR);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 60, 100, 160, 0, 2.5, 0.0, 4.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 86, 100, 150, 0, 4.0, 0.0, 4.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 100, 100, 150, 0, 4.0, 0.0, 4.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            SA_SET(fighter, *SITUATION_KIND_AIR);
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 4.5, 2.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 4.5, 7.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 8.0, 2.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.8, 0.0, 8.0, 8.0, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
            AttackModule::set_no_finish_camera(boma, 1, true, false);
            AttackModule::set_no_finish_camera(boma, 2, true, false);
            AttackModule::set_no_finish_camera(boma, 3, true, false);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 17.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 125, 0, 50, 6.5, 0.0, 9.5, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_COIN_LAST, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 125, 0, 50, 6.5, 0.0, 9.5, 2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_COIN_LAST, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 23.0);
        if is_excute(fighter) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
        }
    }

}


unsafe extern "C" fn mario_special_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mario_superjump_power"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, -0.4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mario_superjump_fnish"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_superjump_power"), false, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("mario_superjump_fnish"), -1);
    }
}


unsafe extern "C" fn mario_special_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mario_superjump_power"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, -0.4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mario_superjump_fnish"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_superjump_power"), false, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("mario_superjump_fnish"), -1);
    }
}


unsafe extern "C" fn mario_special_lw_light(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE_RANGE(fighter, 0.0, 10.0, 5.0);
    if is_excute(fighter){
        ArticleModule::remove_exist(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(fighter, 10.0, 40.0, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 70, 45, 0, 88, 3.2, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(-6.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 70, 45, 0, 88, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(5.0), Some(-0.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(fighter, 40.0, 45.0, 7.0);
    if is_excute(fighter){
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 45.0);
    FT_MOTION_RATE_RANGE(fighter, 45.0, 48.0, 3.0);
    frame(lua_state, 48.0);
    FT_MOTION_RATE_RANGE(fighter, 48.0, 52.0, 7.0); 
    frame(lua_state, 52.0);
    FT_MOTION_RATE_RANGE(fighter, 52.0, 95.0, 35.0);

}

//Galaxy spin special effects

unsafe extern "C" fn effect_special_lw_light(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.5, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(fighter, 0.55);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.45, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(fighter, 0.55);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.5, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(fighter, 0.55);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.45, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(fighter, 0.55);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, 9.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, -9.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, 4.5, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, -4.5, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
}

//Galaxy spin sound effects

unsafe extern "C" fn sound_special_lw_light(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 6.0);
	if is_excute(fighter) {	
		PLAY_SE(fighter, Hash40::new("vc_mario_attack05"));
		PLAY_SE(fighter, Hash40::new("se_mario_special_l01"));
		PLAY_SE(fighter, Hash40::new("se_mario_attackair_l01"));
    }

}


unsafe extern "C" fn expression_special_lw_light(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter){
        if VarModule::is_flag(fighter.battle_object, vars::mario::instance::DISABLE_DSPECIAL_STALL) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        } else {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
        }
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }

}


unsafe extern "C" fn mario_special_air_lw_light(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter){
        ArticleModule::remove_exist(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    FT_MOTION_RATE_RANGE(fighter, 0.0, 10.0, 5.0);
    if !VarModule::is_flag(fighter.battle_object, vars::mario::instance::DISABLE_DSPECIAL_STALL) {
        frame(lua_state, 10.0);
        FT_MOTION_RATE_RANGE(fighter, 10.0, 40.0, 17.0);
        if is_excute(fighter) { 
            ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 70, 45, 0, 88, 3.2, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(-6.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 70, 45, 0, 88, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(5.0), Some(-0.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
           
        }
        frame(lua_state, 40.0);
        FT_MOTION_RATE_RANGE(fighter, 40.0, 45.0, 7.0);
        if is_excute(fighter){
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 45.0);
        FT_MOTION_RATE_RANGE(fighter, 45.0, 48.0, 3.0);
        frame(lua_state, 48.0);
        FT_MOTION_RATE_RANGE(fighter, 48.0, 52.0, 7.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 52.0);
        FT_MOTION_RATE_RANGE(fighter, 52.0, 95.0, 35.0);
    }
    else {
        frame(lua_state, 10.0);
        FT_MOTION_RATE_RANGE(fighter, 10.0, 40.0, 16.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 70, 40, 0, 70, 2.7, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(-6.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 70, 40, 0, 70, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(5.0), Some(-0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        }
        frame(lua_state, 40.0);
        FT_MOTION_RATE_RANGE(fighter, 40.0, 45.0, 7.0);
        if is_excute(fighter){
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 45.0);
        FT_MOTION_RATE_RANGE(fighter, 45.0, 48.0, 3.0);
        frame(lua_state, 48.0);
        FT_MOTION_RATE_RANGE(fighter, 48.0, 52.0, 7.0); 
        frame(lua_state, 52.0);
        FT_MOTION_RATE_RANGE(fighter, 52.0, 95.0, 35.0);
        
    }
}

//Galaxy spin special effects

unsafe extern "C" fn effect_special_air_lw_light(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::mario::instance::DISABLE_DSPECIAL_STALL) { // Effects will disappear if you used galaxy spin in the air
            EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.5, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.045, 0.345, 2.05);
            LAST_EFFECT_SET_ALPHA(fighter, 0.55);
            LAST_EFFECT_SET_RATE(fighter, 0.45);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.45, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.045, 0.345, 2.05);
            LAST_EFFECT_SET_ALPHA(fighter, 0.55);
            LAST_EFFECT_SET_RATE(fighter, 0.45);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.5, 0, 0, 180, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.045, 0.345, 2.05);
            LAST_EFFECT_SET_ALPHA(fighter, 0.55);
            LAST_EFFECT_SET_RATE(fighter, 0.45);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.45, 0, 0, 180, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.045, 0.345, 2.05);
            LAST_EFFECT_SET_ALPHA(fighter, 0.55);
            LAST_EFFECT_SET_RATE(fighter, 0.45);

            EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, 9.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, -9.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, 4.5, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.3, -4.5, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.5, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(fighter, 0.1);
            LAST_EFFECT_SET_RATE(fighter, 0.8);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9.5, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(fighter, 0.1);
            LAST_EFFECT_SET_RATE(fighter, 0.8);

            EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(fighter, 0.2);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_ALPHA(fighter, 0.2);

        }
    }
}

//Galaxy spin sound effects

unsafe extern "C" fn sound_special_air_lw_light(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
	if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::mario::instance::DISABLE_DSPECIAL_STALL) { // Effects will change if you used galaxy spin in the air
			PLAY_SE(fighter, Hash40::new("vc_mario_attack05"));
			PLAY_SE(fighter, Hash40::new("se_mario_special_l01"));
			PLAY_SE(fighter, Hash40::new("se_mario_attackair_l01"));
        }
        else {
            PLAY_SE(fighter, Hash40::new("se_mario_attackair_l01"));
            PLAY_SE(fighter, Hash40::new("vc_mario_attack05"));
        }
	}
}

pub fn install() {
    smashline::Agent::new("mario")
        .game_acmd("game_specialn", mario_special_n_game)
        .game_acmd("game_specialairn", mario_special_n_game)
        .effect_acmd("effect_specialn", mario_special_n_effect)
        .effect_acmd("effect_specialairn", mario_special_n_effect)
        .sound_acmd("sound_specialn", mario_special_n_sound)
        .sound_acmd("sound_specialairn", mario_special_n_sound)
        .expression_acmd("expression_specialn", mario_special_n_expression)
        .expression_acmd("expression_specialairn", mario_special_n_expression)
        .game_acmd("game_specialnfire", mario_special_n_fire_game)
        .game_acmd("game_specialairnfire", mario_special_n_fire_game)
        .effect_acmd("effect_specialnfire", mario_special_n_fire_effect)
        .effect_acmd("effect_specialairnfire", mario_special_n_fire_effect)
        .sound_acmd("sound_specialnfire", mario_special_n_fire_sound)
        .sound_acmd("sound_specialairnfire", mario_special_n_fire_sound)
        .expression_acmd("expression_specialnfire", mario_special_n_fire_expression)
        .expression_acmd(
            "expression_specialairnfire",
            mario_special_n_fire_expression,
        )
        .game_acmd("game_specials", mario_special_s_game)
        .game_acmd("game_specialairs", mario_special_air_s_game)
        .game_acmd("game_specialhi", mario_special_hi_game)
        .game_acmd("game_specialairhi", mario_special_hi_game)
        .effect_acmd("effect_specialhi", mario_special_hi_effect)
        .effect_acmd("effect_specialairhi", mario_special_air_hi_effect)
        .game_acmd("game_speciallwlight", mario_special_lw_light)
        .effect_acmd("effect_speciallwlight", effect_special_lw_light)
        .sound_acmd("sound_speciallwlight", sound_special_lw_light)
        .expression_acmd("expression_speciallwlight", expression_special_lw_light)
        .expression_acmd("expression_specialairlwlight", expression_special_lw_light)
        .game_acmd("game_specialairlwlight", mario_special_air_lw_light)
        .effect_acmd("effect_specialairlwlight", effect_special_air_lw_light)
        .sound_acmd("sound_specialairlwlight", sound_special_air_lw_light)
        .install();
}
