use super::*;
use globals::*;
// status script import

mod attack_air;
mod ground_pound;
mod special_hi;
mod special_n;
mod special_lw;
mod special_s;
mod rebirth;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Reset cape stall flag on landing or ledgegrab
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&fighter.global_table[SITUATION_KIND].get_i32())
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.battle_object, vars::mario::instance::SPECIAL_S_DISABLE);
        VarModule::off_flag(fighter.battle_object, vars::mario::instance::SPECIAL_LW_DISABLE);
    }
    return true.into()
}

// unsafe extern "C" fn special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if VarModule::is_flag(fighter.battle_object, vars::mario::instance::SPECIAL_LW_DISABLE) {
//         return false.into();
//     }
//     return true.into();
// }

unsafe extern "C" fn special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::mario::instance::SPECIAL_S_DISABLE) {
        return false.into();
    }
    return true.into();
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
    // fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(special_lw_callback as *const () as _));
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(special_s_callback as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attack_air::install(agent);
    ground_pound::install(agent);
    special_hi::install(agent);
    special_n::install(agent);
    special_lw::install(agent);
    special_s::install(agent);
    rebirth::install(agent);
}