use super::*;
use globals::*;
utils::import!(popo::{ics_dash});
// status script import
 
pub fn install() {
    install_status_scripts!(
        dash,
        throw
    );
}

// FIGHTER_STATUS_KIND_DASH //

#[status_script(agent = "nana", status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    popo::ics_dash(fighter)
}

#[status_script(agent = "nana", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn throw(fighter: &mut L2CFighterCommon) -> L2CValue {
    // force nana into a random throw
    let selected = app::sv_math::rand(hash40("fighter"), 100);
    let throw_name = match selected {
        0..=24 => "throw_b",
        25..=49 => "throw_f",
        50..=74 => "throw_lw",
        _ => "throw_hi"
    };

    MotionModule::change_motion(fighter.boma(), Hash40::new(throw_name), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Throw_Main as *const () as _))
}