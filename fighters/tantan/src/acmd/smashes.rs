
use super::*;


#[acmd_script( agent = "tantan", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn tantan_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 4.0/(6.0-4.0));
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_A, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        //shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_B, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 14.0, 75, 95, 0, 48, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 16.0, 75, 95, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 16.0, 75, 95, 0, 48, 5.0, 0.0, 1.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_A, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 12.0, 82, 100, 0, 48, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 14.0, 82, 100, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 14.0, 82, 100, 0, 48, 5.0, 0.0, 1.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 10.0, 82, 103, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 12.0, 82, 103, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 12.0, 82, 103, 0, 48, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_B, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "tantan", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn tantan_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 4.5);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 4.5);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 40, 80, 0, 50, 4.5, 0.0, 2.5, 7.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 40, 80, 0, 50, 4.5, 0.0, 2.5, -7.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 30, 77, 0, 50, 5.0, 0.0, 2.5, 12.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 16.0, 30, 77, 0, 50, 5.0, 0.0, 2.5, -12.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "tantan", script = "game_attacks4", category = ACMD_GAME , low_priority)]
unsafe fn tantan_attack_s4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let mut is_dragonized = false;
    let mut is_doubledragon = false;
    let mut damage = 11.0;
    let mut sfx_level = *ATTACK_SOUND_LEVEL_M;
    let mut ranges = [18.0,22.0,25.0,29.0,31.0];
    let mut size = 1.3;

    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 4.5);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 4.5);

        is_dragonized = WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L);
        is_doubledragon = WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 0;
        if (is_doubledragon && is_dragonized) {
            damage = 16.0;
            sfx_level = *ATTACK_SOUND_LEVEL_L;
            ranges = [10.0,20.0,27.0,34.0,41.0];
            size = 5.0;

            ATTACK(fighter, 0, 0, Hash40::new("throw"), damage, 361, 75, 0, 60, 6.0, 0.0, 0.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sfx_level, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
        else if is_dragonized {
            damage = 13.0;
            sfx_level = *ATTACK_SOUND_LEVEL_L;
            ranges = [23.0,28.0,34.0,39.0,41.0];
            size = 2.8;
        }

    }
    let mut range = 0.0;
    for i in 0..5{
        range = ranges[i];
        if (is_doubledragon && is_dragonized) {
            ATTACK(fighter, 1, 0, Hash40::new("throw"), damage, 361, 75, 0, 60, size, 0.0, 0.0, -range, Some(0.0), Some(0.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sfx_level, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
        else{
            ATTACK(fighter, 2, 0, Hash40::new("pl1_gimmickc"), damage, 361, 75, 0, 60, size*1.2, 0.0, 0.0, 0.0, Some(range), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sfx_level, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);

            if is_doubledragon {
                ATTACK(fighter, 3, 0, Hash40::new("pr1_gimmickc"), damage, 361, 75, 0, 60, size*1.2, 0.0, 0.0, 0.0, Some(range), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sfx_level, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            }
        }
        AttackModule::disable_tip(boma);
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        if !(is_doubledragon && is_dragonized) {
            AttackModule::clear(boma,2,false);
            AttackModule::clear(boma,3,false);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "tantan", script = "effect_attacks4", category = ACMD_EFFECT , low_priority)]
unsafe fn tantan_attack_s4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let mut effect_l: u64 = 0;
    let mut effect_r: u64 = 0;
    let mut is_dragonized = false;
    let mut is_doubledragon = false;

    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }

    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_sscope_hold"),false,false);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        is_dragonized = WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L);
        is_doubledragon = WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 0;

        if is_dragonized && is_doubledragon {
            EFFECT(fighter, Hash40::new("tantan_final_shot"), Hash40::new("pl1_muzzle"), 0, 0, 0, 0, 0, 90, 1, 0, 0, 0, 0, 0, 0, true);

            //EFFECT_FOLLOW(fighter, Hash40::new("tantan_final_punch"), Hash40::new("pl1_muzzle_eff"), 3.0, 0, 0.0, 180, 0, 180, 0.75, false);
            EFFECT_FOLLOW(fighter, Hash40::new("tantan_final_punch"), Hash40::new("throw"), -0.045, 0, 0, 0, 90, 0, 1, true);
            LAST_EFFECT_SET_RATE(fighter,1.5);
            effect_l = EffectModule::get_last_handle(boma);
        }
        else {
            let size = if is_dragonized {1.15} else {1.0};
            EFFECT_FOLLOW(fighter, Hash40::new("tantan_dragon_beam1_body"), Hash40::new("pl1_gimmickc"), -2.5, 0, 0, 0, 0, 180, size, true);
            LAST_EFFECT_SET_RATE(fighter,0.75);
            effect_l = EffectModule::get_last_handle(boma);

            if is_dragonized {
                EFFECT_FOLLOW(fighter, Hash40::new("tantan_wepon_shot1"), Hash40::new("pl1_gimmickc"), -1.0, 0, 0, 0, 0, -90, 1.0, true);
                LAST_EFFECT_SET_SCALE_W(fighter, 1.0, 0.75, 1.0);
            }
            if is_doubledragon {
                EFFECT_FOLLOW(fighter, Hash40::new("tantan_dragon_beam1_body"), Hash40::new("pr1_gimmickc"), -2.5, 0, 0, 0, 0, 180, size, true);
                LAST_EFFECT_SET_RATE(fighter,0.75);
                effect_r = EffectModule::get_last_handle(boma);
            }
        }
    }
    if is_dragonized && is_doubledragon {
        frame(lua_state, 24.0);
        if is_excute(fighter) {
            EffectModule::set_rate(boma, effect_l as u32, 0.75);
        }
        frame(lua_state, 26.0);
        if is_excute(fighter) {
            EffectModule::set_rate(boma, effect_l as u32, 1.375);
            EFFECT_DETACH_KIND(fighter,Hash40::new("tantan_final_punch"),-1)
        }
    }
    else{
        frame(lua_state, 19.0);
        if is_excute(fighter) {
            EffectModule::set_rate(boma, effect_l as u32, 0.375);
            EffectModule::set_rate(boma, effect_r as u32, 0.375);
        }
        frame(lua_state, 24.0);
        if is_excute(fighter) {
            EffectModule::set_rate(boma, effect_l as u32, 1.1);
            EffectModule::set_rate(boma, effect_r as u32, 1.1);
        }
    }

}

#[acmd_script( agent = "tantan", script = "sound_attacks4", category = ACMD_SOUND , low_priority)]
unsafe fn tantan_attack_s4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let mut is_dragonized = false;
    let mut is_doubledragon = false;

    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_tantan_attack01_beam_ready"));
    }

    frame(lua_state, 16.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        is_dragonized = WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L);
        is_doubledragon = WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 0;
        if is_dragonized && is_doubledragon {
            PLAY_STATUS(fighter, Hash40::new("se_tantan_final02"));
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        is_dragonized = WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L);
        is_doubledragon = WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 0;
        if is_dragonized && is_doubledragon {
            PLAY_SE(fighter, Hash40::new("vc_tantan_final02"));
        }
        else {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_tantan_rnd_punch_long"));
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        let sfx = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE) {Hash40::new("se_tantan_attack01_beam_max")} else {Hash40::new("se_tantan_attack01_beam")};
        if is_dragonized && is_doubledragon {
        }
        else {
            PLAY_SE(fighter, sfx);
        }
    }
}

#[acmd_script( agent = "tantan", script = "expression_attacks4", category = ACMD_EXPRESSION , low_priority)]
unsafe fn tantan_attack_s4_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 17.0);
    if is_excute(fighter) {
        //rbkind_76_dragonbeam
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_dragonbeam2"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


#[acmd_script( agent = "tantan", script = "effect_attacks4charge", category = ACMD_EFFECT )]
unsafe fn tantan_attack_s4_charge_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    /* 
    let is_doubledragon = WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 0;
    EFFECT_FOLLOW(fighter, Hash40::new("sys_sscope_hold"), Hash40::new("pl1_muzzle"), 1, 0, 0, 0, 0, 0, 0.3,  true);
    if is_doubledragon {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sscope_hold"), Hash40::new("pr1_muzzle"), 1, 0, 0, 0, 0, 0, 0.3,  true);
    }*/

    for i in 1..i32::MAX{
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
        }
        wait(lua_state, 5.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("pl1_gimmickc"), 0, 2, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
    }
}
#[acmd_script( agent = "tantan", script = "sound_attacks4charge", category = ACMD_SOUND )]
unsafe fn tantan_attack_s4_charge_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    }
}
#[acmd_script( agent = "tantan", script = "expression_attacks4charge", category = ACMD_EXPRESSION )]
unsafe fn tantan_attack_s4_charge_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter,*MA_MSC_CMD_PHYSICS_START_CHARGE, 0.8, 0.6, -1, 0.8, 0.8, -1, Hash40::new("invalid"));
        smash::app::sv_module_access::physics(lua_state);
        fighter.pop_lua_stack(1);
        
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 61.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        tantan_attack_hi4_game,
        tantan_attack_lw4_game,

        tantan_attack_s4_game,
        tantan_attack_s4_effect,
        tantan_attack_s4_sound,
        tantan_attack_s4_expression,

        tantan_attack_s4_charge_effect,
        tantan_attack_s4_charge_sound,
        tantan_attack_s4_charge_expression,
    );
}

