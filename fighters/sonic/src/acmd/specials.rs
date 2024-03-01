
use super::*;

unsafe extern "C" fn sonic_specialsbooststart(fighter: &mut L2CAgentBase) {
    // FT_MOTION_RATE(fighter, 0.75);
}

unsafe extern "C" fn sonic_specialsbooststart_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
    }
}

unsafe extern "C" fn sonic_specialsbooststart_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn sonic_specialsboost(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, 1.0 / 3.0);
    frame(fighter.lua_state_agent, 3.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 100, 0, 45, 4.0, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 25, 120, 0, 50, 3.0, 0.0, 4.0, -0.5, Some(0.0), Some(9.0), Some(-0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        VarModule::on_flag(fighter.battle_object, vars::sonic::status::SPECIAL_S_ENABLE_JUMP);
    }
    frame(fighter.lua_state_agent, 14.0);
    FT_MOTION_RATE(fighter, 0.25);
}

unsafe extern "C" fn sonic_specialsboost_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        let eff = if VarModule::is_flag(fighter.battle_object, vars::sonic::status::SPECIAL_S_HOP) {
            Hash40::new("sonic_spintrace_max")
        }
        else {
            Hash40::new("sonic_spintrace")
        };
        EFFECT_FOLLOW_NO_STOP(fighter, eff, Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1.25, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

unsafe extern "C" fn sonic_specialsboost_snd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_special_s01"));
    }
}

unsafe extern "C" fn sonic_specialsboost_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn sonic_specialsboostend(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 8.0);
    FT_MOTION_RATE(fighter, 1.6);
}

unsafe extern "C" fn sonic_specialsboostend_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        let eff = if VarModule::is_flag(fighter.battle_object, vars::sonic::status::SPECIAL_S_HOP) {
            Hash40::new("sonic_spintrace_max")
        }
        else {
            Hash40::new("sonic_spintrace")
        };
        EFFECT_OFF_KIND(fighter, eff, false, false);
    }
    for _ in 0..4 {
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.6, 0, 0, 3, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 4.0);
    }
}

unsafe extern "C" fn sonic_specialsboostend_snd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_dash_stop"));
    }
}

unsafe extern "C" fn sonic_specialsboostend_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn sonic_specialairsboostend(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        VarModule::on_flag(fighter.battle_object, vars::sonic::status::SPECIAL_S_ENABLE_CONTROL);
    }
}

unsafe extern "C" fn sonic_specialairsboostend_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        let eff = if VarModule::is_flag(fighter.battle_object, vars::sonic::status::SPECIAL_S_HOP) {
            Hash40::new("sonic_spintrace_max")
        }
        else {
            Hash40::new("sonic_spintrace")
        };
        EFFECT_OFF_KIND(fighter, eff, false, false);
    }
}

unsafe extern "C" fn sonic_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::shoot_exist(boma, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }   
}

unsafe extern "C" fn sonic_gimmick_jump_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 90, 90, 0, 30, 6.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn sonic_special_lw_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    for i in 1..50 {
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("rot"), 1.0, 90, 0, 0, 58, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
}

unsafe extern "C" fn sonic_special_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, false);
    }
    
}

unsafe extern "C" fn sonic_special_air_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, false);
    }
    
}

unsafe extern "C" fn sonic_special_lw_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 5.0, 60, 57, 0, 97, 3.5, 0.0, 1.5, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATK_SET_SHIELD_SETOFF_MUL(fighter,0, 0.5);
        AttackModule::set_captured_same_time_attack(boma, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_ATTACK_ID_DEFAULT, true);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
}

unsafe extern "C" fn sonic_special_air_lw_dash_game(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn sonic_special_n_homing_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !VarModule::is_flag(fighter.battle_object, vars::sonic::status::SPECIAL_N_BLAST_ATTACK) {
        if is_excute(fighter) {
            SEARCH(fighter, 0, 0, Hash40::new("top"), 40.0, 0.0, 10.0, 10.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
        }
    }
}

unsafe extern "C" fn sonic_special_n_homing_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    let mut bone_hash = match WorkModule::is_flag(boma, *FIGHTER_SONIC_STATUS_SPECIAL_N_HOMING_FLAG_IS_KIRBY){
        true => Hash40::new("rot"),
        false => Hash40::new("hip")
    };

    
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        FT_MOTION_RATE(fighter, 0.013);
        ATTACK(fighter, 0, 0, bone_hash, 8.0, 80, 90, 0, 50, 5.0, 0.5, 1.5, 0.0, Some(0.5), Some(1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
        AttackModule::set_captured_same_time_attack(boma, 0, true);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
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

unsafe extern "C" fn sonic_special_n_homing_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);
    let mut sound = "";

    if rng == 0 { sound = "vc_sonic_004"; }
    else { sound = "vc_sonic_attack02"; }

    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_sonic_rounddash"));
        PLAY_SE(fighter, Hash40::new(sound));
        PLAY_SE(fighter, Hash40::new("se_sonic_special_n01"));
    }
}

unsafe extern "C" fn sonic_special_n_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.3);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        let temp = Vector3f { x: -0.3, y: 1.0, z: 0.0 };
		KineticModule::add_speed(boma, &temp);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

unsafe extern "C" fn sonic_special_n_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
}

unsafe extern "C" fn sonic_special_n_landing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma(); 
}

pub fn install() {
    smashline::Agent::new("sonic")
        .acmd("game_specialsbooststart", sonic_specialsbooststart)
        .acmd("sound_specialsbooststart", sonic_specialsbooststart_snd)
        .acmd(
            "expression_specialsbooststart",
            sonic_specialsbooststart_exp,
        )
        .acmd("game_specialsboost", sonic_specialsboost)
        .acmd("effect_specialsboost", sonic_specialsboost_eff)
        .acmd("sound_specialsboost", sonic_specialsboost_snd)
        .acmd("expression_specialsboost", sonic_specialsboost_exp)
        .acmd("game_specialsboostend", sonic_specialsboostend)
        .acmd("effect_specialsboostend", sonic_specialsboostend_eff)
        .acmd("sound_specialsboostend", sonic_specialsboostend_snd)
        .acmd("expression_specialsboostend", sonic_specialsboostend_exp)
        .acmd("game_specialairsboostend", sonic_specialairsboostend)
        .acmd("effect_specialairsboostend", sonic_specialairsboostend_eff)
        .acmd("game_specialhi", sonic_special_hi_game)
        .acmd("game_speciallwhold", sonic_special_lw_hold_game)
        .acmd("game_specialairlwhold", sonic_special_lw_hold_game)
        .acmd("game_speciallwholdchargehi", sonic_special_lw_hold_game)
        .acmd("game_speciallwholdchargelw", sonic_special_lw_hold_game)
        .acmd("game_speciallwholdchargemiddle", sonic_special_lw_hold_game)
        .acmd("game_speciallwstart", sonic_special_lw_start_game)
        .acmd("game_specialairlwstart", sonic_special_air_lw_start_game)
        .game_acmd(0x1b07509826, sonic_special_lw_dash_game)
        .game_acmd(0x195dc47911, sonic_special_air_lw_dash_game)
        .acmd(
            "game_specialnhomingstart",
            sonic_special_n_homing_start_game,
        )
        .acmd("game_specialnhoming", sonic_special_n_homing_game)
        .game_acmd(0x195dc47911, hash_0x195dc47911)
        .acmd("sound_specialnhoming", sonic_special_n_homing_sound)
        .acmd("game_specialnhit", sonic_special_n_hit_game)
        .acmd("effect_specialnhit", sonic_special_n_hit_effect)
        .acmd("game_specialnlanding", sonic_special_n_landing)
        .install();
    smashline::Agent::new("sonic_gimmickjump")
        .acmd("game_fall", sonic_gimmick_jump_game)
        .install();
}
