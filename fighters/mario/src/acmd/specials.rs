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
        if boma.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
        sv_kinetic_energy!(set_speed, agent, FIGHTER_KINETIC_ENERGY_ID_STOP, -0.75 * PostureModule::lr(boma));
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 101, 0, 52, 3.0, 0.0, 6.5, 4.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 50, 101, 0, 52, 5.0, 0.0, 7.5, 10.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
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
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 6.0, 0.0, 9.0, 5.0, 0.0, 9.0, -5.0, 1.2, 0.8, 50, false, 0.8, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 40, 0, 88, 3.2, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(-6.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 70, 40, 0, 88, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(5.0), Some(-0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 40, 0, 88, 3.2, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(-6.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 70, 40, 0, 88, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(5.0), Some(-0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 50, 0, 72, 3.2, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(-6.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 65, 50, 0, 72, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(5.0), Some(-0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 50, 0, 72, 3.2, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(-6.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 65, 50, 0, 72, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(5.0), Some(-0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 22.0);
    if is_excute(agent){
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent){
        agent.on_flag(*FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 34.0);
    if is_excute(agent){
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if agent.lr() < 0.0 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, false);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, false);
        }
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.8, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.65);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.8, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.65);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.8, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.65);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.8, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.65);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.5, 9.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.5, -9.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.5, 4.5, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.5, -4.5, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
}

unsafe extern "C" fn effect_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.5, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.5, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.5, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.5, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.045, 0.345, 2.05);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.5, 9.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.5, -9.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.5, 4.5, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 9.5, -4.5, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
}

unsafe extern "C" fn sound_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let handle = SoundModule::play_se(boma, Hash40::new("se_mario_special_s01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 0.6, 0);
        PLAY_SE(agent, Hash40::new("se_mario_attackair_l01"));
        let rng = app::sv_math::rand(hash40("fighter"), 3);
        match rng {
            0 => {
                let handle = SoundModule::play_se(agent.module_accessor, Hash40::new("vc_mario_starspin"), false, false, false, false, enSEType(0));
                SoundModule::set_se_vol(agent.module_accessor, handle as i32, 0.35, 0);
            },
            _ => PLAY_SE(agent, Hash40::new("vc_mario_attack05")),
        };
    }
}

unsafe extern "C" fn expression_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let attr = if (WorkModule::is_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY))
        { Hash40::new("collision_attr_mario_local_coin") } else { Hash40::new("collision_attr_coin") };
    let collision_sound_attr = if (WorkModule::is_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY))
        { *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN } else { *COLLISION_SOUND_ATTR_COIN };
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
            HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
            HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_XLU);
        }
        SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 60, 100, 160, 0, 2.5, 0.0, 4.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, collision_sound_attr, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 86, 100, 150, 0, 4.0, 0.0, 4.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, collision_sound_attr, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 100, 100, 150, 0, 4.0, 0.0, 4.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, collision_sound_attr, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 4.5, 2.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, collision_sound_attr, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.6, 97, 100, 170, 0, 3.8, 0.0, 4.5, 7.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, collision_sound_attr, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 8.0, 2.5, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, collision_sound_attr, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.6, 97, 100, 110, 0, 3.0, 0.0, 8.0, 8.0, None, None, None, 0.75, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_M, collision_sound_attr, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        AttackModule::set_no_finish_camera(boma, 2, true, false);
        AttackModule::set_no_finish_camera(boma, 3, true, false);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        let sound_attr = if (WorkModule::is_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY))
            { *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST } else { *COLLISION_SOUND_ATTR_MARIO_COIN_LAST };
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 3.0, 60, 140, 0, 50, 8.75, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, sound_attr, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 3.0, 60, 140, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attr, *ATTACK_SOUND_LEVEL_L, sound_attr, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
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
    frame(lua_state, 6.0);
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
    frame(lua_state, 6.0);
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

unsafe extern "C" fn game_longjump(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        if VarModule::get_int(agent.battle_object, vars::mario::status::SPECIAL_LW_LONG_JUMP_KIND) == vars::mario::LONG_JUMP_B {
            VarModule::on_flag(agent.battle_object, vars::mario::status::SPECIAL_LW_ENABLE_LANDING);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::mario::status::SPECIAL_LW_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::common::status::DISABLE_ECB_SHIFT);
        let lr = PostureModule::lr(agent.module_accessor);
        let speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * lr;
        let back_speed_x_hitbox_max = ParamModule::get_float(agent.battle_object, ParamType::Agent, "long_jump.back_speed_x_hitbox_max");
        let back_speed_x_hitbox_min = ParamModule::get_float(agent.battle_object, ParamType::Agent, "long_jump.back_speed_x_hitbox_min");
        if speed_x <= back_speed_x_hitbox_max {
            let ratio = (speed_x - back_speed_x_hitbox_max) / (back_speed_x_hitbox_min - back_speed_x_hitbox_max);
            let dmg = util::nlerp(9.0, 15.0, 1.0, ratio);
            let hitlag = util::nlerp(1.0, 1.5, 1.0, ratio);
            ATTACK(agent, 0, 0, Hash40::new("top"), dmg, 45, 45, 0, 100, 3.0, 0.0, 5.0, 1.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    }
}

unsafe extern "C" fn sound_longjump(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        if VarModule::get_int(agent.battle_object, vars::mario::status::SPECIAL_LW_LONG_JUMP_KIND) == vars::mario::LONG_JUMP_B {
            // yahoo, the full yahoo, and nothing but the yahoo
            PLAY_SE(agent, Hash40::new("vc_mario_009"));
        }
        else {
            let lr = PostureModule::lr(agent.module_accessor);
            let speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * lr;
            let back_speed_x_hitbox_max = ParamModule::get_float(agent.battle_object, ParamType::Agent, "long_jump.back_speed_x_hitbox_max");
            if speed_x > back_speed_x_hitbox_max {
                // standard long jump
                let handle = SoundModule::play_se(agent.module_accessor, Hash40::new("vc_mario_longjumpback01"), false, false, false, false, enSEType(0));
                SoundModule::set_se_vol(agent.module_accessor, handle as i32, 0.45, 0);
            }
            // else {
            //     // launching off an edge
            //     let rand = sv_math::rand(hash40("halfapress"), 2);
            //     let wahoo = match rand {
            //         //0 => Hash40::new("vc_mario_longjumpback01"),
            //         0 => Hash40::new("vc_mario_longjumpback02"),
            //         _ => Hash40::new("vc_mario_longjumpback03"),
            //     };
            //     let handle = SoundModule::play_se(agent.module_accessor, wahoo, false, false, false, false, enSEType(0));
            //     SoundModule::set_se_vol(agent.module_accessor, handle as i32, 0.45, 0);
            // }
        }
        
    }
}

unsafe extern "C" fn expression_longjump(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn effect_longjumpland(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_longjumpland(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_groundpoundstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 14.0, 19.0);
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        KineticModule::unable_energy_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_groundpoundstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        // EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_groundpoundstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_special_l01"));
        PLAY_SE(agent, Hash40::new("vc_mario_attack01"));
    }
}

unsafe extern "C" fn game_groundpoundfall(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("claviclec"), 14.0, 70, 85, 0, 70, 4.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
        AttackModule::set_attack_height_all(agent.module_accessor, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

unsafe extern "C" fn effect_groundpoundfall(agent: &mut L2CAgentBase) {
    loop {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 1, -90, 0, 0, 1, true);
        }
        wait(agent.lua_state_agent, 3.0);
    }
}

unsafe extern "C" fn expression_groundpoundfall(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_groundpoundland(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 25.0, 29.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 70, 100, 0, 0, 3.8, 0.0, 2.8, -6.7, Some(0.0), Some(2.8), Some(6.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 9, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_groundpoundland(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_special_l02"));
    }
}

unsafe extern "C" fn effect_groundpoundland(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
}

unsafe extern "C" fn expression_groundpoundland(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn, Priority::Low);
    agent.acmd("game_specialairn", game_specialn, Priority::Low);
    agent.acmd("effect_specialn", effect_specialn, Priority::Low);
    agent.acmd("effect_specialairn", effect_specialn, Priority::Low);
    agent.acmd("sound_specialn", sound_specialn, Priority::Low);
    agent.acmd("sound_specialairn", sound_specialn, Priority::Low);
    agent.acmd("expression_specialn", expression_specialn, Priority::Low);
    agent.acmd("expression_specialairn", expression_specialn, Priority::Low);

    agent.acmd("game_specialnfire", game_specialnfire, Priority::Low);
    agent.acmd("game_specialairnfire", game_specialnfire, Priority::Low);
    agent.acmd("effect_specialnfire", effect_specialnfire, Priority::Low);
    agent.acmd("effect_specialairnfire", effect_specialnfire, Priority::Low);
    agent.acmd("sound_specialnfire", sound_specialnfire, Priority::Low);
    agent.acmd("sound_specialairnfire", sound_specialnfire, Priority::Low);
    agent.acmd("expression_specialnfire", expression_specialnfire, Priority::Low);
    agent.acmd("expression_specialairnfire", expression_specialnfire, Priority::Low);

    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialairs", game_specials, Priority::Low);
    agent.acmd("effect_specials", effect_specials, Priority::Low);
    agent.acmd("effect_specialairs", effect_specialairs, Priority::Low);
    agent.acmd("sound_specials", sound_specials, Priority::Low);
    agent.acmd("sound_specialairs", sound_specials, Priority::Low);
    agent.acmd("expression_specials", expression_specials, Priority::Low);
    agent.acmd("expression_specialairs", expression_specials, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi, Priority::Low);
    agent.acmd("effect_specialairhi", effect_specialairhi, Priority::Low);
    
    agent.acmd("sound_speciallwstart", acmd_stub, Priority::Low);

    agent.acmd("game_speciallwjump", game_longjump, Priority::Low);
    agent.acmd("sound_speciallwjump", sound_longjump, Priority::Low);
    agent.acmd("expression_speciallwjump", expression_longjump, Priority::Low);

    agent.acmd("effect_speciallwlanding", effect_longjumpland, Priority::Low);
    agent.acmd("sound_speciallwlanding", acmd_stub, Priority::Low);
    agent.acmd("expression_speciallwlanding", expression_longjumpland, Priority::Low);

    agent.acmd("game_specialairlwstart", game_groundpoundstart, Priority::Low);
    agent.acmd("effect_specialairlwstart", effect_groundpoundstart, Priority::Low);
    agent.acmd("sound_specialairlwstart", sound_groundpoundstart, Priority::Low);

    agent.acmd("game_specialairlwfall", game_groundpoundfall, Priority::Low);
    agent.acmd("effect_specialairlwfall", effect_groundpoundfall, Priority::Low);
    agent.acmd("expression_specialairlwfall", expression_groundpoundfall, Priority::Low);

    agent.acmd("game_specialairlwlanding", game_groundpoundland, Priority::Low);
    agent.acmd("effect_specialairlwlanding", effect_groundpoundland, Priority::Low);
    agent.acmd("sound_specialairlwlanding", sound_groundpoundland, Priority::Low);
    agent.acmd("expression_specialairlwlanding", expression_groundpoundland, Priority::Low);
}