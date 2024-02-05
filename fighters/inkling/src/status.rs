use super::*;

mod special_s;


unsafe extern "C" fn guard_on(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOn()
}


unsafe extern "C" fn guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Guard()
}

// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

// Re-enables the ability to use sideB when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S);
    }
    true.into()
}


extern "C" fn inkling_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_INKLING {
            fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}



pub fn install() {
    special_s::install();
    smashline::Agent::new("inkling")
        .status(Main, *FIGHTER_STATUS_KIND_GUARD_ON, guard_on)
        .status(Main, *FIGHTER_STATUS_KIND_GUARD, guard)
        .on_init(inkling_init)
        .install();
}
