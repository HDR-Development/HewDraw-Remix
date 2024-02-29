use super::*;
use globals::*;
// status script import
 

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE //

pub unsafe extern "C" fn pre_special_hi_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR);
    return 1.into()
}

pub fn install() {
    smashline::Agent::new("koopajr")
        .status(Pre, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE, pre_special_hi_escape)
        .install();
}