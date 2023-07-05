// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_status_end_Guard)]
pub unsafe fn status_end_Guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_GUARD && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0xafae75f05), true, true);
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0x10da0b43c8), true, true);
    }
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hook!(
        status_end_Guard
    );
}