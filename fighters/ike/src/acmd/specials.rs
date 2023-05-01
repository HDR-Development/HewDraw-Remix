
use super::*;


#[acmd_script( agent = "ike", script = "game_specialnend" , category = ACMD_GAME , low_priority)]
unsafe fn ike_special_n_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 275, 52, 0, 45, 4.0, 0.0, -4.0, 8.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 13.0, 0.0, 8.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 10.0, 0.0, 25.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 275, 52, 0, 45, 4.0, 0.0, -4.0, 8.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 15.0, 0.0, 8.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 10.0, 0.0, 25.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "ike", script = "game_specialairnend" , category = ACMD_GAME , low_priority)]
unsafe fn ike_special_air_n_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 275, 52, 0, 45, 4.0, 0.0, -4.0, 8.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 13.0, 0.0, 8.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 10.0, 0.0, 25.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 275, 52, 0, 45, 4.0, 0.0, -4.0, 8.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 15.0, 0.0, 8.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 10.0, 0.0, 25.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "ike", script = "game_specialsstart" , category = ACMD_GAME , low_priority)]
unsafe fn ike_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.500);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CHARGE);
    }
}

#[acmd_script( agent = "ike", script = "game_specialairsstart" , category = ACMD_GAME , low_priority)]
unsafe fn ike_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.500);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CHARGE);
    }
}

#[acmd_script( agent = "ike", script = "effect_specialshold" , category = ACMD_EFFECT , low_priority)]
unsafe fn ike_special_s_hold_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ike_iaigiri_hold"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 7, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 0.5, 3, 10, 3, 0, 0, 0, true);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 0.75, 3, 10, 3, 0, 0, 0, true);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 0.5, 3, 10, 3, 0, 0, 0, true);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 0.75, 3, 10, 3, 0, 0, 0, true);
    }
    wait(lua_state, 2.0);
}

#[acmd_script( agent = "ike", script = "game_specialsdash" , category = ACMD_GAME , low_priority)]
unsafe fn ike_special_s_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 0, 0, 0, 4.0, 0.0, 4.1, -1.0, Some(0.0), Some(4.1), Some(-15.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
}

#[acmd_script( agent = "ike", script = "effect_specialsdash" , category = ACMD_EFFECT , low_priority)]
unsafe fn ike_special_s_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 4, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT(fighter, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ike_iaigiri"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 5.0);
    for _ in 0..3{
        if is_excute(fighter) {
            if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
                EFFECT(fighter, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            }
            FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 4.0);
    }
}

#[acmd_script( agent = "ike", script = "sound_specialsdash" , category = ACMD_SOUND , low_priority)]
unsafe fn ike_special_s_dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            PLAY_SE(fighter, Hash40::new("vc_ike_appeal02"));
        }
        else{
            PLAY_SE(fighter, Hash40::new("vc_ike_special_s01"));
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ike_special_s02"));
    }
}

#[acmd_script( agent = "ike", script = "game_specialairsdash" , category = ACMD_GAME , low_priority)]
unsafe fn ike_special_air_s_dash_game(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        fighter.select_cliff_hangdata_from_name("special_s");
    }
}

#[acmd_script( agent = "ike", script = "game_specialsattack" , category = ACMD_GAME , low_priority)]
unsafe fn ike_special_s_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    // Instakill
    if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 400, 1, 6.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(-8.0), 10.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            //ATTACK(fighter, 1, 1, Hash40::new("top"), 0.0, 361, 0, 0, 0, 6.0, 0.0, 7.0, 10.0, Some(0.0), Some(5.0), Some(-10.0), 5.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.05);
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                SlowModule::set_whole(boma, 5, 30);
                SLOW_OPPONENT(fighter, 100.0, 100.0);
            }
        }
        frame(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 400, 1, 6.0, 0.0, 8.0, 15.0, Some(0.0), Some(8.0), Some(-10.0), 10.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                SlowModule::set_whole(boma, 5, 30);
                SLOW_OPPONENT(fighter, 100.0, 100.0);
            }
        }
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
        }
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c070), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(fighter, 2.0);
            }
        }
    }
    // Normal Quickdraw
    else {
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            FT_MOTION_RATE(fighter, 3.0/(1.1-1.0));
        }
        frame(lua_state, 1.1);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0/(1.3-1.1));
        }
        frame(lua_state, 1.3);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0/(2.0-1.3));
            ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 7.0, 70, 130, 0, 40, 2.75, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 7.0, 70, 130, 0, 40, 3.8, 0.0, 1.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("sword"), 7.0, 70, 130, 0, 40, 3.8, 0.0, 6.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("sword"), 7.0, 70, 130, 0, 40, 3.8, 0.0, 11.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(lua_state, 2.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
        }
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            FT_MOTION_RATE(fighter, 1.0);
        }
        frame(lua_state, 8.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
        }
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c070), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 15.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 17.0/(35.0-15.0));
        }
        frame(lua_state, 35.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
}

#[acmd_script( agent = "ike", script = "effect_specialsattack" , category = ACMD_EFFECT , low_priority)]
unsafe fn ike_special_s_attack_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT(fighter, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT(fighter, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            /* 
            let cbm_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2}; // Brightness vector
            let cbm_vec2 = Vector4f{ /* Red */ x: 0.125, /* Green */ y: 0.4, /* Blue */ z: 1.0, /* Alpha */ w: 0.0}; // Diffuse vector
            ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_vec1, /* Diffuse */ &cbm_vec2, 0.0, 0.0, /*Fadein time*/ 15, /* Display Color */ true);
            */
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword"), true, true);
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT(fighter, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT_OFF_KIND(fighter, Hash40::new("ike_volcano_hold"), false, false);
            EFFECT(fighter, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            ColorBlendModule::cancel_main_color(boma, 0);
        }
    }
}

#[acmd_script( agent = "ike", script = "game_specialairsattack" , category = ACMD_GAME , low_priority)]
unsafe fn ike_special_air_s_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0/(1.1-1.0));
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0/(1.3-1.1));
    }
    frame(lua_state, 1.3);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0/(2.0-1.3));
        ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 7.0, 70, 130, 0, 40, 2.75, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("sword"), 7.0, 70, 130, 0, 40, 3.8, 0.0, 1.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("sword"), 7.0, 70, 130, 0, 40, 3.8, 0.0, 6.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("sword"), 7.0, 70, 130, 0, 40, 3.8, 0.0, 11.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c070), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 17.0/(35.0-15.0));
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "ike", script = "effect_specialairsattack" , category = ACMD_EFFECT , low_priority)]
unsafe fn ike_special_air_s_attack_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, false);
        //LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword"), true, true);
    }
}

#[acmd_script( agent = "ike", script = "effect_specialsend" , category = ACMD_EFFECT , low_priority)]
unsafe fn ike_special_s_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            let cbm_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2}; // Brightness vector
            let cbm_vec2 = Vector4f{ /* Red */ x: 0.125, /* Green */ y: 0.4, /* Blue */ z: 1.0, /* Alpha */ w: 0.0}; // Diffuse vector
            ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_vec1, /* Diffuse */ &cbm_vec2, 0.0, 0.0, /*Fadein time*/ 15, /* Display Color */ true);
        }
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(lua_state, 4.0);
    for _ in 0..2{
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 4.0);
    }
    if is_excute(fighter) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT_OFF_KIND(fighter, Hash40::new("ike_volcano_hold"), false, false);
            ColorBlendModule::cancel_main_color(boma, 0);
        }
    }
}

#[acmd_script( agent = "ike", script = "game_specialairsend" , category = ACMD_GAME , low_priority)]
unsafe fn ike_special_air_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        fighter.select_cliff_hangdata_from_name("special_s");
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_END_NO_LANDING);
    }
}

#[acmd_script( agent = "ike", script = "game_specialhi2" , category = ACMD_GAME , low_priority)]
unsafe fn ike_special_hi2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
        WorkModule::off_flag(boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SWORD_FINAL);
        ArticleModule::generate_article(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, 0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 88, 100, 150, 0, 3.5, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 94, 100, 150, 0, 1.5, 0.0, 3.0, 15.0, Some(0.0), Some(7.0), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 96, 100, 150, 0, 3.5, 0.0, 5.0, 19.0, Some(0.0), Some(5.0), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 86, 100, 145, 0, 3.5, 0.0, 13.5, 7.0, Some(0.0), Some(13.5), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 6.0, 94, 100, 145, 0, 1.5, 0.0, 11.5, 15.0, Some(0.0), Some(15.5), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 6.0, 96, 100, 140, 0, 3.5, 0.0, 13.5, 19.0, Some(0.0), Some(13.5), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_CONTROL);
        WorkModule::set_float(boma, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
        WorkModule::set_float(boma, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("sword"), 1.0, 367, 100, 50, 0, 4.0, 0.0, 4.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("sword"), 1.0, 367, 100, 50, 0, 4.0, 0.0, 10.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c070), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD,  app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "ike", script = "game_specialhi3" , category = ACMD_GAME , low_priority)]
unsafe fn ike_special_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, -20);
        KineticModule::clear_speed_all(boma);
        ADD_SPEED_NO_LIMIT(fighter, 0, -6);
        ATTACK(fighter, 0, 0, Hash40::new("sword"), 3.0, 70, 30, 0, 120, 6.5, 0.0, 6.8, -1.0, Some(0.0), Some(6.8), Some(-12.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("sword"), 3.0, 270, 100, 165, 0, 4.8, 0.0, 6.8, -1.0, Some(0.0), Some(6.8), Some(-13.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("sword"), 3.0, 70, 50, 0, 0, 4.8, 0.0, 6.8, -1.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("sword"), 3.0, 270, 100, 150, 0, 4.8, 0.0, 6.8, -1.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

#[acmd_script( agent = "ike", script = "game_specialairhi2" , category = ACMD_GAME , low_priority)]
unsafe fn ike_special_air_hi2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
        WorkModule::off_flag(boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SWORD_FINAL);
        ArticleModule::generate_article(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, 0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 88, 100, 150, 0, 3.5, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 94, 100, 150, 0, 1.5, 0.0, 3.0, 15.0, Some(0.0), Some(7.0), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 96, 100, 150, 0, 3.5, 0.0, 5.0, 19.0, Some(0.0), Some(5.0), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 86, 100, 145, 0, 3.5, 0.0, 13.5, 7.0, Some(0.0), Some(13.5), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 6.0, 94, 100, 145, 0, 1.5, 0.0, 11.5, 15.0, Some(0.0), Some(15.5), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 6.0, 96, 100, 140, 0, 3.5, 0.0, 13.5, 19.0, Some(0.0), Some(13.5), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_CONTROL);
        WorkModule::set_float(boma, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
        WorkModule::set_float(boma, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("sword"), 1.0, 367, 100, 50, 0, 4.0, 0.0, 4.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("sword"), 1.0, 367, 100, 50, 0, 4.0, 0.0, 10.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c070), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD,  app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    install_acmd_scripts!(
        ike_special_n_end_game,
        ike_special_air_n_end_game,
        ike_special_s_start_game,
        ike_special_air_s_start_game,
        ike_special_s_hold_effect,
        ike_special_s_dash_game,
        ike_special_s_dash_effect,
        ike_special_s_dash_sound,
        ike_special_air_s_dash_game,
        ike_special_s_attack_game,
        ike_special_s_attack_effect,
        ike_special_air_s_attack_game,
        ike_special_air_s_attack_effect,
        ike_special_s_end_effect,
        ike_special_air_s_end_game,
        ike_special_hi2_game,
        ike_special_hi3_game,
        ike_special_air_hi2_game,
    );
}

