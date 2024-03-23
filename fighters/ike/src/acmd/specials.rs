use super::*;

unsafe extern "C" fn game_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 275, 52, 0, 45, 4.0, 0.0, -4.0, 8.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 13.0, 0.0, 8.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 10.0, 0.0, 25.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 275, 52, 0, 45, 4.0, 0.0, -4.0, 8.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 15.0, 0.0, 8.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 10.0, 0.0, 25.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn game_specialairnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 275, 52, 0, 45, 4.0, 0.0, -4.0, 8.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 13.0, 0.0, 8.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 10.0, 0.0, 25.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 275, 52, 0, 45, 4.0, 0.0, -4.0, 8.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 15.0, 0.0, 8.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 80, 110, 0, 40, 10.0, 0.0, 25.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.500);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CHARGE);
    }
}

unsafe extern "C" fn game_specialairsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.500);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CHARGE);
    }
}

unsafe extern "C" fn effect_specialshold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ike_iaigiri_hold"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 7, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 0.5, 3, 10, 3, 0, 0, 0, true);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 0.75, 3, 10, 3, 0, 0, 0, true);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 0.5, 3, 10, 3, 0, 0, 0, true);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 0.75, 3, 10, 3, 0, 0, 0, true);
    }
    wait(lua_state, 2.0);
}

unsafe extern "C" fn game_specialsdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 0, 0, 0, 4.0, 0.0, 4.1, -1.0, Some(0.0), Some(4.1), Some(-15.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
}

unsafe extern "C" fn effect_specialsdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 4, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ike_iaigiri"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 5.0);
    for _ in 0..3{
        if is_excute(agent) {
            if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
                EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            }
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 4.0);
    }
}

unsafe extern "C" fn sound_specialsdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            PLAY_SE(agent, Hash40::new("vc_ike_appeal02"));
        }
        else{
            PLAY_SE(agent, Hash40::new("vc_ike_special_s01"));
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ike_special_s02"));
    }
}

unsafe extern "C" fn game_specialairsdash(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        agent.select_cliff_hangdata_from_name("special_s");
    }
}

unsafe extern "C" fn game_specialsattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    // Instakill
    if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
        frame(lua_state, 1.0);
        if is_excute(agent) {
            agent.select_cliff_hangdata_from_name("special_s");
            AttackModule::clear_all(boma);
            ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 100, 400, 1, 6.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(-8.0), 10.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            //ATTACK(fighter, 1, 1, Hash40::new("top"), 0.0, 361, 0, 0, 0, 6.0, 0.0, 7.0, 10.0, Some(0.0), Some(5.0), Some(-10.0), 5.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.05);
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 100, 400, 1, 6.0, 0.0, 8.0, 15.0, Some(0.0), Some(8.0), Some(-10.0), 10.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(lua_state, 3.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 8.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        }
        frame(lua_state, 12.0);
        if is_excute(agent) {
            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                FT_MOTION_RATE(agent, 2.0);
            }
        }
    }
    // Normal Quickdraw
    else {
        frame(lua_state, 1.0);
        if is_excute(agent) {
            agent.select_cliff_hangdata_from_name("special_s");
            AttackModule::clear_all(boma);
            FT_MOTION_RATE(agent, 3.0/(1.1-1.0));
        }
        frame(lua_state, 1.1);
        if is_excute(agent) {
            FT_MOTION_RATE(agent, 1.0/(1.3-1.1));
        }
        frame(lua_state, 1.3);
        if is_excute(agent) {
            FT_MOTION_RATE(agent, 1.0/(2.0-1.3));
            ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 7.0, 70, 130, 0, 40, 2.75, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 7.0, 70, 130, 0, 40, 3.8, 0.0, 1.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("sword"), 7.0, 70, 130, 0, 40, 3.8, 0.0, 6.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("sword"), 7.0, 70, 130, 0, 40, 3.8, 0.0, 11.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            FT_MOTION_RATE(agent, 1.0);
        }
        frame(lua_state, 3.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            FT_MOTION_RATE(agent, 1.0);
        }
        frame(lua_state, 8.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        }
        frame(lua_state, 15.0);
        if is_excute(agent) {
            FT_MOTION_RATE(agent, 17.0/(35.0-15.0));
        }
        frame(lua_state, 35.0);
        if is_excute(agent) {
            FT_MOTION_RATE(agent, 1.0);
        }
    }
}

unsafe extern "C" fn effect_specialsattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            /* 
            let cbm_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2}; // Brightness vector
            let cbm_vec2 = Vector4f{ /* Red */ x: 0.125, /* Green */ y: 0.4, /* Blue */ z: 1.0, /* Alpha */ w: 0.0}; // Diffuse vector
            ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_vec1, /* Diffuse */ &cbm_vec2, 0.0, 0.0, /*Fadein time*/ 15, /* Display Color */ true);
            */
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ike_sword"), true, true);
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT_OFF_KIND(agent, Hash40::new("ike_volcano_hold"), false, false);
            EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            ColorBlendModule::cancel_main_color(boma, 0);
        }
    }
}

unsafe extern "C" fn game_specialairsattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        agent.select_cliff_hangdata_from_name("special_s");
        FT_MOTION_RATE(agent, 3.0/(1.1-1.0));
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0/(1.3-1.1));
    }
    frame(lua_state, 1.3);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0/(2.0-1.3));
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 7.0, 70, 130, 0, 40, 2.75, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), 7.0, 70, 130, 0, 40, 3.8, 0.0, 1.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword"), 7.0, 70, 130, 0, 40, 3.8, 0.0, 6.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword"), 7.0, 70, 130, 0, 40, 3.8, 0.0, 11.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 17.0/(35.0-15.0));
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn effect_specialairsattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, false);
        //LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ike_sword"), true, true);
    }
}

unsafe extern "C" fn expression_specialairsattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 3, 130, 300, 1, 0, 12, 24, 24);
        crate::sv_animcmd::AREA_WIND_2ND(lua_state);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            RUMBLE_HIT(agent, Hash40::new("rbkind_slash_critical"), 0);
        }
    }
    frame(lua_state, 1.3);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            let cbm_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2}; // Brightness vector
            let cbm_vec2 = Vector4f{ /* Red */ x: 0.125, /* Green */ y: 0.4, /* Blue */ z: 1.0, /* Alpha */ w: 0.0}; // Diffuse vector
            ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_vec1, /* Diffuse */ &cbm_vec2, 0.0, 0.0, /*Fadein time*/ 15, /* Display Color */ true);
        }
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(lua_state, 4.0);
    for _ in 0..2{
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 4.0);
    }
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT_OFF_KIND(agent, Hash40::new("ike_volcano_hold"), false, false);
            ColorBlendModule::cancel_main_color(boma, 0);
        }
    }
}

unsafe extern "C" fn game_specialairsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.select_cliff_hangdata_from_name("special_s");
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_END_NO_LANDING);
    }
}

unsafe extern "C" fn game_specialhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.82);
    
}

unsafe extern "C" fn game_specialairhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
        WorkModule::off_flag(boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SWORD_FINAL);
        ArticleModule::generate_article(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, 0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 88, 100, 150, 0, 3.5, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 94, 100, 150, 0, 1.5, 0.0, 3.0, 15.0, Some(0.0), Some(7.0), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 96, 100, 150, 0, 3.5, 0.0, 5.0, 19.0, Some(0.0), Some(5.0), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 6.0, 86, 100, 145, 0, 3.5, 0.0, 13.5, 7.0, Some(0.0), Some(13.5), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 6.0, 94, 100, 145, 0, 1.5, 0.0, 11.5, 15.0, Some(0.0), Some(15.5), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("top"), 6.0, 96, 100, 140, 0, 3.5, 0.0, 13.5, 19.0, Some(0.0), Some(13.5), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_CONTROL);
        WorkModule::set_float(boma, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
        WorkModule::set_float(boma, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword"), 1.0, 367, 100, 50, 0, 4.0, 0.0, 4.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), 1.0, 367, 100, 50, 0, 4.0, 0.0, 10.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD,  app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
        camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, -20);
        KineticModule::clear_speed_all(boma);
        ADD_SPEED_NO_LIMIT(agent, 0, -6);
        ATTACK(agent, 0, 0, Hash40::new("sword"), 3.0, 70, 30, 0, 120, 6.5, 0.0, 6.8, -1.0, Some(0.0), Some(6.8), Some(-12.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), 3.0, 270, 100, 165, 0, 4.8, 0.0, 6.8, -1.0, Some(0.0), Some(6.8), Some(-13.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword"), 3.0, 70, 50, 0, 0, 4.8, 0.0, 6.8, -1.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), 3.0, 270, 100, 150, 0, 4.8, 0.0, 6.8, -1.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

unsafe extern "C" fn game_specialairhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
        WorkModule::off_flag(boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SWORD_FINAL);
        ArticleModule::generate_article(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, 0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 88, 100, 150, 0, 3.5, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 94, 100, 150, 0, 1.5, 0.0, 3.0, 15.0, Some(0.0), Some(7.0), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 96, 100, 150, 0, 3.5, 0.0, 5.0, 19.0, Some(0.0), Some(5.0), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 6.0, 86, 100, 145, 0, 3.5, 0.0, 13.5, 7.0, Some(0.0), Some(13.5), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 6.0, 94, 100, 145, 0, 1.5, 0.0, 11.5, 15.0, Some(0.0), Some(15.5), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("top"), 6.0, 96, 100, 140, 0, 3.5, 0.0, 13.5, 19.0, Some(0.0), Some(13.5), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_CONTROL);
        WorkModule::set_float(boma, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
        WorkModule::set_float(boma, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword"), 1.0, 367, 100, 50, 0, 4.0, 0.0, 4.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), 1.0, 367, 100, 50, 0, 4.0, 0.0, 10.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD,  app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, -20);
        KineticModule::clear_speed_all(boma);
        ADD_SPEED_NO_LIMIT(agent, 0, -6);
        ATTACK(agent, 0, 0, Hash40::new("sword"), 4.5, 70, 30, 0, 120, 6.5, 0.0, 6.8, -1.0, Some(0.0), Some(6.8), Some(-12.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), 4.5, 270, 120, 0, 30, 4.8, 0.0, 6.8, -1.0, Some(0.0), Some(6.8), Some(-13.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword"), 4.5, 70, 50, 0, 0, 4.8, 0.0, 6.8, -1.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), 4.5, 270, 120, 0, 25, 4.8, 0.0, 6.8, -1.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

unsafe extern "C" fn game_specialhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 50, 90, 0, 50, 10.0, 0.0, 6.0, 11.8, Some(0.0), Some(11.0), Some(11.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {;
        agent.acmd("game_specialnend", game_specialnend);
        agent.acmd("game_specialairnend", game_specialairnend);
        agent.acmd("game_specialsstart", game_specialsstart);
        agent.acmd("game_specialairsstart", game_specialairsstart);
        agent.acmd("effect_specialshold", effect_specialshold);
        agent.acmd("game_specialsdash", game_specialsdash);
        agent.acmd("effect_specialsdash", effect_specialsdash);
        agent.acmd("sound_specialsdash", sound_specialsdash);
        agent.acmd("game_specialairsdash", game_specialairsdash);
        agent.acmd("game_specialsattack", game_specialsattack);
        agent.acmd("effect_specialsattack", effect_specialsattack);
        agent.acmd("game_specialairsattack", game_specialairsattack);
        agent.acmd("effect_specialairsattack", effect_specialairsattack);
        agent.acmd("expression_specialsattack", expression_specialsattack);
        agent.acmd("expression_specialairsattack", expression_specialairsattack);
        agent.acmd("effect_specialsend", effect_specialsend);
        agent.acmd("game_specialairsend", game_specialairsend);
        agent.acmd("game_specialhi1", game_specialhi1);
        agent.acmd("game_specialhi2", game_specialhi2);
        agent.acmd("game_specialairhi2", game_specialairhi2);
        agent.acmd("game_specialhi3", game_specialhi3);
        agent.acmd("game_specialairhi2", game_specialairhi2);
        agent.acmd("game_specialhi3", game_specialhi3);
        agent.acmd("game_specialairhi3", game_specialairhi3);
        agent.acmd("game_specialhi4", game_specialhi4);
}
