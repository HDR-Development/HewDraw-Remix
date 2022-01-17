// status imports
use super::*;
use globals::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon16status_end_GuardEv")]
pub unsafe fn status_end_Guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_GUARD && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0xafae75f05), true, true);
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0x10da0b43c8), true, true);
    }
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        status_end_Guard
    );
}