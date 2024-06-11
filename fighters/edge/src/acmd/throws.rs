use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 6.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.2, 0.0, 8.5, 4.7, Some(0.0), Some(8.5), Some(9.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.2, 0.0, 7.0, 4.7, Some(0.0), Some(7.0), Some(11.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.2, 0.0, 7.0, -4.7, Some(0.0), Some(7.0), Some(-14.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.5, 58, 105, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_THROW_FLAG_REVERSE_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 80, 0, 60, 7.5, 0.0, 11.0, -13.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(agent, -15.0, 8);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 18.0, 12.0);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 98, 138, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE_RANGE(agent, 18.0, 40.0, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 75, 90, 0, 60, 7.0, 0.0, 20.0, 0.0, Some(0.0), Some(40.0), Some(0.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(agent, 15, 14);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light2"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 3);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 12, 0, 0.8, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 12, 0, 0.8, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FLW_POS(agent, Hash40::new("edge_attack_speedline"), Hash40::new("swordl2"), -15.0, 0, 1.5, 90, 0, 180, 0.9, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.4);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        EFFECT(agent, Hash40::new("edge_sword_flash2"), Hash40::new("swordl2"), 12, 0, 0.8, 0, 0, 90, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT(agent, Hash40::new("edge_attack_dash2"), Hash40::new("swordl2"), 12.0, 0, 0.8, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.0);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_attack_speedline"), false, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("edge_attack_dash"), -1);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("edge_attack_dash_aura"), -1);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_attack_dash"), false, false);
    }
}

unsafe extern "C" fn sound_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_attackhard_h01"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_edge_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_edge_attackhard_h02"));
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_attackhard_h03"));
    }
}

unsafe extern "C" fn expression_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("top"), AttackDirectionAxis(*ATTACK_DIRECTION_X_MINUS), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_78_slash"), 12);
    }
}

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 67, 60, 0, 77, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE_RANGE(agent, 18.0, 40.0, 12.0);
    frame(lua_state, 32.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 11.0, 0.0);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn effect_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_catch_handaura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if PostureModule::lr(boma) < 0.0 {
            EFFECT(agent, Hash40::new("edge_throwlw_gravity"), Hash40::new("top"), 12, 0.1, -2, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        else{
            EFFECT(agent, Hash40::new("edge_throwlw_gravity"), Hash40::new("top"), 12, 0.1, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.2);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 12, 0.1, -2, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 12, 0.1, -2, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 12, 0.1, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 12, 0.1, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 12, 0.1, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_catch_handaura"), false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch, Priority::Low);
    agent.acmd("game_catchdash", game_catchdash, Priority::Low);
    agent.acmd("game_catchturn", game_catchturn, Priority::Low);

    agent.acmd("game_throwb", game_throwb, Priority::Low);

    agent.acmd("game_throwhi", game_throwhi, Priority::Low);
    agent.acmd("effect_throwhi", effect_throwhi, Priority::Low);
    agent.acmd("sound_throwhi", sound_throwhi, Priority::Low);
    agent.acmd("expression_throwhi", expression_throwhi, Priority::Low);
    
    agent.acmd("game_throwlw", game_throwlw, Priority::Low);
    agent.acmd("effect_throwlw", effect_throwlw, Priority::Low);
}