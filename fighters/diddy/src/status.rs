use super::*;
use globals::*;

mod special_n;

mod special_s;
mod special_s_jump;

mod special_hi;

// FIGHTER_STATUS_KIND_JUMP_SQUAT

unsafe extern "C" fn end_jump_squat(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_JumpSquat();
    0.into()
}

// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::diddy::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

// Re-enables the ability to use sideB when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::diddy::instance::DISABLE_SPECIAL_S);
    }
    true.into()
}

extern "C" fn diddy_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_DIDDY {
            fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}

pub fn install() {
    special_n::install();
    special_s::install();
    special_s_jump::install();
    special_hi::install();
    smashline::Agent::new("diddy")
        .status(End, *FIGHTER_STATUS_KIND_JUMP_SQUAT, end_jump_squat)
        .on_init(diddy_init)
        .install();
}