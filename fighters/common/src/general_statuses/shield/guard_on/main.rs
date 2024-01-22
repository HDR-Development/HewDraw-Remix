// status imports
use super::*;
use globals::*;

use super::super::misc;

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_on_common)]
unsafe fn sub_status_guard_on_common(fighter: &mut L2CFighterCommon) {
    let shield_min_frame = WorkModule::get_param_int(
        fighter.module_accessor,
        hash40("common"),
        hash40("shield_min_frame")
    );
    WorkModule::set_int(
        fighter.module_accessor,
        shield_min_frame,
        *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME
    );
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("guard_on"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if
        !WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION
        )
    {
        MotionModule::add_motion_2nd(
            fighter.module_accessor,
            Hash40::new("guard"),
            0.0,
            1.0,
            false,
            1.0
        );
        MotionModule::set_rate_2nd(fighter.module_accessor, 0.0);
        misc::sub_ftStatusUniqProcessGuardFunc_updateShield(fighter, true.into());
    }
    misc::sub_guard_cont_pre(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        misc::sub_guard_on_uniq(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(
        &L2CValue::Ptr(misc::sub_guard_on_uniq as *const () as _)
    );
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_on_main_air_common)]
unsafe fn sub_status_guard_on_main_air_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        true.into()
    } else {
        false.into()
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOn_Main)]
unsafe fn status_GuardOn_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    misc::check_enable_cstick_buffer_rolls(fighter);
    if
        !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT) &&
        0.0 < fighter.global_table[CURRENT_FRAME].get_f32()
    {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
        let handle = EffectModule::get_last_handle(fighter.module_accessor) as u32;
        VarModule::set_int(
            fighter.object(),
            vars::common::instance::SHIELD_EFFECT_HANDLE,
            handle as _
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT);
    }

    if
        !sub_status_guard_on_main_air_common(fighter).get_bool() &&
        !misc::sub_guard_cont(fighter).get_bool() &&
        !misc::status_guard_main_common(fighter).get_bool() &&
        MotionModule::is_end(fighter.module_accessor) &&
        fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    {
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD.as_lua_int(), false.into());
    }

    L2CValue::I32(0)
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOn)]
unsafe fn status_GuardOn(fighter: &mut L2CFighterCommon) -> L2CValue {
    misc::check_enable_cstick_buffer_rolls(fighter);
    sub_status_guard_on_common(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_GuardOn_Main as *const () as _))
}

pub fn install() {
    skyline::install_hooks!(
        sub_status_guard_on_common,
        sub_status_guard_on_main_air_common,
        status_GuardOn_Main,
        status_GuardOn
    );
}
