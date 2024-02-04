
use super::*;


unsafe extern "C" fn wario_catch_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        VarModule::on_flag(boma.object(), vars::common::status::PUMMEL_OVERRIDE_GLOBAL_STATS);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("top"), 0.6, 361, 100, 30, 0, 5.0, 0.0, 10.0, 7.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
        AttackModule::set_damage_shake_scale(boma, 0.0);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_NONE, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MotionModule::set_frame(boma.get_grabbed_opponent_boma(), 0.1, false);
    }
    wait(lua_state,1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 361, 100, 30, 0, 5.0, 0.0, 10.0, 7.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
    }
}

unsafe extern "C" fn wario_catch_attack_effect(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 7.25, 11.0, 260, 0, 0, 1.1, true);
    }
}


unsafe extern "C" fn wario_throw_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 89, 135, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 100, 0, 30, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(fighter, -2, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(fighter, 21.0/(55.0-30.0));
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 55.0);
    FT_MOTION_RATE(fighter, 1.0);
}


pub const THROWHI_FRAME_FALL: f32 = 48.0;
pub const THROWHI_FRAME_LAND: f32 = 55.0;

unsafe extern "C" fn wario_throw_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        CameraModule::set_enable_camera(boma,true,0);
        CameraModule::set_enable_update_pos(boma,*CAMERA_UPDATE_POS_Y as u8,0);
        FT_LEAVE_NEAR_OTTOTTO(fighter, -2, 3);

        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 13.0, 80, 80, 0, 72, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    //Affect hitbox size based on scale
    let opponent = boma.get_grabbed_opponent_boma();
    let opponentScale = PostureModule::scale(opponent);
    let weight =  WorkModule::get_param_float(opponent, hash40("weight"), 0);
    let opponent_kind = utility::get_kind(&mut *opponent);
    
    //Factor in mini and megashroom
    let extraLarge = opponentScale > 0.5
    && (opponentScale > 1.5 || [
        *FIGHTER_KIND_KOOPA,
        *FIGHTER_KIND_KROOL,
        *FIGHTER_KIND_DONKEY,
        *FIGHTER_KIND_DEDEDE,
        *FIGHTER_KIND_GANON,
        *FIGHTER_KIND_LIZARDON,
        *FIGHTER_KIND_RIDLEY,
        *FIGHTER_KIND_ROBOT,
        *FIGHTER_KIND_ROSETTA //sorry about this one
    ].contains(&opponent_kind)); 
    let extraHeavy = weight>110.0;

    //Multiply hitbox size if opponent is mega, or opponent is of a large fighter kind
    let factorSize = (if extraLarge {1.5} else {1.0})*opponentScale.max(1.0);
    //Multiply power if large/heavy
    let factorPower = if extraLarge || extraHeavy {1.2} else {1.0};
    
    //Add rate based on item status
    let mut addRate = 0.0;
    if opponentScale != PostureModule::scale(fighter.boma())
    {
        addRate = if {opponentScale > PostureModule::scale(fighter.boma())} {0.25} else {-0.25};
    }
    let leadHead = WorkModule::is_flag(opponent, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAL)
    || WorkModule::is_flag(opponent, *FIGHTER_INSTANCE_WORK_ID_FLAG_GOLD);
    if leadHead {addRate += 0.375};
    
    let factorRate = (if extraLarge || extraHeavy {1.375} else {1.0})+addRate;
    let rise_speedUp = 0.75;

    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, factorRate*rise_speedUp);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }

    //Going up//
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("throw"), 8.0, 50, 70, 0, 100, 5.5*factorSize, 0.0, -5.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }

    frame(lua_state, 38.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    FT_MOTION_RATE(fighter, 0.75);

    frame(lua_state, THROWHI_FRAME_FALL);
    if is_excute(fighter) {
        
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::suspend_energy(boma,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);

        let speed = smash::phx::Vector3f { x: 0.0, y: -3.75, z: 0.0 };
        KineticModule::add_speed(boma, &speed);

        ATTACK_IGNORE_THROW(fighter, 1, 0, Hash40::new("rot"), 8.0*factorPower, 50, 70, 0, 100, 6.0*factorSize, 0.0, 0.0, 0.0, Some(0.0), Some(2.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }
    
    frame(lua_state, THROWHI_FRAME_LAND+1.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 10.0*factorPower, 270, 90, 0, 15, 2.5*factorSize, 0.0, 0.0, -3.5, Some(0.0), Some(0.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);

        ATTACK_IGNORE_THROW(fighter, 1, 0, Hash40::new("rot"), 7.0, 65, 95, 0, 85, 8.25*factorSize, 0.0, 0.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);

        StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_GROUND), false);
        CHECK_FINISH_CAMERA(fighter, 9, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 3.0, y: -3.0, z: 0.0});

    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        let opponent = boma.get_grabbed_opponent_boma();
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}


unsafe extern "C" fn wario_throw_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let opponent = boma.get_grabbed_opponent_boma();
    let opponentScale = PostureModule::scale(opponent);

    frame(lua_state, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, -0.4, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 3, 0, 0, -90, 0, 1, true, *EF_FLIP_YZ, 1);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 2, 0, 0, -90, 0, (opponentScale+0.4), true, *EF_FLIP_YZ, 0.6);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 4, 0, 0, -90, 0, 1, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter,1.25);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 5, 0, 0, -90, 0, (opponentScale+0.4), true, *EF_FLIP_YZ, 0.6);
        LAST_EFFECT_SET_RATE(fighter,1.25);
    }
    frame(lua_state, THROWHI_FRAME_FALL+1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("rot"), 0, 20, 0, 90, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(lua_state, THROWHI_FRAME_LAND+3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}


unsafe extern "C" fn wario_throw_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_wario_jump02"));
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, THROWHI_FRAME_FALL-5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_wario_006"));
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
    frame(lua_state, THROWHI_FRAME_LAND);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_heavy_hit_l"));
    }
}


unsafe extern "C" fn wario_throw_hi_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 13);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, THROWHI_FRAME_FALL);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, THROWHI_FRAME_LAND);
    if is_excute(fighter) {
        //QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}



unsafe extern "C" fn wario_throw_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 30, 60, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 14, 3);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 7.5, y: 1.0, z: 0.0});
    }
    wait(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 32.0/(55.0-13.0));
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 55.0);
    FT_MOTION_RATE(fighter, 1.0);
}


unsafe extern "C" fn wario_throw_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false); 
    }
    //frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn wario_throw_f_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_wario_rnd_attack"));
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

unsafe extern "C" fn wario_throw_f_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        lua_args!(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        smash::app::sv_animcmd::FT_ATTACK_ABS_CAMERA_QUAKE(lua_state);
        fighter.clear_lua_stack();
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
}




pub fn install() {
    smashline::Agent::new("wario")
        .acmd("game_catchattack", wario_catch_attack_game)
        .acmd("effect_catchattack", wario_catch_attack_effect)
        .acmd("game_throwlw", wario_throw_lw_game)
        .acmd("game_throwhi", wario_throw_hi_game)
        .acmd("effect_throwhi", wario_throw_hi_effect)
        .acmd("sound_throwhi", wario_throw_hi_sound)
        .acmd("expression_throwhi", wario_throw_hi_expression)
        .acmd("game_throwf", wario_throw_f_game)
        .acmd("effect_throwf", wario_throw_f_effect)
        .acmd("sound_throwf", wario_throw_f_sound)
        .acmd("expression_throwf", wario_throw_f_expression)
        .install();
}
