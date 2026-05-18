use super::*;

unsafe extern "C" fn game_attackshort(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let is_dragonize = WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE);
    let (bkb, collision_attr) = if is_dragonize { (15, Hash40::new("collision_attr_fire")) } else { (0, Hash40::new("collision_attr_normal")) };
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("have"), 8.0, 52, 87, 0, 30 + bkb, 0.7, 3.1, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        if is_dragonize {
            AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_fire_m"));
        }
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ATTACK(agent, 0, 0, Hash40::new("have"), 8.0, 52, 87, 0, 30 + bkb, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        if is_dragonize {
            AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_fire_m"));
        }
    }
    let clearFrame = if !WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) { 12.0 } else { 9.0 };
    frame(lua_state, clearFrame);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attacklong(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let is_dragonize = WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE);
    let (bkb, collision_attr) = if is_dragonize { (15, Hash40::new("collision_attr_fire")) } else { (0, Hash40::new("collision_attr_normal")) };
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("have"), 9.0, 52, 78, 0, 30 + bkb, 0.7, 3.1, 0.5, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        if is_dragonize {
            AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_fire_m"));
        }
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ATTACK(agent, 0, 0, Hash40::new("have"), 9.0, 52, 78, 0, 30 + bkb, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        if is_dragonize {
            AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_fire_m"));
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND) {
            AttackModule::clear_all(boma);
        }
        else {
            WorkModule::on_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
            ATTACK(agent, 0, 0, Hash40::new("have"), 10.0, 52, 78, 0, 30 + bkb, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            if is_dragonize {
                AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_fire_m"));
            }
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND) {
            AttackModule::clear_all(boma);
        }
        else {
            WorkModule::off_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
            ATTACK(agent, 0, 0, Hash40::new("have"), 9.0, 52, 78, 0, 30 + bkb, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            if is_dragonize {
                AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_fire_m"));
            }
        }
    }
    let clearFrame = if !WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) { 15.0 } else { 11.0 };
    frame(lua_state, clearFrame);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackdragonshootlong(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let is_dragonize = WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE);
    let (bkb, collision_attr) = if is_dragonize { (15, Hash40::new("collision_attr_fire")) } else { (0, Hash40::new("collision_attr_normal")) };
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("have"), 12.0, 52, 78, 0, 30 + bkb, 0.7, 5.4, 0.5, 0.3, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        if is_dragonize {
            AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_fire_l"));
        }
        AttackModule::enable_safe_pos(boma);
        AttackModule::disable_tip(boma);
        AttackModule::set_damage_shake_scale(boma, 0.5);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_76_megabolt"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ATTACK(agent, 0, 0, Hash40::new("have"), 12.0, 52, 78, 0, 30 + bkb, 3.2, 5.4, 0.5, 0.3, Some(1.0), Some(0.5), Some(0.3), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        if is_dragonize {
            AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_fire_l"));
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND) {
            AttackModule::clear_all(boma);
        }
        else {
            WorkModule::on_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
            ATTACK(agent, 0, 0, Hash40::new("have"), 15.0, 52, 78, 0, 30 + bkb, 3.2, 5.4, 0.5, 0.3, Some(1.0), Some(0.5), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            if is_dragonize {
                AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_fire_l"));
            }
        }
    }
    // Dragon Beam linker
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND) {
            AttackModule::clear_all(boma);
        }
        else {
            let (angle, link_bkb, fkb, kbg) = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR)
                { (52, 78, 0, 30 + bkb) } else { (361, 0, 10, 60) };
            ATTACK(agent, 0, 0, Hash40::new("have"), 15.0, angle, kbg, fkb, link_bkb, 3.2, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            // if is_dragonize {
            //     AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_fire_m"));
            // }
            WorkModule::off_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 5.0, false);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND) {
            AttackModule::clear_all(boma);
        }
        else {
            AttackModule::set_power(boma, 0, 14.0, false);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackdragonshootlong(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("tantan_wepon1_wind_big"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_COLOR(agent,1.0,0.8,0.25);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("tantan_wepon_ringwind"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_COLOR(agent,1.0,0.8,0.25);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("tantan_dragon_attack_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, true);
        LAST_EFFECT_SET_COLOR(agent,1.0,0.8,0.25);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("tantan_dragon_eye_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
}

unsafe extern "C" fn game_specialairhiattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let is_dragonized = WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE);
    if is_excute(agent) {
        if is_dragonized {
            ATTACK(agent, 0, 0, Hash40::new("have"), 19.5, 75, 51, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(-6.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_fire_l"));
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("have"), 13.0, 75, 72, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(-6.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if is_dragonized {
            ATTACK(agent, 0, 0, Hash40::new("have"), 19.5, 75, 51, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(0.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_fire_l"));
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("have"), 13.0, 75, 72, 0, 56, 4.5, 3.0, 0.0, 0.3, Some(0.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn sound_attackbeamloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_tantan_attack01_short"));
        STOP_SE(agent, Hash40::new("se_tantan_attack01_long"));
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        let sfx = if WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE) {Hash40::new("se_tantan_attack01_beam_max")} else {Hash40::new("se_tantan_attack01_beam")};
        PLAY_SE(agent, sfx);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackshort", game_attackshort, Priority::Low);
    agent.acmd("game_attacklong", game_attacklong, Priority::Low);

    agent.acmd("game_attackdragonshootlong", game_attackdragonshootlong, Priority::Low);
    agent.acmd("effect_attackdragonshootlong", effect_attackdragonshootlong, Priority::Low);

    agent.acmd("game_specialairhiattack", game_specialairhiattack, Priority::Low);
    agent.acmd("game_specialairhiattackdragon", game_specialairhiattack, Priority::Low);

    agent.acmd("sound_attackbeamloop", sound_attackbeamloop, Priority::Low);
}