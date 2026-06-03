use super::*;
use globals::*;

mod attack;
mod attackair;
mod attacks3;
mod batwithin;
mod escape;
mod specialairs;
mod specialn;
mod specials;
mod specialhi;
mod speciallw;
mod wait;

unsafe extern "C" fn set_lag(fighter: &mut L2CFighterCommon) { 
    // resource usage count
    let resources = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT) as f32;
    let bullet_count = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_BULLET_ARTS_COUNT) as f32;
    let dabk = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::SPECIAL_S_DABK_COUNT) as f32; // lag added to base abk lag
    let abk_total_count = fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) as f32;
    let witch_twist_count = fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT) as f32;
    // lag frame params
    let whiff_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.whiff_lag");
    let bullet_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.bullet_arts_lag");
    let dabk_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.dive_side_special");
    let abk_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.side_special");
    let witch_twist_lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.up_special");
    let base_lag: f32 = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lag.base_lag");
    let special_landing_frame_mul = fighter.get_param_float("special_landing_frame_mul", "");
    // combine
    let special_lag = (resources*whiff_lag)+(bullet_count*bullet_lag)+(dabk*dabk_lag)+(abk_total_count*abk_lag)+(witch_twist_count*witch_twist_lag)+base_lag;
    // after lag frames decided
    let adjusted_special_lag = special_landing_frame_mul * special_lag;
    if adjusted_special_lag < 1.0 {let adjusted_special_lag = 1.0;} // vanilla
    fighter.set_float(adjusted_special_lag, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME); 
}

unsafe extern "C" fn hold_check(fighter: &mut L2CFighterCommon) -> bool {
    let buffer = ControlModule::get_command_life_count_max(fighter.module_accessor) as usize;
    if fighter.is_button_on(Buttons::Special) {
        if fighter.is_button_trigger(Buttons::Special)
        || InputModule::get_trigger_count(fighter.battle_object, Buttons::Special) < buffer {
            return false.into()
        }
        return true.into()
    }
    if fighter.is_button_on(Buttons::Attack) {
        if fighter.is_button_trigger(Buttons::Attack)
        || InputModule::get_trigger_count(fighter.battle_object, Buttons::Attack) < buffer {
            return false.into()
        }
        return true.into()
    }
    false.into()
}

unsafe extern "C" fn var_reset(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_float(0.0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
    fighter.set_int(0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT);
    fighter.set_int(0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_REUSE_FRAME);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    fighter.set_int(0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AFTER_ACTION);
    return 0.into();
}

unsafe extern "C" fn special_buzzer(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::RECOVERY_RESOURCE_INVALID_INPUT) {
        let sound = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_system_beep"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, sound as i32, 0.6, 0);
        VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::RECOVERY_RESOURCE_INVALID_INPUT);
    }
    0.into()
}

unsafe extern "C" fn jump_refresh(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[globals::CURRENT_FRAME].get_i32() as f32;
    if fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) 
    && frame <= fighter.get_param_float("param_special_hi", "jump_count_reset_frame") {
        fighter.set_int(1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        EFFECT(fighter, Hash40::new("bayonetta_witchtime_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("bayonetta_feather_twinkle"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.8, true, 0, 0, 0, 0, 0, false, false);
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor) - (frame * 1.2), z: PostureModule::pos_z(fighter.module_accessor)});
    }
    0.into()
}

unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[globals::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) > 1 // no abks
        || (fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S) // no resource
        || (fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_REUSE_FRAME) <= 0 // frame limit ran out
        && fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) > 0)
        && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::RECOVERY_RESOURCE_BYPASS_CHECK)) {
            //special_buzzer(fighter);
            return false.into();
        }
        jump_refresh(fighter);
    }
    true.into()
}

unsafe extern "C" fn should_use_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[globals::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT) > 1
        || (fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI)
        && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::RECOVERY_RESOURCE_BYPASS_CHECK)) {
            //special_buzzer(fighter);
            return false.into();
        }
        jump_refresh(fighter);
    }
    true.into()
}

unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_LW) {
        return false.into();
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::USE_SPECIAL_HI_CALLBACK].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
    fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attack::install(agent);
    attackair::install(agent);
    attacks3::install(agent);
    batwithin::install(agent);
    escape::install(agent);
    specialairs::install(agent);
    specialn::install(agent);
    specials::install(agent);
    specialhi::install(agent);
    speciallw::install(agent);
    wait::install(agent);
}