use super::*;

unsafe extern "C" fn game_specialnstarth(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 30.0, 15.0);//van + 1
}

unsafe extern "C" fn game_specialnchargef(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.65); //van
}

unsafe extern "C" fn game_specialnendh(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 58.0, 25.0);//32 > 26
}

unsafe extern "C" fn game_specialnendf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 48.0, 25.0);//32 > 26
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
        //fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        GroundModule::set_correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        ATTACK(agent, 0, 0, Hash40::new("footr"), 8.0, 42, 8, 0, 95, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("footr"), 8.0, 47, 8, 0, 90, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("footr"), 7.5, 50, 8, 0, 79, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("footr"), 7.0, 62, 8, 0, 69, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 40.0); //57faf
    FT_MOTION_RATE_RANGE(agent, 40.0, 62.0, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        //fighter.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP);
        //fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
    }
}

unsafe extern "C" fn game_specialsedge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10, 0, 20, 0, false);
        ATTACK(agent, 0, 0, Hash40::new("footr"), 7.0, 62, 8, 0, 69, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        agent.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP);
        //fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_specialsedge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_heelslide_burst"), Hash40::new("kneer"), 9.5, 0, 0, 0, 90, 0, 1.1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        if WorkModule::get_int(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) == 2 {LAST_PARTICLE_SET_COLOR(agent, 1.5, 0.095, 0.163); }
        else {LAST_PARTICLE_SET_COLOR(agent, 0.048, 0.452, 1); }
    }
    frame(lua_state, 14.0);
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
    frame(lua_state, 16.0);
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
        //fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        //fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        AttackModule::clear_all(agent.module_accessor);
        //fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_END_SPECIAL_S);
    }
    frame(lua_state, 34.5);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 37.5);
    FT_MOTION_RATE_RANGE(agent, 37.5, 40.0, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("footr"), 5.0, 98, 66, 0, 60, 4.8, 1.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 5.0, 98, 66, 0, 60, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("waist"), 5.0, 90, 66, 0, 60, 3.9, 0.0, -0.9, -1.1, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
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
    FT_MOTION_RATE_RANGE(agent, 1.0, 12.0, 6.5);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 5, 0, 20, 0, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 5);
    }
    frame(lua_state, 12.0);
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
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::bayonetta::instance::IS_HIT) {
            VarModule::inc_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED);
        }
        JostleModule::set_status(boma, true);
        AttackModule::clear_all(boma);
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(lua_state, 32.0); //28
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::bayonetta::instance::IS_HIT) { //filters out bullet hits
            CancelModule::enable_cancel(agent.module_accessor);
        }
    }
    frame(lua_state, 38.0); //32
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_specialairsu(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_AFTERBURNER_LINE, Hash40::new("top"), 0, 13.0, 12.1, -21.1, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(agent, 1.15);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("top"), 0, 13.8, 13.5, -21, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(agent, 1.15);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_AFTERBURNER_LINE, -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("bayonetta_afterburner_line2"), -1);
    }
}

unsafe extern "C" fn expression_specialairsu(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
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
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
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
        ATTACK(agent, 0, 0, Hash40::new("legr"), 7.5, 86, 35, 0, 95, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 7.5, 86, 35, 0, 95, 5.0, 0.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear_all(boma);
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_LANDING_FALL_SPECIAL);
        VarModule::inc_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
        agent.off_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_specialairsd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_AFTERBURNER_LINE, Hash40::new("top"), 0, 0, 7, 45, -6, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(agent, 0.85);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("top"), 0, 0, 7, 45, -5, 0, 1.1, true);
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

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
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
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 0.5, y: 33.0}, 11, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: -0.3, y: 33.0}, 11, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 368, 10, 0, 0, 4.3, 0.0, 18.2, 0.5, Some(0.0), Some(20.4), Some(0.5), 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.5, 368, 10, 0, 0, 2.6, 0.0, 16.0, 0.5, Some(0.0), Some(23.7), Some(0.5), 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("head"), &Vector2f{x: 0.0, y: 15.5}, 10, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("head"), &Vector2f{x: 0.0, y: 14.5}, 10, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        if agent.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE) {
            VarModule::on_flag(agent.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 22, 35, 0, 40, 4.6, 0.0, 23.1, 0.0, Some(0.0), Some(18.4), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 42, 55, 0, 45, 4.6, 0.0, 23.1, 0.0, Some(0.0), Some(18.4), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2bfb02b69a), true);
        if !VarModule::is_flag(agent.battle_object, vars::bayonetta::instance::IS_HIT) {
            VarModule::inc_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::bayonetta::instance::IS_HIT) {
            CancelModule::enable_cancel(agent.module_accessor);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_GROUND_START) {
            FOOT_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND, Hash40::new("top"), 0, 25.3, 0, 0, 0, 0, 0.86, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 0.82, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        if agent.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE) {
            EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_SPIRAL, Hash40::new("top"), 0, 25.3, 0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 1.7);
        }
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND, -1);
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("bayonetta_afterburner_line2"), false, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND, false, true);
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

unsafe extern "C" fn game_speciallw (agent: &mut L2CAgentBase) {
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
    agent.acmd("game_specialnstarth", game_specialnstarth);
    agent.acmd("game_specialnstartf", game_specialnstarth);
    agent.acmd("game_specialairnstarth", game_specialnstarth);
    agent.acmd("game_specialairnstartf", game_specialnstarth);
    agent.acmd("game_specialnchargef", game_specialnchargef);
    agent.acmd("game_specialnchargeh", game_specialnchargef);
    agent.acmd("game_specialairnchargef", game_specialnchargef);
    agent.acmd("game_specialairnchargeh", game_specialnchargef);
    agent.acmd("game_specialnendh", game_specialnendh);
    agent.acmd("game_specialnendf", game_specialnendf);
    agent.acmd("game_specialairnendh", game_specialnendh);
    agent.acmd("game_specialairnendf", game_specialnendf);

    agent.acmd("game_specials", game_specials);
    agent.acmd("game_specialsedge", game_specialsedge);
    agent.acmd("effect_specialsedge", effect_specialsedge);
    agent.acmd("sound_specialsedge", sound_specialsedge);
    agent.acmd("expression_specialsedge", expression_specialsedge);
    agent.acmd("game_specialsholdend", game_specialsholdend);
    agent.acmd("game_specialairsu", game_specialairsu);
    agent.acmd("effect_specialairsu", effect_specialairsu);
    agent.acmd("expression_specialairsu", expression_specialairsu);
    agent.acmd("game_shootinglegl_atkon_specialairsu", game_shootinglegl_atkon_specialairsu);
    agent.acmd("game_specialairsd", game_specialairsd);
    agent.acmd("effect_specialairsd", effect_specialairsd);
    agent.acmd("expression_specialairsd", expression_specialairsd);
    agent.acmd("game_specialairsdlanding", game_specialairsdlanding);

    agent.acmd("game_specialairhi", game_specialairhi);
    agent.acmd("game_specialhi", game_specialairhi);
    agent.acmd("effect_specialairhi", effect_specialairhi);
    agent.acmd("effect_specialhi", effect_specialairhi);
    agent.acmd("sound_specialhi", sound_specialhi);
    agent.acmd("sound_specialairhi", sound_specialhi);
    
    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("game_specialairlw", game_speciallw);
}
