use super::*;

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 15.0, 11.0);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        VarModule::on_flag(agent.battle_object, vars::miigunner::status::CHECK_BOOSTED_AERIAL);
    }
    frame(lua_state, 17.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.8, 140, 100, 45, 0, 4.5, 0.0, 15.0, -3.0, Some(0.0), Some(15.0), Some(3.0), 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.8, 367, 100, 25, 0, 2.6, 0.0, 55.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.8, 96, 100, 25, 0, 2.6, 0.0, 55.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.8, 130, 100, 25, 0, 3.2, 0.0, 13.0, -0.8, Some(0.0), Some(13.0), Some(0.8), 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 35.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 78, 120, 0, 50, 4.0, 0.0, 58.0, 0.0, Some(0.0), Some(16.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("miigunner_atk_gatling"), Hash40::new("armr"), 5.5, 0, 0, -90, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
}

unsafe extern "C" fn sound_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_miigunner_rnd_attack02"));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_miigunner_attackair_h01_lp"));
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_miigunner_attackair_h01_lp"));
        STOP_SE(agent, Hash40::new("seq_miigunner_rnd_attack02"));
        PLAY_SE(agent, Hash40::new("se_miigunner_attackair_h01"));
    }
}

unsafe extern "C" fn expression_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_attackairhiboost(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 15.0, 11.0);
    frame(lua_state, 15.0);
    FT_MOTION_RATE_RANGE(agent, 15.0, 17.0, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 17.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f::new(0.0, -3.25, 0.0));
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 90, 77, 0, 27, 6.5, 0.0, 28.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 270, 38, 0, 56, 6.0, 0.0, 19.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 72, 0, 30, 5.5, 0.0, 19.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackairhiboost(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4.0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.5);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_smash_flash"), -1);
    }
    frame(lua_state, 16.7);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot5"), Hash40::new("haver"), 0, 0, -3, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
        EFFECT_DETACH_KIND(agent, Hash40::new("miigunner_atk_shot5"), -1);
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot4"), Hash40::new("haver"), 0, 0, 2.5, 90, 0, 0, 0.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot_after"), Hash40::new("armr"), 6, 0, 0, 0, 0, -90, 0.75, true);
        LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
    }
}

unsafe extern "C" fn sound_attackairhiboost(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_miigunner_attackair_h01_lp"));
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miigunner_smash_l01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miigunner_rnd_attack03"));
    }
}

unsafe extern "C" fn expression_attackairhiboost(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.5);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_landingairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if VarModule::is_flag(agent.battle_object, vars::miigunner::instance::BOOSTED_AERIAL_LANDING) {
        let boosted_landing_frame = ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_charge.attack_air_hi_boost_landing_frame");
        if is_excute(agent) {
            let landing_frame = WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_hi"), 0);
            FT_MOTION_RATE(agent, (boosted_landing_frame + 1.0)/landing_frame);
        }
        frame(lua_state, boosted_landing_frame);
        FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn effect_landingairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::miigunner::instance::BOOSTED_AERIAL_LANDING) {
            LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        }
        else {
            LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn sound_landingairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_miigunner_attackair_h01_lp"));
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_miigunner_landing02"));
    }
}

unsafe extern "C" fn expression_landingairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
        if VarModule::is_flag(agent.battle_object, vars::miigunner::instance::BOOSTED_AERIAL_LANDING) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        else {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairhi", game_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi", effect_attackairhi, Priority::Low);
    agent.acmd("sound_attackairhi", sound_attackairhi, Priority::Low);
    agent.acmd("expression_attackairhi", expression_attackairhi, Priority::Low);

    agent.acmd("game_attackairhiboost", game_attackairhiboost, Priority::Low);
    agent.acmd("effect_attackairhiboost", effect_attackairhiboost, Priority::Low);
    agent.acmd("sound_attackairhiboost", sound_attackairhiboost, Priority::Low);
    agent.acmd("expression_attackairhiboost", expression_attackairhiboost, Priority::Low);

    agent.acmd("game_landingairhi", game_landingairhi, Priority::Low);
    agent.acmd("effect_landingairhi", effect_landingairhi, Priority::Low);
    agent.acmd("sound_landingairhi", sound_landingairhi, Priority::Low);
    agent.acmd("expression_landingairhi", expression_landingairhi, Priority::Low);
}