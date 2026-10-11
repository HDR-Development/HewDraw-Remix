use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.875);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.5, 0.0, 8.0, 7.0, Some(0.0), Some(8.0), Some(17.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 3.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.5, 0.0, 6.0, 4.0, Some(0.0), Some(6.0), Some(16.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 3.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.5, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(-18.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 3.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 40, 53, 0, 47, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE_RANGE(agent, 10.0, 23.0, 4.0);
    frame(lua_state, 23.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 26.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 33, 16);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 42, 77, 0, 62, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(agent, 18, 19);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_LEAVE_NEAR_OTTOTTO(agent, -3, 3);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 80, 185, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 60, 8.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 54.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 1, 0);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 59.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let pledge = if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
        let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE)
    } else { *PLEDGE_STATE_NONE };
    if is_excute(agent) {
        let (dmg, agl, kbg, fkb, bkb) = match (pledge) {
            _ if pledge == *PLEDGE_STATE_WATER => (11.0, 60, 30, 0, 85),
            _ if pledge == *PLEDGE_STATE_GRASS => (4.0, 74, 65, 0, 95),
            _ => (5.0, 67, 56, 0, 123)
        };
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, dmg, agl, kbg, fkb, bkb, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 12.0, 4.0);
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 20.0, 24.0, 1.0);
    frame(lua_state, 24.0);
    let game_frames = match (pledge) {
        _ if pledge == *PLEDGE_STATE_WATER => 7.0,
        _ if pledge == *PLEDGE_STATE_GRASS => 49.0,
        _ => 25.0
    };
    FT_MOTION_RATE_RANGE(agent, 24.0, 49.0, game_frames);
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
            let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
            let object = utils::util::get_battle_object_from_id(parent_id);
            if ![*PLEDGE_STATE_NONE, *PLEDGE_STATE_FIRE].contains(&VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE)) {
                let timer = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
                let pledge_use_cost_frame = ParamModule::get_int(agent.battle_object, ParamType::Agent, "param_special_lw.pledge_use_cost_frame");
                VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, timer - pledge_use_cost_frame);
            }
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 100, 0, 0, 5.0, 0.0, 4.0, 2.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 49.0);
    FT_MOTION_RATE_RANGE(agent, 49.0, 55.0, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 54.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 6, 0);
    }
    frame(lua_state, 55.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn effect_throwlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let pledge = if LinkModule::is_link(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
        let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE)
    } else { *PLEDGE_STATE_NONE };
    frame(lua_state, 11.0);
    if pledge == *PLEDGE_STATE_WATER {
        if is_excute(agent) {
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_steam2"), Hash40::new("head"), 0, 5, 0, 0, 0, 0, 0.7, false, 3.0);
            EFFECT_FOLLOW(agent, Hash40::new("sys_drown_out"), Hash40::new("mouth2"), 0, 0, 0, 180, 0, 0, 0.6, false);
        }
    }
    else if pledge == *PLEDGE_STATE_GRASS {
        let mut handle = 0;
        for _ in 0..2 {
            if is_excute(agent) {
                handle = EffectModule::req_follow(boma, Hash40::new("pfushigisou_atk_hi4"), Hash40::new("mouth2"), &Vector3f::zero(), &Vector3f::zero(), 0.0, false, 0, 0, 0, 0, 0, false, false);
                EffectModule::set_rate(boma, handle as u32, 9.0);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EffectModule::set_scale(boma, handle as u32, &Vector3f::new(0.45, 0.45, 0.45));
                EffectModule::set_rate(boma, handle as u32, 0.5);
                EffectModule::detach(boma, handle as u32, 0);
            }
            wait(lua_state, 2.0);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        let mut flame_color = Vector3f::new(1.0, 1.0, 1.0);
        let mut flame_size = 1.0;
        let mut flame_alpha = 1.0;
        let mut flame_rate = 2.0;
        if pledge == *PLEDGE_STATE_WATER {
            EFFECT_OFF_KIND(agent, Hash40::new("sys_drown_out"), false, false);
            EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_steam"), Hash40::new("sys_steam"), Hash40::new("top"), -1, 5, 10, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 5.0);
            LAST_EFFECT_SET_SCALE_W(agent, 1.8, 1.5, 1.8);
            LAST_EFFECT_SET_COLOR(agent, 0.7, 0.7, 0.74);
            LAST_EFFECT_SET_RATE(agent, 0.8);
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 8, 6, 0, 90, 0, 2, false, 1.0);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 3.0);
            // EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_hit_fire"), Hash40::new("top"), 0, 8, 16, 0, 90, 0, 0.5, false, 0.8);
            // LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 3.0);

            flame_color = Vector3f::new(0.6, 1.0, 6.0);
            flame_size = 1.1;
            flame_alpha = 0.85;
        }
        else if pledge == *PLEDGE_STATE_GRASS {
            EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_steam"), Hash40::new("sys_steam"), Hash40::new("top"), -1, 5, 10, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 1.2);
            LAST_EFFECT_SET_COLOR(agent, 2.0, 3.0, 0.5);
            LAST_EFFECT_SET_RATE(agent, 0.7);
            // EFFECT_FLIP(agent, Hash40::new("sys_grass_landing"), Hash40::new("sys_grass_landing"), Hash40::new("top"), -1, 5, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            // LAST_EFFECT_SET_COLOR(agent, 4.0, 3.0, 1.0);
            // LAST_EFFECT_SET_RATE(agent, 0.4);
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 8, 6, 0, 90, 0, 2, false, 0.8);
            LAST_EFFECT_SET_COLOR(agent, 0.6, 1.8, 1.0);

            flame_color = Vector3f::new(0.3, 0.9, 0.5);
            flame_size = 1.1;
            flame_alpha = 0.7;
            flame_rate = 1.0;
        }

        EFFECT_OFF_KIND(agent, Hash40::new("plizardon_flare_blitz_hold"), false, false);
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("plizardon_atk_mouth_fire"), Hash40::new("plizardon_atk_mouth_fire"), Hash40::new("head"), -1, 3, 0, 0, 180, 25, flame_size, true, *EF_FLIP_ROT_X, flame_alpha);
        // EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("plizardon_atk_mouth_fire"), Hash40::new("head"), -1, 3, 0, 0, 0, 25, 1.1, true);
        LAST_EFFECT_SET_SCALE_W(agent, 1.2, 1.9, 1.2);
        LAST_EFFECT_SET_COLOR(agent, flame_color.x, flame_color.y, flame_color.z);
        LAST_EFFECT_SET_RATE(agent, flame_rate);
    }
    for _ in 0..3 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 5, 0, 5, 0, 0, 0, 0.8, 0, 0, 5, 0, 0, 0, false);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.7, 0.4, 0.4);
        }
        wait(lua_state, 3.0);
        if is_excute(agent) {
            FLASH_FRM(agent, 2, 0, 0, 0, 0);
        }
        wait(lua_state, 4.0);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("plizardon_atk_mouth_fire"), false, false);
    }
    frame(lua_state, 49.0);
    if is_excute(agent) {
        let mut flame_color = Vector3f::new(1.0, 1.0, 1.0);
        let mut flame_alpha = 1.0;
        if pledge == *PLEDGE_STATE_WATER {
            flame_color = Vector3f::new(0.6, 1.0, 6.0);
            flame_alpha = 0.85;
        }
        else if pledge == *PLEDGE_STATE_GRASS {
            flame_color = Vector3f::new(0.3, 0.9, 0.5);
            flame_alpha = 0.7;
        }
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("plizardon_atk_mouth_fire"), Hash40::new("plizardon_atk_mouth_fire"), Hash40::new("head"), -1, 3, 0, 0, 180, 25, 1.7, true, *EF_FLIP_ROT_X, flame_alpha);
        LAST_EFFECT_SET_COLOR(agent, flame_color.x, flame_color.y, flame_color.z);
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 5, 0, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 5, 0, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("plizardon_atk_mouth_fire"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_steam2"), false, false); 
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch, Priority::Low);
    agent.acmd("game_catchdash", game_catchdash, Priority::Low);
    agent.acmd("game_catchturn", game_catchturn, Priority::Low);
    
    agent.acmd("game_throwf", game_throwf, Priority::Low);
    agent.acmd("game_throwb", game_throwb, Priority::Low);
    agent.acmd("game_throwhi", game_throwhi, Priority::Low);
    agent.acmd("game_throwlw", game_throwlw, Priority::Low);
    agent.acmd("effect_throwlw", effect_throwlw, Priority::Low);
}