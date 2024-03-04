
use super::*;



unsafe extern "C" fn richter_attack_s4_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 24.0);
    for _ in 0..5 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 365, 100, 30, 0, 2.0, 0.0, 13.6, 44.5, Some(0.0), Some(7.6), Some(7.5), 0.75, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 50, 85, 0, 60, 3.0, 0.0, 13.6, 39.0, Some(0.0), Some(7.6), Some(7.5), 0.75, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 85, 0, 60, 4.0, 0.0, 13.7, 45.0, Some(0.0), Some(13.5), Some(43.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 3.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn richter_attack_s4_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -2, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.3, true);        
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.2, true);        
        LAST_EFFECT_SET_RATE(fighter, 3.0);
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_firebar_trace"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        COL_NORMAL(fighter);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_firebar_trace"), Hash40::new("throw"), 0, 0, 0.5, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.32, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.7, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("haver"), 0, 0, 0, 0, 65, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.3);
        COL_NORMAL(fighter);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.34, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.7, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.36, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.7, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("richter_whip_straight"), false, true);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_firebar_trace"), false, true);
    }
}

unsafe extern "C" fn richter_attack_s4_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_whip_holding"));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s01"));
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack06"));
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s02"));
        PLAY_SE(fighter, Hash40::new("se_item_firebar_ll"));
    }
}

unsafe extern "C" fn richter_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 24.0);
    for _ in 0..5 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 365, 100, 30, 0, 2.0, 0.0, 7.6, 44.5, Some(0.0), Some(7.6), Some(7.5), 0.75, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 50, 85, 0, 60, 3.0, 0.0, 7.6, 39.0, Some(0.0), Some(7.6), Some(7.5), 0.75, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 85, 0, 60, 4.0, 0.0, 7.6, 45.0, Some(0.0), Some(7.6), Some(43.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 3.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn richter_attack_s4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -2, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.3, true);        
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.2, true);        
        LAST_EFFECT_SET_RATE(fighter, 3.0);
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_firebar_trace"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        COL_NORMAL(fighter);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_firebar_trace"), Hash40::new("throw"), 0, 0, 0.5, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.32, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.7, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("haver"), 0, 0, 0, 0, 65, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.3);
        COL_NORMAL(fighter);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.34, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.7, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.36, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.7, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("richter_whip_straight"), false, true);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_firebar_trace"), false, true);
    }
}

unsafe extern "C" fn richter_attack_s4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_whip_holding"));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s01"));
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack06"));
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s02"));
        PLAY_SE(fighter, Hash40::new("se_item_firebar_ll"));
    }
}

unsafe extern "C" fn richter_attack_s4_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 24.0);
    for _ in 0..5 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 365, 100, 30, 0, 2.0, 0.0, 1.6, 44.5, Some(0.0), Some(7.6), Some(7.5), 0.75, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 50, 85, 0, 60, 3.0, 0.0, 1.6, 39.0, Some(0.0), Some(7.6), Some(7.5), 0.75, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 85, 0, 60, 4.0, 0.0, 1.5, 45.0, Some(0.0), Some(1.7), Some(43.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 3.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn richter_attack_s4_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -2, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.3, true);        
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.2, true);        
        LAST_EFFECT_SET_RATE(fighter, 3.0);
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_firebar_trace"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        COL_NORMAL(fighter);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_firebar_trace"), Hash40::new("throw"), 0, 0, 0.5, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.32, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.7, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("haver"), 0, 0, 0, 0, 65, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.3);
        COL_NORMAL(fighter);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.34, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.7, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.36, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.7, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
        FLASH(fighter, 1.0, 0.4, 0.0, 0.2);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("richter_whip_straight"), false, true);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_firebar_trace"), false, true);
    }
}

unsafe extern "C" fn richter_attack_s4_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_whip_holding"));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s01"));
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack06"));
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s02"));
        PLAY_SE(fighter, Hash40::new("se_item_firebar_ll"));
    }
}

unsafe extern "C" fn richter_whip_attack_s4_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

unsafe extern "C" fn richter_whip_attack_s4_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}


unsafe extern "C" fn richter_whip_attack_s4_charge_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

unsafe extern "C" fn richter_whip_attack_s4_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot11"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);    
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot11"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);    
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot11"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);  
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);  
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_damage_fire"), Hash40::new("hookshot27"), 0, 3.6, -2.5, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 3.0);    
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_steam2"), Hash40::new("hookshot27"), 0, 0, 0, 0, 0, 0, 0.2, true);        
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        LAST_EFFECT_SET_COLOR(fighter, 0.3, 0.3, 0.3);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_steam2"), false, true);
    }
}

unsafe extern "C" fn richter_whip_attack_s4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot11"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);    
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot11"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);    
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot11"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);  
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);  
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_damage_fire"), Hash40::new("hookshot27"), 0, 3.6, -2.5, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 3.0);    
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_steam2"), Hash40::new("hookshot27"), 0, 0, 0, 0, 0, 0, 0.2, true);        
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        LAST_EFFECT_SET_COLOR(fighter, 0.3, 0.3, 0.3);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_steam2"), false, true);
    }
}

unsafe extern "C" fn richter_whip_attack_s4_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot11"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);    
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot11"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);    
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot11"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);  
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.4, 0.0);  
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_damage_fire"), Hash40::new("hookshot27"), 0, 3.6, -2.5, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 3.0);    
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_steam2"), Hash40::new("hookshot27"), 0, 0, 0, 0, 0, 0, 0.2, true);        
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        LAST_EFFECT_SET_COLOR(fighter, 0.3, 0.3, 0.3);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_steam2"), false, true);
    }
}

unsafe extern "C" fn richter_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 93, 100, 0, 60, 6.7, 0.0, 27.5, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 93, 100, 0, 60, 5.7, 0.0, 27.5, -4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 93, 100, 0, 60, 5.7, 0.0, 27.5, 10.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 11.0, 91, 100, 0, 60, 5.0, 0.0, 17.5, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 11.0, 91, 100, 0, 60, 5.5, 0.0, 10.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
      
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 93, 98, 0, 60, 6.4, 0.0, 27.5, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 93, 98, 0, 60, 5.4, 0.0, 27.5, -4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 93, 98, 0, 60, 5.4, 0.0, 27.5, 10.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 91, 98, 0, 60, 5.0, 0.0, 17.5, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 9.0, 91, 98, 0, 60, 5.5, 0.0, 10.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
      
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn richter_attack_hi4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, false);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
        LAST_EFFECT_SET_ALPHA(fighter, 1.0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("richter_whip_hi3"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.025, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 1.0);
    }
}

unsafe extern "C" fn richter_attack_hi4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_whip_holding"));
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_attackhard_h01"));
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack05"));
        PLAY_SE(fighter, Hash40::new("se_richter_attackhard_s01"));
    }
}

unsafe extern "C" fn richter_whip_attack_hi4_charge_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

unsafe extern "C" fn richter_whip_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

unsafe extern "C" fn richter_whip_attack_hi4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot23"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot18"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot14"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot4"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot23"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot18"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot14"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot4"), 0, 0, 0, 0, 0, 0, 1.2, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
}

unsafe extern "C" fn richter_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 0.8);
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 48, 85, 0, 65, 6.0, 0.0, 4.5, 10.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 48, 85, 0, 65, 4.5, 0.0, 3.5, 20.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 48, 82, 0, 65, 4.5, 0.0, 3.0, 29.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 2.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        /*
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 109, 57, 0, 70, 5.5, 0.0, 3.0, 28.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        }
        */
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 48, 85, 0, 65, 6.0, 0.0, 4.5, -5.8, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 48, 85, 0, 65, 4.5, 0.0, 3.5, -13.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 48, 82, 0, 65, 4.5, 0.0, 3.0, -22.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 2.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        /*
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 109, 62, 0, 70, 4.0, 0.0, 3.0, -21.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        }
        */
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn richter_whip_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 0.8);
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}

pub fn install() {
    smashline::Agent::new("richter")
        .acmd("game_attacks4hi", richter_attack_s4_hi_game)
        .acmd("effect_attacks4hi", richter_attack_s4_hi_effect)
        .acmd("sound_attacks4hi", richter_attack_s4_hi_sound)
        .acmd("game_attacks4", richter_attack_s4_s_game)
        .acmd("effect_attacks4", richter_attack_s4_effect)
        .acmd("sound_attacks4", richter_attack_s4_sound)
        .acmd("game_attacks4lw", richter_attack_s4_lw_game)
        .acmd("effect_attacks4lw", richter_attack_s4_lw_effect)
        .acmd("sound_attacks4lw", richter_attack_s4_lw_sound)
        .acmd("game_attackhi4", richter_attack_hi4_game)
        .acmd("effect_attackhi4", richter_attack_hi4_effect)
        .acmd("sound_attackhi4", richter_attack_hi4_sound)
        .acmd("game_attacklw4", richter_attack_lw4_game)
        .install();
    smashline::Agent::new("richter_whip")
        .acmd("game_attackhi4", richter_whip_attack_hi4_game)
        .acmd("game_attacklw4", richter_whip_attack_lw4_game)
        .acmd("game_attacks4charge", richter_whip_attack_s4_charge_game)
        .acmd("game_attacks4hi", richter_whip_attack_s4_hi_game)
        .acmd("game_attacks4lw", richter_whip_attack_s4_lw_game)
        .acmd("effect_attacks4", richter_whip_attack_s4_effect)
        .acmd("effect_attacks4lw", richter_whip_attack_s4_lw_effect)
        .acmd("game_attackhi4charge", richter_whip_attack_hi4_charge_game)
        .acmd("effect_attackhi4", richter_whip_attack_hi4_effect)
        .acmd("game_attacklw4", richter_whip_attack_lw4_game)
        .install();
}
