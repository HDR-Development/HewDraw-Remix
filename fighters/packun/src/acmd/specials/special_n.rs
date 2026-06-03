use super::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 1.0);
    if stance == STANCE_PRICKLY {
        VarModule::set_float(agent.battle_object, vars::packun::instance::SPECIAL_N_PTOOIE_SCALE, 1.3);
        FT_MOTION_RATE(agent, 11.0/(9.0 - 1.0));
    }
    else {
        VarModule::set_float(agent.battle_object, vars::packun::instance::SPECIAL_N_PTOOIE_SCALE, 1.0);
        FT_MOTION_RATE(agent, 0.7);
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE_SPIKEBALL);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 0.7);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialairnstart", game_specialnstart, Priority::Low);
}