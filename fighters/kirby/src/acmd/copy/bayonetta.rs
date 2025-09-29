use super::*;
use globals::*;

unsafe extern "C" fn game_specialnstarth(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    let startup_frame = 14.0;
    FT_MOTION_RATE_RANGE(agent,1.0,30.0, startup_frame); // match to bayo
    if VarModule::is_flag(agent.battle_object, vars::common::instance::WAS_PREV_STATUS_CANCELABLE) {
        VarModule::off_flag(agent.battle_object, vars::bayonetta::instance::WAS_CANCEL);
    }
}

unsafe extern "C" fn game_specialnchargeh(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if VarModule::is_flag(agent.battle_object, vars::bayonetta::instance::WAS_CANCEL) {
        MotionModule::set_rate(boma, (15.0 - 1.0)/18.0); // van - 4, 35f total
    } else {
        MotionModule::set_rate(boma, (15.0 - 1.0)/28.0); // van + 4, 44f total
    }
}

unsafe extern "C" fn game_specialnendh(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    // check for accumulated special lag on a2g BA
    let cancel_frame_param = agent.get_param_int("param_special_n", "cancel_frame") as f32;
    let special_lag = agent.get_float(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
    // check for accumulated BA lag
    let max_repeat = agent.get_param_int("param_special_n", "add_fire_max");
    let remaining_repeats = VarModule::get_int(agent.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE);
    let used_rounds = (max_repeat - remaining_repeats) as f32;
    let lag_per_round = if agent.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_n.lag_per_round")} else {5.0};
    let base_endlag= if agent.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_n.base_endlag") -1.0} else {24.0}; // 32 faf van, 25 here and 40 max
    let cancel_frame= if agent.kind() == *FIGHTER_KIND_BAYONETTA {58.0} else {58.0};
    if agent.is_status(statuses::bayonetta::SPECIAL_N_CANCEL) {
        MotionModule::set_rate(boma, (cancel_frame - 1.0)/base_endlag);
    } else if special_lag < cancel_frame_param {
        MotionModule::set_rate(boma, (cancel_frame - 1.0)/(base_endlag + lag_per_round*used_rounds));
    } // do not change motion rate on special lag cancel anim
}

unsafe extern "C" fn game_specialnendf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    let max_repeat = agent.get_param_int("param_special_n", "add_fire_max");
    let remaining_repeats = VarModule::get_int(agent.battle_object, vars::bayonetta::instance::SPECIAL_N_CANCEL_TYPE);
    let used_rounds = (max_repeat - remaining_repeats) as f32;
    let lag_per_round = if agent.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_n.lag_per_round")} else {5.0};
    let base_endlag= if agent.kind() == *FIGHTER_KIND_BAYONETTA {ParamModule::get_float(agent.battle_object, ParamType::Agent, "param_special_n.base_endlag") -1.0} else {24.0}; // 32 faf van, 25 here and 40 max
    let cancel_frame= if agent.kind() == *FIGHTER_KIND_BAYONETTA {48.0} else {48.0};
    if agent.is_status(statuses::bayonetta::SPECIAL_N_CANCEL) {
        MotionModule::set_rate(boma, (cancel_frame - 1.0)/base_endlag);
    } else {
        MotionModule::set_rate(boma, (cancel_frame - 1.0)/(base_endlag + lag_per_round*used_rounds));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_bayonettaspecialnstarth", game_specialnstarth, Priority::Low);
    agent.acmd("game_bayonettaspecialnstartf", game_specialnstarth, Priority::Low);
    agent.acmd("game_bayonettaspecialairnstarth", game_specialnstarth, Priority::Low);
    agent.acmd("game_bayonettaspecialairnstartf", game_specialnstarth, Priority::Low);
    agent.acmd("game_bayonettaspecialnchargeh", game_specialnchargeh, Priority::Low);
    agent.acmd("game_bayonettaspecialnchargef", game_specialnchargeh, Priority::Low);
    agent.acmd("game_bayonettaspecialairnchargeh", game_specialnchargeh, Priority::Low);
    agent.acmd("game_bayonettaspecialairnchargef", game_specialnchargeh, Priority::Low);
    agent.acmd("game_bayonettaspecialnendh", game_specialnendh, Priority::Low);
    agent.acmd("game_bayonettaspecialnendf", game_specialnendf, Priority::Low);
    agent.acmd("game_bayonettaspecialairnendh", game_specialnendh, Priority::Low);
    agent.acmd("game_bayonettaspecialairnendf", game_specialnendf, Priority::Low);
}
