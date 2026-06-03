use super::*;
use globals::*;

unsafe extern "C" fn game_specialnstarth(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    let startup_frame = if agent.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_n.startup_frame") -1.0} else {14.0};
    MotionModule::set_rate(boma, 30.0/startup_frame); // van (15f before charge)
    let prev_inflict_status = VarModule::get_int(agent.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
    let prev_status_0 = StatusModule::prev_status_kind(boma, 0);
    VarModule::off_flag(agent.battle_object, vars::bayonetta::instance::WAS_CANCEL);
    if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR].contains(&prev_status_0)
    && prev_inflict_status & *COLLISION_KIND_MASK_HIT != 0 {
        VarModule::on_flag(agent.battle_object, vars::bayonetta::instance::WAS_CANCEL);
    } // faster charge if hit cancelled into
}

unsafe extern "C" fn game_specialnchargeh(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    let charge_frame_max = if agent.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_n.charge_frame_max") -1.0} else {28.0} ;
    let charge_frame_max_cancel = if agent.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_n.charge_frame_max_cancel") -1.0} else {18.0};
    if VarModule::is_flag(agent.battle_object, vars::bayonetta::instance::WAS_CANCEL) {
        MotionModule::set_rate(boma, (15.0-1.0)/charge_frame_max_cancel); // van - 4, 35f total
    } else {
        MotionModule::set_rate(boma, (15.0-1.0)/charge_frame_max); // van + 4, 44f total (1 decimal off goes to 46f??)
    }
}

unsafe extern "C" fn game_specialnendh(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    // check for accumulated special lag on a2g BA
    let cancel_frame_param = agent.get_param_int("param_special_n", "cancel_frame") as f32;
    let special_lag = agent.get_float(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
    // check for accumulated BA lag
    let max_repeat = agent.get_param_int("param_special_n", "add_fire_max");
    let remaining_repeats = VarModule::get_int(agent.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE);
    let used_rounds = (max_repeat - remaining_repeats) as f32;
    let lag_per_round = if agent.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_n.lag_per_round")} else {5.0};
    let base_endlag= if agent.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_n.base_endlag") -1.0} else {24.0}; // 32 faf van, 25 here and 40 max
    let cancel_frame= if agent.kind() == *FIGHTER_KIND_BAYONETTA {58.0} else {58.0};
    if agent.is_status(statuses::bayonetta::SPECIAL_N_CANCEL) {
        MotionModule::set_rate(boma, (cancel_frame - 1.0)/base_endlag);
    } else if special_lag < cancel_frame_param {
        MotionModule::set_rate(boma, (cancel_frame - 1.0)/(base_endlag + lag_per_round*used_rounds));
    }// do not change motion rate on special lag cancel anim
}

unsafe extern "C" fn game_specialnendf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    let max_repeat = agent.get_param_int("param_special_n", "add_fire_max");
    let remaining_repeats = VarModule::get_int(agent.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE);
    let used_rounds = (max_repeat - remaining_repeats) as f32;
    let lag_per_round = if agent.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_n.lag_per_round")} else {5.0};
    let base_endlag= if agent.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_n.base_endlag") -1.0} else {24.0}; // 32 faf van, 25 here and 40 max
    let cancel_frame= if agent.kind() == *FIGHTER_KIND_BAYONETTA {48.0} else {48.0};
    if agent.is_status(statuses::bayonetta::SPECIAL_N_CANCEL) {
        MotionModule::set_rate(boma, (cancel_frame - 1.0)/base_endlag);
    } else {
        MotionModule::set_rate(boma, (cancel_frame - 1.0)/(base_endlag + lag_per_round*used_rounds));
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 5.0, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10, 0, 20, 0, false);
    }
    frame(lua_state, 5.0); // 17 see iff snappier
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 15.0); // 17 see iff snappier
    if is_excute(agent) {
        CHECK_BA(agent, true);
        GroundModule::set_correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        ATTACK(agent, 0, 0, Hash40::new("footr"), 8.0, 361, 15, 0, 80, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("footr"), 8.0, 361, 15, 0, 70, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("footr"), 7.5, 55, 15, 0, 60, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("footr"), 7.0, 65, 15, 0, 55, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 36.0); // 38
    FT_MOTION_RATE_RANGE(agent, 36.0, 62.0, 24.0); // 62 faf (+2)
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        agent.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP);
        agent.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        VarModule::on_flag(agent.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK); // allow held-kick on hitbox clear frame
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
    }
    frame(lua_state, 49.0); // 50
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT_BEFORE_GUARD) {
            CancelModule::enable_cancel(boma);
        }
    }
}

unsafe extern "C" fn game_specialsedge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
        agent.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        agent.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP);
    }
}

unsafe extern "C" fn effect_specialsedge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
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
    FT_MOTION_RATE_RANGE(agent, 1.0, 34.5, 12.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 0, 0, 0, 0, false);
        CHECK_BA(agent, true);
        if AttackModule::is_attack(boma, 0, false) {
            ATTACK(agent, 0, 0, Hash40::new("footr"), 7.0, 70, 15, 0, 50, 4.0, 0.5, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        AttackModule::clear_all(boma);
        // agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_END_SPECIAL_S);
    }
    frame(lua_state, 34.5);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 37.5);
    FT_MOTION_RATE_RANGE(agent, 37.5, 40.0, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("footr"), 5.0, 101, 101, 0, 55, 4.8, 1.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 5.0, 101, 101, 0, 55, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("waist"), 5.0, 101, 101, 0, 55, 3.9, 0.0, -0.9, -1.1, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(agent, 40.0, 44.0, 2.0);
    frame(lua_state, 44.0);
    FT_MOTION_RATE_RANGE(agent, 44.0, 61.0, 14.0); // 35 faf kick
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
        agent.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        agent.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP);
    }
}

unsafe extern "C" fn game_specialairsu(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 13.0, 10.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10, 0, 20, 0, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 10);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE_RANGE(agent, 13.0, 25.0, 11.0);
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
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 6.5, 51, 100, 73, 0, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 7.0, 51, 100, 70, 0, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 7.5, 55, 100, 58, 0, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 25.0); // 21
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
        AttackModule::clear_all(boma);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::inc_int(boma.object(), vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
        } else {
            VarModule::on_flag(agent.battle_object, vars::bayonetta::status::SPECIAL_HIT_NO_BULLET);
        }
        if CHECK_BA(agent, true) {
            VarModule::inc_int(agent.battle_object, vars::bayonetta::instance::SPECIAL_BULLET_ARTS_COUNT);
            agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
    }
    frame(lua_state, 36.0); // 32
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::bayonetta::status::SPECIAL_HIT_NO_BULLET) {
            CancelModule::enable_cancel(boma);
        }
    }
    frame(lua_state, 37.0); // 33 (12 after)
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 39.0); // 35
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_specialairsu(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_AFTERBURNER_LINE, Hash40::new("rot"), 0, 0.4, 12.0, -21.1, 0, 0, 0.95, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
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

unsafe extern "C" fn game_specialairsd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 10, 0, 20, 0, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10);
    }
    frame(lua_state, 8.0); // 12
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("legr"), 7.0, 86, 42, 0, 85, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 7.0, 86, 42, 0, 85, 5.0, 0.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 14.0); // 18
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("legr"), 8.5, 86, 36, 0, 86, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 8.5, 86, 36, 0, 86, 5.0, 0.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 25.0); // 29
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::inc_int(boma.object(), vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
        if CHECK_BA(agent, true) {
            agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_LANDING_FALL_SPECIAL);
            VarModule::inc_int(agent.battle_object, vars::bayonetta::instance::SPECIAL_BULLET_ARTS_COUNT);
            agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
    }
    frame(lua_state, 30.0); // 35
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_LANDING_FALL_SPECIAL);
    }
    frame(lua_state, 33.0); // 37 (8 after clear)
    FT_MOTION_RATE_RANGE(agent, 33.0, 41.0, 12.0); // faf 46 -> 49
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        agent.clear_lua_stack();
        lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let og_speed_mul = app::sv_kinetic_energy::get_speed_mul(lua_state);
        sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 11.0/8.0 * og_speed_mul); // partial comp movement during endlag
    }
    frame(lua_state, 37.66); // 47
    if is_excute(agent) {
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    frame(lua_state, 41.0); // 49
    //FT_MOTION_RATE(agent, 1.0); keep it rated for appearance
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL); // clear mot energy
    }
}

unsafe extern "C" fn effect_specialairsd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_AFTERBURNER_LINE, Hash40::new("rot"), 0, -11.0, 6.5, 45, -6, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(agent, 0.85);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("rot"), 0, -11.0, 6.5, 45, -5, 0, 1.1, true);
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
    frame(lua_state, 16.0);
    if is_excute(agent) {
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
}

unsafe extern "C" fn game_specialairsbounce(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 4.0, 4.0);
    frame(lua_state, 4.0); // 5
    FT_MOTION_RATE_RANGE(agent, 4.0, 41.0, 27.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK);
    }
    frame(lua_state, 41.0);
    FT_MOTION_RATE(agent, 1.0); // faf 25 -> 32
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 14.0, 7.0);
    frame(lua_state, 14.0); // 8
    FT_MOTION_RATE_RANGE(agent, 14.0, 18.0, 6.0);
    if is_excute(agent) {
        let speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let pos_diff: f32 = speed*4.0;
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 368, 100, 30, 0, 3.0, 0.0, 4.5, 0.0, Some(0.0), Some(12.5), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 368, 100, 30, 0, 3.5, 0.0, 5.5, 5.0, Some(0.0), Some(10.0), Some(5.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 1.0 + pos_diff, y: 16.0}, 4, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 1.0 + pos_diff, y: 16.0}, 4, false);
    }
    frame(lua_state, 14.5); // 9
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 16.0); // 11
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0); // 14
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let pos_diff: f32 = speed*4.75;
        VarModule::on_flag(agent.battle_object, vars::bayonetta::status::SPECIAL_1F_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 368, 100, 30, 0, 3.5, 0.0, 13.0, 0.5, Some(0.0), Some(13.0), Some(0.5), 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new( "top"), 3.0, 368, 100, 30, 0, 2.85, 0.0, 23.7, 0.5, Some(0.0), Some(23.7), Some(0.5), 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 368, 100, 30, 0, 4.85, 0.0, 17.0, 0.5, Some(0.0), Some(20.3), Some(0.5), 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("trans"), &Vector2f{x: 1.0 + pos_diff, y: 35.0}, 8, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("trans"), &Vector2f{x: 1.0 + pos_diff, y: 35.0}, 8, false);
        AttackModule::set_vec_target_pos(boma, 2, Hash40::new("trans"), &Vector2f{x: 1.0 + pos_diff, y: 35.0}, 8, false);
    }
    frame(lua_state, 24.0); // 19
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if agent.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 15, 35, 0, 40, 3.5, 0.0, 23.5, 0.5, Some(0.0), Some(14.0), Some(0.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.5, 15, 35, 0, 40, 5.5, 0.0, 18.5, 0.5, Some(0.0), Some(19.5), Some(0.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 32, 55, 0, 45, 3.5, 0.0, 23.5, 0.5, Some(0.0), Some(14.0), Some(0.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.5, 32, 55, 0, 45, 5.5, 0.0, 18.5, 0.5, Some(0.0), Some(19.5), Some(0.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 27.0); // 22, 35 faf?
    FT_MOTION_RATE_RANGE(agent, 27.0, 36.0, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2bfb02b69a), true);
        let prev_inflict_status = VarModule::get_int(agent.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        && prev_inflict_status & *COLLISION_KIND_MASK_HIT == 0 {
            VarModule::inc_int(boma.object(), vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
        }
    }
    frame(lua_state, 30.46); // 27
    frame(lua_state, 31.15); // 28
    if is_excute(agent) {
        let prev_inflict_status = VarModule::get_int(agent.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        || prev_inflict_status & *COLLISION_KIND_MASK_HIT != 0 {
            CancelModule::enable_cancel(boma);
        }
    }
    frame(lua_state, 31.5); // 29
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    frame(lua_state, 32.53); // 30
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 36.0); // 35
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            FOOT_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, -0.5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, -0.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND, Hash40::new("top"), 0, 25.1, 0, 0, 0, 0, 0.84, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("top"), 0, 24.7, 0, -90, 0, 0, 0.82, true);
        if agent.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE) {
            EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_SPIRAL, Hash40::new("top"), 0, 25.3, 0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(agent, 1.8);
        }
    }
    frame(lua_state, 28.1);
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
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_bayonetta_rnd_attack01"));
        PLAY_SE(agent, Hash40::new("se_bayonetta_special_h01"));
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_attackair_f02"));
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_special_h02"));
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackss"), 6);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) { // 14
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 30, 0, 1, 0, false); // max rounds?, rounds until flinch spawns, first shoot frame, frames to return after releasing BA, something ab vibrating??
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 30);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 30);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 30);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_KEEP);
        let speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let pos_diff: f32 = speed*4.5;
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 368, 100, 30, 0, 3.5, 0.0, 13.0, 0.5, Some(0.0), Some(13.0), Some(0.5), 0.75, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 368, 100, 30, 0, 2.85, 0.0, 23.7, 0.5, Some(0.0), Some(23.7), Some(0.5), 0.75, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 368, 100, 30, 0, 4.85, 0.0, 17.0, 0.5, Some(0.0), Some(20.3), Some(0.5), 0.75, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("trans"), &Vector2f{x: 1.5 + pos_diff, y: 29.0}, 6, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("trans"), &Vector2f{x: 1.5 + pos_diff, y: 29.0}, 6, false);
        AttackModule::set_vec_target_pos(boma, 2, Hash40::new("trans"), &Vector2f{x: 1.5 + pos_diff, y: 29.0}, 6, false);
    }
    frame(lua_state, 7.0); // 20
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.75, 120, 100, 65, 0, 3.5, 0.0, 13.0, 0.5, None, None, None, 0.75, 0.75, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.75, 260, 100, 10, 0, 2.85, 0.0, 23.7, 0.5, None, None, None, 0.75, 0.75, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.75, 140, 100, 60, 0, 4.85, 0.0, 17.0, 0.5, Some(0.0), Some(20.3), Some(0.5), 0.75, 0.75, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 14.7, false);
    }
    frame(lua_state, 15.0); // 28
    if is_excute(agent) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
        AttackModule::clear(boma, 2, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 40, 100, 0, 45, 3.5, 0.0, 23.5, 0.5, Some(0.0), Some(14.0), Some(0.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.5, 40, 100, 0, 45, 5.5, 0.0, 18.5, 0.5, Some(0.0), Some(19.5), Some(0.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0); // 31
    FT_MOTION_RATE_RANGE(agent, 18.0, 28.0, 15.0); // looks better
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2bfb02b69a), true);
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
        let prev_inflict_status = VarModule::get_int(agent.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
        if !agent.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_NO_SHOOTING_ENABLE_CANCEL)
        && prev_inflict_status & *COLLISION_KIND_MASK_HIT == 0 {
            VarModule::inc_int(boma.object(), vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
        }
    }
    frame(lua_state, 22.0); // 37, stall for a bit
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    frame(lua_state, 24.0); // 40
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_KEEP);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        agent.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP);
    }
    frame(lua_state, 24.66); // 41
    if is_excute(agent) {
        let prev_inflict_status = VarModule::get_int(agent.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
        if VarModule::is_flag(agent.battle_object, vars::bayonetta::status::SPECIAL_HIT_NO_BULLET)
        || prev_inflict_status & *COLLISION_KIND_MASK_HIT != 0 {
            CancelModule::enable_cancel(boma);
        }
    }
    frame(lua_state, 26.0); // 43
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES); //  (12 after)
    }
    frame(lua_state, 28.0); // 49 FAF
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND, Hash40::new("top"), 0, 25.1, 0, 0, 0, 0, 0.84, true);
        LAST_EFFECT_SET_RATE(agent, 0.85);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("top"), 0, 24.7, 0, -90, 0, 0, 0.82, true);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_SPIRAL, Hash40::new("top"), 0, 25.3, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        let mut twist = "bayonetta_witchtwist_wind_blue";
        if agent.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) == 2 {twist = "bayonetta_witchtwist_wind_red"; }
        agent.clear_lua_stack();
        lua_args!(agent, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new(twist), true, true);
        sv_module_access::effect(agent.lua_state_agent);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_SPIRAL, true, true);
    }
}

unsafe extern "C" fn sound_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_attackhard_s02"));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_attackhard_s01"));
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_bayonetta_special_h02"));
    }
}

unsafe extern "C" fn expression_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackss"), 6);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 2.0);
	frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 8.0, 3.0);
	if is_excute(agent) {
		agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
	}
    frame(lua_state, 8.0); // 4
    FT_MOTION_RATE_RANGE(agent, 8.0, 44.0, 25.0);
    frame(lua_state, 9.0); // 5
	if is_excute(agent) {
        agent.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        SEARCH(agent, 0, 0, Hash40::new("top"), 11.5, -2.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), true);// BAT FAF 60 normal faf 37 -> 50 / 40
    }
    frame(lua_state, 9.5); // 6
	if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), true);
    }
	frame(lua_state, 18.0); // 11
	if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), false);
    }
	frame(lua_state, 26.7); // 17
	if is_excute(agent) {
		agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 44.0); // 29, faf 45
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
		agent.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
	}
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0.75);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_witchtime_flash"), Hash40::new("top"), -2.0, 10.0, 0.0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 10.0/14.0);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstarth", game_specialnstarth, Priority::Low);
    agent.acmd("game_specialnstartf", game_specialnstarth, Priority::Low);
    agent.acmd("game_specialairnstarth", game_specialnstarth, Priority::Low);
    agent.acmd("game_specialairnstartf", game_specialnstarth, Priority::Low);
    agent.acmd("game_specialnchargeh", game_specialnchargeh, Priority::Low);
    agent.acmd("game_specialnchargef", game_specialnchargeh, Priority::Low);
    agent.acmd("game_specialairnchargeh", game_specialnchargeh, Priority::Low);
    agent.acmd("game_specialairnchargef", game_specialnchargeh, Priority::Low);
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

    agent.acmd("game_specialairsd", game_specialairsd, Priority::Low);
    agent.acmd("effect_specialairsd", effect_specialairsd, Priority::Low);

    agent.acmd("game_specialairsdlanding", game_specialairsdlanding, Priority::Low);

    agent.acmd("game_specialairsuwallend", game_specialairsbounce, Priority::Low);
    agent.acmd("game_specialairsdhit", game_specialairsbounce, Priority::Low);
    agent.acmd("game_specialairsdwallend", game_specialairsbounce, Priority::Low);

    agent.acmd("game_specialairsdlandingedge", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairsdlandingedge", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairsdlandingedge", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairsdlandingedge", acmd_stub, Priority::Low);
    
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi, Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi, Priority::Low);
    agent.acmd("expression_specialhi", expression_specialhi, Priority::Low);
    
    agent.acmd("game_specialairhi", game_specialairhi, Priority::Low);
    agent.acmd("effect_specialairhi", effect_specialairhi, Priority::Low);
    agent.acmd("sound_specialairhi", sound_specialairhi, Priority::Low);
    agent.acmd("expression_specialairhi", expression_specialairhi, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw, Priority::Low);
}
