use super::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 5.0, 4.0);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0 * stance.damage_other, 80, 100, 20, 0, 2.5, 0.0, 6.3, 5.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0 * stance.damage_other, 80, 100, 20, 0, 2.5, 0.0, 6.3, 8.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0 * stance.damage_other, 80, 100, 20, 0, 3.0, 0.0, 6.3, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 2.0 * stance.damage_other, 80, 100, 20, 0, 3.0, 0.0, 6.3, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        // Jab lock hitbox
        ATTACK(agent, 4, 0, Hash40::new("top"), 2.0 * stance.damage_other, 361, 15, 0, 30, 3.0, 0.0, 3.5, 5.3, Some(0.0), Some(3.5), Some(12.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_MIDDLE), false);
        AttackModule::set_down_only(boma, 4, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 5.0, 4.0);
        if stance.label == STANCE_PRICKLY {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        }
        if stance.label == STANCE_PUTRID {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0 * stance.damage_other, 70, 15, 0, 30, 3.2, 0.0, 6.5, 6.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0 * stance.damage_other, 75, 15, 0, 30, 3.4, 0.0, 6.5, 10.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0 * stance.damage_other, 80, 15, 0, 30, 4.0, 0.0, 6.5, 15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if stance.label != STANCE_PRICKLY {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        }
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
}

unsafe extern "C" fn game_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    if stance.label == STANCE_PRICKLY {
        FT_DESIRED_RATE(agent, 7.0, 9.0);
    }
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 5.0, 4.0);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let (kbg, sound_level, sound_attr) = if stance.label == STANCE_PRICKLY
            { (52, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY) } else { (0, *COLLISION_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH) };
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0 * stance.damage_head, 38, 70 + kbg, 0, 50, 6.5, 0.0, 7.0, 15.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_level, sound_attr, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("virtualhit2"), 5.0 * stance.damage_head, 38, 70 + kbg, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_level, sound_attr, *ATTACK_REGION_HEAD);
        ATTACK(agent, 2, 0, Hash40::new("virtualhit3"), 5.0 * stance.damage_head, 38, 70 + kbg, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_level, sound_attr, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 13, 5, 8, 0, 0, 1.2, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 15, 7, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_attack100end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if stance.label == STANCE_PUTRID {
            ATTACK(agent, 0, 0, Hash40::new("top"), 4.0 * stance.damage_bite, 50, 150, 0, 40, 6.0, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            ATTACK(agent, 1, 1, Hash40::new("top"), 0.0, 0, 0, 0, 0, 6.0, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(15.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BITE);
            AttackModule::set_poison_param(boma, 0, 241, 60, 2.0, false);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 4.0 * stance.damage_bite, 50, 150, 0, 40, 6.0, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attack100end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
        if stance.label == STANCE_PUTRID {
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 9, 12.5, 0, 0, 0, 0.95, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("top"), -1.2, 9, 12.5, 0, 0, 0, 0.85, true);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_atk_100_finish"), Hash40::new("packun_atk_100_finish"), Hash40::new("top"), -8, 8, 15, 10, -90, 0, 1.4, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("packun_bite_line"), Hash40::new("mouth"), 3, 0, 0, 0, 0, 10, 1.1, true);
        LAST_EFFECT_SET_RATE(agent, 2.5);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("neck"), 0, 0, 0, 0, 90, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 0.3);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("packun_bite_line2"), Hash40::new("mouth"), 5, 0, 0, -20, 0, 0, 0.9, 0, 1, 0, 0, 0, 0, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_atk_100_finish"), -1);
    }
}

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    // base 0.93
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.8);
    if stance.label == STANCE_PRICKLY {
        FT_MOTION_RATE(agent, (10.0/7.0));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 8.0, 5.0);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("potc"), 12.0 * stance.damage_other, 45, 100, 0, 45, 5.3, -5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0 * stance.damage_other, 45, 100, 0, 45, 4.0, 0.0, 11.0, 0.0, Some(0.0), Some(5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        if stance.label == STANCE_PRICKLY {
            HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_XLU);
            HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_XLU);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.5);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.8);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("potc"), 8.0 * stance.damage_other, 50, 100, 0, 40, 4.3, -4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0 * stance.damage_other, 50, 100, 0, 40, 4.0, 0.0, 11.0, -2.0, Some(0.0), Some(5.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.0, 5.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attack11", game_attack11, Priority::Low);
    agent.acmd("game_attack12", game_attack12, Priority::Low);
    agent.acmd("game_attack13", game_attack13, Priority::Low);
    agent.acmd("effect_attack13", effect_attack13, Priority::Low);
    agent.acmd("game_attack100end", game_attack100end, Priority::Low);
    agent.acmd("effect_attack100end", effect_attack100end, Priority::Low);

    agent.acmd("game_attackdash", game_attackdash, Priority::Low);
}