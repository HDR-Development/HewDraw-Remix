use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 4.5);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 8.0, 5.0);
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(agent, 8.0, 15.0, 4.0);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 16.0);
    let is_dragonized = WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L);
    let is_doubledragon = WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 0;
    if is_dragonized && is_doubledragon {
        FT_MOTION_RATE_RANGE(agent, 16.0, 18.0, 7.0);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 4.5);
        if is_doubledragon {
            if is_dragonized {
                // powered double dragon
                ATTACK(agent, 1, 0, Hash40::new("throw"), 25.0, 361, 57, 0, 57, 6.0, 0.0, 0.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
                AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_heavy_hit_l"));
            }
        }
        ATTACK(agent, 3, 0, Hash40::new("arml2"), 12.0, 361, 94, 0, 48, 3.0, 2.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 4, 0, Hash40::new("arml2"), 12.0, 361, 94, 0, 48, 3.5, 5.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    for i in 0..5 {
        if is_doubledragon {
            if is_dragonized {
                // powered double dragon
                let offsets = [11.0, 18.0, 24.0, 31.0, 36.0];
                ATTACK(agent, 0, 0, Hash40::new("throw"), 20.0, 361, 57, 0, 57, 5.0, 0.0, 0.0, -offsets[i], Some(0.0), Some(0.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
                AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_heavy_hit_l"));
            }
            else {
                // normal double dragon
                let offsets = [18.0, 22.0, 25.0, 29.0, 31.0];
                ATTACK(agent, 0, 0, Hash40::new("pl1_gimmickc"), 15.0, 361, 94, 0, 48, 1.5, 0.0, 0.0, 0.0, Some(offsets[i]), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
                ATTACK(agent, 1, 0, Hash40::new("pr1_gimmickc"), 15.0, 361, 94, 0, 48, 1.5, 0.0, 0.0, 0.0, Some(offsets[i]), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            }
        }
        else {
            if is_dragonized {
                // power dragon
                let offsets = [23.5, 29.0, 35.0, 40.0, 41.5];
                ATTACK(agent, 0, 0, Hash40::new("pl1_gimmickc"), 16.0, 361, 94, 0, 48, 3.3, 0.0, 0.0, 0.0, Some(offsets[i]), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
                AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_kick_hit_l"));
            }
            else {
                // single dragon
                let offsets = [18.0, 22.0, 25.0, 29.0, 31.0];
                ATTACK(agent, 0, 0, Hash40::new("pl1_gimmickc"), 13.0, 361, 94, 0, 48, 1.5, 0.0, 0.0, 0.0, Some(offsets[i]), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            }
        }
        wait(lua_state, 1.0);
    }
    if is_excute(agent) {
        if !(is_doubledragon && is_dragonized) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 39.0);
    FT_MOTION_RATE_RANGE(agent, 39.0, 48.0, 16.0);
    frame(lua_state, 48.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("sys_sscope_hold"), false, false);
    }
    frame(lua_state, 18.0);
    let is_dragonized = WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L);
    let is_doubledragon = WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 0;
    let mut effect_l: u64 = 0;
    let mut effect_r: u64 = 0;
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if is_doubledragon {
            if is_dragonized {
                // powered double dragon
                EFFECT_FOLLOW(agent, Hash40::new("tantan_wepon_shot1"), Hash40::new("pl1_gimmickc"), -1.0, 0, 0, 0, 0, -90, 1.0, true);
                LAST_EFFECT_SET_SCALE_W(agent, 1.0, 0.75, 1.0);
                EFFECT(agent, Hash40::new("tantan_final_shot"), Hash40::new("pl1_muzzle"), 0, 0, 0, 0, 0, 90, 1, 0, 0, 0, 0, 0, 0, true);
                EFFECT_FOLLOW(agent, Hash40::new("tantan_final_punch"), Hash40::new("throw"), -0.045, 0, 0, 0, 90, 0, 1, true);
                LAST_EFFECT_SET_RATE(agent, 1.5);
                effect_l = EffectModule::get_last_handle(boma);
            }
            else {
                // normal double dragon
                EFFECT_FOLLOW(agent, Hash40::new("tantan_dragon_beam1_body"), Hash40::new("pl1_gimmickc"), -2.5, 0, 0, 0, 0, 180, 1.0, true);
                LAST_EFFECT_SET_RATE(agent, 0.75);
                effect_l = EffectModule::get_last_handle(boma);
                EFFECT_FOLLOW(agent, Hash40::new("tantan_dragon_beam1_body"), Hash40::new("pr1_gimmickc"), -2.5, 0, 0, 0, 0, 180, 1.0, true);
                LAST_EFFECT_SET_RATE(agent, 0.75);
                effect_r = EffectModule::get_last_handle(boma);
            }
        }
        else {
            if is_dragonized {
                // power dragon
                EFFECT_FOLLOW(agent, Hash40::new("tantan_dragon_beam1_body"), Hash40::new("pl1_gimmickc"), -2.5, 0, 0, 0, 0, 180, 1.15, true);
                LAST_EFFECT_SET_RATE(agent, 0.75);
                effect_l = EffectModule::get_last_handle(boma);
                EFFECT_FOLLOW(agent, Hash40::new("tantan_wepon_shot1"), Hash40::new("pl1_gimmickc"), -1.0, 0, 0, 0, 0, -90, 1.0, true);
                LAST_EFFECT_SET_SCALE_W(agent, 1.0, 0.75, 1.0);
            }
            else {
                // single dragon
                EFFECT_FOLLOW(agent, Hash40::new("tantan_dragon_beam1_body"), Hash40::new("pl1_gimmickc"), -2.5, 0, 0, 0, 0, 180, 1.0, true);
                LAST_EFFECT_SET_RATE(agent, 0.75);
                effect_l = EffectModule::get_last_handle(boma);
            }
        }
    }
    if is_dragonized && is_doubledragon {
        frame(lua_state, 24.0);
        if is_excute(agent) {
            EffectModule::set_rate(boma, effect_l as u32, 0.75);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            EffectModule::set_rate(boma, effect_l as u32, 1.375);
            EFFECT_DETACH_KIND(agent, Hash40::new("tantan_final_punch"), -1);
        }
    }
    else {
        frame(lua_state, 19.0);
        if is_excute(agent) {
            EffectModule::set_rate(boma, effect_l as u32, 0.375);
            EffectModule::set_rate(boma, effect_r as u32, 0.375);
        }
        frame(lua_state, 24.0);
        if is_excute(agent) {
            EffectModule::set_rate(boma, effect_l as u32, 1.1);
            EffectModule::set_rate(boma, effect_r as u32, 1.1);
        }
    }
}

unsafe extern "C" fn sound_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_smash_start"));
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_tantan_attack01_beam_ready"));
    }
    frame(lua_state, 16.0);
    let is_dragonized = WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L);
    let is_doubledragon = WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 0;
    if is_excute(agent) {
        if is_dragonized && is_doubledragon {
            PLAY_STATUS(agent, Hash40::new("se_tantan_final02"));
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if is_dragonized && is_doubledragon {
            let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("vc_tantan_final02"), true, false, false, false, app::enSEType(0));
            SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
        }
        else {
            PLAY_SEQUENCE(agent, Hash40::new("seq_tantan_rnd_punch_long"));
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if is_dragonized {
            PLAY_SE(agent, Hash40::new("se_tantan_attack01_beam_max"));
        }
        else {
            PLAY_SE(agent, Hash40::new("se_tantan_attack01_beam"));
        }
    }
}

unsafe extern "C" fn expression_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        //rbkind_76_dragonbeam
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_dragonbeam2"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn effect_attacks4charge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let is_doubledragon = WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 0;
    EFFECT_FOLLOW(agent, Hash40::new("sys_sscope_hold"), Hash40::new("pl1_muzzle"), 1, 0.5, 0, 0, 0, 0, 0.3, true);
    if is_doubledragon {
        EFFECT_FOLLOW(agent, Hash40::new("sys_sscope_hold"), Hash40::new("pr1_muzzle"), 1, 0.5, 0, 0, 0, 0, 0.3, true);
    }
    for i in 1..i32::MAX{
        frame(lua_state, 5.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
        }
        wait(lua_state, 5.0);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("pl1_gimmickc"), 3, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_attacks4charge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_smash_start"));
    }
}

unsafe extern "C" fn expression_attacks4charge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent,*MA_MSC_CMD_PHYSICS_START_CHARGE, 0.8, 0.6, -1, 0.8, 0.8, -1, Hash40::new("invalid"));
        smash::app::sv_module_access::physics(lua_state);
        agent.pop_lua_stack(1);
        
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 61.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Right arm charging punch//

unsafe extern "C" fn sound_attacks4charger(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_smash_start"));
    }
    if armType==1 {
        frame(lua_state, 6.0);
        if is_excute(agent) {
            PLAY_STATUS(agent, Hash40::new("se_tantan_attack02_charge"));
        }
    }
    else if armType==2 {
        frame(lua_state, 2.0);
        if is_excute(agent) {
            PLAY_STATUS(agent, Hash40::new("se_tantan_attack03_charge"));
        }
    }
    else{
        frame(lua_state, 8.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_tantan_attack01_beam_ready"));
        }
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE_RANGE(agent, 4.0, 6.0, 6.0);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_A, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 17.0, 82, 88, 0, 48, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 17.0, 82, 88, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footl"), 17.0, 82, 88, 0, 48, 5.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_A, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        ATTACK(agent, 0, 0, Hash40::new("legl"), 14.0, 82, 100, 0, 48, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 14.0, 82, 100, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footl"), 14.0, 82, 100, 0, 48, 5.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 11.0, 82, 111, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 11.0, 82, 111, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footl"), 11.0, 82, 111, 0, 48, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 4.5);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 4.5);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 40, 85, 0, 50, 4.5, 0.0, 2.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 40, 85, 0, 50, 4.5, 0.0, 2.5, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 15.0, 40, 82, 0, 50, 5.0, 0.0, 2.5, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("top"), 15.0, 40, 82, 0, 50, 5.0, 0.0, 2.5, -12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);
    agent.acmd("effect_attacks4", effect_attacks4, Priority::Low);
    agent.acmd("sound_attacks4", sound_attacks4, Priority::Low);
    agent.acmd("expression_attacks4", expression_attacks4, Priority::Low);
    agent.acmd("effect_attacks4charge", effect_attacks4charge, Priority::Low);
    agent.acmd("sound_attacks4charge", sound_attacks4charge, Priority::Low);
    agent.acmd("expression_attacks4charge", expression_attacks4charge, Priority::Low);

    agent.acmd("sound_attacks4charger", sound_attacks4charger, Priority::Low);
    agent.acmd("sound_attacks4chargerb", sound_attacks4charger, Priority::Low);

    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);
}