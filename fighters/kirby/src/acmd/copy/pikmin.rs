use super::*;

unsafe extern "C" fn game_pikminspecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 23.0, 12.0);
    frame(lua_state, 24.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_SHOOT_PIKMIN);
    }
    frame(lua_state, 28.0);
    FT_MOTION_RATE_RANGE(agent, 28.0, 36.0, 10.0);
}

unsafe extern "C" fn game_pikminspecialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 80, 223, 0, 20, 4.0, 0.0, 2.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_pikminspecialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, true);
    }
}

unsafe extern "C" fn sound_pikminspecialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pikmin_special_n02"));
    }
}

unsafe extern "C" fn game_pikminspecialnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 80, 223, 0, 20, 4.0, 0.0, 2.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_pikminspecialnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, true);
    }
}

unsafe extern "C" fn sound_pikminspecialnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pikmin_special_n02"));
    }
}

unsafe extern "C" fn expression_pikminspecialnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 4);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_pikminspecialn", game_pikminspecialn, Priority::Low);
    agent.acmd("game_pikminspecialairn", game_pikminspecialairn, Priority::Low);
    agent.acmd("effect_pikminspecialairn", effect_pikminspecialairn, Priority::Low);
    agent.acmd("sound_pikminspecialairn", sound_pikminspecialairn, Priority::Low);

    agent.acmd("game_pikminspecialnfailure", game_pikminspecialnfailure, Priority::Low);
    agent.acmd("effect_pikminspecialnfailure", effect_pikminspecialnfailure, Priority::Low);
    agent.acmd("sound_pikminspecialnfailure", sound_pikminspecialnfailure, Priority::Low);
    agent.acmd("expression_pikminspecialnfailure", expression_pikminspecialnfailure, Priority::Low);
}
