use super::*;

mod special_n;
mod special_s;
mod special_hi;
mod special_lw;
mod special_lw_cancel;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::littlemac::instance::KO_MOTION_AIRTIME);
    }
    true.into()
}

unsafe extern "C" fn littlemac_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_LITTLEMAC {
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}

pub fn install() {
    special_n::install();
    special_s::install();
    special_hi::install();
    special_lw::install();
    special_lw_cancel::install();
    smashline::Agent::new("littlemac")
        .on_start(littlemac_init)
        .install();
}