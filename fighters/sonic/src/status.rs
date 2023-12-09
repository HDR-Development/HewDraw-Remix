use super::*;

mod wait;
mod dash;
mod special_n;
mod special_s;
mod special_s_dash;
mod special_lw_hold;
mod special_hi;

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

#[smashline::fighter_init]
fn sonic_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_SONIC {
            fighter.global_table[globals::USE_SPECIAL_N_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
            fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
            fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
            fighter.global_table[globals::USE_SPECIAL_HI_CALLBACK].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
    }
}
}

pub fn install() {
    install_agent_init_callbacks!(sonic_init);
    wait::install();
    dash::install();
    special_n::install();
    special_s::install();
    special_s_dash::install();
    special_lw_hold::install();
    special_hi::install();
}