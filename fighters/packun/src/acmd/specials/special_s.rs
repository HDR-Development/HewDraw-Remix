use super::*;

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 10.0, 5.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
}

// Piranha/Putrid
unsafe extern "C" fn game_specialsshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE);
    let charged = WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60;
    let hit = false;
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 10.0, 3.0);
    }
    if stance == STANCE_PIRANHA {
        // Fiery Breath
        frame(lua_state, 1.0);
        FT_DESIRED_RATE(agent, 5.0, 6.0);
        frame(lua_state, 5.0);  // f16
        FT_MOTION_RATE(agent, 1.0);
        if is_excute(agent) {
            if charged {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 15.0, 30, 65, 0, 60, 6.0, 9.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
                ATTACK(agent, 1, 0, Hash40::new("mouth"), 15.0, 30, 65, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
            }
            else {
                ArticleModule::generate_article(boma, articles::packun::FIREBREATH, false, -1);
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 3.0, 30, 150, 0, 11, 5.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
                ATTACK(agent, 1, 0, Hash40::new("mouth"), 3.0, 30, 150, 0, 11, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
            }
        }
        frame(lua_state, 10.0);
        FT_MOTION_RATE_RANGE(agent, 10.0, 50.0, 24.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 50.0); // f45
        FT_MOTION_RATE(agent, 1.0);
    }
    else if stance == STANCE_PUTRID {
        // Poison Breath
        frame(lua_state, 2.0);
        if !WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 18, 100, 30, 0, 5.0, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(10.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            }
        }
        frame(lua_state, 10.0);
        if !WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                ArticleModule::generate_article(boma, *FIGHTER_PACKUN_GENERATE_ARTICLE_POISONBREATH, false, -1);
            }
        }
        frame(lua_state, 21.0);
        FT_MOTION_RATE(agent, 0.9);
        if is_excute(agent) {
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 5.0, 5.0);
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 31.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC);
            CancelModule::enable_cancel(boma);
        }
        frame(lua_state, 20.0);
        FT_MOTION_RATE(agent, 0.55);
    }
}

unsafe extern "C" fn effect_specialsshoot(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    let stance = VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if stance == STANCE_PIRANHA {
        // Piranha
        frame(lua_state, 4.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("packun_spikeball_shoot"), Hash40::new("mouth"), 2, -0.6, 0, 0, 90, -100, 1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.35, 0.02);
            let (effect, size) = if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60
                { (Hash40::new("sys_flame"), 0.8) } else { (Hash40::new("packun_atk_air_b_fire"), 1.5) };
            EFFECT_FOLLOW(agent, effect, Hash40::new("mouth"), 7.5, 0, 0, 0, 0, 0, size, true);
        }
    }
    else {
        // Putrid
        frame(lua_state, 6.0);
        if WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_breath2"), Hash40::new("mouth"), 5, -0.6, 0, 0, 90, -100, 1.2, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(lua_state);
                agent.pop_lua_stack(1);
            }
        }
        else {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_breath"), Hash40::new("mouth"), 5, -0.6, 0, 0, 90, -100, 1.1, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(lua_state);
                agent.pop_lua_stack(1);
                LAST_EFFECT_SET_RATE(agent, 1.6);
            }
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_poison_breath"), -1);
    }
}

unsafe extern "C" fn sound_specialsshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE);
    if stance == STANCE_PIRANHA {
        frame(lua_state, 1.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_packun_special_n03"));
            SoundModule::set_se_pitch_ratio(boma, Hash40::new("se_packun_special_n03"), 3.0);
            if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60 {
                PLAY_SE(agent, Hash40::new("se_common_fire_m"));
            }
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            SoundModule::set_se_pitch_ratio(boma, Hash40::new("se_packun_special_n03"), 1.0);
        }
    }
    else {  // STANCE_PUTRID
        frame(lua_state, 3.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_packun_special_s03"));
        }
    }
}

unsafe extern "C" fn expression_specialsshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    if stance == 0 {
        frame(lua_state, 5.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
            if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60 {
                RUMBLE_HIT(agent, Hash40::new("rbkind_explosionm"), 0);
            }
            else {
                RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
            }
        }
    }
    else {
        frame(lua_state, 2.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 5.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 8.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

// Prickly
unsafe extern "C" fn game_specialsshoots(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60;
    let hit = false;
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 10.0, 3.0);
    }
    FT_DESIRED_RATE(agent, 11.0, 4.0);
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if charged {
            ATTACK(agent, 0, 0, Hash40::new("mouth"), 15.0, 80, 20, 0, 50, 7.5, -0.5, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 0.35, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            ATTACK(agent, 1, 0, Hash40::new("mouth"), 15.0, 80, 20, 0, 50, 7.5, -0.5, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 0.35, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("mouth"), 10.0, 80, 27, 0, 50, 7.5, -0.5, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 0.35, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 19.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            ATTACK(agent, 1, 0, Hash40::new("mouth"), 10.0, 80, 27, 0, 50, 7.5, -0.5, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 0.35, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 19.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE_RANGE(agent, 23.0, 31.0, 12.0);
    frame(lua_state, 31.0);
    FT_MOTION_RATE_RANGE(agent, 31.0, 47.0, 13.0);
    frame(lua_state, 47.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialsshoots(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60 {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_unblockable_flash"), Hash40::new("sys_unblockable_flash"), Hash40::new("top"), 7, 18, -5, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("packun_bite_line"), Hash40::new("packun_bite_line"), Hash40::new("top"), -5, 11, 19, 0, -130, 35, 1, true, *EF_FLIP_YZ);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("top"), -12, 9, 20, 10, 50, 10, 0.8, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("top"), -9, 11, 18, 0, -120, 20, 1, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn sound_specialsshoots(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_attackhard_s03"));
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60 {
            PLAY_SE(agent, Hash40::new("se_packun_smash_h02"));
        }
        PLAY_SE(agent, Hash40::new("se_packun_attackhard_s04"));
    }
}

unsafe extern "C" fn expression_specialsshoots(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60 {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
        else {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
    }
}

unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        match stance {
            STANCE_PIRANHA => {
                EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("mouth"), 2, -2, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
                EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("mouth"), 3, -1, 0, 0, 0, 0, 0.6, true);
            },
            STANCE_PUTRID => {
                EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 3, -1, 0, 0, 0, 0, 1, true);
            }
            _ => {  // STANCE_PRICKLY
                EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("mouth"), 5, -3, 0, 10, 50, -20, 0.9, true, *EF_FLIP_YZ);
                LAST_EFFECT_SET_RATE(agent, 0.5);
                EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("mouth"), 3, 0, 0, 0, -150, 20, 0.9, true, *EF_FLIP_YZ);
                LAST_EFFECT_SET_RATE(agent, 0.5);
            }
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_poison_max"), -1);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialairs", game_specials, Priority::Low);

    agent.acmd("game_specialsshoot", game_specialsshoot, Priority::Low);
    agent.acmd("game_specialairsshoot", game_specialsshoot, Priority::Low);
    agent.acmd("effect_specialsshoot", effect_specialsshoot, Priority::Low);
    agent.acmd("effect_specialairsshoot", effect_specialsshoot, Priority::Low);
    agent.acmd("sound_specialsshoot", sound_specialsshoot, Priority::Low);
    agent.acmd("sound_specialairsshoot", sound_specialsshoot, Priority::Low);
    agent.acmd("expression_specialsshoot", expression_specialsshoot, Priority::Low);
    agent.acmd("expression_specialairsshoot", expression_specialsshoot, Priority::Low);

    agent.acmd("game_specialsshoots", game_specialsshoots, Priority::Low);
    agent.acmd("game_specialairsshoots", game_specialsshoots, Priority::Low);
    agent.acmd("effect_specialsshoots", effect_specialsshoots, Priority::Low);
    agent.acmd("effect_specialairsshoots", effect_specialsshoots, Priority::Low);
    agent.acmd("sound_specialsshoots", sound_specialsshoots, Priority::Low);
    agent.acmd("sound_specialairsshoots", sound_specialsshoots, Priority::Low);
    agent.acmd("expression_specialsshoots", expression_specialsshoots, Priority::Low);
    agent.acmd("expression_specialairsshoots", expression_specialsshoots, Priority::Low);

    agent.acmd("effect_specialsend", effect_specialsend, Priority::Low);
    agent.acmd("effect_specialairsend", effect_specialsend, Priority::Low);
}