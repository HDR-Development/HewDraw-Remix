use super::*;

unsafe extern "C" fn game_specialnhomingstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !VarModule::is_flag(agent.battle_object, vars::sonic::status::SPECIAL_N_BLAST_ATTACK) {
        if is_excute(agent) {
            SEARCH(agent, 0, 0, Hash40::new("top"), 40.0, 0.0, 10.0, 10.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
        }
    }
}

unsafe extern "C" fn game_specialnhoming(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut bone_hash = match WorkModule::is_flag(boma, *FIGHTER_SONIC_STATUS_SPECIAL_N_HOMING_FLAG_IS_KIRBY){
        true => Hash40::new("rot"),
        false => Hash40::new("hip")
    };
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        FT_MOTION_RATE(agent, 0.013);
        ATTACK(agent, 0, 0, bone_hash, 8.0, 80, 90, 0, 50, 5.0, 0.5, 1.5, 0.0, Some(0.5), Some(1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        AttackModule::set_captured_same_time_attack(boma, 0, true);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
}

unsafe extern "C" fn sound_specialnhoming(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);
    let mut sound = "";
    if rng == 0 { sound = "vc_sonic_004"; }
    else { sound = "vc_sonic_attack02"; }
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_sonic_rounddash"));
        PLAY_SE(agent, Hash40::new(sound));
        PLAY_SE(agent, Hash40::new("se_sonic_special_n01"));
    }
}

unsafe extern "C" fn game_specialnhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.3);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let temp = Vector3f { x: -0.3, y: 1.0, z: 0.0 };
		KineticModule::add_speed(boma, &temp);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn hash_0x195dc47911(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        UNABLE_AREA(agent, *FIGHTER_AREA_KIND_TREAD_JUMP_CHECK);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::set_captured_same_time_attack(boma, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SONIC_STATUS_SPIN_JUMP_WORK_ID_FLAG_ENABLE_JUMP_AERIAL);
        ENABLE_AREA(agent, *FIGHTER_AREA_KIND_TREAD_JUMP_CHECK);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
    ATTACK(agent, 0, 0, Hash40::new("hip"), 6.0, 60, 60, 0, 80, 4.0, 0.0, 1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_specialsbooststart(agent: &mut L2CAgentBase) {
    // FT_MOTION_RATE(agent, 0.75);
}

unsafe extern "C" fn sound_specialsbooststart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_sonic_rnd_attack"));
    }
}

unsafe extern "C" fn expression_specialsbooststart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialsboost(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 1.0 / 3.0);
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 100, 0, 45, 4.0, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 25, 120, 0, 50, 3.0, 0.0, 4.0, -0.5, Some(0.0), Some(9.0), Some(-0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        VarModule::on_flag(agent.battle_object, vars::sonic::status::SPECIAL_S_ENABLE_JUMP);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 0.25);
}

unsafe extern "C" fn effect_specialsboost(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let eff = if VarModule::is_flag(agent.battle_object, vars::sonic::status::SPECIAL_S_HOP) {
            Hash40::new("sonic_spintrace_max")
        }
        else {
            Hash40::new("sonic_spintrace")
        };
        EFFECT_FOLLOW_NO_STOP(agent, eff, Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1.25, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_specialsboost(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_sonic_special_s01"));
    }
}

unsafe extern "C" fn expression_specialsboost(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn game_specialsboostend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.6);
}

unsafe extern "C" fn effect_specialsboostend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        let eff = if VarModule::is_flag(agent.battle_object, vars::sonic::status::SPECIAL_S_HOP) {
            Hash40::new("sonic_spintrace_max")
        }
        else {
            Hash40::new("sonic_spintrace")
        };
        EFFECT_OFF_KIND(agent, eff, false, false);
    }
    for _ in 0..4 {
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.6, 0, 0, 3, 0, 0, 0, false);
        }
        wait(lua_state, 4.0);
    }
}

unsafe extern "C" fn sound_specialsboostend(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_sonic_dash_stop"));
    }
}

unsafe extern "C" fn expression_specialsboostend(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialairsboostend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        VarModule::on_flag(agent.battle_object, vars::sonic::status::SPECIAL_S_ENABLE_CONTROL);
    }
}

unsafe extern "C" fn effect_specialairsboostend(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let eff = if VarModule::is_flag(agent.battle_object, vars::sonic::status::SPECIAL_S_HOP) {
            Hash40::new("sonic_spintrace_max")
        }
        else {
            Hash40::new("sonic_spintrace")
        };
        EFFECT_OFF_KIND(agent, eff, false, false);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::shoot_exist(boma, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }   
}

unsafe extern "C" fn game_speciallwhold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    for i in 1..50 {
        wait(lua_state, 6.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("rot"), 1.0, 90, 0, 0, 58, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 3.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
    }
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, false);
    }
}

unsafe extern "C" fn game_specialairlwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, false);
    }
}

unsafe extern "C" fn game_speciallwdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 5.0, 60, 57, 0, 97, 3.5, 0.0, 1.5, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATK_SET_SHIELD_SETOFF_MUL(agent,0, 0.5);
        AttackModule::set_captured_same_time_attack(boma, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_ATTACK_ID_DEFAULT, true);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
}

unsafe extern "C" fn game_specialairlwdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        UNABLE_AREA(agent, *FIGHTER_AREA_KIND_TREAD_JUMP_CHECK);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 4.0, 60, 106, 0, 55, 3.5, 0.0, 1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_captured_same_time_attack(boma, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SONIC_STATUS_SPIN_JUMP_WORK_ID_FLAG_ENABLE_JUMP_AERIAL);
        ENABLE_AREA(agent, *FIGHTER_AREA_KIND_TREAD_JUMP_CHECK);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn stub(agent: &mut L2CAgentBase) {} 

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnhomingstart", game_specialnhomingstart);
    agent.acmd("game_specialnhoming", game_specialnhoming);
    agent.acmd("sound_specialnhoming", sound_specialnhoming);
    agent.acmd("game_specialnhit", game_specialnhit);
    agent.acmd("effect_specialnhit", stub);
    agent.acmd("game_specialnlanding", stub);
    agent.game_acmd(0x195dc47911, hash_0x195dc47911);

    agent.acmd("game_specialsbooststart", game_specialsbooststart);
    agent.acmd("sound_specialsbooststart", sound_specialsbooststart);
    agent.acmd("expression_specialsbooststart", expression_specialsbooststart);
    agent.acmd("game_specialsboost", game_specialsboost);
    agent.acmd("effect_specialsboost", effect_specialsboost);
    agent.acmd("sound_specialsboost", sound_specialsboost);
    agent.acmd("expression_specialsboost", expression_specialsboost);
    agent.acmd("game_specialsboostend", game_specialsboostend);
    agent.acmd("effect_specialsboostend", effect_specialsboostend);
    agent.acmd("sound_specialsboostend", sound_specialsboostend);
    agent.acmd("expression_specialsboostend", expression_specialsboostend);
    agent.acmd("game_specialairsboostend", game_specialairsboostend);
    agent.acmd("effect_specialairsboostend", effect_specialairsboostend);

    agent.acmd("game_specialhi", game_specialhi);

    agent.acmd("game_speciallwhold", game_speciallwhold);
    agent.acmd("game_specialairlwhold", game_speciallwhold);
    agent.acmd("game_speciallwholdchargehi", game_speciallwhold);
    agent.acmd("game_speciallwholdchargelw", game_speciallwhold);
    agent.acmd("game_speciallwholdchargemiddle", game_speciallwhold);
    agent.acmd("game_speciallwstart", game_speciallwstart);
    agent.acmd("game_specialairlwstart", game_specialairlwstart);
    agent.game_acmd(0x1b07509826, game_speciallwdash);
    agent.game_acmd(0x195dc47911, game_specialairlwdash);
}
