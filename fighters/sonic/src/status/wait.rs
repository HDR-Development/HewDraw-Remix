use super::*;

// FIGHTER_STATUS_KIND_WAIT

pub unsafe extern "C" fn wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_wait_common();
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("wait_4"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Wait_Main as *const () as _))
}
pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_WAIT, wait_main);
}
