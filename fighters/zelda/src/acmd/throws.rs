use super::*;

unsafe extern "C" fn zelda_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
	FT_MOTION_RATE(fighter, 0.7);
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 10.0);
	FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 9.0, 0.0, Some(0.0), Some(9.0), Some(10.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

unsafe extern "C" fn zelda_catch_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(12.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn zelda_catch_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 9.0, -4.0, Some(0.0), Some(9.0), Some(-18.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn zelda_throw_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 60, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 17, 4);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn zelda_throw_f_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("zelda_throw_trace"), Hash40::new("zelda_throw_trace"), Hash40::new("havel"), -0.5, 0, 0, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 1.0);
    for _ in 0..7 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.5, 8, 8, 8, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        wait(lua_state, 4.0);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("zelda_throw_flash"), Hash40::new("throw"), 0, 5, 0, 0, 0, 0, 1.1, 5, 5, 5, 0, 0, 0, true);
    }
}

unsafe extern "C" fn zelda_throw_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE_RANGE(fighter, 1.0, 26.0, 29.0);
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 45, 95, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(fighter, 11, 7);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn zelda_throw_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 87, 90, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 1, 25);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(fighter, 0.8);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

pub fn install() {
    smashline::Agent::new("zelda")
        .acmd("game_catch", zelda_catch_game)
        .acmd("game_catchdash", zelda_catch_dash_game)
        .acmd("game_catchturn", zelda_catch_turn_game)
        .acmd("game_throwf", zelda_throw_f_game)
        .acmd("effect_throwf", zelda_throw_f_effect)
        .acmd("game_throwb", zelda_throw_b_game)
        .acmd("game_throwhi", zelda_throw_hi_game)
        .install();
}
