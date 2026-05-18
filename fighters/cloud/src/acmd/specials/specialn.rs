use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
        if VarModule::is_flag(agent.battle_object, vars::cloud::status::SPECIAL_N_HOLD) {
            MotionModule::set_rate(boma, 0.375);
        }
        else {
            MotionModule::set_rate(boma, 1.5);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 3.0, 0.0, 6.5, 5.5, Some(0.0), Some(6.5), Some(4.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ArticleModule::generate_article(boma, *FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE, false, -1);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
        if VarModule::is_flag(agent.battle_object, vars::cloud::status::SPECIAL_N_HOLD) {
            MotionModule::set_rate(boma, 23.0 / 18.0); // 18F to F45
        } else {
            MotionModule::set_rate(boma, 23.0 / 30.0); // 30F to F45
        }
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_hakogeki_flash"), Hash40::new("haver"), 1, 6, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_hakogeki_scrape"), Hash40::new("top"), -1.6, 0, -10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

unsafe extern "C" fn effect_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_hakogeki_flash"), Hash40::new("haver"), 1, 6, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

// LIMIT BREAK

unsafe extern "C" fn game_specialn_lb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
        if VarModule::is_flag(agent.battle_object, vars::cloud::status::SPECIAL_N_HOLD) {
            MotionModule::set_rate(boma, 0.375);
        }
        else {
            MotionModule::set_rate(boma, 1.5);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        ArticleModule::generate_article(boma, *FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE, false, -1);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x25813802b6));
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
        if VarModule::is_flag(agent.battle_object, vars::cloud::status::SPECIAL_N_HOLD) {
            MotionModule::set_rate(boma, 23.0 / 18.0); // 18F to F45
        } else {
            MotionModule::set_rate(boma, 23.0 / 30.0); // 30F to F45
        }
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
}

unsafe extern "C" fn effect_specialn_lb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_hakogeki_flash"), Hash40::new("haver"), 1, 10, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.8);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_hakogeki_scrape"), Hash40::new("top"), -1.6, 0, -10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, false, true);
    }
}

unsafe extern "C" fn effect_specialairn_lb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_hakogeki_flash"), Hash40::new("haver"), 1, 10, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.8);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, false, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn, Priority::Low);
    agent.acmd("game_specialairn", game_specialn, Priority::Low);
    agent.acmd("effect_specialn", effect_specialn, Priority::Low);
    agent.acmd("effect_specialairn", effect_specialairn, Priority::Low);

    agent.acmd("game_specialn_lb", game_specialn_lb, Priority::Low);
    agent.acmd("game_specialairn_lb", game_specialn_lb, Priority::Low);
    agent.acmd("effect_specialn_lb", effect_specialn_lb, Priority::Low);
    agent.acmd("effect_specialairn_lb", effect_specialairn_lb, Priority::Low);
}