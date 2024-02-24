use super::*;
use globals::*;

// FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END

pub unsafe extern "C" fn special_s_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(End, fighter, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END)(fighter);
    // Land cancel on sideB rebound
    if fighter.is_motion_one_of(&[Hash40::new("special_air_s_finish"), Hash40::new("special_air_s_finish_c2"), Hash40::new("special_air_s_finish_c3"), Hash40::new("special_air_s_finish_c4")])
    && VarModule::is_flag(fighter.battle_object, vars::metaknight::instance::SIDE_SPECIAL_HIT) {
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_frame"), 0);
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    }
    ret
}

pub fn install() {
    smashline::Agent::new("metaknight")
        .status(End, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END, special_s_end_end)
        .install();
}