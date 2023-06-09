use super::*;
use smashline::*;

mod special_s;

/// Prevents down b being reused
unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::pacman::instance::DISABLE_SPECIAL_LW) {
        false.into()
    } else {
        true.into()
    }
}

// Re-enables the ability to use sideB when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD]) {
        VarModule::off_flag(fighter.battle_object, vars::pacman::instance::DISABLE_SPECIAL_LW);
    }
    true.into()
}

#[smashline::fighter_init]
fn pacman_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_PACMAN {
            fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
        }
    }
}


pub fn install() {
    smashline::install_agent_init_callbacks!(pacman_init);
    special_s::install();
}