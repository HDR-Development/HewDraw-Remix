use super::*;
use globals::*;
// status script import

mod attack_air;
mod attacks3;
mod special_hi;
mod special_n;
mod special_s;
mod landing;

unsafe extern "C" fn use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, vars::duckhunt::instance::SPECIAL_LW_GUNMAN_TIMER) != 0 {
        return false.into();
    }
    
    return true.into();
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.object(), vars::duckhunt::instance::SPECIAL_HI2_ENABLE);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(use_special_lw_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
    VarModule::off_flag(fighter.object(), vars::duckhunt::instance::SPECIAL_HI2_ENABLE);
    VarModule::set_int(fighter.battle_object, vars::duckhunt::instance::SPECIAL_LW_GUNMAN_TIMER, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attack_air::install(agent);
    attacks3::install(agent);
    special_hi::install(agent);
    special_n::install(agent);
    special_s::install(agent);
    landing::install(agent);
}