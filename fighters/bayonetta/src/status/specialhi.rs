use super::*;

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(End, fighter, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP)(fighter);
    set_lag(fighter);
    ret
}

unsafe fn set_lag(fighter: &mut L2CFighterCommon) { 
    //vanilla: if special lag variable < lag to be set from current status, skips it to keep the higher number (the problem w whiff lag). Multiplies special lag by landing frame mul then sets it over lag variable (not sure if applicable here but idk)
    let resources = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT) as f32;
    let dabk = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_S_DABK_COUNT) as f32; //lag added to base abk lag
    let abk_total_count = fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) as f32;
    let witch_twist_count = fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT) as f32;
    let whiff_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.whiff_lag"); //6
    let dabk_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.dive_side_special");//9
    let abk_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.side_special");//6
    let witch_twist_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.up_special");//6
    let base_lag: f32 = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.whiff_lag"); //8
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
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(End, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_end);
}