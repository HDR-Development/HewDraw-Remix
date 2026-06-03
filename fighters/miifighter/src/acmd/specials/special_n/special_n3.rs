use super::*;

// ================================================================================================
// ======================================== WILD THROW ============================================
// ================================================================================================

unsafe extern "C" fn game_specialn3catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 3.5, 7.0);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 3.5);
    FT_MOTION_RATE_RANGE(agent, 3.5, 6.0, 3.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
        CATCH(agent, 2, Hash40::new("top"), 5.0, 0.0, 7.0, 9.0, Some(0.0), Some(7.0), Some(9.5), *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.2);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            let air_accel_y = agent.get_param_float("air_accel_y", "");
            let air_speed_y_stable = agent.get_param_float("air_speed_y_stable", "");
            sv_kinetic_energy!(set_accel, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
            sv_kinetic_energy!(set_limit_speed, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
        }
    }
}

unsafe extern "C" fn effect_specialn3catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_guard_mark"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_counter_success"), Hash40::new("top"), -1, 7, 8, 0, 90, 0, 0.6, false);
    }
}

unsafe extern "C" fn game_specialn3throw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        REVERSE_LR(agent);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 14.0, 275, 100, 25, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_CATCH_STOP(agent, 7, 1);
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 7.0, 10.0);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("armr"), 9.0, 361, 85, 0, 80, 5.0, 6.0, 0.0, 0.0, Some(6.0), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 9.0, 361, 85, 0, 80, 4.0, 0.0, 3.0, -5.0, Some(0.0), Some(3.0), Some(-11.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        let opponent_boma = agent.get_grabbed_opponent_boma();
        if opponent_boma.is_fighter() && VarModule::has_var_module(opponent_boma.object()) {
            VarModule::on_flag(opponent_boma.object(), vars::common::instance::FORCE_TUMBLE_NO_BOUNCE);
        }
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 47.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairn3throw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING);
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        REVERSE_LR(agent);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 14.0, 280, 28, 0, 33, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_CATCH_STOP(agent, 7, 1);
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 7.0, 10.0);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("armr"), 9.0, 361, 85, 0, 80, 5.0, 6.0, 0.0, 0.0, Some(6.0), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        let opponent_boma = agent.get_grabbed_opponent_boma();
        if opponent_boma.is_fighter() {
            VarModule::on_flag(opponent_boma.object(), vars::common::instance::FORCE_TUMBLE_NO_BOUNCE);
        }
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialairn3throw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        COL_PRI(agent, 101);
        FLASH(agent, 1, 1, 1, 0);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_catch"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("miifighter_counter_arc"), Hash40::new("miifighter_counter_arc"), Hash40::new("top"), -1, 8, 1, 0, 112, 90, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw3catch", game_specialn3catch, Priority::Low);
    agent.acmd("game_specialairlw3catch", game_specialn3catch, Priority::Low);
    agent.acmd("effect_speciallw3catch", effect_specialn3catch, Priority::Low);
    agent.acmd("effect_specialairlw3catch", effect_specialn3catch, Priority::Low);
    
    agent.acmd("game_speciallw3throw", game_specialn3throw, Priority::Low);
    agent.acmd("game_specialairlw3throw", game_specialairn3throw, Priority::Low);
    agent.acmd("effect_specialairlw3throw", effect_specialairn3throw, Priority::Low);
}