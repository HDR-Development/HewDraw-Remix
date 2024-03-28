use super::*;

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_ENABLE_SPECIAL_LW_INSTALL) {
            VarModule::set_flag(
                agent.battle_object, 
                vars::shotos::status::IS_ENABLE_MAGIC_SERIES_CANCEL, 
                MeterModule::level(agent.battle_object) >= 4
            );
            MeterModule::drain_direct(agent.battle_object, 1.0 * MeterModule::meter_per_level(agent.battle_object));
        } else {
            VarModule::off_flag(agent.battle_object, vars::shotos::status::IS_ENABLE_MAGIC_SERIES_CANCEL);
        }
        VarModule::off_flag(agent.battle_object, vars::shotos::instance::IS_ENABLE_SPECIAL_LW_INSTALL);
    }
}

unsafe extern "C" fn game_speciallwinstall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 11.0, 5.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallwinstall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_speciallwinstall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ryu_appeal_l01"));
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ryu_appeal03"));
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        let sfx_handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_common_final_cutin"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn expression_speciallwinstall(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        AREA_WIND_2ND_arg10(agent, 0, 1, 70, 8, 0.8, 0, 6, 32, 12, 80);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    let lv = agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV);
    MeterModule::watch_damage(agent.battle_object, true);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_DISABLE_SUPER_ARMOR);
        if lv == *FIGHTER_RYU_SAVING_LV_1 {
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 60, 10, 0, 100, 3.5, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 60, 10, 0, 100, 3.5, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
        } else if lv == *FIGHTER_RYU_SAVING_LV_2 {
            ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 60, 100, 0, 0, 4.3, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 100, 0, 0, 4.3, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 5.4, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 5.4, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        }
        AttackModule::set_attack_level(agent.module_accessor, 0, lv as u8);
        AttackModule::set_attack_level(agent.module_accessor, 1, lv as u8);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, false);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_REVERSE_MATERIAL_ANIM);
    }
}

unsafe extern "C" fn game_speciallwturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
    frame(agent.lua_state_agent, 11.0);
    let lv = agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_DISABLE_SUPER_ARMOR);
        MeterModule::watch_damage(agent.battle_object, true);
        if lv == *FIGHTER_RYU_SAVING_LV_1 {
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 60, 10, 0, 100, 3.5, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 60, 10, 0, 100, 3.5, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
        } else if lv == *FIGHTER_RYU_SAVING_LV_2 {
            ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 60, 100, 0, 0, 4.3, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 100, 0, 0, 4.3, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 5.4, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 5.4, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        }
        AttackModule::set_attack_level(agent.module_accessor, 0, lv as u8);
        AttackModule::set_attack_level(agent.module_accessor, 1, lv as u8);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, false);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_REVERSE_MATERIAL_ANIM);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallwstart", game_speciallwstart);
    agent.acmd("game_specialairlwstart", game_speciallwstart);
    agent.acmd("game_speciallwinstall", game_speciallwinstall);
    agent.acmd("effect_speciallwinstall", effect_speciallwinstall);
    agent.acmd("sound_speciallwinstall", sound_speciallwinstall);
    agent.acmd("expression_speciallwinstall", expression_speciallwinstall);
    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("game_specialairlw", game_speciallw);
    agent.acmd("game_speciallwturn", game_speciallwturn);
    agent.acmd("game_specialairlwturn", game_speciallwturn);
}
