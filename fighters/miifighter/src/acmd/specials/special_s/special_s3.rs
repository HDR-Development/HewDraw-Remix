use super::*;

// ================================================================================================
// ======================================== SUPLEX ================================================
// ================================================================================================

unsafe extern "C" fn game_specials3dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE_RANGE(agent, 4.0, 8.0, 8.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 4.5, 4.5, 6.0, 6.0);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE_RANGE(agent, 9.0, 22.0, 13.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.0, 0.5, Some(0.0), Some(8.0), Some(0.5), *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 1, Hash40::new("top"), 1.0, 0.0, 8.0, 3.5, Some(0.0), Some(8.0), Some(2.0), *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN, *COLLISION_SITUATION_MASK_GA);
        }
        else {
            CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 6.0, -1.0, None, None, None, *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_AIR_CAPTURED, *COLLISION_SITUATION_MASK_G);
            CATCH(agent, 1, Hash40::new("top"), 3.0, 0.0, 7.0, 1.0, Some(0.0), Some(8.0), Some(1.0), *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_AIR_CAPTURED, *COLLISION_SITUATION_MASK_GA);
        }
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 22.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
        if agent.is_situation(*SITUATION_KIND_AIR) {
            WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_SUPLEX_FLAG_REQUEST_GRAVITY);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 4.0, 3.0, 7.0, 7.0);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 32.0);
    FT_MOTION_RATE_RANGE(agent, 32.0, 52.0, 24.0);
    frame(lua_state, 52.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairs3landing(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 65, 230, 0, 45, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 65, 30, 0, 60, 4.0, 0.0, 4.0, 4.2, Some(0.0), Some(4.0), Some(-3.2), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(agent, -6, 7);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 0.8);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 41.0);
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials3dash", game_specials3dash, Priority::Low);
    agent.acmd("game_specialairs3dash", game_specials3dash, Priority::Low);
    agent.acmd("game_specialairs3landing", game_specialairs3landing, Priority::Low);
}