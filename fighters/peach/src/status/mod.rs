use super::*;
use globals::*;
// status script import

mod attack_air;
mod jump_aerial;
mod special_n;
mod special_hi;
mod special_s;
mod special_lw;
mod uniq_float_start;
mod uniq_float;

// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::peach::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

// Re-enables the ability to use sideB when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.battle_object, vars::peach::instance::DISABLE_SPECIAL_S);
        fighter.on_flag(*FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_SPECIAL_N_RAISE);
    }
    true.into()
}

unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    return true.into()
}

extern "Rust" {
    #[link_name = "float_check_air_jump"]
    fn float_check_air_jump(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
    #[link_name = "float_check_air_jump_aerial"]
    fn float_check_air_jump_aerial(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
}

unsafe extern "C" fn air_jump_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_check_air_jump(fighter, FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START.into())
}

unsafe extern "C" fn air_jump_aerial_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_check_air_jump_aerial(fighter, FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START.into())
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
    fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
    fighter.global_table[0x26].assign(&false.into()); //transition term handler
    fighter.global_table[0x32].assign(&L2CValue::Ptr(air_jump_uniq as *const () as _));
    fighter.global_table[0x33].assign(&L2CValue::Ptr(air_jump_aerial_uniq as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    
    attack_air::install(agent);
    jump_aerial::install(agent);
    special_n::install(agent);
    special_hi::install(agent);
    special_s::install(agent);
    special_lw::install(agent);
    uniq_float_start::install(agent);
    uniq_float::install(agent);
}
