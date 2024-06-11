use super::*;

unsafe extern "C" fn game_specialnshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_CANNONBALL) {
           WorkModule::on_flag(boma, *FIGHTER_KOOPAJR_STATUS_SPECIAL_N_FLAG_FAIL);
        }
        else {
            WorkModule::on_flag(boma, *FIGHTER_KOOPAJR_STATUS_SPECIAL_N_FLAG_SHOOT);
        }
    }
}

unsafe extern "C" fn effect_specialnshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_KOOPAJR_STATUS_SPECIAL_N_FLAG_FAIL) {
            EFFECT(agent, Hash40::new("koopajr_cannon_miss"), Hash40::new("clowntongue2"), 3, 0, 0, 0, 0, -90, 0.5, 0, 0, 0, 0, 0, 0, true);
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        else {
            let offset = if agent.is_situation(*SITUATION_KIND_GROUND) { 0 } else { 2 };
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("clownhip"), 4.0, 80, 95, 0, 60, 3.6, 0.0, 1.5, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("clownhip"), 4.0, 80, 95, 0, 60, 1.5, 0.0, 3.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn game_specialsspin(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
        ATTACK(agent, 0, 0, Hash40::new("throw"), 10.0, 361, 65, 0, 60, 5.2, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
}

unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL);
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("clownhip"), 2.0, 55, 100, 0, 65, 3.6, 0.0, 1.5, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("clownhip"), 2.0, 55, 100, 0, 65, 1.5, 0.0, 3.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn game_specialairsspin(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
        ATTACK(agent, 0, 0, Hash40::new("throw"), 8.0, 361, 70, 0, 60, 5.2, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_NORMAL_GRAVITY);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
            JostleModule::set_status(boma, true);
    }
}

unsafe extern "C" fn game_specialhijrrise(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPAJR_STATUS_SPECIAL_HI_SHOOT_FLAG_ACTION);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_specialhijrfall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.object(), vars::koopajr::instance::DISABLE_MECHAKOOPA) {
            ArticleModule::generate_article(boma, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA, false, 0);
            VarModule::on_flag(agent.object(), vars::koopajr::instance::DISABLE_MECHAKOOPA);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnshoot", game_specialnshoot, Priority::Low);
    agent.acmd("game_specialairnshoot", game_specialnshoot, Priority::Low);
    agent.acmd("effect_specialnshoot", effect_specialnshoot, Priority::Low);

    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialsspin", game_specialsspin, Priority::Low);
    agent.acmd("game_specialairs", game_specialairs, Priority::Low);
    agent.acmd("game_specialairsspin", game_specialairsspin, Priority::Low);

    agent.acmd("game_specialhijrrise", game_specialhijrrise, Priority::Low);
    agent.acmd("game_specialhijrfall", game_specialhijrfall, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
}
