use super::*;
use globals::*;
use smashline::*;

pub fn install() {
  install_status_scripts!(
    wait_main
  );
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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