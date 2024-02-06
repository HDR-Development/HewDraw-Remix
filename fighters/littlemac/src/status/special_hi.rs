use super::*;
use globals::*;


// FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP


unsafe extern "C" fn special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Main, fighter, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP)(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_super_jump_punch_main as *const () as _))
}

pub fn install() {
    smashline::Agent::new("littlemac")
        .status(Main, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_main)
        .install();
}