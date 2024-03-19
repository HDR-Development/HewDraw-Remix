use super::*;
use smash2;

unsafe extern "C" fn game_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 9.0, 7.0, Some(0.0), Some(9.0), Some(11.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn demon_throw_hi_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
	    ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_attacks"), 0, false, 0 as u32);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 2.0);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_80_beam"), 0, false, 0 as u32);
    }
}

unsafe extern "C" fn demon_throw_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !smash2::app::FighterCutInManager::is_vr_mode() {
        if smash2::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const smash2::app::BattleObjectModuleAccessor)) {
            if is_excute(fighter) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(boma);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(boma);
            }
            if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if is_excute(fighter) {
                    CHECK_VALID_START_CAMERA(fighter, 0, 7, 0, 35, 0, 0, false);
                }
                if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                    if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                        if is_excute(fighter) {
                            REQ_MOTION_CAMERA(fighter, Hash40::new("e01throwb.nuanmb"), false);
                        }
                    }
                }
                if is_excute(fighter) {
                    let scale = PostureModule::scale(boma);
                    CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                }
            }
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        //ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 1.0, 82, 180, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 1.0, 270, 100, 30, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 58, 55, 0, 60, 4.0, 0.0, 2.0, -12.0, Some(0.0), Some(2.0), Some(-22.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(fighter, 17, 0);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        let opponent_boma = fighter.get_grabbed_opponent_boma();
        VarModule::on_flag(opponent_boma.object(), vars::common::instance::IS_KNOCKDOWN_THROW);
        CAM_ZOOM_OUT(fighter);
    }
    frame(lua_state, 48.0);
    FT_MOTION_RATE(fighter, 0.375);
}

unsafe extern "C" fn demon_blaster_fly_throw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 80, 65, 0, 85, 8.0, 0.0, 0.0, 1.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 80, 65, 0, 85, 8.0, 0.0, 0.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.1);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
    }
}

unsafe extern "C" fn demon_throw_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !smash2::app::FighterCutInManager::is_vr_mode() {
        if smash2::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const smash2::app::BattleObjectModuleAccessor)) {
            if is_excute(fighter) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(boma);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(boma);
            }
            if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if is_excute(fighter) {
                    CHECK_VALID_START_CAMERA(fighter, 0, 0, 0, 0, 0, 0, false);
                }
                if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                    if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                        if is_excute(fighter) {
                            REQ_MOTION_CAMERA(fighter, Hash40::new("e01throwlw.nuanmb"), false);
                        }
                    }
                }
                if is_excute(fighter) {
                    let scale = PostureModule::scale(boma);
                    CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                }
            }
        }
    }
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 1.0, 80, 135, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 50, 65, 0, 50, 6.0, 0.0, 12.0, 7.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(fighter, 9, 4);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        CAM_ZOOM_OUT(fighter);
    }
    frame(lua_state, 36.0);
    FT_MOTION_RATE(fighter, 0.9);
    
}

unsafe extern "C" fn demon_throw_command_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !smash2::app::FighterCutInManager::is_vr_mode() {
        if smash2::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const smash2::app::BattleObjectModuleAccessor)) {
            if is_excute(fighter) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(boma);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(boma);
            }
            if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if is_excute(fighter) {
                    CHECK_VALID_START_CAMERA(fighter, 0, 7, 0, 50, 30, 0, true);
                }
                if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                    if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                        if is_excute(fighter) {
                            REQ_MOTION_CAMERA(fighter, Hash40::new("e01throwcommand.nuanmb"), false);
                        }
                    }
                }
                if is_excute(fighter) {
                    let scale = PostureModule::scale(boma);
                    CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                }
            }
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 1.0, 165, 150, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 80, 0, 50, 5.0, 0.0, 8.0, -3.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 80.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_THROW_COMMAND_FLAG_USE_OTHER_PARAM) {
            CHECK_FINISH_CAMERA(fighter, 18, 2);
        }
        else {
            CHECK_FINISH_CAMERA(fighter, 18, 15);
        }
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 19, 50, 0, 30, 6.0, 0.0, 7.5, -14.0, Some(0.0), Some(6.0), Some(-14.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 19, 50, 0, 30, 4.0, 0.0, 8.0, -6.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 81.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        CAM_ZOOM_OUT(fighter);
    }
}

pub fn install() {
    smashline::Agent::new("demon_blaster")
        .acmd("game_flythrow", demon_blaster_fly_throw_game)
        .install();
    smashline::Agent::new("demon")
        .acmd("game_catch", game_catch)
        .acmd("expression_throwhi", demon_throw_hi_expression)
        .acmd("game_throwb", demon_throw_b_game)
        .acmd("game_throwlw", demon_throw_lw_game)
        .acmd("game_throwcommand", demon_throw_command_game)
        .install();
}
