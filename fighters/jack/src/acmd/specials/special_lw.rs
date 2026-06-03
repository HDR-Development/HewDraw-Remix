use super::*;

unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("jack_counter_mask_fire2"), Hash40::new("mask"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn sound_speciallwstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_jack_rnd_special_l01"));
    }
}

unsafe extern "C" fn expression_speciallwstart(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}

unsafe extern "C" fn game_speciallwend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.5);
}

unsafe extern "C" fn effect_speciallwend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("jack_counter_mask_fire2"), false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallwstart", acmd_stub, Priority::Low);
    agent.acmd("effect_speciallwstart", effect_speciallwstart, Priority::Low);
    agent.acmd("sound_speciallwstart", sound_speciallwstart, Priority::Low);
    agent.acmd("expression_speciallwstart", expression_speciallwstart, Priority::Low);

    agent.acmd("game_specialairlwstart", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairlwstart", effect_speciallwstart, Priority::Low);
    agent.acmd("sound_specialairlwstart", sound_speciallwstart, Priority::Low);
    agent.acmd("expression_specialairlwstart", expression_speciallwstart, Priority::Low);

    agent.acmd("game_speciallwend", game_speciallwend, Priority::Low);
    agent.acmd("effect_speciallwend", effect_speciallwend, Priority::Low);

    agent.acmd("game_specialairlwend", game_speciallwend, Priority::Low);
    agent.acmd("effect_specialairlwend", effect_speciallwend, Priority::Low);
}