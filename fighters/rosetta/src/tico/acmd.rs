use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_4_WORK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 361, 183, 0, 43, 5.0, 6.0, 0.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 5.0, 361, 219, 0, 23, 2.0, 0.0, 0.0, 0.0, Some(-4.0), Some(0.0), Some(0.4), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        SEARCH(agent, *WEAPON_ROSETTA_TICO_SEARCH_KIND_FREE_ATTACK_LR as u64, 0, Hash40::new("top"), 7.0, 0.0, 6.0, 3.0, Some(0.0), Some(6.0), Some(-3.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_4_WORK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 6.0, 85, 142, 0, 65, 5.5, 3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 5.0, 85, 142, 0, 65, 5.5, 3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_4_WORK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("rot"), 4.0, 33, 170, 0, 40, 3.5, 0.0, 2.0, -15.0, Some(0.0), Some(2.0), Some(-9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 2.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("rot"), 5.0, 33, 171, 0, 40, 3.5, 0.0, 2.5, 7.0, Some(0.0), Some(2.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 4.0, 43, 150, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_TOGGLE_TWINKLE_EFFECT);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("rot"), -0.5, 0, 1, -5, 180, 0, 0.9, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("rot"), 0, 0, -5, -5, 180, 0, 1, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("trans"), 0, 2, -23, 0, 0, 0, 1, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_TOGGLE_TWINKLE_EFFECT);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 88, 140, 0, 50, 4.0, 0.0, 6.5, 2.0, Some(0.0), Some(11.5), Some(2.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 5.0, 45, 153, 0, 50, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_TOGGLE_TWINKLE_EFFECT);
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("rot"), 0, 8, 1, 90, 0, 0, 0.9, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("trans"), 0, 15, 0, 90, 0, 0, 1, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("trans"), 0, -2.2, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_TOGGLE_TWINKLE_EFFECT);
    }
}


unsafe extern "C" fn effect_specialncharge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    VarModule::set_int(agent.battle_object, vars::rosetta::instance::TICO_CHARGE_LEVEL, 0);
    wait(lua_state, 5.0);
    for _ in 0..3 {
        if is_excute(agent) {
            VarModule::inc_int(agent.battle_object, vars::rosetta::instance::TICO_CHARGE_LEVEL);
            EFFECT_FOLLOW(agent, Hash40::new("rosetta_attack_flash1"), Hash40::new("top"), 15.0 * boma.lr(), 3.5, 0, 0, 0, 0, 0.45, true);
            LAST_EFFECT_SET_RATE(agent, 0.75);
        }
        wait(lua_state, 20.0);
    }
    if is_excute(agent) {
        VarModule::inc_int(agent.battle_object, vars::rosetta::instance::TICO_CHARGE_LEVEL);
        EFFECT_FOLLOW(agent, Hash40::new("rosetta_attack_flash3"), Hash40::new("top"), 15.0 * boma.lr(), 3.5, 0, 0, 0, 0, 0.48, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
}

unsafe extern "C" fn sound_specialncharge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    wait(lua_state, 5.0);
    for _ in 0..3 {
        if is_excute(agent) {
            let sound = SoundModule::play_status_se(boma, Hash40::new("se_tico_swing_m"), true, false, false);
            SoundModule::set_se_vol(boma, sound as i32, 0.75, 0);
        }
        wait(lua_state, 20.0);
    }
    if is_excute(agent) {
        let sound = SoundModule::play_status_se(boma, Hash40::new("se_tico_swing_l"), true, false, false);
        SoundModule::set_se_vol(boma, sound as i32, 0.75, 0);
    }
}

unsafe extern "C" fn game_specialnfly(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn effect_specialnfly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_TOGGLE_TWINKLE_EFFECT);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

unsafe extern "C" fn sound_specialnfly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_rosetta_special_n02_s"));
    } //always play weak sfx
}

unsafe extern "C" fn game_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    MotionModule::set_rate(boma, (10.0-2.0)/5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("waist"), 3.34, 75, 100, 0, 25, 4.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 10.0);
    MotionModule::set_rate(boma, (38.0-10.0)/24.0);//30 faf
    if is_excute(agent) {
        sv_kinetic_energy!(set_speed, agent, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.05 * boma.lr(), 0.0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("rosetta_attack_flash1"), Hash40::new("top"), 15.0 * boma.lr(), 3.5, 0, 0, 0, 0, 0.5, false);
    }
}

unsafe extern "C" fn sound_specialnend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_tico_swing_s"));
    }
}

unsafe extern "C" fn game_freerecall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);//standby
    frame(lua_state, 20.0);
    frame(lua_state, 40.0);//burst
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_OFF), 0);
    }
    frame(lua_state, 50.0);
    MotionModule::set_rate(boma, 1.0);
    frame(lua_state, 60.0);
    MotionModule::set_rate(boma, (65.0-60.0)/8.0);//7f activity
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosionl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ATTACK(agent, 0, 0, Hash40::new("waist"), 20.0, 55, 75, 0, 65, 10.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 30.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BOMB);
    }
}

unsafe extern "C" fn effect_freerecall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("rosetta_ticoshot_tico_hold"), Hash40::new("waist"), 0, 0.0, 0, 0, 0, 0, 1.15, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.3);
    }
    frame(lua_state, 25.0);
    for _ in 0..3 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("rosetta_tico_worry"), Hash40::new("top"), 1, 5, 1, 40, 0, 0, 1, true);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("rosetta_tico_worry"), Hash40::new("top"), 1, 5, -1, 40, 180, 0, 1, true);
        }
        wait(lua_state, 3.0);
    }
    frame(lua_state, 40.1);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("rosetta_attack_flash3"), Hash40::new("top"), 0.2, 6.0, -15.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.7, 0.02, 0.005);
        LAST_EFFECT_SET_RATE(agent, 0.65);
    }
    frame(lua_state, 55.5);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        EFFECT(agent, Hash40::new("rosetta_final_bomb"), Hash40::new("top"), 0.0, 6.0, 0.0, 0, 0, 0, 0.52, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 2.0);
        EFFECT(agent, Hash40::new("rosetta_final_smoke"), Hash40::new("top"), 0.0, 6.0, 0.0, 0, 90, 0, 0.71, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(lua_state, 59.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
}

unsafe extern "C" fn sound_freerecall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 40.1);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_tico_sad"));
    }
    frame(lua_state, 59.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_common_bomb_ll"));
        PLAY_SE(agent, Hash40::new("se_tico_dead"));
    }
}

unsafe extern "C" fn effect_death(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !agent.is_prev_status(statuses::rosetta_tico::POP) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("rosetta_tico_dead"), Hash40::new("hip"), 1, 0, 0, 0, 0, -90, 1.1, 0, 0, 0, 0, 0, 0, true);
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_CAMERA_OFF);
    }
}

unsafe extern "C" fn sound_death(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if !agent.is_prev_status(statuses::rosetta_tico::POP) {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_tico_dead"));
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);
    agent.acmd("game_attacks4hi", game_attacks4, Priority::Low);
    agent.acmd("game_attacks4lw", game_attacks4, Priority::Low);

    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);

    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);

    agent.acmd("game_attackairb", game_attackairb, Priority::Low);
    agent.acmd("effect_attackairb", effect_attackairb, Priority::Low);
    
    agent.acmd("game_attackairhi", game_attackairhi, Priority::Low);

    agent.acmd("game_attackairlw", game_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw", effect_attackairlw, Priority::Low);
    
    agent.acmd("effect_specialncharge", effect_specialncharge, Priority::Low);
    agent.acmd("sound_specialncharge", sound_specialncharge, Priority::Low);

    agent.acmd("game_specialnfly", game_specialnfly, Priority::Low);
    agent.acmd("effect_specialnfly", effect_specialnfly, Priority::Low);
    agent.acmd("sound_specialnfly", sound_specialnfly, Priority::Low);

    agent.acmd("game_specialnend", game_specialnend, Priority::Low);
    agent.acmd("effect_specialnend", effect_specialnend, Priority::Low);
    agent.acmd("sound_specialnend", sound_specialnend, Priority::Low);

    agent.acmd("game_freerecall", game_freerecall, Priority::Low);
    agent.acmd("effect_freerecall", effect_freerecall, Priority::Low);
    agent.acmd("sound_freerecall", sound_freerecall, Priority::Low);

    agent.acmd("effect_death", effect_death, Priority::Low);
    agent.acmd("sound_death", sound_death, Priority::Low);
}