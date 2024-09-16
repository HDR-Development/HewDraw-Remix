use super::*;
use globals::*;

unsafe extern "C" fn game_specialnstarth(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 30.0, 15.0);//van + 1
}

unsafe extern "C" fn game_specialnendh(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let cancel_frame_param = agent.get_param_int("param_special_n", "cancel_frame") as f32;
    let special_lag = agent.get_float(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
    frame(lua_state, 1.0);
    if !agent.is_status(statuses::bayonetta::SPECIAL_N_CANCEL) || special_lag < cancel_frame_param { 
        MotionModule::set_rate(boma, (58.0 - 1.0)/25.0);//32 > 26
    }//do not change motion rate on special lag cancel anim
}

unsafe extern "C" fn game_specialnendf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    MotionModule::set_rate(boma, (48.0 - 1.0)/25.0);//32 > 26
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10, 0, 20, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        GroundModule::set_correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        ATTACK(agent, 0, 0, Hash40::new("footr"), 8.0, 361, 8, 0, 85, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("footr"), 8.0, 361, 8, 0, 80, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("footr"), 7.5, 50, 8, 0, 75, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("footr"), 7.0, 62, 8, 0, 65, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 36.0); //57faf -> 60
    FT_MOTION_RATE_RANGE(agent, 36.0, 62.0, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        agent.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP);
        agent.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    }
    frame(lua_state, 41.0); //41
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
    }
    frame(lua_state, 51.0); //50 frames
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT_BEFORE_GUARD) {
            CancelModule::enable_cancel(boma);
        }
    }
}

unsafe extern "C" fn game_specialsedge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    let status_frame = boma.status_frame() - 15;
    let slide_frame = status_frame.clamp(1, 24) / 4;//6
    //let game_frame = 14.0 - slide_frame;
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 10.0, 8.0 - slide_frame as f32); //14 -> 8f faf spaced
    if is_excute(agent) {
        if AttackModule::is_attack(boma, 0, false) {
            ATTACK(agent, 0, 0, Hash40::new("footr"), 7.0, 62, 8, 0, 65, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10, 0, 20, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
        agent.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        agent.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        sv_kinetic_energy!(set_accel, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.015);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        CancelModule::enable_cancel(boma);
    }
}

unsafe extern "C" fn effect_specialsedge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("bayonetta_heelslide_burst"), false, false);
    }
}

unsafe extern "C" fn sound_specialsedge(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn expression_specialsedge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

unsafe extern "C" fn game_specialsholdend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 34.5, 11.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 2, 0, 0, 0, false);
        ATTACK(agent, 0, 0, Hash40::new("footr"), 7.0, 70, 8, 0, 50, 4.0, 0.5, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        AttackModule::clear_all(boma);
        //agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_END_SPECIAL_S);
    }
    frame(lua_state, 34.5);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 37.5);
    FT_MOTION_RATE_RANGE(agent, 37.5, 40.0, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("footr"), 5.0, 101, 66, 0, 60, 4.8, 1.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 5.0, 101, 66, 0, 60, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("waist"), 5.0, 101, 66, 0, 60, 3.9, 0.0, -0.9, -1.1, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(agent, 40.0, 44.0, 2.0);
    frame(lua_state, 44.0);
    FT_MOTION_RATE_RANGE(agent, 44.0, 61.0, 15.0); //35 faf kick
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairsu(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 13.0, 8.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 5, 0, 20, 0, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 5);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 6.0, 65, 27, 0, 72, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);   
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 6.0, 70, 27, 0, 66, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 7.0, 47, 100, 70, 0, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 7.7, 47, 100, 65, 0, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 7.7, 51, 100, 50, 0, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
        AttackModule::clear_all(boma);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::inc_int(boma.object(), vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
        }
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
    }
    frame(lua_state, 32.0); //28
    if is_excute(agent) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            CancelModule::enable_cancel(boma);
        }
    }
    frame(lua_state, 38.0); //32
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_specialairsu(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_AFTERBURNER_LINE, Hash40::new("rot"), 0, 0.4, 12.0, -21.1, 0, 0, 0.95, true);
        LAST_EFFECT_SET_RATE(agent, 1.15);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("rot"), 0, 0.4, 12.0, -21.1, 0, 0, 0.9, true);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_AFTERBURNER_LINE, -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("bayonetta_afterburner_line2"), -1);
    }
    frame(lua_state, 290.0);
    if is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("bayonetta_afterburner_line2"), true, true);
    }
}

unsafe extern "C" fn game_shootinglegl_atkon_specialairsu(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if !agent.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
            ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00 as u64, 0, Hash40::new("top"), 0.5, 340, 100, 0, 10, 3.5, 0.0, 4.5, -0.5, Some(0.0), Some(4.5), Some(3.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        }
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01 as u64, 0, Hash40::new("top"), 0.5, 340, 0, 0, 0, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(-18.0), Some(59.4), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, 6, 4.05);
    }
}

unsafe extern "C" fn game_specialairsd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("legr"), 7.5, 86, 35, 0, 85, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 7.5, 86, 35, 0, 85, 5.0, 0.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::inc_int(boma.object(), vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_LANDING_FALL_SPECIAL);
    }
}

unsafe extern "C" fn effect_specialairsd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_AFTERBURNER_LINE, Hash40::new("top"), 0, 0.5, 6.5, 45, -6, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(agent, 0.85);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("top"), 0, 0.5, 6.5, 45, -5, 0, 1.1, true);
    }
}

unsafe extern "C" fn expression_specialairsd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialairsdlanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 20, 0, 15, 0, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 20);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 368, 10, 0, 0, 3.0, 0.0, 4.5, 0.0, Some(0.0), Some(12.5), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 368, 10, 0, 0, 3.5, 0.0, 5.5, 5.0, Some(0.0), Some(10.0), Some(5.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 0.5, y: 16.0}, 4, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: -0.3, y: 16.0}, 4, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 368, 10, 0, 0, 4.3, 0.0, 18.2, 0.5, Some(0.0), Some(20.4), Some(0.5), 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.5, 368, 10, 0, 0, 2.6, 0.0, 16.0, 0.5, Some(0.0), Some(23.7), Some(0.5), 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("trans"), &Vector2f{x: 0.0, y: 35.0}, 8, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("trans"), &Vector2f{x: 0.0, y: 35.0}, 8, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        if agent.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE) {
            VarModule::on_flag(agent.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 15, 35, 0, 40, 4.6, 0.0, 23.1, 0.0, Some(0.0), Some(18.4), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 32, 55, 0, 45, 4.6, 0.0, 23.1, 0.0, Some(0.0), Some(18.4), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2bfb02b69a), true);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::inc_int(boma.object(), vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            CancelModule::enable_cancel(boma);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_GROUND_START) {
            FOOT_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND, Hash40::new("top"), 0, 25.5, 0, 0, 0, 0, 0.84, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("top"), 0, 25.1, 0, -90, 0, 0, 0.82, true);
        if agent.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE) {
            EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_SPIRAL, Hash40::new("top"), 0, 25.3, 0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 1.7);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND, -1);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        let mut twist = "bayonetta_witchtwist_wind_blue";
        if agent.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) == 2 {twist = "bayonetta_witchtwist_wind_red"; }
        agent.clear_lua_stack();
        lua_args!(agent, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new(twist), true, true);
        sv_module_access::effect(agent.lua_state_agent);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_SPIRAL, false, true);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_bayonetta_rnd_attack01"));
        PLAY_SE(agent, Hash40::new("se_bayonetta_special_h01"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_attackair_f02"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_special_h02"));
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 4.0);
	if is_excute(agent) {
		agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
        sv_kinetic_energy!(set_stable_speed, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.3);
	}
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(agent, 8.0, 15.0, 4.0); //5
	if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), true);
        }
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        SEARCH(agent, 0, 0, Hash40::new("top"), 11.5, -2.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
	frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0); //9
	if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), true);
    }
	frame(lua_state, 21.0);
    FT_MOTION_RATE_RANGE(agent, 21.0, 31.0, 7.0); //15
	if is_excute(agent) {
		agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 31.0);//22
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        FT_MOTION_RATE_RANGE(agent, 31.0, 66.0, 28.0); 
    } else { FT_MOTION_RATE_RANGE(agent, 31.0, 66.0, 23.0); }
	if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), false);
    }
	frame(lua_state, 44.0);
	if is_excute(agent) {
		agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
	}
} //faf 50/45

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstarth", game_specialnstarth, Priority::Low);
    agent.acmd("game_specialnstartf", game_specialnstarth, Priority::Low);
    agent.acmd("game_specialairnstarth", game_specialnstarth, Priority::Low);
    agent.acmd("game_specialairnstartf", game_specialnstarth, Priority::Low);
    agent.acmd("game_specialnendh", game_specialnendh, Priority::Low);
    agent.acmd("game_specialnendf", game_specialnendf, Priority::Low);
    agent.acmd("game_specialairnendh", game_specialnendh, Priority::Low);
    agent.acmd("game_specialairnendf", game_specialnendf, Priority::Low);

    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialsedge", game_specialsedge, Priority::Low);
    agent.acmd("effect_specialsedge", effect_specialsedge, Priority::Low);
    agent.acmd("sound_specialsedge", sound_specialsedge, Priority::Low);
    agent.acmd("expression_specialsedge", expression_specialsedge, Priority::Low);
    agent.acmd("game_specialsholdend", game_specialsholdend, Priority::Low);
    agent.acmd("game_specialairsu", game_specialairsu, Priority::Low);
    agent.acmd("effect_specialairsu", effect_specialairsu, Priority::Low);
    agent.acmd("game_shootinglegl_atkon_specialairsu", game_shootinglegl_atkon_specialairsu, Priority::Low);
    agent.acmd("game_specialairsd", game_specialairsd, Priority::Low);
    agent.acmd("effect_specialairsd", effect_specialairsd, Priority::Low);
    agent.acmd("game_specialairsdlanding", game_specialairsdlanding, Priority::Low);

    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi, Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi, Priority::Low);
    
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
}
