use super::*;

unsafe extern "C" fn game_final(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.65);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        CHECK_VALID_FINAL_START_CAMERA(agent, 0, 0, 20, 0, 0, 0);
        SLOW_OPPONENT(agent, 10.0, 70.0);
    }
    if !boma.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(lua_state, 5.0);
        if is_excute(agent) {
            FT_SET_FINAL_FEAR_FACE(agent, 40);
            REQ_FINAL_START_CAMERA(agent, Hash40::new("d04final.nuanmb"), true);
            FT_START_CUTIN(agent);
        }
    } else {
        if is_excute(agent) {
            FT_START_CUTIN(agent);
        }
        if is_excute(agent) {
            camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            CAM_ZOOM_IN_arg5(agent, 1.8, 0.0, PostureModule::scale(boma) * 3.0, 0.0, 0.0);
        }
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 365, 100, 48, 17, 11.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 365, 100, 48, 17, 11.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_no_uniq_effect_all(boma, true, false);
        AttackModule::set_damage_shake_scale(boma, 0.18);
    }
    if(PostureModule::scale(boma) > 1.4){
        if is_excute(agent) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 0.0}, 5, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 0.0}, 5, false);
        }
    } else {
        if(PostureModule::scale(boma) <= 0.5){
            if is_excute(agent) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 20.0, y: 0.0}, 2, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 20.0, y: 0.0}, 2, false);
            }
        } else {
            if is_excute(agent) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 13.0, y: 5.0}, 5, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 4.0}, 10, false);
            }
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 22.0);
    frame(lua_state, 25.0);
    if is_excute(agent) {
        CAM_ZOOM_OUT(agent);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
    if(PostureModule::scale(boma) > 1.4){
        if is_excute(agent) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 12.0, y: 5.0}, 13, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 20.0}, 5, false);
        }
    } else {
        if(PostureModule::scale(boma) < 0.5){
            if is_excute(agent) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 28.0, y: 5.0}, 13, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 37.0, y: 10.0}, 15, false);
            }
        } else {
            if is_excute(agent) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 1.0}, 9, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 19.0, y: 5.0}, 15, false);
            }
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
    if(PostureModule::scale(boma) > 1.4){
        if is_excute(agent) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 13.0, y: 4.0}, 13, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 13.0, y: 7.0}, 15, false);
        }
    } else {
        if(PostureModule::scale(boma) < 0.5){
            if is_excute(agent) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 37.0, y: 4.0}, 13, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 37.0, y: 7.0}, 15, false);
            }
        } else {
            if is_excute(agent) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 20.0, y: 2.0}, 13, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 20.0, y: 4.0}, 15, false);
            }
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
    if(PostureModule::scale(boma) > 1.4){
        if is_excute(agent) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 13.0, y: 8.0}, 10, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 13.0, y: 10.0}, 15, false);
        }
    } else {
        if(PostureModule::scale(boma) < 0.5){
            if is_excute(agent) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 46.0, y: 8.0}, 10, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 46.0, y: 12.0}, 15, false);
            }
        } else {
            if is_excute(agent) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 22.0, y: 4.0}, 10, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 22.0, y: 7.0}, 15, false);
            }
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
    if(PostureModule::scale(boma) > 1.4){
        if is_excute(agent) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 14.0, y: 8.0}, 14, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 16.0, y: 10.0}, 15, false);
        }
    } else {
        if(PostureModule::scale(boma) < 0.5){
            if is_excute(agent) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 44.0, y: 8.0}, 14, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 52.0, y: 10.0}, 15, false);
            }
        } else {
            if is_excute(agent) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 22.0, y: 4.0}, 14, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 26.0, y: 5.0}, 15, false);
            }
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_LOCK_ATTACK);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 50, 95, 40, 0, 11.0, 0.0, 8.0, 8.0, Some(0.0), Some(10.0), Some(8.0), 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
    }
    // PostureModule::scale(boma, 0);
    // 0x16e550(1760657085, 1.4);
    // PostureModule::scale(boma);
    // 0x16e550(0, 0.5);
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_BRANCH_HIT);
        SlowModule::clear_whole(boma);
    }
}

unsafe extern "C" fn game_finalhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.65);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        CAM_ZOOM_OUT(agent);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.3, 367, 100, 60, 0, 9.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL02, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_REMOVE_FINAL_AURA);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_ATTACK_END_SET_PARAM);
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.3, 367, 100, 50, 0, 10.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(14.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL02, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_ATTACK_END_SET_PARAM);
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.3, 367, 100, 40, 0, 10.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(14.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL02, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_ATTACK_END_SET_PARAM);
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.3, 367, 100, 40, 0, 10.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(14.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL02, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_ATTACK_END_SET_PARAM);
    }
    if(PostureModule::scale(boma) > 1.4){
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 80, 100, 80, 0, 10.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(14.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL02, *ATTACK_REGION_KICK);
        }
    }
    else{
        if(PostureModule::scale(boma) < 0.5){
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 80, 100, 40, 0, 10.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(14.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL02, *ATTACK_REGION_KICK);
            }
        }
        else{
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 4.3, 80, 100, 60, 0, 10.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(14.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL02, *ATTACK_REGION_KICK);
            }
        }
    }
    if is_excute(agent) {
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_ATTACK_END_SET_PARAM);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_ATTACK_END);
    }
    if is_excute(agent) {
        camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 10, 0);
        CAM_ZOOM_IN_arg5(agent, 1.8, 0.0, PostureModule::scale(boma) * 3.0, 0.0, 0.0);
    }
    if is_excute(agent) {
        SlowModule::set_whole(boma, 2, 0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 85, 82, 0, 96, 10.0, 0.0, 17.0, 10.0, Some(0.0), Some(17.0), Some(15.0), 5.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL03, *ATTACK_REGION_KICK);
        AttackModule::set_force_reaction(boma, 0, true, false);
        boma.set_int(*FIGHTER_RYU_FINAL_CAMERA_OFFSET_7, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_CAMERA_OFFSET_TYPE);
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_CAMERA_OFFSET);
        // FILL_SCREEN_MODEL_COLOR(fighter, 1, 0, 1, 1, 1, 0, 0, 0, 0.9, 1.7, EffectScreenLayer:*CHAR, *EFFECT_SCREEN_PRIO_FINAL);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        SlowModule::clear_whole(boma);
        CANCEL_FILL_SCREEN(agent, 1, 10);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        CAM_ZOOM_OUT(agent);
        boma.set_int(*FIGHTER_RYU_FINAL_CAMERA_OFFSET_RETURN, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_CAMERA_OFFSET_TYPE);
    }
}

unsafe extern "C" fn game_final_com2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 100, 48, 17, 11.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 365, 100, 48, 17, 11.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_no_uniq_effect_all(boma, true, false);
        AttackModule::set_damage_shake_scale(boma, 0.18);
    }
}

unsafe extern "C" fn game_final_hit1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 25.0);
    if is_excute(agent) {
        CAM_ZOOM_OUT(agent);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
}

unsafe extern "C" fn game_final_hit2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
}

unsafe extern "C" fn game_final_hit3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 55.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
}

unsafe extern "C" fn game_final_hit4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 64.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
}

unsafe extern "C" fn game_final_hitfinal(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 76.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_LOCK_ATTACK);
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 50, 95, 40, 0, 11.0, 0.0, 8.0, 8.0, Some(0.0), Some(10.0), Some(8.0), 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
    }
}

unsafe extern "C" fn game_final2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0 / (13.0 - 1.0));
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 8.0 / (36.0 - 13.0));
    if is_excute(agent) {
        CHECK_VALID_FINAL_START_CAMERA(agent, 0, 0, 20, 0, 0, 0);
        SLOW_OPPONENT(agent, 10.0, 70.0);
    }
    if !boma.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(lua_state, 10.0);
        if is_excute(agent) {
            FT_START_CUTIN(agent);
            FT_SET_FINAL_FEAR_FACE(agent, 40);
            REQ_FINAL_START_CAMERA(agent, Hash40::new("d04final2.nuanmb"), true);
        }
    } else {
        if is_excute(agent) {
            // PostureModule::scale(boma, 3, 0);
            // 0x16e550(1760657085, 1.8);
            CAM_ZOOM_IN_arg5(agent, 1.8, 0.0, PostureModule::scale(boma) * 3.0, 0.0, 0.0);
            FT_START_CUTIN(agent);
        }
        frame(lua_state, 40.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
        }
    }
    frame(lua_state, 36.0);
    FT_MOTION_RATE(agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 100, 48, 17, 11.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 365, 100, 48, 17, 11.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_no_uniq_effect_all(boma, true, false);
        AttackModule::set_damage_shake_scale(boma, 0.18);
    }
    if(PostureModule::scale(boma) > 1.4){
        if is_excute(agent) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 6.0, y: 0.0}, 5, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 6.0, y: 0.0}, 5, false);
        }
    } else {
        if(PostureModule::scale(boma) <= 0.5){
            if is_excute(agent) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 11.0, y: 0.0}, 2, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 11.0, y: 0.0}, 2, false);
            }
        } else {
            if is_excute(agent) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 4.0, y: 5.0}, 5, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 6.0, y: 4.0}, 10, false);
            }
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 52.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        camera!(agent, *MA_MSC_CMD_CAMERA_CAM_RECT, 40, -40, 20, 0);
        boma.on_flag(*FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_ADJUST_SHINRYUKEN_POS);
    }
    frame(lua_state, 90.0);
}

unsafe extern "C" fn ken_shinryuken_effect_final(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 51.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ken_final_shinryuken_beam"), Hash40::new("top"), 0, 0, 8, 0, 90, 0, 1.0, false);
        EffectModule::set_scale_last(boma, &Vector3f::new(0.6, 0.36, 0.6));
    }
    frame(lua_state, 97.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("ken_final_shinryuken_beam"), -1);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_final", game_final, Priority::Low);
    agent.acmd("game_finalair", game_final, Priority::Low);
    agent.acmd("game_finalhit", game_finalhit, Priority::Low);
    agent.acmd("game_finalhit_com", game_finalhit, Priority::Low);
    agent.acmd("game_final_com2", game_final_com2, Priority::Low);
    agent.acmd("game_final_hit1", game_final_hit1, Priority::Low);
    agent.acmd("game_final_hit2", game_final_hit2, Priority::Low);
    agent.acmd("game_final_hit3", game_final_hit3, Priority::Low);
    agent.acmd("game_final_hit4", game_final_hit4, Priority::Low);
    agent.acmd("game_final_hitfinal", game_final_hitfinal, Priority::Low);
    
    agent.acmd("game_final2", game_final2, Priority::Low);
    agent.acmd("game_finalair2", game_final2, Priority::Low);
}
