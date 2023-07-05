
use super::*;

#[acmd_script( agent = "master", script = "effect_attacks4charge" , category = ACMD_EFFECT , low_priority)]
unsafe fn master_attack_s4_charge_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(fighter, Hash40::new("master_spear_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(fighter, 0.3);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FLW_POS(fighter, Hash40::new("master_spear_aura_particle"), Hash40::new("haver"), -8, 6, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(fighter, 0.3);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(fighter, 0.3);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(fighter, 0.3);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 24, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "master", script = "game_attacks4hi" , category = ACMD_GAME , low_priority)]
unsafe fn master_attack_s4_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 3.5);
        }
        else{
            FT_MOTION_RATE(fighter, 0.800);
        } 
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        // Reel-in hit 1
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            // Ground-only
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.0, 270, 100, 10, 0, 3.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.0, 270, 100, 10, 0, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            // Air-only
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.0, 367, 100, 50, 0, 3.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 4.0, 367, 100, 50, 0, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        // Normal fsmash
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 90, 0, 45, 4.5, 0.0, 5.5, 6.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 13.0, 361, 90, 0, 45, 2.0, 0.0, 3.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 19.0, 36, 90, 0, 30, 3.0, -0.5, 16.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 1.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 19.0, 36, 90, 0, 30, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 1.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 4, 0, Hash40::new("shoulderr"), 11.0, 36, 90, 0, 50, 2.5, -0.5, 0.0, 0.0, None, None, None, 1.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 26.0);
    for _ in 0..4{
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                    // Ground-only
                    ATTACK(fighter, 4, 1, Hash40::new("top"), 0.0, 365, 100, 10, 0, 15.0, 0.0, 10.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                    // Air-only
                    ATTACK(fighter, 5, 1, Hash40::new("top"), 0.0, 367, 100, 10, 0, 15.0, 0.0, 10.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                }
            }
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(fighter, 2.0/(46.0-30.0));
            }
        }
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        // Reel-in hit 2
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 0.5);
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                ATTACK(fighter, 0, 1, Hash40::new("haver"), 15.0, 20, 100, 60, 0, 10.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 1, Hash40::new("haver"), 15.0, 20, 100, 60, 0, 10.5, -0.5, 35.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
                AttackModule::set_add_reaction_frame(boma, 0, 15.0, false);
                AttackModule::set_add_reaction_frame(boma, 1, 15.0, false);
            }
        }
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        // Reel-in hit 2
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(fighter, 2.0);
            }
            else{
                FT_MOTION_RATE(fighter, 1.1);
            }
        }
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(fighter, 1.0);
            }
        }
    }
    frame(lua_state, 85.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "master", script = "effect_attacks4hi" , category = ACMD_EFFECT , low_priority)]
unsafe fn master_attack_s4_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -20, 18, 13.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 17, 13.25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(fighter, Hash40::new("master_spear_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FLW_POS(fighter, Hash40::new("master_spear_aura_particle"), Hash40::new("haver"), -8, 6, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("master_spearflare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 2);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("master_smash_s_wind"), Hash40::new("top"), 0, 19, 34.5, -18, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            
        }
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("master_smash_s_speedline"), Hash40::new("haver"), 0, 26, 0, -90, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("master_smash_s_line"), Hash40::new("haver"), 0, 26, 0, -90, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(fighter, Hash40::new("master_smash_s_flash"), Hash40::new("haver"), 0, 28, -1.25, 0, 0, 0, 1.3, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("master_smash_s_line"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("master_smash_s_speedline"), -1);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("master_smash_s_line"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("master_smash_s_wind"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
                EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
            }
        }
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
                EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
                EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
                EFFECT_FOLLOW(fighter, Hash40::new("master_axe_hold_end"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 1.0, true);
                LAST_EFFECT_SET_RATE(fighter, 0.8);
                EFFECT_FOLLOW(fighter, Hash40::new("master_axe_slash_particle"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 0.75, true);
                EFFECT_FOLLOW(fighter, Hash40::new("master_wire_hit"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 1.0, true);
            }
        }
    }
}

#[acmd_script( agent = "master", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn master_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 3.5);
        }
        else{
            FT_MOTION_RATE(fighter, 0.800);
        } 
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        // Reel-in hit 1
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            // Ground-only
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.0, 270, 100, 10, 0, 3.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.0, 270, 100, 10, 0, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            // Air-only
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.0, 367, 100, 50, 0, 3.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 4.0, 367, 100, 50, 0, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        // Normal fsmash
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 90, 0, 45, 4.5, 0.0, 5.5, 6.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 13.0, 361, 90, 0, 45, 2.0, 0.0, 3.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 19.0, 36, 90, 0, 30, 3.0, -0.5, 16.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 1.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 19.0, 36, 90, 0, 30, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 1.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 4, 0, Hash40::new("shoulderr"), 11.0, 36, 90, 0, 50, 2.5, -0.5, 0.0, 0.0, None, None, None, 1.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 26.0);
    for _ in 0..4{
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                    // Ground-only
                    ATTACK(fighter, 4, 1, Hash40::new("top"), 0.0, 365, 100, 10, 0, 15.0, 0.0, 10.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                    // Air-only
                    ATTACK(fighter, 5, 1, Hash40::new("top"), 0.0, 367, 100, 10, 0, 15.0, 0.0, 10.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                }
            }
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(fighter, 2.0/(46.0-30.0));
            }
        }
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        // Reel-in hit 2
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 0.5);
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                ATTACK(fighter, 0, 1, Hash40::new("haver"), 15.0, 20, 100, 60, 0, 10.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 1, Hash40::new("haver"), 15.0, 20, 100, 60, 0, 10.5, -0.5, 35.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
                AttackModule::set_add_reaction_frame(boma, 0, 15.0, false);
                AttackModule::set_add_reaction_frame(boma, 1, 15.0, false);
            }
        }
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        // Reel-in hit 2
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(fighter, 2.0);
            }
            else{
                FT_MOTION_RATE(fighter, 1.1);
            }
        }
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(fighter, 1.0);
            }
        }
    }
    frame(lua_state, 85.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "master", script = "effect_attacks4" , category = ACMD_EFFECT , low_priority)]
unsafe fn master_attack_s4_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -20, 18, 13.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 17, 13.25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(fighter, Hash40::new("master_spear_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FLW_POS(fighter, Hash40::new("master_spear_aura_particle"), Hash40::new("haver"), -8, 6, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("master_spearflare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 2);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("master_smash_s_wind"), Hash40::new("top"), 0, 10, 35, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            
        }
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("master_smash_s_speedline"), Hash40::new("haver"), 0, 26, 0, -90, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("master_smash_s_line"), Hash40::new("haver"), 0, 26, 0, -90, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(fighter, Hash40::new("master_smash_s_flash"), Hash40::new("haver"), 0, 28, -1.25, 0, 0, 0, 1.3, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("master_smash_s_line"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("master_smash_s_speedline"), -1);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("master_smash_s_line"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("master_smash_s_wind"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
                EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
            }
        }
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
                EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
                EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
                EFFECT_FOLLOW(fighter, Hash40::new("master_axe_hold_end"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 1.0, true);
                LAST_EFFECT_SET_RATE(fighter, 0.8);
                EFFECT_FOLLOW(fighter, Hash40::new("master_axe_slash_particle"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 0.75, true);
                EFFECT_FOLLOW(fighter, Hash40::new("master_wire_hit"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 1.0, true);
            }
        }
    }
}

#[acmd_script( agent = "master", script = "game_attacks4lw" , category = ACMD_GAME , low_priority)]
unsafe fn master_attack_s4_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 3.5);
        }
        else{
            FT_MOTION_RATE(fighter, 0.800);
        } 
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        // Reel-in hit 1
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            // Ground-only
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.0, 270, 100, 10, 0, 3.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.0, 270, 100, 10, 0, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            // Air-only
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.0, 367, 100, 50, 0, 3.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 4.0, 367, 100, 50, 0, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        // Normal fsmash
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 90, 0, 45, 4.5, 0.0, 5.5, 6.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 13.0, 361, 90, 0, 45, 2.0, 0.0, 3.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("haver"), 19.0, 36, 90, 0, 30, 3.0, -0.5, 16.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 1.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 3, 0, Hash40::new("haver"), 19.0, 36, 90, 0, 30, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 1.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 4, 0, Hash40::new("shoulderr"), 11.0, 36, 90, 0, 50, 2.5, -0.5, 0.0, 0.0, None, None, None, 1.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 26.0);
    for _ in 0..4{
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                    // Ground-only
                    ATTACK(fighter, 4, 1, Hash40::new("top"), 0.0, 365, 100, 10, 0, 15.0, 0.0, 10.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                    // Air-only
                    ATTACK(fighter, 5, 1, Hash40::new("top"), 0.0, 367, 100, 10, 0, 15.0, 0.0, 10.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                }
            }
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(fighter, 2.0/(46.0-30.0));
            }
        }
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        // Reel-in hit 2
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(fighter, 0.5);
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                ATTACK(fighter, 0, 1, Hash40::new("haver"), 15.0, 20, 100, 60, 0, 10.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
                ATTACK(fighter, 1, 1, Hash40::new("haver"), 15.0, 20, 100, 60, 0, 10.5, -0.5, 35.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
                AttackModule::set_add_reaction_frame(boma, 0, 15.0, false);
                AttackModule::set_add_reaction_frame(boma, 1, 15.0, false);
            }
        }
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        // Reel-in hit 2
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(fighter, 2.0);
            }
            else{
                FT_MOTION_RATE(fighter, 1.1);
            }
        }
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(fighter, 1.0);
            }
        }
    }
    frame(lua_state, 85.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "master", script = "effect_attacks4lw" , category = ACMD_EFFECT , low_priority)]
unsafe fn master_attack_s4_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -20, 18, 13.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 17, 13.25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(fighter, Hash40::new("master_spear_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FLW_POS(fighter, Hash40::new("master_spear_aura_particle"), Hash40::new("haver"), -8, 6, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("master_spearflare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 2);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("master_smash_s_wind"), Hash40::new("top"), 0, 1, 34, 16, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            
        }
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("master_smash_s_speedline"), Hash40::new("haver"), 0, 26, 0, -90, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("master_smash_s_line"), Hash40::new("haver"), 0, 26, 0, -90, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(fighter, Hash40::new("master_smash_s_flash"), Hash40::new("haver"), 0, 28, -1.25, 0, 0, 0, 1.3, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("master_smash_s_line"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("master_smash_s_speedline"), -1);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("master_smash_s_line"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("master_smash_s_wind"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
                EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
            }
        }
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
                EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
                EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
                EFFECT_FOLLOW(fighter, Hash40::new("master_axe_hold_end"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 1.0, true);
                LAST_EFFECT_SET_RATE(fighter, 0.8);
                EFFECT_FOLLOW(fighter, Hash40::new("master_axe_slash_particle"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 0.75, true);
                EFFECT_FOLLOW(fighter, Hash40::new("master_wire_hit"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 1.0, true);
            }
        }
    }
}


#[acmd_script( agent = "master", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn master_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, Hash40::new("attack_lw4"), false, -1.0);
    }
    frame(lua_state, 3.0);
    app::sv_animcmd::execute(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(fighter) {
            let frame = WorkModule::get_float(boma, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
            ArticleModule::change_motion(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, Hash40::new("attack_lw4"), true, 3.0);
        }
    }
    frame(lua_state, 5.0);
        FT_MOTION_RATE(fighter, 1.3);
    frame(lua_state, 15.0);
        FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 23.0, 63, 66, 0, 74, 3.7, 0.0, 3.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 23.0, 63, 66, 0, 74, 4.8, 0.0, 13.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 23.0, 63, 66, 0, 74, 3.7, 0.0, 3.5, -11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 23.0, 63, 66, 0, 74, 4.8, 0.0, 13.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 78.0);

    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE,app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}


pub fn install() {
    install_acmd_scripts!(
        master_attack_s4_charge_effect,
        master_attack_s4_hi_game,
        master_attack_s4_hi_effect,
        master_attack_s4_s_game,
        master_attack_s4_s_effect,
        master_attack_s4_lw_game,
        master_attack_s4_lw_effect,
        master_attack_lw4_game,
    );
}

