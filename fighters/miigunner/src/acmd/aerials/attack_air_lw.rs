use super::*;

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miigunner::status::CHECK_BOOSTED_AERIAL);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE_RANGE(agent, 14.0, 18.0, 1.0);
    frame(lua_state, 18.0);
    FT_MOTION_RATE_RANGE(agent, 18.0, 20.0, 1.0);
    frame(lua_state, 20.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 15.0, 270, 50, 0, 18, 4.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("handr"), 15.0, 270, 50, 0, 18, 5.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 12.0, 361, 70, 0, 30, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("handr"), 12.0, 361, 70, 0, 30, 4.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 54.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot5"), Hash40::new("haver"), 0, 0, -3, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot4"), Hash40::new("haver"), 0, 0, 2.5, 90, 0, 0, 0.3, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("miigunner_atk_shot5"), -1);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot_after"), Hash40::new("armr"), 6, 0, 0, 0, 0, -90, 0.75, true);
    }
}

unsafe extern "C" fn game_attackairlwboost(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE_RANGE(agent, 14.0, 20.0, 4.0);
    frame(lua_state, 20.0);
    FT_MOTION_RATE(agent, 1.0);
    let charge = VarModule::get_float(agent.battle_object, vars::miigunner::status::ATTACK_CHARGE);
    let charge_mul = 1.0 + (charge * 0.025);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.object(), vars::miigunner::instance::BOOSTED_ATTACK_AIR_LW_AIRTIME) {
            VarModule::on_flag(agent.object(), vars::miigunner::instance::BOOSTED_ATTACK_AIR_LW_AIRTIME);
        }
        let speed_x_mul = ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_charge.attack_air_lw_charge_speed_x_mul");
        let speed_y_mul = ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_charge.attack_air_lw_charge_speed_y_mul");
        SET_SPEED_EX(agent,
            (KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(boma) * (20.0 - charge)/20.0) * speed_x_mul,
            (1.125 + (0.125 * charge)) * speed_y_mul,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN
        );
        ATTACK(agent, 0, 0, Hash40::new("handr"), 12.0 * charge_mul, 90, 65, 0, 50, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("handr"), 12.0 * charge_mul, 90, 65, 0, 50, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("handr"), 14.0 * charge_mul, 90, 65, 0, 50, 5.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 8.0 * charge_mul, 75, 90, 0, 30, 3.0, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("handr"), 8.0 * charge_mul, 75, 90, 0, 30, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("handr"), 8.0 * charge_mul, 75, 90, 0, 30, 4.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 28.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 43.0);
    if charge > 5.0 {
        let charge_mul = if VarModule::is_flag(agent.battle_object, vars::miigunner::instance::BOOSTED_ATTACK_AIR_LW_AIRTIME) { 1.0 } else { 2.0 };
        FT_MOTION_RATE_RANGE(agent, 43.0, 50.0, 9.0 + charge_mul * VarModule::get_float(agent.battle_object, vars::miigunner::status::ATTACK_CHARGE));
    }
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 50.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairlwboost(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        let handle = EffectModule::req_follow(boma, Hash40::new("sys_smash_flash"), Hash40::new("top"), &Vector3f::new(0.0, 10.0, -1.0), &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, false, false);
        VarModule::set_int64(agent.battle_object, vars::miigunner::instance::SPECIAL_HI1_LAUNCH_EFFECT_HANDLE, handle);
        EffectModule::set_rate(boma, handle as u32, 0.1);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot5"), Hash40::new("haver"), 0, 0, -3, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot4"), Hash40::new("haver"), 0, 0, 2.5, 90, 0, 0, 0.3, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("miigunner_atk_shot5"), -1);
        LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_gimmckjump"), Hash40::new("armr"), 6, 0, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        LAST_EFFECT_SET_COLOR(agent, 0.15, 1.0, 10.0);
        if VarModule::get_float(agent.battle_object, vars::miigunner::status::ATTACK_CHARGE) >= 10.0 {
            EFFECT_FLW_POS(agent, Hash40::new("miigunner_gimmck_attack"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miigunner_atk_shot_after"), Hash40::new("armr"), 6, 0, 0, 0, 0, -90, 0.75, true);
        LAST_EFFECT_SET_COLOR(agent, 0.15, 0.55, 10.0);
    }
}

unsafe extern "C" fn sound_attackairlwboost(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        //PLAY_SE(agent, Hash40::new("se_miigunner_special_c3_s02"));
        let handle = SoundModule::play_se(boma, Hash40::new("se_miigunner_special_c3_s02"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 2.0, 0);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_miigunner_special_c3_s02"));
        PLAY_SE(agent, Hash40::new("se_miigunner_attackair_l01"));
        PLAY_SE(agent, Hash40::new("vc_mii_attack07"));
    }
}

unsafe extern "C" fn expression_attackairlwboost(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::stop_rumble(boma, false);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_landingairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if VarModule::is_flag(agent.battle_object, vars::miigunner::instance::BOOSTED_AERIAL_LANDING) {
        let boosted_landing_frame = ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_charge.attack_air_lw_boost_landing_frame");
        if is_excute(agent) {
            let landing_frame = WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_lw"), 0);
            FT_MOTION_RATE(agent, (boosted_landing_frame + 1.0)/landing_frame);
        }
        frame(lua_state, boosted_landing_frame);
        FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn effect_landingairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        // let handle = VarModule::get_int64(agent.battle_object, vars::miigunner::instance::SPECIAL_HI1_LAUNCH_EFFECT_HANDLE);
        // EffectModule::set_scale(boma, handle as u32, &Vector3f::zero());
        // VarModule::set_int(agent.battle_object, vars::miigunner::instance::SPECIAL_HI1_LAUNCH_EFFECT_HANDLE, -1);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_smash_flash"), false, false);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_landingairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_miigunner_special_c3_s02"));
        PLAY_LANDING_SE(agent, Hash40::new("se_miigunner_landing02"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairlw", game_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw", effect_attackairlw, Priority::Low);

    agent.acmd("game_attackairlwboost", game_attackairlwboost, Priority::Low);
    agent.acmd("effect_attackairlwboost", effect_attackairlwboost, Priority::Low);
    agent.acmd("sound_attackairlwboost", sound_attackairlwboost, Priority::Low);
    agent.acmd("expression_attackairlwboost", expression_attackairlwboost, Priority::Low);

    agent.acmd("game_landingairlw", game_landingairlw, Priority::Low);
    agent.acmd("effect_landingairlw", effect_landingairlw, Priority::Low);
    agent.acmd("sound_landingairlw", sound_landingairlw, Priority::Low);
}