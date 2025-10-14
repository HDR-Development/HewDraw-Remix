use super::*;

unsafe extern "C" fn game_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if agent.kind() != *FIGHTER_KIND_KIRBY {
            MeterModule::add(agent.battle_object, 2.0 * MeterModule::damage_gain_mul(agent.battle_object));
        }
        agent.on_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        MotionModule::set_rate(boma, 1.0);
    } else {
        MotionModule::set_rate(boma, 0.83);
    }

    frame(lua_state, 18.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        if agent.kind() != *FIGHTER_KIND_KIRBY {
            MeterModule::add(agent.battle_object, 2.0 * MeterModule::damage_gain_mul(agent.battle_object));
        }
        agent.on_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialairn", game_specialairn, Priority::Low);
    agent.acmd("game_specialn", game_specialn, Priority::Low);
}
