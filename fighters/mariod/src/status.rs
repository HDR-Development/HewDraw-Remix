use super::*;
use globals::*;

mod special_n;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Reset cape stall flag on landing or ledgegrab
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&fighter.global_table[SITUATION_KIND].get_i32())
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD]) {
        VarModule::off_flag(fighter.battle_object, vars::mariod::instance::SPECIAL_S_DISABLE_STALL);
    }
    true.into()
}

extern "C" fn mariod_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_MARIOD {
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}

pub fn install() {
    special_n::install();
    smashline::Agent::new("mariod")
        .on_start(mariod_init)
        .install();
}
