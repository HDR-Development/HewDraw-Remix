use super::*;

unsafe extern "C" fn special_hi_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(End, fighter, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP)(fighter);
    set_lag(fighter);
    ret
}

unsafe fn set_lag(fighter: &mut L2CFighterCommon) { 
    //vanilla: if special lag variable < lag to be set from current status, skips it to keep the higher number (the problem w whiff lag). Multiplies special lag by landing frame mul then sets it over lag variable (not sure if applicable here but idk)
    let resources = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) as f32;
    let dabk = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::DABK_COUNT) as f32; //lag added to base abk lag
    let abk_total_count = fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) as f32;
    let witch_twist_count = fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT) as f32;
    let whiff_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.whiff_lag"); //6
    let dabk_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.dive_side_special");//9
    let abk_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.side_special");//6
    let witch_twist_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.up_special");//6
    let base_lag: f32 = 8.0;
    let special_landing_frame_mul = fighter.get_param_float("special_landing_frame_mul", "");
    //normal special lag calc 
    //reworked from hard coded value based on move order (contrived) ->
    //calculate all burned resources, and add a base value. 14 -> 50 range
    let special_lag = (resources*whiff_lag)+(dabk*dabk_lag)+(abk_total_count*abk_lag)+(witch_twist_count*witch_twist_lag)+base_lag;
    //after lag frames decided
    let adjusted_special_lag = special_landing_frame_mul * special_lag;
    if adjusted_special_lag < 1.0 {let adjusted_special_lag = 1.0;} //vanilla
    fighter.set_float(adjusted_special_lag, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME); 
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_end);
}