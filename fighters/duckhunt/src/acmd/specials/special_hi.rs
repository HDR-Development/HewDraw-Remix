use super::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::duckhunt::status::SPECIAL_HI_JUMP);
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 72, 108, 0, 28, 6.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::duckhunt::status::SPECIAL_HI_ENABLE_SHOT);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let team_color = FighterUtil::get_team_color(boma);
        let effect_team_color = &FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
        //EFFECT(agent, Hash40::new("duckhunt_target"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        let eff_handle = EffectModule::req_on_joint(boma, Hash40::new("duckhunt_target"), Hash40::new("top"), &Vector3f::new(0.0, 3.0, 0.0), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        VarModule::set_int64(agent.battle_object, vars::duckhunt::status::SPECIAL_HI_RETICLE_EFFECT_HANDLE, eff_handle as u64);
        LAST_EFFECT_SET_COLOR(agent, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
        EFFECT_FOLLOW(agent, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("duckhunt_target"), false, false);
        EFFECT(agent, Hash40::new("duckhunt_target_impact"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_duckhunt_special_h01"));
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let duck_handle = SoundModule::play_se(boma, Hash40::new("vc_duckhunt_duck_attack01"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, duck_handle as i32, 1.5, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_duckhunt_dog_attack01"));
        let shot_handle = SoundModule::play_se(boma, Hash40::new("se_duckhunt_special_l03"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, shot_handle as i32, 1.5, 0);
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::duckhunt::status::SPECIAL_HI_JUMP);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 63, 100, 0, 30, 6.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::duckhunt::status::SPECIAL_HI_ENABLE_SHOT);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::duckhunt::status::SPECIAL_HI_ENABLE_SHOT);
    }
}

unsafe extern "C" fn effect_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let team_color = FighterUtil::get_team_color(boma);
        let effect_team_color = &FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
        //EFFECT(agent, Hash40::new("duckhunt_target"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        let eff_handle = EffectModule::req_on_joint(boma, Hash40::new("duckhunt_target"), Hash40::new("top"), &Vector3f::new(0.0, 3.0, 0.0), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        VarModule::set_int64(agent.battle_object, vars::duckhunt::status::SPECIAL_HI_RETICLE_EFFECT_HANDLE, eff_handle as u64);
        LAST_EFFECT_SET_COLOR(agent, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
        EFFECT_FOLLOW(agent, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("duckhunt_target"), false, false);
        EFFECT(agent, Hash40::new("duckhunt_target_impact2"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_duckhunt_special_h01"));
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let duck_handle = SoundModule::play_se(boma, Hash40::new("vc_duckhunt_duck_attack02"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, duck_handle as i32, 1.5, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_duckhunt_dog_attack02"));
        let shot_handle = SoundModule::play_se(boma, Hash40::new("se_duckhunt_special_l03"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, shot_handle as i32, 1.5, 0);
    }
}

unsafe extern "C" fn expression_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        DamageModule::add_damage(boma, 2.0, 0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 63, 74, 0, 34, 8.0, 0.0, 7.0, -3.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        let team_color = FighterUtil::get_team_color(boma);
        let effect_team_color = &FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
        //EFFECT(agent, Hash40::new("duckhunt_target"), Hash40::new("top"), -3, 7, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        let eff_handle = EffectModule::req_on_joint(boma, Hash40::new("duckhunt_target"), Hash40::new("top"), &Vector3f::new(-3.0, 7.0, 0.0), &Vector3f::zero(), 1.2, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        VarModule::set_int64(agent.battle_object, vars::duckhunt::status::SPECIAL_HI_RETICLE_EFFECT_HANDLE, eff_handle as u64);
        LAST_EFFECT_SET_COLOR(agent, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
        EFFECT_FOLLOW(agent, Hash40::new("duckhunt_feather_long"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("duckhunt_target"), false, false);
        EFFECT(agent, Hash40::new("duckhunt_target_impact3"), Hash40::new("top"), -3, 7, -5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("duckhunt_trick_smoke"), Hash40::new("top"), 0, 7, -3, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
}

unsafe extern "C" fn sound_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_punch_hit_l"));
        let shot_handle = SoundModule::play_se(boma, Hash40::new("se_duckhunt_special_l03"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, shot_handle as i32, 1.5, 0);
    }
}

unsafe extern "C" fn expression_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_explosionm"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi, Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi, Priority::Low);
    agent.acmd("expression_specialhi", expression_specialhi, Priority::Low);

    agent.acmd("game_specialhi2", game_specialhi2, Priority::Low);
    agent.acmd("effect_specialhi2", effect_specialhi2, Priority::Low);
    agent.acmd("sound_specialhi2", sound_specialhi2, Priority::Low);
    agent.acmd("expression_specialhi2", expression_specialhi2, Priority::Low);

    agent.acmd("game_specialhi3", game_specialhi3, Priority::Low);
    agent.acmd("effect_specialhi3", effect_specialhi3, Priority::Low);
    agent.acmd("sound_specialhi3", sound_specialhi3, Priority::Low);
    agent.acmd("expression_specialhi3", expression_specialhi3, Priority::Low);
}