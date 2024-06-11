use smash::app::sv_animcmd::EFFECT_WORK;

use super::*;

unsafe fn will_bayonet(agent: &mut L2CAgentBase) -> bool {
    let boma = agent.boma();
    let is_csticking = ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0;
    if (is_csticking) {
        return true;
    }
    return false;
}

unsafe extern "C" fn game_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL) {
            WorkModule::on_flag(boma, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_GENERATE_BULLET);
            WorkModule::on_flag(boma, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_ENABLE_SHOOT);
        }
    }
}

unsafe extern "C" fn effect_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 
    && !VarModule::is_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL) {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("buddy_missile_shot_l"), Hash40::new("top"), 0, 10, 9, 0, 0, 0, 1, false);
        }
        else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("buddy_missile_shot_r"), Hash40::new("top"), 0, 10, 9, 0, 0, 0, 1, false);
        }
    }
}
if is_excute(agent) {
    EFFECT_FOLLOW_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_all"), 0, -6, 0, 0, 0, 0, 0.8, true);
}
frame(lua_state, 16.0);
if is_excute(agent) {
    EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
}
}

unsafe extern "C" fn sound_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_n01"));
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL) {
            PLAY_SE(agent, Hash40::new("se_buddy_special_n04_01"));
        }
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_step_left_m"));
    }
    frame(lua_state, 61.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_step_right_m"));
    }
    frame(lua_state, 73.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_n04_02"));
    }
}

unsafe extern "C" fn expression_specialnfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::buddy::instance::SPECIAL_N_LAND_CANCEL) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

unsafe extern "C" fn game_specialnupperfire(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    for _ in 0..4 {
        if is_excute(agent) && will_bayonet(agent){
            VarModule::on_flag(boma.object(), vars::buddy::instance::BAYONET_ACTIVE);
            agent.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S3, true);
            VarModule::set_int(boma.object(), vars::buddy::instance::BAYONET_EGGS,*FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
            break;
        }
        wait(lua_state, 1.0);
    }
    if !VarModule::is_flag(boma.object(), vars::buddy::instance::BAYONET_ACTIVE) {
        frame(lua_state, 4.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_GENERATE_BULLET);
        }
        frame(lua_state, 5.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_START_PRECEDE_CHECK);
        }
    }
}

unsafe extern "C" fn game_specialnfire2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    for _ in 0..4 {
        if is_excute(agent) && will_bayonet(agent){
            VarModule::on_flag(boma.object(), vars::buddy::instance::BAYONET_ACTIVE);
                       
            agent.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S3, true);
            return;
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_GENERATE_BULLET);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_START_PRECEDE_CHECK);
    }
}

unsafe extern "C" fn game_specialsdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag( boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR);
        JostleModule::set_status( boma, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 43, 72, 0, 66, 3.8, 0.0, 6.2, 1.8, Some(0.0), Some(6.2), Some(3.2), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 43, 72, 0, 66, 4.2, 0.0, 9.2, 3.8, Some(0.0), Some(9.2), Some(5.4), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        //AttackModule::set_captured_same_time_attack(boma, 0, true);
        //AttackModule::set_captured_same_time_attack(boma, 1, true);
        //AttackModule::set_captured_same_time_attack_damage_mul(boma, 0, 0.25);
        //AttackModule::set_captured_same_time_attack_damage_mul(boma, 1, 0.25);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 0.48);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        HIT_NO(agent, 0, *HIT_STATUS_INVINCIBLE);
        HIT_NO(agent, 1, *HIT_STATUS_INVINCIBLE);
        HIT_NO(agent, 2, *HIT_STATUS_INVINCIBLE);
        HIT_NO(agent, 3, *HIT_STATUS_INVINCIBLE);
        HIT_NO(agent, 4, *HIT_STATUS_INVINCIBLE);
        HIT_NO(agent, 5, *HIT_STATUS_INVINCIBLE);
        HIT_NO(agent, 6, *HIT_STATUS_INVINCIBLE);
        HIT_NO(agent, 7, *HIT_STATUS_INVINCIBLE);
        HIT_NO(agent, 8, *HIT_STATUS_INVINCIBLE);
        HIT_NO(agent, 9, *HIT_STATUS_INVINCIBLE);
        HIT_NO(agent, 10, *HIT_STATUS_INVINCIBLE);
        HIT_NO(agent, 11, *HIT_STATUS_NORMAL);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        WorkModule::on_flag( boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_CLIFF_CHECK);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 43, 63, 0, 56, 3.8, 0.0, 6.2, 1.8, Some(0.0), Some(6.2), Some(3.2), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 43, 63, 0, 56, 4.2, 0.0, 9.2, 3.8, Some(0.0), Some(9.2), Some(5.4), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 12.0);
        //AttackModule::set_captured_same_time_attack(boma, 0, true);
        //AttackModule::set_captured_same_time_attack(boma, 1, true);
        //AttackModule::set_captured_same_time_attack_damage_mul(boma, 0, 0.25);
        //AttackModule::set_captured_same_time_attack_damage_mul(boma, 1, 0.25);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 0.28);
    }
}

unsafe extern "C" fn game_specialairsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;    
    let boma = agent.boma();
    if is_excute(agent) {
        //Clear speed
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP); 
        KineticModule::clear_speed_all(boma);
        GroundModule::set_attach_ground(boma, false);
        sv_kinetic_energy!(clear_speed, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_accel, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        SET_SPEED_EX(agent, -0.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 10.0);
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(agent, 1.0);

    frame(lua_state, 12.0);
    if is_excute(agent) {
        SET_SPEED_EX(agent, 2.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

unsafe extern "C" fn effect_specialairsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent; 
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("buddy_special_s_start"), Hash40::new("rot"), -2, -2, -14, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_COLOR(agent,1,0.5,0);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("buddy_special_s_hold"), Hash40::new("virtualcenter"), 1.5, 0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_COLOR(agent,1,0.5,0);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.4, 0, 0.2);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 4, 0, 0, 0, 0);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_all"), 0, -6, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.4, 0, 0.3);
    }
}

unsafe extern "C" fn sound_specialairsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent; 
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) && !WorkModule::is_flag(boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL){
        PLAY_SE(agent, Hash40::new("se_buddy_special_s01"));
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let play_vc = app::sv_math::rand(hash40("fighter"), 3);
        if play_vc == 0 {
            PLAY_SE(agent, Hash40::new("vc_buddy_attack05"));
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_s01"));
    }
}

unsafe extern "C" fn game_specialairsdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;    
    let boma = agent.boma();
    if is_excute(agent) {
        //Set control
        VarModule::on_flag(boma.object(), vars::buddy::instance::BEAKBOMB_ACTIVE);
        VarModule::set_int(boma.object(), vars::buddy::instance::BEAKBOMB_FRAME,0);
        VarModule::set_int(boma.object(), vars::buddy::instance::BEAKBOMB_BOUNCE,0);
        let stick_y: f32 = ControlModule::get_stick_y(boma);
        VarModule::set_float(boma.object(), vars::buddy::instance::BEAKBOMB_ANGLE,stick_y.signum());
        if stick_y.abs() < 0.1 {
            VarModule::set_float(boma.object(), vars::buddy::instance::BEAKBOMB_ANGLE,0.0);
        }
        //Clear gravity speed (prevents exponential movement from cliff jump)
        sv_kinetic_energy!(clear_speed, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_accel, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        let shieldDamage = 4;
        JostleModule::set_status( boma, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 43, 70, 0, 76, 3.2, 0.0, 9.2, 8.8, Some(0.0), Some(9.2), Some(12.4), 1.125, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, shieldDamage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 43, 70, 0, 76, 4.2, 0.0, 4.2, 2.8, None,None,None, 1.125, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, shieldDamage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        WorkModule::on_flag( boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_CLIFF_CHECK);
        HIT_NO(agent, 12, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 13, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 14, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 15, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 16, *HIT_STATUS_NORMAL);
        HIT_NO(agent, 17, *HIT_STATUS_NORMAL);
    }
    //Weaker hitbox
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 43, 57, 0, 66, 3.2, 0.0, 9.2, 8.8, Some(0.0), Some(9.2), Some(12.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 43, 57, 0, 66, 4.2, 0.0, 4.2, 2.8, None,None,None, 1.125, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn effect_specialairsdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent; 
    let boma = agent.boma(); 
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("buddy_special_s_flash1"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 1, 0.3, 0);
        EFFECT_FLW_POS(agent, Hash40::new("buddy_special_s_flash2"), Hash40::new("k_head"), -1, 0, -4, 0, 0, 0, 0.5, true);
        LAST_EFFECT_SET_COLOR(agent,1,0.5,0);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    if (smash::app::sv_animcmd::get_value_float(lua_state,*SO_VAR_FLOAT_LR)<=0.0){
        if is_excute(agent) {
            //EFFECT_FOLLOW(agent, Hash40::new("buddy_special_s_impact"), Hash40::new("throw"), 2, 0, 3, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(agent, Hash40::new("buddy_special_s_wind"), Hash40::new("top"), 2, 4, 0, 0, 0, 0, 0.7, true);
        }
    }
    else if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("buddy_special_s_impact"), Hash40::new("throw"), -2, 0, 3, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("buddy_special_s_wind"), Hash40::new("top"), -2, 4, 0, 0, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    if is_excute(agent) {
        FLASH(agent, 1, 1, 0.6, 0.3);
        EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        //buddy_meter(agent,boma);
        EffectModule::enable_sync_init_pos_last(boma);
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_specialairsdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;    
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_buddy_special_s05"));
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        let play_vc = app::sv_math::rand(hash40("fighter"), 3);
        if play_vc == 0 {
            PLAY_SE(agent, Hash40::new("vc_buddy_smash_h01_vc"));
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_s02_02"));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_s02_03"));
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_s02_03"));
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_buddy_special_s02_03"));
    }
}

unsafe extern "C" fn expression_specialairsdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent; 
    let boma = agent.boma(); 
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            ItemModule::set_have_item_visibility(boma, false,0);
        }
        //slope!(agent,*MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        }
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_73_bodyattack1"), 0, false, 0);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_73_bodyattack1"), 0, false, 0);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_73_bodyattack1"), 0, false, 0);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_73_bodyattack1"), 0, false, 0);
    }
    wait(lua_state, 6.0);
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    wait(lua_state, 2.0);
}

unsafe extern "C" fn game_specialairsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;  
    let boma = agent.boma();  
    frame(lua_state, 36.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY);
    }
}

unsafe extern "C" fn game_specialairswall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;    
    let boma = agent.boma(); 
    frame(lua_state, 15.0);
    if is_excute(agent) {
        let has_hit_shield = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
        if (!has_hit_shield
        && VarModule::get_int(boma.object(), vars::buddy::instance::BEAKBOMB_BOUNCE)==0) {
            CancelModule::enable_cancel(boma);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        let has_hit_shield = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
        if (!has_hit_shield
        && VarModule::get_int(boma.object(), vars::buddy::instance::BEAKBOMB_BOUNCE)==0) {
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if (WorkModule::is_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY)) {
            FT_MOTION_RATE(agent, 1.0); //.75
        } else {
            FT_MOTION_RATE(agent, 1.0);
        }
    }
    //frame(lua_state, 42.0);
    //FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 47.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY);
    }
}

unsafe extern "C" fn effect_specialairswall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent; 
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 
    let has_hit_shield = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
    let lr = PostureModule::lr(boma)==0.0;
    let size = if (has_hit_shield) {0.5} else {0.75};
    let xRot = if (lr) {-90.0} else {90.0};
    let yRot = if (lr) {0.0} else {180.0};
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("buddy_special_s_wall"), Hash40::new("top"), 5, 12.5, 0, xRot, yRot, 0, size, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_SCATTER, Hash40::new("top"), -7, 12.5, 0, xRot, yRot, 0, 1, false);
    }
}

unsafe extern "C" fn sound_specialairswall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent; 
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_down_m_01"));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let play_vc = app::sv_math::rand(hash40("fighter"), 2);
        if play_vc == 0 {
            PLAY_SE(agent, Hash40::new("vc_buddy_missfoot02"));
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_down_m_02"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnfire", game_specialnfire, Priority::Low);
    agent.acmd("effect_specialnfire", effect_specialnfire, Priority::Low);
    agent.acmd("sound_specialnfire", sound_specialnfire, Priority::Low);
    agent.acmd("expression_specialnfire", expression_specialnfire, Priority::Low);
    agent.acmd("game_specialnupperfire", game_specialnupperfire, Priority::Low);
    agent.acmd("game_specialnfire2", game_specialnfire2, Priority::Low);

    agent.acmd("game_specialsdash", game_specialsdash, Priority::Low);

    agent.acmd("game_specialairsstart", game_specialairsstart, Priority::Low);
    agent.acmd("effect_specialairsstart", effect_specialairsstart, Priority::Low);
    agent.acmd("sound_specialairsstart", sound_specialairsstart, Priority::Low);
    agent.acmd("game_specialairsdash", game_specialairsdash, Priority::Low);
    agent.acmd("effect_specialairsdash", effect_specialairsdash, Priority::Low);
    agent.acmd("sound_specialairsdash", sound_specialairsdash, Priority::Low);
    agent.acmd("expression_specialairsdash", expression_specialairsdash, Priority::Low);
    agent.acmd("game_specialairsend", game_specialairsend, Priority::Low);
    agent.acmd("game_specialairswall", game_specialairswall, Priority::Low);
    agent.acmd("effect_specialairswall", effect_specialairswall, Priority::Low);
    agent.acmd("sound_specialairswall", sound_specialairswall, Priority::Low);
}