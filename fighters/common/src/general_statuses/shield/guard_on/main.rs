// status imports
use super::*;
use globals::*;

use super::super::misc;

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon26sub_status_guard_on_commonEv")]
unsafe fn sub_status_guard_on_common(fighter: &mut L2CFighterCommon) {
    let shield_min_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_min_frame"));
    WorkModule::set_int(fighter.module_accessor, shield_min_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("guard_on"), 0.0, 1.0, false, 0.0, false, false);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("guard"), 0.0, 1.0, false, 1.0);
        MotionModule::set_rate_2nd(fighter.module_accessor, 0.0);
        misc::sub_ftStatusUniqProcessGuardFunc_updateShield(fighter, true.into());
    }
    misc::sub_guard_cont_pre(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        misc::sub_guard_on_uniq(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(misc::sub_guard_on_uniq as *const () as _));
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon35sub_status_guard_on_main_air_commonEv")]
unsafe fn sub_status_guard_on_main_air_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status(
            FIGHTER_STATUS_KIND_FALL.into(),
            false.into()
        );
        true.into()
    } else {
        false.into()
    }
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon19status_GuardOn_MainEv")]
unsafe fn status_GuardOn_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT) 
        && 0.0 < fighter.global_table[CURRENT_FRAME].get_f32() {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT);
    }

    if !sub_status_guard_on_main_air_common(fighter).get_bool() 
        && !misc::sub_guard_cont(fighter).get_bool() 
        && !misc::status_guard_main_common(fighter).get_bool()
        && MotionModule::is_end(fighter.module_accessor)
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.change_status(
            FIGHTER_STATUS_KIND_GUARD.as_lua_int(),
            false.into()
        );
    }

    L2CValue::I32(0)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_ON, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon14status_GuardOnEv")]
unsafe fn status_GuardOn(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_status_guard_on_common(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_GuardOn_Main as *const () as _))
}

pub fn install() {
    install_status_scripts!(
        status_GuardOn
    );

    install_hooks!(
        sub_status_guard_on_common,
        sub_status_guard_on_main_air_common,
        status_GuardOn_Main
    );
}