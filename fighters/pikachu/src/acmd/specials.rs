use super::*;


unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_s");
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        WorkModule::on_flag(boma, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_ATTACK_TRIGGER);
        ATTACK(agent, 0, 0, Hash40::new("rot"), 1.0, 361, 78, 0, 40, 3.0, 0.0, -0.7, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        WorkModule::on_flag(boma, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_CALC_ATTACK_POWER);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 3.0);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    wait(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
    }
}

unsafe extern "C" fn game_specialairsmissend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_s");
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_MISS_END_RUMBLE_2);
    }
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 13.0, 10.0);
    frame(lua_state, 13.0);
}

unsafe extern "C" fn game_specialhi1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 2.0, 70, 100, 20, 0, 1.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        JostleModule::set_status(boma, false);
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 3.0, 70, 150, 0, 20, 1.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        JostleModule::set_status(boma, false);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_KAMINARI_GENERATE);
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 361, 85, 0, 70, 13.0, 0.0, 10.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 17, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialairsmissend", game_specialairsmissend, Priority::Low);

    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialairhistart", game_specialhistart, Priority::Low);

    agent.acmd("game_specialhi1", game_specialhi1, Priority::Low);
    agent.acmd("game_specialairhi1", game_specialhi1, Priority::Low);

    agent.acmd("game_specialhi2", game_specialhi2, Priority::Low);
    agent.acmd("game_specialairhi2", game_specialhi2, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("expression_speciallw", expression_speciallw, Priority::Low);
    agent.acmd("expression_specialairlw", expression_speciallw, Priority::Low);
    agent.acmd("game_speciallwhit", game_speciallwhit, Priority::Low);
}