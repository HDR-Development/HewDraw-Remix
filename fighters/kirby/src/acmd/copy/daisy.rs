use super::*;

unsafe extern "C" fn game_daisyspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //ArticleModule::generate_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, false, 0);
    }
}

unsafe extern "C" fn effect_daisyspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("arml"), 0.8, 0, 0, 0, 105, 0, 0.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 2.5, 24, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_daisyspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_kirby_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_common_smashswing_02"));
    }
}

unsafe extern "C" fn expression_daisyspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
}

unsafe extern "C" fn game_daisyspecialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //ArticleModule::generate_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, false, 0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_DIVE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 315, 100, 50, 0, 4.5, 0.0, 6.5, 6.0, Some(0.0), Some(6.5), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        VarModule::off_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_DIVE);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 51.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_AUTOCANCEL);
        //ArticleModule::remove_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn effect_daisyspecialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("arml"), 0.8, 0, 0, 0, 105, 0, 0.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 12, 19, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11, -4, -120, 0, 0, 1.0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 0.3, 0.3);
    }
    for i in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 15, 0, -120, 0, 0, 1.0, false);
            LAST_EFFECT_SET_COLOR(agent, 0.3, 0.3, 0.3);
            EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("arml"), 2, 1, 0, 0, 0, 0, (0.30 - (i as f32 * 0.015)), false);
            LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        }
        wait(lua_state, 5.0);
    }
    frame(lua_state, 51.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_daisyspecialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_kirby_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_common_smashswing_02"));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_critical_l_tail"));
    }
}

unsafe extern "C" fn expression_daisyspecialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
}

unsafe extern "C" fn game_daisyspecialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::on_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_CRYSTAL_ACTIVE);
        if !VarModule::is_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_AIR_START) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 75, 83, 0, 64, 4.0, 0.0, 4.0, -6.5, Some(0.0), Some(4.0), Some(12.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 60, 83, 0, 64, 4.0, 0.0, 4.0, -6.5, Some(0.0), Some(4.0), Some(12.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        //ArticleModule::remove_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(0));
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_CRYSTAL_ACTIVE);
    }
}

unsafe extern "C" fn effect_daisyspecialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        let offset = if VarModule::is_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_AIR_START) { 3 } else { 0 };
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        EFFECT_FLIP(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 3 + offset, 0, 9, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.2);
        EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("top"), -7 + offset, 1, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 13 + offset, 1, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, 7, 13 + offset, 0, 200, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EffectModule::set_scale_last(boma, &Vector3f::new(0.25, 0.8, 0.25));
        EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, 7, -7 + offset, 0, 200, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EffectModule::set_scale_last(boma, &Vector3f::new(0.25, 0.8, 0.25));
        EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 10, 3.7, 7 + offset, 0, 200, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EffectModule::set_scale_last(boma, &Vector3f::new(0.15, 0.4, 0.15));
        EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 10, 4.7, -2 + offset, 0, 200, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EffectModule::set_scale_last(boma, &Vector3f::new(0.15, 0.5, 0.15));
        EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), -10, 4.7, 8 + offset, 0, 200, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EffectModule::set_scale_last(boma, &Vector3f::new(0.15, 0.5, 0.15));
        EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), -10, 3.7, -1 + offset, 0, 200, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EffectModule::set_scale_last(boma, &Vector3f::new(0.15, 0.4, 0.15));
        // if VarModule::is_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_AIR_START) {
        //     EFFECT_DETACH_KIND(agent, Hash40::new("sys_ice"), -1);
        // }
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        let offset = if VarModule::is_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_AIR_START) { 0 } else { 3 };
        EFFECT_OFF_KIND(agent, Hash40::new("sys_ice"), false, false);
        EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("top"), -10 + offset, 1, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 10 + offset, 1, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0 + offset, 1, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
    }
}

unsafe extern "C" fn sound_daisyspecialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_frieze_l"));
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_freeze"));
    }
}

unsafe extern "C" fn expression_daisyspecialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_daisyspecialn", game_daisyspecialn, Priority::Low);
    agent.acmd("effect_daisyspecialn", effect_daisyspecialn, Priority::Low);
    agent.acmd("sound_daisyspecialn", sound_daisyspecialn, Priority::Low);
    agent.acmd("expression_daisyspecialn", expression_daisyspecialn, Priority::Low);

    agent.acmd("game_daisyspecialairn", game_daisyspecialairn, Priority::Low);
    agent.acmd("effect_daisyspecialairn", effect_daisyspecialairn, Priority::Low);
    agent.acmd("sound_daisyspecialairn", sound_daisyspecialairn, Priority::Low);
    agent.acmd("expression_daisyspecialairn", expression_daisyspecialairn, Priority::Low);

    agent.acmd("game_daisyspecialnattack", game_daisyspecialnattack, Priority::Low);
    agent.acmd("effect_daisyspecialnattack", effect_daisyspecialnattack, Priority::Low);
    agent.acmd("sound_daisyspecialnattack", sound_daisyspecialnattack, Priority::Low);
    agent.acmd("expression_daisyspecialnattack", expression_daisyspecialnattack, Priority::Low);
}