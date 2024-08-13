// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_status_end_GuardOff)]
unsafe fn status_end_GuardOff(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::disable_gold_eye(fighter.module_accessor);
    WorkModule::unable_transition_term_forbid(
        fighter.module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON
    );
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hook!(status_end_GuardOff);
}
