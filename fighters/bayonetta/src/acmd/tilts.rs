use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 22.0, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, true, false, false, 10, 0, 10, 5, true);
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 24.0); // 9
    FT_MOTION_RATE_RANGE(agent, 24.0, 30.0, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 60, 25, 0, 33, 3.5, -4.2, 1.0, 1.9, Some(-4.2), Some(1.0), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 4.0, 80, 25, 0, 33, 4.8, -2.7, 1.0, 3.2, Some(3.7), Some(1.0), Some(3.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE_RANGE(agent, 30.0, 48.0, 15.0); // 26
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            CHECK_BA(agent, true);
            agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    }
    frame(lua_state, 30.1);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 48.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_LINE2, Hash40::new("top"), 0, 11.75, -2.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_SCALE_W(agent, 1.25, 1.125, 1.25);
        LAST_EFFECT_SET_RATE(agent, 0.45);
        let effect = EffectModule::get_last_handle(boma) as u64;
        VarModule::set_int64(agent.battle_object, vars::bayonetta::status::EFFECT_HANDLE, effect);
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 25.25);
    if is_excute(agent) {
        let effect = VarModule::get_int64(agent.battle_object, vars::bayonetta::status::EFFECT_HANDLE);
        EffectModule::set_pos(boma, effect as u32, &Vector3f{x: 0.0, y: 11.75, z: -1.0});
    }
    frame(lua_state, 26.5);
    if is_excute(agent) {
        let effect = VarModule::get_int64(agent.battle_object, vars::bayonetta::status::EFFECT_HANDLE);
        EffectModule::set_pos(boma, effect as u32, &Vector3f{x: 0.0, y: 11.75, z: -0.5});
    }
    frame(lua_state, 27.75);
    if is_excute(agent) {
        let effect = VarModule::get_int64(agent.battle_object, vars::bayonetta::status::EFFECT_HANDLE);
        EffectModule::set_pos(boma, effect as u32, &Vector3f{x: 0.0, y: 11.75, z: 0.0});
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        let effect = VarModule::get_int64(agent.battle_object, vars::bayonetta::status::EFFECT_HANDLE);
        EffectModule::set_rate(boma, effect as u32, 2.0);
        EffectModule::set_scale(boma, effect as u32, &Vector3f::new(1.1, 0.85, 1.1));
        EffectModule::set_pos(boma, effect as u32, &Vector3f{x: 0.0, y: 11.75, z: 3.7});
    } //quick fadeout effect??
}

unsafe extern "C" fn expression_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

unsafe extern "C" fn game_attacks32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.8);
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 16.75, 8.0);
    if is_excute(agent) {
        KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION); //prevents diff combo timings changing momentum
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, true, false, false, 10, 0, 10, 5, true); // activates both legs for some reason??? but with no hitbox
    }
    frame(lua_state, 16.75);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 18.75); //11
    FT_MOTION_RATE_RANGE(agent, 18.75, 23.0, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 5.0, 52, 20, 0, 35, 4.25, -1.5, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 5.0, 66, 20, 0, 35, 4.5, -2.75, 0.0, -2.75, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneel"), 5.0, 79, 20, 0, 35, 4.75, 4.25, 0.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("trans"), 5.0, 82, 20, 0, 35, 3.0, 0.0, 8.5, 6.5, Some(0.0), Some(9.5), Some(10.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE_RANGE(agent, 23.0, 47.0, 12.0); // 27faf
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        && CHECK_BA(agent, false);
            agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 47.0);
    FT_MOTION_RATE_RANGE(agent, 47.0, 78.0, 27.0); // for aesthetics only
}

unsafe extern "C" fn effect_attacks32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC1, Hash40::new("top"), 0, 12, 1, 13, -20, 155, 1.00, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_attacks32(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 3);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

unsafe extern "C" fn game_attacks33(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.95);
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 21.0, 11.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, true, false, false, 10, 0, 10, 5, true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, false, 10);
        KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    frame(lua_state, 21.0); // 12
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 6.0, 78, 158, 0, 45, 5.0, -1.0, 0.0, 4.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 6.0, 78, 158, 0, 45, 5.0, -0.5, 0.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneer"), 6.0, 78, 158, 0, 45, 5.0, 5.5, 0.0, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::set_offset(boma, 1, &Vector3f{ x: -0.5, y: 0.0, z: 3.0});
        AttackModule::set_offset(boma, 2, &Vector3f{ x: 5.5, y: 0.0, z: 2.0});
    }
    frame(lua_state, 26.0); // 16
    FT_MOTION_RATE_RANGE(agent, 26.0, 50.0, 16.0); // 33
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        CHECK_BA(agent, true);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(lua_state, 50.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attacks33(agent: &mut L2CAgentBase) { //closer to P -> K's angle
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 19.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_LINE2, Hash40::new("top"), 0, 11.2, 5.1, -64, 0, 0, 1.08, true);
        LAST_EFFECT_SET_RATE(agent, 0.65);
        let effect = EffectModule::get_last_handle(boma) as u64;
        VarModule::set_int64(agent.battle_object, vars::bayonetta::status::EFFECT_HANDLE, effect);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        let effect = VarModule::get_int64(agent.battle_object, vars::bayonetta::status::EFFECT_HANDLE);
        EffectModule::set_rate(boma, effect as u32, 2.0);
        EffectModule::set_scale(boma, effect as u32, &Vector3f::new(1.08, 0.8, 1.08));
        EffectModule::set_pos(boma, effect as u32, &Vector3f{x: 0.5, y: 12.2, z: 6.0});
    }
}

unsafe extern "C" fn expression_attacks33(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.75, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 0, 10, 5, true);
    }
    frame(lua_state, 6.75); // 7
    FT_MOTION_RATE_RANGE(agent, 6.75, 9.75, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 2.0, 368, 100, 30, 0, 3.25, 0.0, 0.0, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("handr"), 2.0, 368, 100, 30, 0, 3.75, 1.25, 0.0, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 1.0, y: 20.0}, 7, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 1.0, y: 20.0}, 8, false);
    }
    frame(lua_state, 7.5); // 8
    if is_excute(agent) {
        AttackModule::set_offset(boma, 1, &Vector3f{ x: 0.75, y: 0.0, z: 0.0});
    }
    frame(lua_state, 8.25); // 9
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 2.0, 368, 100, 30, 0, 3.25, 0.0, 0.0, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("handr"), 2.0, 368, 100, 30, 0, 3.75, 1.5, 0.0, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 1.0, y: 20.0}, 5, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 1.0, y: 20.0}, 5, false);
    }
    frame(lua_state, 9.0); // 10
    if is_excute(agent) {
        AttackModule::set_offset(boma, 1, &Vector3f{ x: 2.75, y: 0.0, z: 0.0});
    }
    frame(lua_state, 9.75); // 11
    FT_MOTION_RATE_RANGE(agent, 9.75, 12.0, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 97, 68, 0, 80, 3.5, 0.0, 0.0, -0.75, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("handr"), 6.0, 97, 68, 0, 80, 4.25, 3.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.3125); // 12
    if is_excute(agent) {
        AttackModule::set_offset(boma, 1, &Vector3f{ x: 4.25, y: 0.0, z: 0.0});
    }
    frame(lua_state, 12.0); // 15
    FT_MOTION_RATE_RANGE(agent, 12.0, 13.0, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        CHECK_BA(agent, true);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(lua_state, 13.0); // 18
    FT_MOTION_RATE_RANGE(agent, 13.0, 28.0, 14.0); // faf 32
    frame(lua_state, 28.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let facing = PostureModule::lr(boma);
    frame(lua_state, 5.5);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) == 2 {EFFECT_FOLLOW(agent, Hash40::new("bayonetta_attack_arc4_red"), Hash40::new("top"), 1.5, 14.1, -0.58, -90.0, 56, 20, 1.15, true); }
        else {EFFECT_FOLLOW(agent, Hash40::new("bayonetta_attack_arc4_blue"), Hash40::new("top"), 1.5, 14.1, -0.55, -90.0, 56, 20, 1.15, true); }
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.75);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 26, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(lua_state, 12.3);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, -10, -60, -10, 1, true);
        if agent.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) == 2 {LAST_EFFECT_SET_COLOR(agent, 1, 0.118, 0.118); }
        else {LAST_EFFECT_SET_COLOR(agent, 0.059, 0.38, 1); }
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("bayonetta_beretta_rotation"), true, true);
    }
}

unsafe extern "C" fn expression_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.5);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 12.0, 5.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, false, 10, 0, 15, 5, true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, false, 10);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, false, 10);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 5.0, 6.5);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0); // 26faf
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 7.0, 79, 80, 0, 55, 3.2, 1.0, 0.0, 0.4, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 7.0, 84, 80, 0, 55, 3.4, 1.0, 0.0, 0.2, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footr"), 8.0, 94, 80, 0, 55, 3.6, 1.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        CHECK_BA(agent, true);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
}

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC1, Hash40::new("top"), 0.0, 3.55, 3.1, 2.4, 3, 2, 1.31, true);
        LAST_EFFECT_SET_RATE(agent, 1.05);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 5.7, 0, 0, 0, 0, 0, 1.05, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3, Priority::Low);
    agent.acmd("effect_attacks3", effect_attacks3, Priority::Low);
    agent.acmd("expression_attacks3", expression_attacks3, Priority::Low);

    agent.acmd("game_attacks32", game_attacks32, Priority::Low);
    agent.acmd("effect_attacks32", effect_attacks32, Priority::Low);
    agent.acmd("expression_attacks32", expression_attacks32, Priority::Low);

    agent.acmd("game_attacks33", game_attacks33, Priority::Low);
    agent.acmd("effect_attacks33", effect_attacks33, Priority::Low);
    agent.acmd("expression_attacks33", expression_attacks33, Priority::Low);

    agent.acmd("game_attackhi3", game_attackhi3, Priority::Low);
    agent.acmd("effect_attackhi3", effect_attackhi3, Priority::Low);

    agent.acmd("game_attacklw3", game_attacklw3, Priority::Low);
    agent.acmd("effect_attacklw3", effect_attacklw3, Priority::Low);
    agent.acmd("expression_attacklw3", expression_attacklw3, Priority::Low);
}