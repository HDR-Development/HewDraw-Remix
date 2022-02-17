// status imports
use super::*;
use globals::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS,
    symbol = "_ZN7lua2cpp16L2CFighterCommon42sub_ftStatusUniqProcessGuardOff_initStatusEv")]
unsafe fn ftStatusUniqProcessGuardOff_initStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        ControlModule::set_command_life_extend(fighter.module_accessor, 5);
    }
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        ftStatusUniqProcessGuardOff_initStatus
    );
}