use super::*;

// ===============================================================================================
// ======================================== BLADE COUNTER ========================================
// ===============================================================================================

unsafe extern "C" fn game_speciallw1hit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 21.0, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 80, 60, 0, 85, 8.8, 0.0, 8.0, 15.0, Some(0.0), Some(8.0), Some(3.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(boma, 0, true, false);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 47.0, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 47.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairlw1hit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 21.0, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 21.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 73, 60, 0, 85, 10.5, 0.0, 7.0, 14.0, Some(0.0), Some(7.0), Some(4.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(boma, 0, true, false);
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(agent, 24.0, 47.0, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 47.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_speciallw1hitlv1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 50, 6.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(-30.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        //WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
}

unsafe extern "C" fn game_specialairlw1hitlv1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 11.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("haver"), 2.0, 365, 0, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 50, 6.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(-30.0), 1.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        //WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
}

// Kinesis Blade - 2 Charges

unsafe extern "C" fn game_speciallw1hitlv2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.1, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 63.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 73.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairlw1hitlv2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.1, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 40, 12.0, 0.0, 12.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
    }
    frame(lua_state, 63.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 73.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

// ================================================================================================================
// ================================================ SHOCK SPELL ===================================================
// ================================================================================================================

unsafe extern "C" fn game_speciallw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 12.0, 16.0);
    if is_excute(agent) {
        if boma.is_button_on(Buttons::Special) {
            VarModule::on_flag(agent.object(), vars::miiswordsman::status::SPECIAL_LW2_SHOCK_SPELL_HOLD);
        }
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL);
        let hold = if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SPECIAL_LW2_SHOCK_SPELL_HOLD) { 15.0 } else { 0.0 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 60, 55, 0, 40, 5.0, 0.0, 9.0, 15.0 + hold, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 30, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 60, 55, 0, 40, 2.0, 0.0, 2.0, 15.0 + hold, Some(0.0), Some(27.0), Some(15.0 + hold), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 24, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 60, 55, 0, 40, 3.0, 0.0, 30.0, 12.0 + hold, Some(0.0), Some(30.0), Some(18.0 + hold), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 24, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(agent, 16.0, 23.0, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL);
    }
}

unsafe extern "C" fn effect_speciallw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -1.5, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_reflect_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0.0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.65);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("arml"), 4, 0, 0, 0, 0, 0, 0.4, true);
        if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SPECIAL_LW2_SHOCK_SPELL_HOLD) {
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15.0, 8.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        let hold = if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SPECIAL_LW2_SHOCK_SPELL_HOLD) { 13 } else { 0 };
        EFFECT_FOLLOW(agent, Hash40::new("sys_smokescreen"), Hash40::new("top"), 0, 25, 13 + hold, 0, 0.0, 0, 0.4, true);
        LAST_EFFECT_SET_SCALE_W(agent, 0.4, 0.25, 0.4);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.1, 0.1);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(lua_state, 10.5);
    if is_excute(agent) {
        let hold = if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SPECIAL_LW2_SHOCK_SPELL_HOLD) { 13 } else { 0 };
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0.5, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        }
        EFFECT_FOLLOW(agent, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 15, 13 + hold, 0, 0, 180, 0.15, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        let hold = if VarModule::is_flag(agent.object(), vars::miiswordsman::status::SPECIAL_LW2_SHOCK_SPELL_HOLD) { 13 } else { 0 };
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_paralysis"), Hash40::new("arml"), 4, 0, 0, 0, 0, 0, 0.35, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 8, 13 + hold, 0, 0, 0, 0.4, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.84, 0.17);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_smokescreen"), false, false);
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_smokescreen"), -1);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_paralysis"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder_flash"), true, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_hit_elec"), -1);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_reflect_sword"), false, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("miiswordsman_reflect1"), -1);
    }
}

unsafe extern "C" fn sound_speciallw2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miiswordsman_special_c2_l01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_special_c2_l01"));
    }
    frame(lua_state, 10.5);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_m"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_s"));
    }
}

// ================================================================================================================
// =========================================== POWER THRUST =======================================================
// ================================================================================================================

unsafe extern "C" fn game_speciallw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("haver"), 15.0, 40, 72, 0, 50, 4.0, 0.0, 3.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 40, 72, 0, 50, 2.7, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 40, 72, 0, 50, 4.0, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 40, 72, 0, 50, 2.7, 0.0, 6.0, 11.0, Some(0.0), Some(6.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_WALL_CHECK);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 42, 72, 0, 50, 4.0, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 42, 72, 0, 50, 2.7, 0.0, 6.0, 11.0, Some(0.0), Some(6.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 50, 72, 0, 50, 4.0, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 50, 72, 0, 50, 2.7, 0.0, 6.0, 11.0, Some(0.0), Some(6.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairlw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    sv_kinetic_energy!(set_speed_mul_2nd, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.4, 1.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_WALL_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 275, 50, 0, 40, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 361, 90, 0, 40, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 275, 50, 0, 40, 3.0, 0.0, -7.0, 3.0, Some(0.0), Some(-9.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 13.0, 361, 90, 0, 40, 3.0, 0.0, -7.0, 3.0, Some(0.0), Some(-9.0), Some(4.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0);
    sv_kinetic_energy!(set_speed_mul_2nd, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.0, 1.0);
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 75, 0, 40, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 361, 75, 0, 40, 3.0, 0.0, -7.0, 3.0, Some(0.0), Some(-9.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 75, 0, 40, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 75, 0, 40, 3.0, 0.0, -7.0, 3.0, Some(0.0), Some(-9.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairlw3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.79);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_SP_BRAKE);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 45, 120, 0, 60, 3.5, 0.0, 3.5, -10.0, Some(0.0), Some(3.5), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_SP_BRAKE);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw1hit", game_speciallw1hit, Priority::Low);
    agent.acmd("game_specialairlw1hit", game_specialairlw1hit, Priority::Low);

    agent.acmd("game_speciallw2", game_speciallw2, Priority::Low);
    agent.acmd("game_specialairlw2", game_speciallw2, Priority::Low);
    agent.acmd("effect_speciallw2", effect_speciallw2, Priority::Low);
    agent.acmd("effect_specialairlw2", effect_speciallw2, Priority::Low);
    agent.acmd("sound_speciallw2", sound_speciallw2, Priority::Low);
    agent.acmd("sound_specialairlw2", sound_speciallw2, Priority::Low);
    
    agent.acmd("game_speciallw3", game_speciallw3, Priority::Low);
    agent.acmd("game_specialairlw3", game_specialairlw3, Priority::Low);
    agent.acmd("game_specialairlw3end", game_specialairlw3end, Priority::Low);
}