// status imports
use super::*;
use globals::*;

use super::super::misc;

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_common)]
unsafe fn sub_status_guard_common(fighter: &mut L2CFighterCommon) {
    misc::sub_guard_cont_pre(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        misc::sub_guard_on_uniq(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(
        &L2CValue::Ptr(misc::sub_guard_on_uniq as *const () as _)
    );
}

#[skyline::hook(replace = L2CFighterCommon_status_guard_main_common_air)]
unsafe fn status_guard_common_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        true.into()
    } else {
        false.into()
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_Guard_Main)]
unsafe fn status_Guard_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !status_guard_common_air(fighter).get_bool() {
        if !misc::sub_guard_cont(fighter).get_bool() {
            misc::status_guard_main_common(fighter);
        }
    }
    L2CValue::I32(0)
}

#[skyline::hook(replace = L2CFighterCommon_status_Guard)]
unsafe fn status_Guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_status_guard_common(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_Guard_Main as *const () as _))
}

pub fn install() {
    skyline::install_hooks!(
        status_Guard,
        status_Guard_Main,
        status_guard_common_air,
        sub_status_guard_common
    );
}
