
use super::*;


#[acmd_script( agent = "kirby", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.2);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.1, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(9.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "kirby", script = "game_catchdash" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_catch_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.1, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(12.25), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "kirby", script = "game_catchturn" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_catch_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.1, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(-17.25), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "kirby", script = "game_throwf", category = ACMD_GAME , low_priority)]
unsafe fn kirby_throw_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 75, 125, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 7, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.8);
        //FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        // reduce fthrow bounce height
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        // enable aerial drift before fthrow anim is over
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

#[acmd_script( agent = "kirby", script = "game_throwb", category = ACMD_GAME , low_priority)]
unsafe fn kirby_throw_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 115, 120, 0, 30, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, -7, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.8);
        //FighterCutInManager::set_throw_finish_offset(boma, 0, -10, 0);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "kirby", script = "game_throwhi", category = ACMD_GAME, low_priority )]
unsafe fn kirby_throw_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_LEAVE_NEAR_OTTOTTO(fighter, -2.5, 2.5);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 14.0, 78, 74, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_THROW_FLAG_STOP);
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 7.0, 65, 95, 0, 85, 9.5, 0.0, 6.5, 2.0, Some(1.0), Some(1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::set_float(boma, 5.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FINISH_CAMERA_THROW_RAY_LENGTH);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RAY_CHECK_FINISH_CAMERA_THROW);
        CHECK_FINISH_CAMERA(fighter, 15, 7);
        //lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.8);
        //lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    frame(lua_state, 51.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

#[acmd_script( agent = "kirby", script = "game_throwlw", category = ACMD_GAME , low_priority)]
unsafe fn kirby_throw_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 270, 100, 30, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 9.0);
    for _ in 0..9 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 270, 100, 10, 0, 5.8, 0.0, 4.0, 3.2, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_catch_only_all(boma, true, false);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 2.0);
    }
    frame(lua_state, 56.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 90, 100, 0, 10, 5.8, 0.0, 4.0, 2.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(fighter, 8, 4);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.8);
        //FighterCutInManager::set_throw_finish_offset(boma, 0, 0, 0);
    }
    frame(lua_state, 58.0);
    FT_MOTION_RATE(fighter, 0.75);
    if is_excute(fighter) {
        ModelModule::set_joint_translate(boma, Hash40::new("throw"), &Vector3f{x: 0.0, y: -5.0, z: 6.868900}, false, false);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(boma);
        let opponent_boma = fighter.get_grabbed_opponent_boma();
        VarModule::on_flag(opponent_boma.object(), vars::common::instance::IS_KNOCKDOWN_THROW);
    }
    frame(lua_state, 68.0); {
        if is_excute(fighter) {
            CancelModule::enable_cancel(boma);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        kirby_catch_game,
        kirby_catch_dash_game,
        kirby_catch_turn_game,
        kirby_throw_f_game,
        kirby_throw_b_game,
        kirby_throw_hi_game,
        kirby_throw_lw_game,
    );
}

