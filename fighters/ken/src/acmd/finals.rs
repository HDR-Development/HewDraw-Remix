use super::*;

pub fn install() {
    install_acmd_scripts!(
        game_final
    );
}

#[acmd_script( agent = "ken", script = "game_final", category = ACMD_GAME, low_priority )]
unsafe fn game_final(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 0, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 10.0, 70.0);
    }
    if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            FT_SET_FINAL_FEAR_FACE(fighter, 40);
            REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), true);
            FT_START_CUTIN(fighter);
        }
    } else {
        if is_excute(fighter) {
            FT_START_CUTIN(fighter);
        }
        if is_excute(fighter) {
            camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            CAM_ZOOM_IN_arg5(fighter, 1.8, 0.0, PostureModule::scale(boma) * 3.0, 0.0, 0.0);
        }
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 365, 100, 48, 17, 11.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 365, 100, 48, 17, 11.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_no_uniq_effect_all(boma, true, false);
        AttackModule::set_damage_shake_scale(boma, 0.18);
    }
    if(PostureModule::scale(boma) > 1.4){
        if is_excute(fighter) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 0.0}, 5, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 0.0}, 5, false);
        }
    } else {
        if(PostureModule::scale(boma) <= 0.5){
            if is_excute(fighter) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 20.0, y: 0.0}, 2, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 20.0, y: 0.0}, 2, false);
            }
        } else {
            if is_excute(fighter) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 13.0, y: 5.0}, 5, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 4.0}, 10, false);
            }
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 22.0);
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
    if(PostureModule::scale(boma) > 1.4){
        if is_excute(fighter) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 12.0, y: 5.0}, 13, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 20.0}, 5, false);
        }
    } else {
        if(PostureModule::scale(boma) < 0.5){
            if is_excute(fighter) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 28.0, y: 5.0}, 13, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 37.0, y: 10.0}, 15, false);
            }
        } else {
            if is_excute(fighter) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 1.0}, 9, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 19.0, y: 5.0}, 15, false);
            }
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
    if(PostureModule::scale(boma) > 1.4){
        if is_excute(fighter) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 13.0, y: 4.0}, 13, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 13.0, y: 7.0}, 15, false);
        }
    } else {
        if(PostureModule::scale(boma) < 0.5){
            if is_excute(fighter) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 37.0, y: 4.0}, 13, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 37.0, y: 7.0}, 15, false);
            }
        } else {
            if is_excute(fighter) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 20.0, y: 2.0}, 13, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 20.0, y: 4.0}, 15, false);
            }
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
    if(PostureModule::scale(boma) > 1.4){
        if is_excute(fighter) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 13.0, y: 8.0}, 10, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 13.0, y: 10.0}, 15, false);
        }
    } else {
        if(PostureModule::scale(boma) < 0.5){
            if is_excute(fighter) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 46.0, y: 8.0}, 10, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 46.0, y: 12.0}, 15, false);
            }
        } else {
            if is_excute(fighter) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 22.0, y: 4.0}, 10, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 22.0, y: 7.0}, 15, false);
            }
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 64.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
    if(PostureModule::scale(boma) > 1.4){
        if is_excute(fighter) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 14.0, y: 8.0}, 14, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 16.0, y: 10.0}, 15, false);
        }
    } else {
        if(PostureModule::scale(boma) < 0.5){
            if is_excute(fighter) {
                AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 44.0, y: 8.0}, 14, false);
                AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 52.0, y: 10.0}, 15, false);
            }
        } else {
            if is_excute(fighter) {
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 22.0, y: 4.0}, 14, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: 26.0, y: 5.0}, 15, false);
            }
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 76.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_LOCK_ATTACK);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 50, 95, 40, 0, 11.0, 0.0, 8.0, 8.0, Some(0.0), Some(10.0), Some(8.0), 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(boma, true, false);
    }
    // PostureModule::scale(boma, 0);
    // 0x16e550(1760657085, 1.4);
    // PostureModule::scale(boma);
    // 0x16e550(0, 0.5);
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_BRANCH_HIT);
        SlowModule::clear_whole(boma);
    }
}