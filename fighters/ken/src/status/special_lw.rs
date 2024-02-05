use super::*;
use globals::*;
use smashline::*;



// FIGHTER_STATUS_KIND_SPECIAL_LW //


pub unsafe extern "C" fn init_special_lw(fighter: &mut L2CFighterCommon) -> L2CValue {
    // once-per-airtime (refreshes on hit)
    VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_LW);
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

pub fn install() {
    smashline::Agent::new("ken")
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, init_special_lw)
        .install();
}
