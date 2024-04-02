use super::*;

unsafe extern "C" fn effect_attacks4charge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(agent, Hash40::new("master_spear_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent, 0.3);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FLW_POS(agent, Hash40::new("master_spear_aura_particle"), Hash40::new("haver"), -8, 6, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent, 0.3);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.3);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.3);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 24, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, true);
    }
}

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0);
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){
            VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 3.5);
        }
        else{
            FT_MOTION_RATE(agent, 0.800);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 1.0);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        // Reel-in hit 1
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 270, 100, 10, 0, 3.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 4.0, 270, 100, 10, 0, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            // Air-only
            ATTACK(agent, 2, 0, Hash40::new("haver"), 4.0, 367, 100, 50, 0, 3.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("haver"), 4.0, 367, 100, 50, 0, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        // Normal fsmash
        else{
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 90, 0, 45, 4.5, 0.0, 5.5, 6.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 13.0, 361, 90, 0, 45, 2.0, 0.0, 3.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("haver"), 19.0, 36, 90, 0, 30, 3.0, -0.5, 16.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("haver"), 19.0, 36, 90, 0, 30, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 4, 0, Hash40::new("shoulderr"), 11.0, 36, 90, 0, 50, 2.5, -0.5, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 26.0);
    for _ in 0..4{
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                    // Ground-only
                    ATTACK(agent, 4, 1, Hash40::new("top"), 0.0, 365, 100, 10, 0, 15.0, 0.0, 10.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                    // Air-only
                    ATTACK(agent, 5, 1, Hash40::new("top"), 0.0, 367, 100, 10, 0, 15.0, 0.0, 10.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                }
            }
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(agent, 2.0/(46.0-30.0));
            }
        }
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        // Reel-in hit 2
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 0.5);
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                ATTACK(agent, 0, 1, Hash40::new("haver"), 15.0, 20, 100, 60, 0, 10.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 1, Hash40::new("haver"), 15.0, 20, 100, 60, 0, 10.5, -0.5, 35.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
                AttackModule::set_add_reaction_frame(boma, 0, 15.0, false);
                AttackModule::set_add_reaction_frame(boma, 1, 15.0, false);
            }
        }
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        // Reel-in hit 2
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(agent, 2.0);
            }
            else{
                FT_MOTION_RATE(agent, 1.1);
            }
        }
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(agent, 1.0);
            }
        }
    }
    frame(lua_state, 85.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -20, 18, 13.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 17, 13.25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(agent, Hash40::new("master_spear_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FLW_POS(agent, Hash40::new("master_spear_aura_particle"), Hash40::new("haver"), -8, 6, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_spearflare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 2);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("master_smash_s_wind"), Hash40::new("top"), 0, 10, 35, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){

        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_smash_s_speedline"), Hash40::new("haver"), 0, 26, 0, -90, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("master_smash_s_line"), Hash40::new("haver"), 0, 26, 0, -90, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("master_smash_s_flash"), Hash40::new("haver"), 0, 28, -1.25, 0, 0, 0, 1.3, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("master_smash_s_line"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("master_smash_s_speedline"), -1);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_smash_s_line"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_smash_s_wind"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
            }
        }
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
                EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
                EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold_end"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 1.0, true);
                LAST_EFFECT_SET_RATE(agent, 0.8);
                EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash_particle"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 0.75, true);
                EFFECT_FOLLOW(agent, Hash40::new("master_wire_hit"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 1.0, true);
            }
        }
    }
}

unsafe extern "C" fn expression_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_INSTANCE_WORK_ID_FLAG_SWORD_OFF_EFFECT_AURA);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    app::sv_animcmd::execute(lua_state, 14.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new_raw(0x1782d1b9f2), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        RUMBLE_HIT(agent, Hash40::new("rbkind_pierces"), 0);
        RUMBLE_HIT(agent, Hash40::new("rbkind_pierces"), 1);
        RUMBLE_HIT(agent, Hash40::new("rbkind_piercel"), 2);
        RUMBLE_HIT(agent, Hash40::new("rbkind_piercel"), 3);
        RUMBLE_HIT(agent, Hash40::new("rbkind_pierces"), 4);
    }
    frame(lua_state, 44.0);
    frame(lua_state, 46.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
        }
    }
    frame(lua_state, 85.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MASTER_INSTANCE_WORK_ID_FLAG_SWORD_REQ_EFFECT_AURA);
    }
}

unsafe extern "C" fn game_attacks4hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0);
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){
            VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 3.5);
        }
        else{
            FT_MOTION_RATE(agent, 0.800);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 1.0);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        // Reel-in hit 1
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 270, 100, 10, 0, 3.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 4.0, 270, 100, 10, 0, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            // Air-only
            ATTACK(agent, 2, 0, Hash40::new("haver"), 4.0, 367, 100, 50, 0, 3.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("haver"), 4.0, 367, 100, 50, 0, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        // Normal fsmash
        else{
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 90, 0, 45, 4.5, 0.0, 5.5, 6.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 13.0, 361, 90, 0, 45, 2.0, 0.0, 3.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("haver"), 19.0, 36, 90, 0, 30, 3.0, -0.5, 16.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("haver"), 19.0, 36, 90, 0, 30, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 4, 0, Hash40::new("shoulderr"), 11.0, 36, 90, 0, 50, 2.5, -0.5, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 26.0);
    for _ in 0..4{
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                    // Ground-only
                    ATTACK(agent, 4, 1, Hash40::new("top"), 0.0, 365, 100, 10, 0, 15.0, 0.0, 10.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                    // Air-only
                    ATTACK(agent, 5, 1, Hash40::new("top"), 0.0, 367, 100, 10, 0, 15.0, 0.0, 10.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                }
            }
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(agent, 2.0/(46.0-30.0));
            }
        }
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        // Reel-in hit 2
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 0.5);
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                ATTACK(agent, 0, 1, Hash40::new("haver"), 15.0, 20, 100, 60, 0, 10.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 1, Hash40::new("haver"), 15.0, 20, 100, 60, 0, 10.5, -0.5, 35.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
                AttackModule::set_add_reaction_frame(boma, 0, 15.0, false);
                AttackModule::set_add_reaction_frame(boma, 1, 15.0, false);
            }
        }
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        // Reel-in hit 2
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(agent, 2.0);
            }
            else{
                FT_MOTION_RATE(agent, 1.1);
            }
        }
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(agent, 1.0);
            }
        }
    }
    frame(lua_state, 85.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_attacks4hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -20, 18, 13.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 17, 13.25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(agent, Hash40::new("master_spear_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FLW_POS(agent, Hash40::new("master_spear_aura_particle"), Hash40::new("haver"), -8, 6, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_spearflare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 2);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("master_smash_s_wind"), Hash40::new("top"), 0, 19, 34.5, -18, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){

        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_smash_s_speedline"), Hash40::new("haver"), 0, 26, 0, -90, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("master_smash_s_line"), Hash40::new("haver"), 0, 26, 0, -90, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("master_smash_s_flash"), Hash40::new("haver"), 0, 28, -1.25, 0, 0, 0, 1.3, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("master_smash_s_line"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("master_smash_s_speedline"), -1);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_smash_s_line"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_smash_s_wind"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
            }
        }
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
                EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
                EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold_end"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 1.0, true);
                LAST_EFFECT_SET_RATE(agent, 0.8);
                EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash_particle"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 0.75, true);
                EFFECT_FOLLOW(agent, Hash40::new("master_wire_hit"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 1.0, true);
            }
        }
    }
}

unsafe extern "C" fn game_attacks4lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0);
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){
            VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 3.5);
        }
        else{
            FT_MOTION_RATE(agent, 0.800);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 1.0);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        // Reel-in hit 1
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 270, 100, 10, 0, 3.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 4.0, 270, 100, 10, 0, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            // Air-only
            ATTACK(agent, 2, 0, Hash40::new("haver"), 4.0, 367, 100, 50, 0, 3.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("haver"), 4.0, 367, 100, 50, 0, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        // Normal fsmash
        else{
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 90, 0, 45, 4.5, 0.0, 5.5, 6.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 13.0, 361, 90, 0, 45, 2.0, 0.0, 3.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("haver"), 19.0, 36, 90, 0, 30, 3.0, -0.5, 16.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("haver"), 19.0, 36, 90, 0, 30, 2.5, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 4, 0, Hash40::new("shoulderr"), 11.0, 36, 90, 0, 50, 2.5, -0.5, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 26.0);
    for _ in 0..4{
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                    // Ground-only
                    ATTACK(agent, 4, 1, Hash40::new("top"), 0.0, 365, 100, 10, 0, 15.0, 0.0, 10.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                    // Air-only
                    ATTACK(agent, 5, 1, Hash40::new("top"), 0.0, 367, 100, 10, 0, 15.0, 0.0, 10.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, true, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                }
            }
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(agent, 2.0/(46.0-30.0));
            }
        }
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        // Reel-in hit 2
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            FT_MOTION_RATE(agent, 0.5);
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                ATTACK(agent, 0, 1, Hash40::new("haver"), 15.0, 20, 100, 60, 0, 10.0, -0.5, 14.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 1, Hash40::new("haver"), 15.0, 20, 100, 60, 0, 10.5, -0.5, 35.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
                AttackModule::set_add_reaction_frame(boma, 0, 15.0, false);
                AttackModule::set_add_reaction_frame(boma, 1, 15.0, false);
            }
        }
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        // Reel-in hit 2
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(agent, 2.0);
            }
            else{
                FT_MOTION_RATE(agent, 1.1);
            }
        }
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(agent, 1.0);
            }
        }
    }
    frame(lua_state, 85.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_attacks4lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -20, 18, 13.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else{
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 17, 13.25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            EFFECT_FOLLOW(agent, Hash40::new("master_spear_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FLW_POS(agent, Hash40::new("master_spear_aura_particle"), Hash40::new("haver"), -8, 6, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold2"), Hash40::new("haver"), 0.0, 22.0, 0.0, 0, 0, 0, 1.0, true);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("master_spearflare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 2);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("master_smash_s_wind"), Hash40::new("top"), 0, 1, 34, 16, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){

        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("master_smash_s_speedline"), Hash40::new("haver"), 0, 26, 0, -90, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("master_smash_s_line"), Hash40::new("haver"), 0, 26, 0, -90, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("master_smash_s_flash"), Hash40::new("haver"), 0, 28, -1.25, 0, 0, 0, 1.3, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("master_smash_s_line"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("master_smash_s_speedline"), -1);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("master_smash_s_line"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("master_smash_s_wind"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
            }
        }
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK){
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
                EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
                EFFECT_FOLLOW(agent, Hash40::new("master_axe_hold_end"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 1.0, true);
                LAST_EFFECT_SET_RATE(agent, 0.8);
                EFFECT_FOLLOW(agent, Hash40::new("master_axe_slash_particle"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 0.75, true);
                EFFECT_FOLLOW(agent, Hash40::new("master_wire_hit"), Hash40::new("top"), 0, 10, 24, 0.0, 0, 0, 1.0, true);
            }
        }
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, Hash40::new("attack_lw4"), false, -1.0);
    }
    frame(lua_state, 3.0);
    app::sv_animcmd::execute(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(agent) {
            let frame = WorkModule::get_float(boma, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
            ArticleModule::change_motion(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, Hash40::new("attack_lw4"), true, 3.0);
        }
    }
    frame(lua_state, 5.0);
        FT_MOTION_RATE(agent, 1.3);
    frame(lua_state, 15.0);
        FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 23.0, 63, 66, 0, 74, 3.7, 0.0, 3.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 23.0, 63, 66, 0, 74, 4.8, 0.0, 13.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 23.0, 63, 66, 0, 74, 3.7, 0.0, 3.5, -11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 23.0, 63, 66, 0, 74, 4.8, 0.0, 13.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 78.0);

    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE,app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_attacks4charge", effect_attacks4charge);

    agent.acmd("game_attacks4", game_attacks4);
    agent.acmd("effect_attacks4", effect_attacks4);
    agent.acmd("expression_attacks4", expression_attacks4);
    agent.acmd("game_attacks4hi", game_attacks4hi);
    agent.acmd("effect_attacks4hi", effect_attacks4hi);
    agent.acmd("expression_attacks4hi", expression_attacks4);
    agent.acmd("game_attacks4lw", game_attacks4lw);
    agent.acmd("effect_attacks4lw", effect_attacks4lw);
    agent.acmd("expression_attacks4lw", expression_attacks4);

    agent.acmd("game_attacklw4", game_attacklw4);
}
