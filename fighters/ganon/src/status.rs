use super::*;
use globals::*;

mod attack_lw3;

mod special_n;
mod special_n_float;
mod special_lw;
mod special_s;
mod special_air_s_catch;

/// Prevents side b from being used again in air when it has been disabled by up-b fall
unsafe extern "C" fn should_use_special_n_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::ganon::instance::DISABLE_SPECIAL_N) {
        false.into()
    } else {
        true.into()
    }
}

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::ganon::instance::DISABLE_SPECIAL_N);
    }
    true.into()
}

extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on agent init
        fighter.global_table[globals::USE_SPECIAL_N_CALLBACK].assign(&L2CValue::Ptr(should_use_special_n_callback as *const () as _));
        fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
    }
}

pub unsafe fn ganon_set_air(fighter: &mut L2CFighterCommon) {
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    attack_lw3::install(agent);
    special_n::install(agent);
    special_n_float::install(agent);
    special_lw::install(agent);
    special_s::install(agent);
    special_air_s_catch::install(agent);
}
