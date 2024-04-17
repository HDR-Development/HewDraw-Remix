use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            FT_MOTION_RATE(agent, 0.588);
        }
        if !WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            FT_MOTION_RATE(agent, 0.800);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        MeterModule::add(agent.battle_object, 3.0);
    }
}

unsafe extern "C" fn game_specialsfstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        MeterModule::add(agent.battle_object, 1.0);
     }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            FT_MOTION_RATE(agent, 0.667);
        }
        else {
            FT_MOTION_RATE(agent, 1.000);
        }
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if MeterModule::drain(agent.battle_object, 1) {
                FT_MOTION_RATE(agent, 8.0/(7.0-6.0));
                VarModule::on_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
            }
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FT_MOTION_RATE(agent, 1.0);
        }
    }
}

unsafe extern "C" fn effect_specialsfstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);    
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND){
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 13, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.3);    
        }
        else{
            EFFECT(agent, Hash40::new("dolly_drive_flash"), Hash40::new("top"), 10, 12, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.3);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 3.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 0.6, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }   
    frame(lua_state, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.2);

    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            COL_PRI(agent, 200);
            FLASH(agent, 1.0, 0.71, 0.115, 1.75);
        }
    }
}

unsafe extern "C" fn game_specialairsfstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        MeterModule::add(agent.battle_object, 1.0);
     }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            FT_MOTION_RATE(agent, 0.667);
        }
        else {
            FT_MOTION_RATE(agent, 1.000);
        }
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if MeterModule::drain(agent.battle_object, 1) {
                FT_MOTION_RATE(agent, 8.0/(7.0-6.0));
                VarModule::on_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
            }
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FT_MOTION_RATE(agent, 1.0);
        }
    }
}

unsafe extern "C" fn effect_specialairsfstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);    
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND){
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 13, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.3);    
        }
        else{
            EFFECT(agent, Hash40::new("dolly_drive_flash"), Hash40::new("top"), 10, 12, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.3);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 3.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 0.6, true);
            LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }   
    frame(lua_state, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.2);

    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            COL_PRI(agent, 200);
            FLASH(agent, 1.0, 0.71, 0.115, 1.75);
        }
    }
}

unsafe extern "C" fn game_specialsfattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if WorkModule::is_flag(boma,  *FIGHTER_DOLLY_STATUS_SPECIAL_S_WORK_FLAG_AIR_ATTACK) {  }
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            FT_MOTION_RATE(agent, 0.667);
        }
         else {
            FT_MOTION_RATE(agent, 0.833);
        }
        if WorkModule::is_flag(boma,  *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        }
        if WorkModule::is_flag(boma,  *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 65, 42, 0, 77, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
             }
             else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 40, 83, 0, 70, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
             }
        }
         else {
            if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 65, 42, 0, 77, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
             }
             else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 40, 88, 0, 69, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
             }
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FT_MOTION_RATE(agent, 1.250);
            HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
            ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 43, 71, 0, 73, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 2.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            MeterModule::watch_damage(agent.battle_object, false);
        } else {
            MeterModule::watch_damage(agent.battle_object, true);
            MeterModule::set_damage_gain_mul(agent.battle_object, 0.5);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma,  *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 65, 42, 0, 77, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
             }
             else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 40, 83, 0, 70, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
             }
        }
         else {
            if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 65, 42, 0, 77, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
             }
             else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 40, 88, 0, 69, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
             }
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FT_MOTION_RATE(agent, 1.250);
            HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
            ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 43, 71, 0, 73, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 2.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
         }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma,  *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 90, 42, 0, 82, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 40, 95, 0, 54, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 43, 72, 0, 63, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 2.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        if WorkModule::is_flag(boma,  *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 8.5, 90, 42, 0, 82, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 40, 100, 0, 57, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 43, 72, 0, 63, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 2.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_specialsfattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        COL_NORMAL(agent);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            /*
            EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("dolly_drive_tail_s1"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.8, 0.5, 0.5);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_drive_punch_s"), Hash40::new("dolly_drive_punch_s"), Hash40::new("havel"), 0.4, 0, 0, 0, 20, 0, 1.0, true, *EF_FLIP_ZX);
            LAST_EFFECT_SET_COLOR(agent, 0.8, 0.5, 0.5);
            LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("dolly_drive_start1"), Hash40::new("top"), 0, 12, 14, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.8, 0.5, 0.5);
            LAST_EFFECT_SET_RATE(agent, 0.9);
            */
            EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 2.0, true);
            //EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("dolly_down_tail_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(agent, Hash40::new("dolly_down_s"), Hash40::new("arml"), 3.0, 0, 0, 0, 90, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 1.1);
            EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("dolly_drive_start1"), Hash40::new("top"), 0, 12, 14, 0, 0, 0, 1, true);
            LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        else{
            if WorkModule::is_flag(boma,  *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
                if WorkModule::get_int64(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
                    EFFECT_FLW_POS_NO_STOP(agent,Hash40::new("dolly_drive_tail_s0"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
                    EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_drive_punch_s"), Hash40::new("dolly_drive_punch_s"), Hash40::new("havel"), 0.4, 0, 0, 0, 20, 0, 1.0, true, *EF_FLIP_ZX);
                }
                 else {
                    EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("dolly_drive_tail_s1"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
                    EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_drive_punch_s"), Hash40::new("dolly_drive_punch_s"), Hash40::new("havel"), 0.4, 0, 0, 0, 20, 0, 1.0, true, *EF_FLIP_ZX);
                }
            }
             else {
                if WorkModule::get_int64(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
                    EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("dolly_drive_tail_n0"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
                    EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_drive_punch_n"), Hash40::new("dolly_drive_punch_n"), Hash40::new("havel"), 0.4, 0, 0, 0, 20, 0, 1.0, true, *EF_FLIP_ZX);
                }
                 else {
                    EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("dolly_drive_tail_n1"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
                    EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_drive_punch_n"), Hash40::new("dolly_drive_punch_n"), Hash40::new("havel"), 0.4, 0, 0, 0, 20, 0, 1.0, true, *EF_FLIP_ZX);
                }
            }
            if WorkModule::get_int64(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
                LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
                EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("dolly_drive_start0"), Hash40::new("top"), 0, 12, 14, 0, 0, 0, 1, true);
            }
             else {
                LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                //EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("dolly_drive_start1"), Hash40::new("top"), 0, 12, 14, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("dolly_drive_start1"), Hash40::new("top"), 0, 12, 14, 0, 0, 0, 0.75, true);
                LAST_EFFECT_SET_RATE(agent, 1.25);
            }
        }
        
    }
    frame(lua_state, 3.0);
    for _ in 0..4 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_PRI(agent, 200);
                FLASH(agent, 1.0, 0.825, 0.115, 1.75);
            }
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                //FLASH(agent, 1.0, 1.0, 1.0, 1.5);
                FLASH(agent, 1.0, 0.825, 0.115, 0.6);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_NORMAL(agent);
            }
        }
        wait(lua_state, 1.0);
    }
    for _ in 0..3 {
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_PRI(agent, 200);
                FLASH(agent, 1.0, 0.825, 0.115, 2.0);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                //FLASH(agent, 1.0, 1.0, 1.0, 1.5);
                FLASH(agent, 1.0, 0.825, 0.115, 0.6);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_NORMAL(agent);
            }
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn game_specialsfend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
    }
}

unsafe extern "C" fn game_specialairsfend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_specialsbstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        //FT_MOTION_RATE(agent, 9.0/(7.0-1.0));
        MeterModule::add(agent.battle_object, 1.0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if MeterModule::drain(agent.battle_object, 1) {
                VarModule::on_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
            }
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            WorkModule::set_int(boma, *FIGHTER_DOLLY_STRENGTH_S, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);
        }
        if WorkModule::get_int64(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            FT_MOTION_RATE(agent, 1.0);
        }
        else{
            FT_MOTION_RATE(agent, 4.0);
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            FT_MOTION_RATE(agent, 1.0);
        }
    }
}

unsafe extern "C" fn effect_specialsbstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            COL_PRI(agent, 200);
            FLASH(agent, 1.0, 0.71, 0.115, 1.75);
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13.5, -2.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.3);
            EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 2.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 0.6, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
        }
        else{
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND){
                EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13.5, -2.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);    
            }
        }
    }
}

unsafe extern "C" fn game_specialairsbstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        //FT_MOTION_RATE(agent, 9.0/(7.0-1.0));
        MeterModule::add(agent.battle_object, 1.0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if MeterModule::drain(agent.battle_object, 1) {
                VarModule::on_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
            }
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            WorkModule::set_int(boma, *FIGHTER_DOLLY_STRENGTH_S, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);
        }
        if WorkModule::get_int64(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            FT_MOTION_RATE(agent, 1.0);
        }
        else{
            FT_MOTION_RATE(agent, 4.0);
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            FT_MOTION_RATE(agent, 1.0);
        }
    }
}

unsafe extern "C" fn effect_specialairsbstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL){
            COL_PRI(agent, 200);
            FLASH(agent, 1.0, 0.71, 0.115, 1.75);
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13.5, -2.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.3);
            EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 2.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 0.6, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
        }
        else{
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND){
                EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13.5, -2.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);    
            }
        }
    }
}

unsafe extern "C" fn game_specialsbattackw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut add_jump_h_speed = -1.4;
    let mut add_jump_v_speed = 0.5;
    if !WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_S_WORK_FLAG_AIR_ATTACK) {
        add_jump_h_speed = -0.7;
        add_jump_v_speed = 0.25;
    }
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(add_jump_h_speed, add_jump_v_speed, 0.0));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(0.0, -0.2*add_jump_v_speed, 0.0));
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 30, 100, 60, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 30, 100, 60, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("legr"), 3.0, 30, 100, 60, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 3.0, 30, 100, 60, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Air-only
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 4.0, 0, 100, 110, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 5, 0, Hash40::new("kneer"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 6, 0, Hash40::new("legr"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 7, 0, Hash40::new("hip"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
         else {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 30, 100, 60, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 30, 100, 60, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("legr"), 3.0, 30, 100, 60, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 3.0, 30, 100, 60, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Air-only
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 4.0, 0, 100, 110, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 5, 0, Hash40::new("kneer"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 6, 0, Hash40::new("legr"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 7, 0, Hash40::new("hip"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(0.0, -0.2*add_jump_v_speed, 0.0));
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 30, 100, 60, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 30, 100, 60, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("legr"), 3.0, 30, 100, 60, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 3.0, 30, 100, 60, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Air-only
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 4.0, 350, 100, 90, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 5, 0, Hash40::new("kneer"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 6, 0, Hash40::new("legr"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 7, 0, Hash40::new("hip"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
         else {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 30, 100, 60, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 30, 100, 60, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("legr"), 3.0, 30, 100, 60, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 3.0, 30, 100, 60, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Air-only
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 4.0, 350, 100, 90, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 5, 0, Hash40::new("kneer"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 6, 0, Hash40::new("legr"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 7, 0, Hash40::new("hip"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 30, 100, 50, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 30, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("legr"), 3.0, 30, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 3.0, 30, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Air-only
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 4.0, 340, 100, 40, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 5, 0, Hash40::new("kneer"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 6, 0, Hash40::new("legr"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 7, 0, Hash40::new("hip"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
         else {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 30, 100, 50, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 30, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("legr"), 3.0, 30, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 3.0, 30, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Air-only
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 4.0, 340, 100, 40, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 5, 0, Hash40::new("kneer"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 6, 0, Hash40::new("legr"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 7, 0, Hash40::new("hip"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.3*add_jump_h_speed, 0.0, 0.0));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 30, 100, 50, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 30, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("legr"), 3.0, 30, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 3.0, 30, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Air-only
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 4.0, 0, 100, 50, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 5, 0, Hash40::new("kneer"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 6, 0, Hash40::new("legr"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 7, 0, Hash40::new("hip"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
         else {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 30, 100, 50, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 30, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("legr"), 3.0, 30, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 3.0, 30, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Air-only
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 4.0, 0, 100, 50, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 5, 0, Hash40::new("kneer"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 6, 0, Hash40::new("legr"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 7, 0, Hash40::new("hip"), 3.0, 30, 100, 30, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            ATTACK(agent, 0, 1, Hash40::new("kneer"), 7.0, 55, 55, 0, 65, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 1, Hash40::new("kneer"), 7.0, 55, 55, 0, 65, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 1, Hash40::new("legr"), 7.0, 55, 55, 0, 65, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 1, Hash40::new("hip"), 7.0, 55, 55, 0, 65, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
         else {
            ATTACK(agent, 0, 1, Hash40::new("kneer"), 7.0, 55, 55, 0, 65, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 1, Hash40::new("kneer"), 7.0, 55, 55, 0, 65, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 1, Hash40::new("legr"), 7.0, 55, 55, 0, 65, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 1, Hash40::new("hip"), 7.0, 55, 55, 0, 65, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(0.0, -0.25*add_jump_v_speed, 0.0));
        KineticModule::add_speed(boma, &Vector3f::new(-0.3*add_jump_h_speed, 0.0, 0.0));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(0.0, -0.35*add_jump_v_speed, 0.0));
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(0.0, -0.35*add_jump_v_speed, 0.0));
        KineticModule::add_speed(boma, &Vector3f::new(-0.2*add_jump_h_speed, 0.0, 0.0));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
}

unsafe extern "C" fn effect_specialsbattackw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(agent, 200);
            FLASH(agent, 1.0, 0.825, 0.115, 2.0);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        // EX
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(agent);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_kick_arc_w_command"), Hash40::new("dolly_kick_arc_w_command"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ);
            EffectModule::set_disable_render_offset_last(boma);
            EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("footr"), 1, 0, 0, 0, 0, 0, 2.0, true);
            EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("kneer"), 1, 0, 0, 0, 0, 0, 1.5, true);
        }
        // Regular
        else{
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND){
                EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_kick_arc_s_command"), Hash40::new("dolly_kick_arc_s_command"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ);
                EffectModule::set_disable_render_offset_last(boma);
            }
            else{
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_w01"), Hash40::new("dolly_kick_arc_w01"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_w02"), Hash40::new("dolly_kick_arc_w02"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_w03"), Hash40::new("dolly_kick_arc_w03"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_w04"), Hash40::new("dolly_kick_arc_w04"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_w05"), Hash40::new("dolly_kick_arc_w05"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_w06"), Hash40::new("dolly_kick_arc_w06"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_w07"), Hash40::new("dolly_kick_arc_w07"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_w08"), Hash40::new("dolly_kick_arc_w08"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                else{
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_w01"), Hash40::new("dolly_kick_arc_w01"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                EffectModule::set_disable_render_offset_last(boma);
            }
        }
        
    }
    frame(lua_state, 4.0);
    for _ in 0..5 {
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_PRI(agent, 200);
                FLASH(agent, 1.0, 0.825, 0.115, 2.0);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(agent, 1.0, 0.825, 0.115, 0.6);
            }
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_NORMAL(agent);
            }
        }
        wait(lua_state, 2.0);
    }
}

unsafe extern "C" fn game_specialsbattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let  mut add_jump_h_speed = -1.0;
    let mut add_jump_v_speed = 0.5;
    if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        add_jump_h_speed = -1.3;
        add_jump_v_speed = 2.0;
    }
    else if !WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_S_WORK_FLAG_AIR_ATTACK) {
        add_jump_h_speed = -0.3;
        add_jump_v_speed = 0.3;
    }
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(add_jump_h_speed, add_jump_v_speed, 0.0));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(0.0, -0.2*add_jump_v_speed, 0.0));
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("kneer"), 5.0, 30, 100, 70, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 30, 100, 70, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("legr"), 3.0, 30, 100, 70, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 3.0, 30, 100, 70, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Air-only
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 5.0, 345, 100, 90, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 5, 0, Hash40::new("kneer"), 3.0, 0, 100, 80, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 6, 0, Hash40::new("legr"), 3.0, 0, 100, 80, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 7, 0, Hash40::new("hip"), 3.0, 0, 100, 80, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
         else {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("kneer"), 5.0, 30, 100, 70, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 30, 100, 70, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("legr"), 3.0, 30, 100, 70, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 3.0, 30, 100, 70, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Air-only
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 5.0, 345, 100, 90, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 5, 0, Hash40::new("kneer"), 3.0, 0, 100, 80, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 6, 0, Hash40::new("legr"), 3.0, 0, 100, 80, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 7, 0, Hash40::new("hip"), 3.0, 0, 100, 80, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(0.0, -0.2*add_jump_v_speed, 0.0));
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("kneer"), 5.0, 68, 50, 0, 95, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Air-only
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 4.0, 68, 50, 0, 95, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
         else {
            // Ground-only
            ATTACK(agent, 0, 0, Hash40::new("kneer"), 5.0, 68, 50, 0, 95, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            // Air-only
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 4.0, 68, 50, 0, 95, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            KineticModule::add_speed(boma, &Vector3f::new(-0.4*add_jump_h_speed, 0.0, 0.0));
        }
        else{
            KineticModule::add_speed(boma, &Vector3f::new(-0.3*add_jump_h_speed, 0.0, 0.0));
        }

    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            ATTACK(agent, 0, 1, Hash40::new("kneer"), 10.0, 68, 40, 0, 95, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 1, Hash40::new("kneer"), 6.0, 68, 40, 0, 95, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 1, Hash40::new("legr"), 6.0, 68, 40, 0, 95, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 1, Hash40::new("hip"), 6.0, 68, 40, 0, 95, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
         else {
            ATTACK(agent, 0, 1, Hash40::new("kneer"), 10.0, 68, 40, 0, 95, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 1, Hash40::new("kneer"), 6.0, 68, 40, 0, 95, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 1, Hash40::new("legr"), 6.0, 68, 40, 0, 95, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 1, Hash40::new("hip"), 6.0, 68, 40, 0, 95, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 1, Hash40::new("kneer"), 1.0, 340, 100, 50, 20, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 1, Hash40::new("kneer"), 1.0, 340, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 1, Hash40::new("legr"), 1.0, 340, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 3, 1, Hash40::new("hip"), 1.0, 340, 100, 50, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            ATTACK(agent, 0, 1, Hash40::new("kneer"), 10.0, 78, 50, 0, 95, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
         else {
            ATTACK(agent, 0, 1, Hash40::new("kneer"), 10.0, 78, 50, 0, 95, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            AttackModule::clear_all(boma);
            /* Ground-only */
            ATTACK(agent, 0, 1, Hash40::new("kneer"), 10.0, 275, 40, 0, 100, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 1, Hash40::new("kneer"), 10.0, 275, 40, 0, 100, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 1, Hash40::new("legr"), 10.0, 275, 40, 0, 100, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            /* Air-only */
            ATTACK(agent, 3, 1, Hash40::new("kneer"), 10.0, 275, 66, 0, 30, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 4, 1, Hash40::new("kneer"), 10.0, 275, 66, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 5, 1, Hash40::new("legr"), 10.0, 275, 66, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        // Speed handling
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            KineticModule::add_speed(boma, &Vector3f::new(0.0, -0.5*add_jump_v_speed, 0.0));
            KineticModule::add_speed(boma, &Vector3f::new(-0.3*add_jump_h_speed, 0.0, 0.0));
        }
        else{
            KineticModule::add_speed(boma, &Vector3f::new(0.0, -0.2*add_jump_v_speed, 0.0));
            KineticModule::add_speed(boma, &Vector3f::new(-0.3*add_jump_h_speed, 0.0, 0.0));
        }
        MeterModule::watch_damage(agent.battle_object, true);
        
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            KineticModule::add_speed(boma, &Vector3f::new(0.0, -0.4*add_jump_v_speed, 0.0));
        }
        else{
            KineticModule::add_speed(boma, &Vector3f::new(0.0, -0.35*add_jump_v_speed, 0.0));
        }
        
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.2*add_jump_h_speed, 0.0, 0.0));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
}

unsafe extern "C" fn effect_specialsbattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(agent, 200);
            FLASH(agent, 1.0, 0.825, 0.115, 2.0);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        // EX
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(agent);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_kick_arc_s_command"), Hash40::new("dolly_kick_arc_s_command"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ);
            EffectModule::set_disable_render_offset_last(boma);
            EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("footr"), 1, 0, 0, 0, 0, 0, 2.0, true);
            EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("kneer"), 1, 0, 0, 0, 0, 0, 1.5, true);
        }
        // Regular
        else{
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND){
                EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_kick_arc_s_command"), Hash40::new("dolly_kick_arc_s_command"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ);
                EffectModule::set_disable_render_offset_last(boma);
            }
            else{
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_s01"), Hash40::new("dolly_kick_arc_s01"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_s02"), Hash40::new("dolly_kick_arc_s02"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_s03"), Hash40::new("dolly_kick_arc_s03"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_s04"), Hash40::new("dolly_kick_arc_s04"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_s05"), Hash40::new("dolly_kick_arc_s05"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_s06"), Hash40::new("dolly_kick_arc_s06"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_s07"), Hash40::new("dolly_kick_arc_s07"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_s08"), Hash40::new("dolly_kick_arc_s08"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                else{
                    EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("dolly_kick_arc_s01"), Hash40::new("dolly_kick_arc_s01"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);
                }
                EffectModule::set_disable_render_offset_last(boma);
            }
        }
        
    }
    frame(lua_state, 4.0);
    for _ in 0..5 {
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_PRI(agent, 200);
                FLASH(agent, 1.0, 0.825, 0.115, 2.0);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(agent, 1.0, 0.825, 0.115, 0.6);
            }
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_NORMAL(agent);
            }
        }
        wait(lua_state, 2.0);
    }
}

unsafe extern "C" fn game_specialsbend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FT_MOTION_RATE(agent, 1.1);
        }
        else{
            FT_MOTION_RATE(agent, 0.75);
        }
    }
}

unsafe extern "C" fn game_specialhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        MeterModule::add(agent.battle_object, 1.0);
     }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_XLU);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 0, 0, Hash40::new("arml"), 2.0, 98, 100, 100, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 2.0, 98, 100, 100, 90, 4.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 2.0, 98, 100, 100, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 2.0, 98, 100, 100, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 2.0, 98, 100, 100, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 5, 0, Hash40::new("kneel"), 2.0, 98, 100, 100, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 6, 0, Hash40::new("kneel"), 2.0, 98, 100, 100, 30, 4.0, 6.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            FT_MOTION_RATE(agent, 0.75);
        }
        else{
            ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 98, 110, 110, 30, 4.0, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 75, 110, 110, 30, 4.0, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-3.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        // Light Rising Tackle early hit
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 0, 0, Hash40::new("arml"), 10.0, 85, 50, 0, 100, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 10.0, 85, 50, 0, 100, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 10.0, 85, 50, 0, 100, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 10.0, 85, 50, 0, 100, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 10.0, 85, 50, 0, 100, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 5, 0, Hash40::new("kneel"), 10.0, 85, 50, 0, 100, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 6, 0, Hash40::new("kneel"), 10.0, 85, 50, 0, 100, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            FT_MOTION_RATE(agent, 0.75);
        }
        // Heavy Rising Tackle
        else{
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 4.5, 0.0, Some(0.0), Some(4.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 17.0, 1.0, Some(0.0), Some(19.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 10, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 4.5, -4.0, Some(0.0), Some(4.5), Some(-6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
            AttackModule::clear_all(boma);
        }
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        // Light Rising Tackle late hit
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 0, 0, Hash40::new("arml"), 8.0, 65, 50, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 8.0, 65, 50, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 8.0, 65, 50, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 3, 0, Hash40::new("hip"), 8.0, 65, 50, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 4, 0, Hash40::new("kneer"), 8.0, 65, 50, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 5, 0, Hash40::new("kneel"), 8.0, 65, 50, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 6, 0, Hash40::new("kneel"), 8.0, 65, 50, 0, 70, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        // Heavy Rising Tackle
        else{
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 3.5, 6.0, Some(0.0), Some(3.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 3.5, -4.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        }
        else{
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 3.5, 0.0, Some(0.0), Some(3.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 3.5, -4.0, Some(0.0), Some(3.5), Some(-6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            
        }
         else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 83, 55, 0, 90, 7.0, 0.0, 7.0, 2.0, Some(0.0), Some(14.0), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            FT_MOTION_RATE(agent, 0.500);
        }
         else {
            FT_MOTION_RATE(agent, 0.667);
        }
    }
}

unsafe extern "C" fn game_specialhicommand(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        MeterModule::add(agent.battle_object, 1.0);
     }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if MeterModule::drain(agent.battle_object, 1) {
                 VarModule::on_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
            }
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_XLU);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        }
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 98, 110, 110, 30, 4.5, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 75, 110, 110, 30, 4.5, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-3.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 7.5, 0.0, Some(0.0), Some(7.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 20.0, 1.0, Some(0.0), Some(22.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 20, 4.5, 0.0, 13.0, 3.0, Some(0.0), Some(13.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 13.0, 3.0, Some(0.0), Some(13.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 7.5, -4.0, Some(0.0), Some(7.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 6.5, 7.5, Some(0.0), Some(6.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 19.0, 1.0, Some(0.0), Some(21.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 12.0, 3.0, Some(0.0), Some(12.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 12.0, 3.0, Some(0.0), Some(12.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 6.5, -4.0, Some(0.0), Some(6.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 7.5, Some(0.0), Some(5.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 4.5, 7.5, Some(0.0), Some(4.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 17.0, 1.0, Some(0.0), Some(19.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 4.5, -4.0, Some(0.0), Some(4.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        }
         else {
            HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        }
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 90, 30, 50, 30, 3.5, 0.0, 3.5, 5.0, Some(0.0), Some(3.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 45, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 50, 45, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 90, 65, 0, 90, 7.0, 0.0, 8.0, 2.0, Some(0.0), Some(15.0), Some(2.0), 1.25, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_BODY);
        }
        else{
            if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 83, 73, 0, 90, 7.0, 0.0, 8.0, 2.0, Some(0.0), Some(15.0), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            }
             else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 83, 62, 0, 90, 7.0, 0.0, 8.0, 2.0, Some(0.0), Some(15.0), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            }
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            FT_MOTION_RATE(agent, 0.500);
        }
         else {
            FT_MOTION_RATE(agent, 0.667);
        }
    }
}

unsafe extern "C" fn effect_specialhicommand(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(agent, 200);
            FLASH(agent, 1.0, 0.825, 0.115, 1.75);
            //FLASH_SET_DIRECTION(agent, 1, 0, 0);

            LAST_EFFECT_SET_RATE(agent, 1.3);
            EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);

            EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("kneer"), 3.0, 0, 0, 0, 0, 0, 2.0, true);
            EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("handr"), 1.0, 0, 0, 0, 0, 0, 1.5, true);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(agent);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0 {
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("dolly_roll_l_color1"), Hash40::new("throw"), 0, 2.5, 0, 0, 0, 0, 1, true, 0.8);
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
                LAST_EFFECT_SET_COLOR(agent, 0.146, 0.205, 0.333);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
                LAST_EFFECT_SET_COLOR(agent, 0.245, 0.325, 0.297);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
                LAST_EFFECT_SET_COLOR(agent, 0.212, 0.365, 0.332);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
                LAST_EFFECT_SET_COLOR(agent, 0.389, 0.342, 0.25);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
                LAST_EFFECT_SET_COLOR(agent, 0.372, 0.545, 0.579);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                LAST_EFFECT_SET_COLOR(agent, 0.33, 0.458, 0.611);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
                LAST_EFFECT_SET_COLOR(agent, 0.079, 0.079, 0.079);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                LAST_EFFECT_SET_COLOR(agent, 0.627, 0.627, 0.627);
            }
            else{
                LAST_EFFECT_SET_COLOR(agent, 0.146, 0.205, 0.333);
            }
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("dolly_roll_l_color2"), Hash40::new("throw"), 0, 2.5, 0, 0, 0, 0, 1, true, 0.8);
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
                LAST_EFFECT_SET_COLOR(agent, 0.587, 0.126, 0.169);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
                LAST_EFFECT_SET_COLOR(agent, 0.109, 0.122, 0.27);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
                LAST_EFFECT_SET_COLOR(agent, 0.317, 0.14, 0.131);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
                LAST_EFFECT_SET_COLOR(agent, 0.17, 0.317, 0.151);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
                LAST_EFFECT_SET_COLOR(agent, 0.524, 0.087, 0.087);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                LAST_EFFECT_SET_COLOR(agent, 0.079, 0.079, 0.079);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
                LAST_EFFECT_SET_COLOR(agent, 0.079, 0.079, 0.079);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                LAST_EFFECT_SET_COLOR(agent, 0.444, 0.329, 0.145);
            }
            else{
                LAST_EFFECT_SET_COLOR(agent, 0.587, 0.126, 0.169);
            }
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("dolly_roll_l"), Hash40::new("throw"), 0, 2.5, 0, 0, 0, 0, 1, true, 0.8);
        }
        else{
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("dolly_roll_l_colorr"), Hash40::new("throw"), 0, 2.5, 0, 0, 0, 0, 1, true, 0.8);
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
                LAST_EFFECT_SET_COLOR(agent, 0.146, 0.205, 0.333);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
                LAST_EFFECT_SET_COLOR(agent, 0.245, 0.325, 0.297);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
                LAST_EFFECT_SET_COLOR(agent, 0.212, 0.365, 0.332);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
                LAST_EFFECT_SET_COLOR(agent, 0.389, 0.342, 0.25);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
                LAST_EFFECT_SET_COLOR(agent, 0.372, 0.545, 0.579);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                LAST_EFFECT_SET_COLOR(agent, 0.33, 0.458, 0.611);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
                LAST_EFFECT_SET_COLOR(agent, 0.079, 0.079, 0.079);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                LAST_EFFECT_SET_COLOR(agent, 0.627, 0.627, 0.627);
            }
            else{
                LAST_EFFECT_SET_COLOR(agent, 0.146, 0.205, 0.333);
            }
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("dolly_roll_r_color2"), Hash40::new("throw"), 0, 2.5, 0, 0, 0, 0, 1, true, 0.8);
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
                LAST_EFFECT_SET_COLOR(agent, 0.587, 0.126, 0.169);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
                LAST_EFFECT_SET_COLOR(agent, 0.109, 0.122, 0.27);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
                LAST_EFFECT_SET_COLOR(agent, 0.317, 0.14, 0.131);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
                LAST_EFFECT_SET_COLOR(agent, 0.17, 0.317, 0.151);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
                LAST_EFFECT_SET_COLOR(agent, 0.524, 0.087, 0.087);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                LAST_EFFECT_SET_COLOR(agent, 0.079, 0.079, 0.079);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
                LAST_EFFECT_SET_COLOR(agent, 0.079, 0.079, 0.079);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                LAST_EFFECT_SET_COLOR(agent, 0.444, 0.329, 0.145);
            }
            else{
                LAST_EFFECT_SET_COLOR(agent, 0.587, 0.126, 0.169);
            }
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("dolly_roll_r"), Hash40::new("throw"), 0, 2.5, 0, 0, 0, 0, 1, true, 0.8);
            EffectModule::enable_sync_init_pos_last(boma);
        }

        
    }
    for _ in 0..4 {
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(agent, 1.0, 0.825, 0.115, 1.75);
            }
            else{
                FLASH(agent, 0.05, 0.1, 0.6, 0.6);
            }
            //FLASH_SET_DIRECTION(agent, 1, 0, 0);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_PRI(agent, 200);
            FLASH(agent, 1, 1, 1, 0.4);
            //FLASH_SET_DIRECTION(agent, 1, 0, 0);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0.4);
        //FLASH_SET_DIRECTION(agent, 1, 0, 0);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 8.5, 1, 0, 0, 0, 2, true, 0.6);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 1.0, 0.825, 0.115, 1.75);
        }
        else{
            FLASH(agent, 0.05, 0.1, 0.6, 0.6);
        }
        //FLASH_SET_DIRECTION(agent, 1, 0, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_roll_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_roll_r_color1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_roll_r_color2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_roll_l"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_roll_l_color1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_roll_l_color2"), false, true);
    }
}

unsafe extern "C" fn game_specialairhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        MeterModule::add(agent.battle_object, 1.0);
     }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_XLU);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 98, 110, 110, 30, 4.0, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 75, 110, 110, 30, 4.0, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-3.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 4.5, 0.0, Some(0.0), Some(4.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 17.0, 1.0, Some(0.0), Some(19.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 10, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 4.5, -4.0, Some(0.0), Some(4.5), Some(-6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 3.5, 6.0, Some(0.0), Some(3.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 3.5, -4.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 3.5, 0.0, Some(0.0), Some(3.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 3.5, -4.0, Some(0.0), Some(3.5), Some(-6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 83, 65, 0, 90, 7.0, 0.0, 7.0, 2.0, Some(0.0), Some(14.0), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 83, 55, 0, 90, 7.0, 0.0, 7.0, 2.0, Some(0.0), Some(14.0), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            FT_MOTION_RATE(agent, 0.500);
        }
         else {
            FT_MOTION_RATE(agent, 0.667);
        }
    }
}

unsafe extern "C" fn game_specialairhicommand(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        MeterModule::add(agent.battle_object, 1.0);
     }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if MeterModule::drain(agent.battle_object, 1) {
                VarModule::on_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
            }
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_XLU);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        }
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 98, 110, 110, 30, 4.5, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 75, 110, 110, 30, 4.5, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-3.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_finish_camera(boma, 0, true, false);
        AttackModule::set_no_finish_camera(boma, 1, true, false);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 7.5, 0.0, Some(0.0), Some(7.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 20.0, 1.0, Some(0.0), Some(22.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 20, 4.5, 0.0, 13.0, 3.0, Some(0.0), Some(13.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 13.0, 3.0, Some(0.0), Some(13.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 7.5, -4.0, Some(0.0), Some(7.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 6.5, 7.5, Some(0.0), Some(6.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 19.0, 1.0, Some(0.0), Some(21.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 12.0, 3.0, Some(0.0), Some(12.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 12.0, 3.0, Some(0.0), Some(12.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 6.5, -4.0, Some(0.0), Some(6.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 7.5, Some(0.0), Some(5.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(1.5),0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 4.5, 7.5, Some(0.0), Some(4.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 17.0, 1.0, Some(0.0), Some(19.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 4.5, -4.0, Some(0.0), Some(4.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        }
         else {
            HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        }
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 90, 30, 50, 30, 3.5, 0.0, 3.5, 5.0, Some(0.0), Some(3.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 45, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
         else {
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 50, 45, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 90, 65, 0, 90, 7.0, 0.0, 8.0, 2.0, Some(0.0), Some(15.0), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_BODY);
        }
        else{
            if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 83, 73, 0, 90, 7.0, 0.0, 8.0, 2.0, Some(0.0), Some(15.0), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            }
             else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 83, 62, 0, 90, 7.0, 0.0, 8.0, 2.0, Some(0.0), Some(15.0), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
            }
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            FT_MOTION_RATE(agent, 0.500);
        }
         else {
            FT_MOTION_RATE(agent, 0.667);
        }
    }
}

unsafe extern "C" fn effect_specialairhicommand(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(agent, 200);
            FLASH(agent, 1.0, 0.825, 0.115, 1.75);
            //FLASH_SET_DIRECTION(agent, 1, 0, 0);

            LAST_EFFECT_SET_RATE(agent, 1.3);
            EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);

            EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("kneer"), 3.0, 0, 0, 0, 0, 0, 2.0, true);
            EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("handr"), 1.0, 0, 0, 0, 0, 0, 1.5, true);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_NORMAL(agent);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if PostureModule::lr(boma) > 0.0 {
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("dolly_roll_l_color1"), Hash40::new("throw"), 0, 2.5, 0, 0, 0, 0, 1, true, 0.8);
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
                LAST_EFFECT_SET_COLOR(agent, 0.146, 0.205, 0.333);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
                LAST_EFFECT_SET_COLOR(agent, 0.245, 0.325, 0.297);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
                LAST_EFFECT_SET_COLOR(agent, 0.212, 0.365, 0.332);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
                LAST_EFFECT_SET_COLOR(agent, 0.389, 0.342, 0.25);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
                LAST_EFFECT_SET_COLOR(agent, 0.372, 0.545, 0.579);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                LAST_EFFECT_SET_COLOR(agent, 0.33, 0.458, 0.611);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
                LAST_EFFECT_SET_COLOR(agent, 0.079, 0.079, 0.079);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                LAST_EFFECT_SET_COLOR(agent, 0.627, 0.627, 0.627);
            }
            else{
                LAST_EFFECT_SET_COLOR(agent, 0.146, 0.205, 0.333);
            }
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("dolly_roll_l_color2"), Hash40::new("throw"), 0, 2.5, 0, 0, 0, 0, 1, true, 0.8);
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
                LAST_EFFECT_SET_COLOR(agent, 0.587, 0.126, 0.169);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
                LAST_EFFECT_SET_COLOR(agent, 0.109, 0.122, 0.27);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
                LAST_EFFECT_SET_COLOR(agent, 0.317, 0.14, 0.131);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
                LAST_EFFECT_SET_COLOR(agent, 0.17, 0.317, 0.151);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
                LAST_EFFECT_SET_COLOR(agent, 0.524, 0.087, 0.087);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                LAST_EFFECT_SET_COLOR(agent, 0.079, 0.079, 0.079);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
                LAST_EFFECT_SET_COLOR(agent, 0.079, 0.079, 0.079);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                LAST_EFFECT_SET_COLOR(agent, 0.444, 0.329, 0.145);
            }
            else{
                LAST_EFFECT_SET_COLOR(agent, 0.587, 0.126, 0.169);
            }
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("dolly_roll_l"), Hash40::new("throw"), 0, 2.5, 0, 0, 0, 0, 1, true, 0.8);
        }
        else{
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("dolly_roll_l_colorr"), Hash40::new("throw"), 0, 2.5, 0, 0, 0, 0, 1, true, 0.8);
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
                LAST_EFFECT_SET_COLOR(agent, 0.146, 0.205, 0.333);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
                LAST_EFFECT_SET_COLOR(agent, 0.245, 0.325, 0.297);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
                LAST_EFFECT_SET_COLOR(agent, 0.212, 0.365, 0.332);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
                LAST_EFFECT_SET_COLOR(agent, 0.389, 0.342, 0.25);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
                LAST_EFFECT_SET_COLOR(agent, 0.372, 0.545, 0.579);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                LAST_EFFECT_SET_COLOR(agent, 0.33, 0.458, 0.611);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
                LAST_EFFECT_SET_COLOR(agent, 0.079, 0.079, 0.079);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                LAST_EFFECT_SET_COLOR(agent, 0.627, 0.627, 0.627);
            }
            else{
                LAST_EFFECT_SET_COLOR(agent, 0.146, 0.205, 0.333);
            }
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("dolly_roll_r_color2"), Hash40::new("throw"), 0, 2.5, 0, 0, 0, 0, 1, true, 0.8);
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
                LAST_EFFECT_SET_COLOR(agent, 0.587, 0.126, 0.169);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
                LAST_EFFECT_SET_COLOR(agent, 0.109, 0.122, 0.27);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
                LAST_EFFECT_SET_COLOR(agent, 0.317, 0.14, 0.131);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
                LAST_EFFECT_SET_COLOR(agent, 0.17, 0.317, 0.151);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
                LAST_EFFECT_SET_COLOR(agent, 0.524, 0.087, 0.087);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                LAST_EFFECT_SET_COLOR(agent, 0.079, 0.079, 0.079);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
                LAST_EFFECT_SET_COLOR(agent, 0.079, 0.079, 0.079);
            }
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                LAST_EFFECT_SET_COLOR(agent, 0.444, 0.329, 0.145);
            }
            else{
                LAST_EFFECT_SET_COLOR(agent, 0.587, 0.126, 0.169);
            }
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("dolly_roll_r"), Hash40::new("throw"), 0, 2.5, 0, 0, 0, 0, 1, true, 0.8);
            EffectModule::enable_sync_init_pos_last(boma);
        }

        
    }
    for _ in 0..4 {
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                FLASH(agent, 1.0, 0.825, 0.115, 1.75);
            }
            else{
                FLASH(agent, 0.05, 0.1, 0.6, 0.6);
            }
            //FLASH_SET_DIRECTION(agent, 1, 0, 0);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_PRI(agent, 200);
            FLASH(agent, 1, 1, 1, 0.4);
            //FLASH_SET_DIRECTION(agent, 1, 0, 0);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0.4);
        //FLASH_SET_DIRECTION(agent, 1, 0, 0);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 8.5, 1, 0, 0, 0, 2, true, 0.6);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 1.0, 0.825, 0.115, 1.75);
        }
        else{
            FLASH(agent, 0.05, 0.1, 0.6, 0.6);
        }
        //FLASH_SET_DIRECTION(agent, 1, 0, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_roll_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_roll_r_color1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_roll_r_color2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_roll_l"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_roll_l_color1"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_roll_l_color2"), false, true);
    }
}

unsafe extern "C" fn game_specialhifall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        FT_MOTION_RATE(agent, 0.667);
    }
}

unsafe extern "C" fn game_specialairhiend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_1);
        MeterModule::add(agent.battle_object, 1.0);
     }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if MeterModule::drain(agent.battle_object, 1) {
                VarModule::on_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
                
            }
        }
        else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
            if MeterModule::drain(agent.battle_object, 1) {
                VarModule::on_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_1);
            }
        }
    }
}

unsafe extern "C" fn game_specialairlwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_1);
        MeterModule::add(agent.battle_object, 1.0);
     }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if MeterModule::drain(agent.battle_object, 1) {
                VarModule::on_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
            }
        }
    }
}

unsafe extern "C" fn game_specialairlwrisew(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::clear_speed_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) || VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_1) {
            //KineticModule::add_speed(boma, &Vector3f::new(0.0, -5.0, 0.0));
            SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 85, 25, 0, 100, 5.5, 0.0, 10.0, 1.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 85, 25, 0, 100, 5.5, 0.0, 10.0, 3.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            FT_MOTION_RATE(agent, 10.0);
            AttackModule::set_add_reaction_frame(boma, 0, 6.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 6.0, false);
        }
        else{
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 87, 100, 50, 50, 5.5, 0.0, 10.0, 1.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 87, 100, 50, 50, 5.5, 0.0, 10.0, 3.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        MeterModule::watch_damage(agent.battle_object, true);
        
    }
    frame(lua_state, 1.3);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_1) {
            //boma.change_status_req(*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, false);
            boma.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 2.0, 367, 100, 40, 10, 6.0, 0.0, 10.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 1, Hash40::new("top"), 2.0, 367, 100, 40, 10, 6.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 2, Hash40::new("top"), 2.0, 55, 100, 30, 10, 6.0, 0.0, 10.0, 1.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 2, Hash40::new("top"), 2.0, 55, 100, 30, 10, 6.0, 0.0, 10.0, 3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if !VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::add_speed(boma, &Vector3f::new(5.0, -1.5, 0.0));
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
}

unsafe extern "C" fn effect_specialairlwrisew(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("dolly_down_rise"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_1) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("kneer"), 3.0, 10.0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 1.1);
            //EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 0.6, true);
            //LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
}

unsafe extern "C" fn game_specialairlwrise(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::clear_speed_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) || VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_1) {
            //KineticModule::add_speed(boma, &Vector3f::new(0.0, -5.0, 0.0));
            SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 85, 25, 0, 100, 5.5, 0.0, 10.0, 1.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 85, 25, 0, 100, 5.5, 0.0, 10.0, 3.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            FT_MOTION_RATE(agent, 10.0);
            AttackModule::set_add_reaction_frame(boma, 0, 6.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 6.0, false);
        }
        else{
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 85, 100, 45, 50, 5.5, 0.0, 10.0, 3.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 80, 100, 45, 50, 5.5, 0.0, 10.0, 3.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
        MeterModule::watch_damage(agent.battle_object, true);
        
    }
    frame(lua_state, 1.3);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_1) {
            //boma.change_status_req(*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, false);
            boma.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 0.5, 367, 100, 50, 20, 7.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 1, Hash40::new("top"), 0.5, 367, 100, 30, 10, 7.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 2, Hash40::new("top"), 2.0, 367, 100, 50, 20, 7.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 2, Hash40::new("top"), 2.0, 367, 100, 30, 10, 7.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 3, Hash40::new("top"), 2.0, 40, 100, 30, 10, 7.0, 0.0, 10.0, 3.0, Some(0.0), Some(9.0), Some(1.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if !VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::add_speed(boma, &Vector3f::new(0.3, -1.5, 0.0));
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
}

unsafe extern "C" fn effect_specialairlwrise(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("dolly_down_rise"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            COL_PRI(agent, 200);
            FLASH(agent, 1.0, 0.71, 0.115, 1.75);
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13.5, -2.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.3);
            EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 1.4, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
        }
        else if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_1) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("kneer"), 3.0, 10.0, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 1.1);
            //EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("top"), 3.0, 10.0, 0, 0, 0, 0, 0.6, true);
            //LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    frame(lua_state, 3.0);
    for _ in 0..6 {
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_PRI(agent, 200);
                FLASH(agent, 1.0, 0.825, 0.115, 1.75);
            }
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                //FLASH(agent, 1.0, 1.0, 1.0, 1.5);
                FLASH(agent, 1.0, 0.825, 0.115, 0.6);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_NORMAL(agent);
            }
        }
        wait(lua_state, 1.0);
    }
            
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 0.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            let addSpeed1 = Vector3f{ x: 0.3, y: -1.0, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        }
         else {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            let addSpeed1 = Vector3f{ x: 1.3, y: -1.5, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        }
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            let addSpeed1 = Vector3f{ x: 0.0, y: -0.3, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
         else {
            let addSpeed1 = Vector3f{ x: 1.3, y: -1.0, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
         else {
            let addSpeed1 = Vector3f{ x: 1.3, y: -0.5, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 350, 35, 0, 30, 6.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        }
        else{
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
                if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                    ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 70, 0, 80, 6.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
                }
                 else {
                    ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 75, 0, 80, 6.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
                }
            }
             else {
                if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                    ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 50, 65, 0, 80, 6.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
                }
                 else {
                    ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 50, 70, 0, 80, 6.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
                }
            }
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.05, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
         else {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.2, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 350, 35, 0, 30, 6.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        }
        else{
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
                if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                    ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
                }
                 else {
                    /* Ground-only */
                    ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 310, 95, 0, 30, 6.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
                    /* Air-only */
                    ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 310, 60, 0, 30, 6.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
                }
            }
             else {
                if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                    ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
                }
                 else {
                    ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 6.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
                }
            }
            MeterModule::watch_damage(agent.battle_object, true);
        }
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            FT_MOTION_RATE(agent, 0.833);
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.05, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
         else {
            FT_MOTION_RATE(agent, 1.000);
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.2, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.05, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
         else {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.2, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.05, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
         else {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.2, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.05, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
         else {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.2, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.05, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
         else {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.2, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.05, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
         else {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.2, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
         else {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.2, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
         else {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.2, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
         else {
            let addSpeed1 = Vector3f{ x: 0.0, y: 0.2, z: 0.0 };
            KineticModule::add_speed(boma, &addSpeed1);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 350, 35, 0, 30, 6.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        }
        else{
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
                if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                    ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
                }
            }
        }
        MeterModule::watch_damage(agent.battle_object, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            FT_MOTION_RATE(agent, 0.100);
        }
         else {
            FT_MOTION_RATE(agent, 0.100);
        }
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_DOLLY_SPECIAL_LW_FALL);
        FT_MOTION_RATE(agent, 1.000);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_LANDING_HEAVY);
    }
}

unsafe extern "C" fn effect_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 2.0, true);
            EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("shoulderr"), 0, 0, 0, 0, 0, 0, 2.0, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("dolly_down_tail_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("dolly_down_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);   
        }
        else{
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND){
                EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("dolly_down_tail_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("dolly_down_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);        
            }
            else{
                EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("dolly_down_tail_n"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("dolly_down_n"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_OFF_KIND(agent, Hash40::new("dolly_wave_aura"), true, true);
        }
    }
    frame(lua_state, 3.0);
    for _ in 0..7 {
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_PRI(agent, 200);
                FLASH(agent, 1.0, 0.825, 0.115, 1.75);
            }
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                //FLASH(agent, 1.0, 1.0, 1.0, 1.5);
                FLASH(agent, 1.0, 0.825, 0.115, 0.6);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
                COL_NORMAL(agent);
            }
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_down_tail_n"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_down_n"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_down_tail_s"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_down_s"), false, true);
    }
}

unsafe extern "C" fn game_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_2);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            AttackModule::clear_all(boma);
            SET_SPEED_EX(agent, 0.5, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            
            // Star Dunk Volcano
            if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                if MeterModule::drain(agent.battle_object, 3) {
                    VarModule::on_flag(boma.object(), vars::shotos::instance::IS_TARGET_COMBO_2);
                }
            }
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_2) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 0, 100, 70, 0, 6.5, 0.0, 6.5, 0.0, Some(0.0), Some(6.5), Some(5.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 0, 100, 70, 0, 6.5, 0.0, 6.5, 0.0, Some(0.0), Some(6.5), Some(5.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
            else{
                ATTACK(agent, 0, 1, Hash40::new("top"), 3.0, 277, 35, 0, 120, 6.5, 0.0, 6.5, 0.0, Some(0.0), Some(6.5), Some(5.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 1, 1, Hash40::new("top"), 3.0, 85, 35, 0, 110, 6.5, 0.0, 6.5, 0.0, Some(0.0), Some(6.5), Some(5.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
                AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
            }
            MeterModule::watch_damage(agent.battle_object, true);
        }
        else{
            SET_SPEED_EX(agent, 1.5, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        // Star Dunk Volcano
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_2) {
            ArticleModule::generate_article(boma, *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST, false, 0);
        }
        
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        // Star Dunk Volcano
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_2) {
            SlowModule::set_whole(boma, 2, 20);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            // Star Dunk Volcano
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_2) {
                FT_MOTION_RATE(agent, 4.000);
            }
            else{
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                    FT_MOTION_RATE(agent, 0.5);
                }
                else{
                    FT_MOTION_RATE(agent, 2.000);
                }
            }
            
        }
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            AttackModule::clear_all(boma);
        }
        MeterModule::watch_damage(agent.battle_object, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FT_MOTION_RATE(agent, 1.000);
        }
    }
}

unsafe extern "C" fn effect_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        // EX/Star Dunk Volcano
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            EFFECT_DETACH_KIND(agent, Hash40::new("dolly_down_s"), -1);
            EFFECT(agent, Hash40::new("dolly_down_ground_s"), Hash40::new("top"), 0, 0, 11, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_TARGET_COMBO_2) {
                LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(agent, 0.5);
            }
            else{
                LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
            }
        }
        else{
            LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 11, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND){
                EFFECT_DETACH_KIND(agent, Hash40::new("dolly_down_s"), -1);
                EFFECT(agent, Hash40::new("dolly_down_ground_s"), Hash40::new("top"), 0, 0, 11, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
            else{
                EFFECT_DETACH_KIND(agent, Hash40::new("dolly_down_n"), -1);
                EFFECT(agent, Hash40::new("dolly_down_ground_n"), Hash40::new("top"), 0, 0, 11, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        }
    }   
}

unsafe extern "C" fn game_superspecial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 4.0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10.0);
        if VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL){
            SLOW_OPPONENT(agent, 15.0, 40.0);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL){
            FT_MOTION_RATE(agent, 15.0/(8.0-6.0));
        }
        else{
            FT_MOTION_RATE(agent, 8.0/(8.0-6.0));
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST, false, 0);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL){
            FT_MOTION_RATE(agent, 1.0);
        }
        else{
            FT_MOTION_RATE(agent, 2.0/3.0);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 0.0, 8.0);
    }
}

unsafe extern "C" fn effect_superspecial(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_aura"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 1, true);

        if VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL){
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 13, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.0);
            EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_flash"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 0.7, true);
            LAST_EFFECT_SET_COLOR(agent, 0.6, 0.4, 0.8);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_aura_s"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.65, true);
            LAST_EFFECT_SET_COLOR(agent, 0.6, 0.4, 0.8);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.65, true);
            LAST_EFFECT_SET_COLOR(agent, 0.6, 0.4, 0.8);
            LAST_EFFECT_SET_RATE(agent, 0.8);
        }
        else{
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 13, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.3);

            EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_flash"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 0.6, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_aura_s"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.65, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.65, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_wave_arc"), Hash40::new("dolly_wave_arc"), Hash40::new("top"), 0, 10, 4, 69, -46, -45, 1.2, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn game_superspecial2start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
        if VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL){
            FT_MOTION_RATE(agent, 1.0);
            SLOW_OPPONENT(agent, 30.0, 30.0);
        }
        else{
            //FT_MOTION_RATE(agent, 0.5);
            FT_MOTION_RATE(agent, 1.0);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL){
            FT_MOTION_RATE(agent, 1.0);
        }
        else{
            FT_MOTION_RATE(agent, 2.0);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL){
            FT_MOTION_RATE(agent, 15.0/(9.0-5.0));
        }
        else{
            //FT_MOTION_RATE(agent, 0.5);
            FT_MOTION_RATE(agent, 8.0/(9.0-5.0));
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL){
            FT_MOTION_RATE(agent, 0.75);
        }
        else{
            FT_MOTION_RATE(agent, 0.5);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 45, 100, 100, 30, 6.0, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(boma, true, false);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 45, 100, 100, 30, 6.0, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        ArticleModule::generate_article(boma, *FIGHTER_DOLLY_GENERATE_ARTICLE_FIRE, false, 0);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        //damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_superspecial2start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        
        //EFFECT_FOLLOW(agent, Hash40::new("sys_smash_ball_damage"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 0.25, true);
        //EFFECT_FOLLOW(agent, Hash40::new("sys_smash_ball"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 3.0, true);
        /*
        EFFECT_FOLLOW(agent, Hash40::new("sys_absorption"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 3.0, true);
        LAST_EFFECT_SET_RATE(agent, 0.25);
        */

        //EFFECT_FOLLOW(agent, Hash40::new("sys_cross_bomb"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 0.25, true);
        //LAST_EFFECT_SET_RATE(agent, 3.0);

        EFFECT_FOLLOW(agent, Hash40::new("dolly_wave_hold"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 1, true);

        if VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL){
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -15, 16, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.0);
            EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_flash"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 0.65, true);
            LAST_EFFECT_SET_COLOR(agent, 0.6, 0.4, 0.8);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_aura_s"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_COLOR(agent, 0.6, 0.4, 0.8);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_COLOR(agent, 0.6, 0.4, 0.8);
            LAST_EFFECT_SET_RATE(agent, 0.75);
        }
        else{
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -15, 16, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.1);

            EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_flash"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 0.6, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_aura_s"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.65, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_normal_s"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.65, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
        }
        
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("dolly_buster_punch"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("dolly_buster_dash"), Hash40::new("top"), 0, 0, -8, 0, 0, 0, 1, false);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("dolly_buster_punch"), 6);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("dolly_buster_dash"), false, true);
    }
}

unsafe extern "C" fn game_superspecial2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 20.0, 45, 79, 0, 40, 0.8, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 45, 63, 0, 90, 8.0, 0.0, 8.0, 5.0, Some(0.0), Some(8.0), Some(15.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 45, 48, 0, 90, 3.0, 0.0, 2.0, -5.0, Some(0.0), Some(25.0), Some(17.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 45, 48, 0, 90, 3.0, 0.0, 2.0, -5.0, Some(0.0), Some(20.0), Some(25.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 45, 48, 0, 90, 3.0, 0.0, 2.0, -5.0, Some(0.0), Some(12.0), Some(30.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 4, 0, Hash40::new("top"), 12.0, 45, 48, 0, 90, 3.0, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(33.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_TARGET_OBJECT_ID), WorkModule::get_int64(boma, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn);

    agent.acmd("game_specialsfstart", game_specialsfstart);
    agent.acmd("effect_specialsfstart", effect_specialsfstart);
    agent.acmd("game_specialairsfstart", game_specialairsfstart);
    agent.acmd("effect_specialairsfstart", effect_specialairsfstart);

    agent.acmd("game_specialsfattack", game_specialsfattack);
    agent.acmd("effect_specialsfattack", effect_specialsfattack);

    agent.acmd("game_specialsfend", game_specialsfend);
    agent.acmd("game_specialairsfend", game_specialairsfend);

    agent.acmd("game_specialsbstart", game_specialsbstart);
    agent.acmd("effect_specialsbstart", effect_specialsbstart);
    agent.acmd("game_specialairsbstart", game_specialairsbstart);
    agent.acmd("effect_specialairsbstart", effect_specialairsbstart);

    agent.acmd("game_specialsbattackw", game_specialsbattackw);
    agent.acmd("effect_specialsbattackw", effect_specialsbattackw);

    agent.acmd("game_specialsbattack", game_specialsbattack);
    agent.acmd("effect_specialsbattack", effect_specialsbattack);

    agent.acmd("game_specialsbend", game_specialsbend);

    agent.acmd("game_specialhi1", game_specialhi1);

    agent.acmd("game_specialhicommand", game_specialhicommand);
    agent.acmd("effect_specialhicommand", effect_specialhicommand);

    agent.acmd("game_specialairhi1", game_specialairhi1);

    agent.acmd("game_specialairhicommand", game_specialairhicommand);
    agent.acmd("effect_specialairhicommand", effect_specialairhicommand);

    agent.acmd("game_specialhifall", game_specialhifall);

    agent.acmd("game_specialairhiend", game_specialairhiend);

    agent.acmd("game_speciallwstart", game_speciallwstart);

    agent.acmd("game_specialairlwstart", game_specialairlwstart);

    agent.acmd("game_specialairlwrisew", game_specialairlwrisew);
    agent.acmd("effect_specialairlwrisew", effect_specialairlwrisew);

    agent.acmd("game_specialairlwrise", game_specialairlwrise);
    agent.acmd("effect_specialairlwrise", effect_specialairlwrise);

    agent.acmd("game_specialairlw", game_specialairlw);
    agent.acmd("effect_specialairlw", effect_specialairlw);

    agent.acmd("game_speciallwend", game_speciallwend);
    agent.acmd("effect_speciallwend", effect_speciallwend);

    agent.acmd("game_superspecial", game_superspecial);
    agent.acmd("effect_superspecial", effect_superspecial);

    agent.acmd("game_superspecial2start", game_superspecial2start);
    agent.acmd("effect_superspecial2start", effect_superspecial2start);

    agent.acmd("game_superspecial2", game_superspecial2);
}
