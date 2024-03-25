use super::*;

mod wait;
mod dash;
mod special_n;
mod special_s;
mod special_s_dash;
mod special_lw_hold;
mod special_hi;
mod special_n_hit;

/// Prevents side b from being used again in air
unsafe extern "C" fn should_use_special_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::sonic::instance::USED_AIR_ACTION)) || fighter.is_status(*FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP)  {
        false.into()
    } else {
        true.into()
    }
}

/// Prevents side b from being used again in air
unsafe extern "C" fn should_use_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_status(*FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP)  {
        false.into()
    } else {
        true.into()
    }
}

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD]) {
        VarModule::off_flag(fighter.battle_object, vars::sonic::instance::USED_AIR_ACTION);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_N_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
    fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
    fighter.global_table[globals::USE_SPECIAL_HI_CALLBACK].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    wait::install(agent);
    dash::install(agent);
    special_n::install(agent);
    special_s::install(agent);
    special_s_dash::install(agent);
    special_lw_hold::install(agent);
    special_hi::install(agent);
    special_n_hit::install(agent);
}
