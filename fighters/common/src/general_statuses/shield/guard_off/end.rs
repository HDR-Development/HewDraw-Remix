// status imports
use super::*;
use globals::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon19status_end_GuardOffEv")]
unsafe fn status_end_GuardOff(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        status_end_GuardOff
    );
}