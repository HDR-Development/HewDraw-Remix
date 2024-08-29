use super::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::duckhunt::status::SPECIAL_HI_JUMP);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 60, 100, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_duckhunt_special_h01"));
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        //PLAY_SE(agent, Hash40::new("vc_duckhunt_duck_attack01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_duckhunt_rnd_attack_duck"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_duckhunt_dog_attack01"));
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::duckhunt::status::SPECIAL_HI_JUMP);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 60, 100, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::duckhunt::status::SPECIAL_HI2_ENABLE_SHOT);
    }
}

unsafe extern "C" fn effect_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
}

unsafe extern "C" fn sound_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        //PLAY_SE(agent, Hash40::new("vc_duckhunt_duck_attack02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_duckhunt_rnd_attack_duck"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_duckhunt_dog_attack02"));
    }
}

unsafe extern "C" fn expression_specialhi2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    
}

unsafe extern "C" fn game_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 60, 100, 0, 30, 7.5, 0.0, 7.0, -3.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("duckhunt_trick_smoke"), Hash40::new("top"), 0, 7, -3, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
}

unsafe extern "C" fn sound_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_duckhunt_special_n07"));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_punch_hit_l"));
    }
}

unsafe extern "C" fn expression_specialhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi, Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi, Priority::Low);
    agent.acmd("expression_specialhi", expression_specialhi, Priority::Low);

    agent.acmd("game_specialhi2", game_specialhi2, Priority::Low);
    agent.acmd("effect_specialhi2", effect_specialhi2, Priority::Low);
    agent.acmd("sound_specialhi2", sound_specialhi2, Priority::Low);
    agent.acmd("expression_specialhi2", expression_specialhi2, Priority::Low);

    agent.acmd("game_specialhi3", game_specialhi3, Priority::Low);
    agent.acmd("effect_specialhi3", effect_specialhi3, Priority::Low);
    agent.acmd("sound_specialhi3", sound_specialhi3, Priority::Low);
    agent.acmd("expression_specialhi3", expression_specialhi3, Priority::Low);
}