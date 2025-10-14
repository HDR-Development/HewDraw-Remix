use super::*;

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR)
        && !VarModule::is_flag(agent.battle_object, vars::cloud::instance::SPECIAL_S_DISABLE_STALL) {
            // some extra vertical momentum if stall is allowed
            let speed_y = 0.75;
            sv_kinetic_energy!(reset_energy, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
        } else {
            agent.off_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_DETACH_EFFECT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 80, 100, 66, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(15.0), Some(17.5), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 80, 100, 66, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(11.0), Some(12.5), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if !agent.is_situation(*SITUATION_KIND_AIR)
        || !VarModule::is_flag(agent.battle_object, vars::cloud::instance::SPECIAL_S_DISABLE_STALL) {
            // only enable followup attacks if stall is allowed
            agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_HIT_ATTACK);
        }
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        agent.off_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}

unsafe extern "C" fn game_specials2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::cloud::status::SPECIAL_S_STALL);
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_DETACH_EFFECT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(15.0), Some(17.5), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(11.0), Some(12.5), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_HIT_ATTACK);
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        VarModule::off_flag(agent.battle_object, vars::cloud::status::SPECIAL_S_STALL);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        agent.off_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}

unsafe extern "C" fn game_specials3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 2.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::cloud::status::SPECIAL_S_STALL);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(15.0), Some(17.5), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(11.0), Some(12.5), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(15.0), Some(17.5), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(11.0), Some(12.5), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE_RANGE(agent, 14.0, 25.0, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 50, 140, 0, 30, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(15.0), Some(17.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 50, 140, 0, 30, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(11.0), Some(12.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::off_flag(agent.battle_object, vars::cloud::status::SPECIAL_S_STALL);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}

unsafe extern "C" fn game_specials1_lb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 0.75);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::cloud::status::SPECIAL_S_STALL);
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_DETACH_EFFECT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(15.0), Some(17.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(11.0), Some(12.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_HIT_ATTACK);
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
}

unsafe extern "C" fn game_specials2_lb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 2.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::cloud::status::SPECIAL_S_STALL);
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_DETACH_EFFECT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(15.0), Some(17.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(11.0), Some(12.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_HIT_ATTACK);
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
}

unsafe extern "C" fn game_specials3_lb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::cloud::status::SPECIAL_S_STALL);
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(15.0), Some(17.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(11.0), Some(12.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(15.0), Some(17.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 366, 100, 80, 0, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(11.0), Some(12.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 50, 140, 0, 20, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(15.0), Some(17.5), 1.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 12, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 50, 140, 0, 20, 7.5, 0.0, 11.0, 17.5, Some(0.0), Some(11.0), Some(12.5), 1.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 12, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
        AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
        agent.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_LB_SCENE);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::off_flag(agent.battle_object, vars::cloud::status::SPECIAL_S_STALL);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x25813802b6));
        agent.off_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials1", game_specials1, Priority::Low);
    agent.acmd("game_specialairs1", game_specials1, Priority::Low);
    agent.acmd("game_specials2", game_specials2, Priority::Low);
    agent.acmd("game_specialairs2", game_specials2, Priority::Low);
    agent.acmd("game_specials3", game_specials3, Priority::Low);
    agent.acmd("game_specialairs3", game_specials3, Priority::Low);

    agent.acmd("game_specials1_lb", game_specials1_lb, Priority::Low);
    agent.acmd("game_specialairs1_lb", game_specials1_lb, Priority::Low);
    agent.acmd("game_specials2_lb", game_specials2_lb, Priority::Low);
    agent.acmd("game_specialairs2_lb", game_specials2_lb, Priority::Low);
    agent.acmd("game_specials3_lb", game_specials3_lb, Priority::Low);
    agent.acmd("game_specialairs3_lb", game_specials3_lb, Priority::Low);
}