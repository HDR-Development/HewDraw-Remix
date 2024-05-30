// status imports
use super::*;
use globals::*;

use super::super::*;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_execStatus_common)]
unsafe fn sub_ftStatusUniqProcessGuardDamage_execStatus_common(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        super::super::fighter_status_guard::set_just_shield_scale(fighter);
        return;
    }

    let shield_level = fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let scale = fighter_status_guard::calc_shield_scale(fighter, shield_level.into()).get_f32();
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &(Vector3f {x: scale, y: scale, z: scale,}));
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_execStatus)]
unsafe fn ftStatusUniqProcessGuardDamage_execStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuardDamage_execStatus_common(fighter);
    return 0.into();
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_execStop_common)]
unsafe fn sub_ftStatusUniqProcessGuardDamage_execStop_common(fighter: &mut L2CFighterCommon) {
    if fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        fighter_status_guard::set_just_shield_scale(fighter);
        guard_on::exec::sub_ftStatusUniqProcessGuardOn_execStop_Inner(fighter, FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_DELAY_MUL.into());
        return;
    }

    if WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_PREV_SHIELD_SCALE_FRAME, 0) {
        let handle = fighter.get_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE_EFFECT_HANDLE);
        if handle != 0 {
            EffectModule::set_scale(fighter.module_accessor, handle as u32, &(Vector3f {x: 0.1, y: 0.1, z: 0.1,}));
        }

        let shield_level = fighter.get_float(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_PREV_SHIELD);
        let scale = fighter_status_guard::calc_shield_scale(fighter, shield_level.into()).get_f32();
        ModelModule::set_joint_scale( fighter.module_accessor, Hash40::new("throw"), &(Vector3f {x: scale, y: scale, z: scale}));
        EffectModule::detach_kind(fighter.module_accessor, Hash40::new("sys_shield_damage3"), 5);

        let frame = StopModule::get_hit_stop_real_frame(fighter.module_accessor) as i32;
        fighter.set_int(frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_SCALE_FADE_START_FRAME);
    }

    let scale = if fighter.get_int(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_PREV_SHIELD_SCALE_FRAME) > 0 {
        let shield_level = fighter.get_float(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_PREV_SHIELD);
        fighter_status_guard::calc_shield_scale(fighter, shield_level.into()).get_f32()
    } else {
        let shield_level = fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        let scale = fighter_status_guard::calc_shield_scale(fighter, shield_level.into()).get_f32();
        let handle = fighter.get_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE2_EFFECT_HANDLE);
        if handle != 0 {
            let size = if fighter.get_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_SCALE_FADE_START_FRAME) > 0 {
                let prev_shield = fighter.get_float(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_PREV_SHIELD);
                let prev_scale = fighter_status_guard::calc_shield_scale(fighter, prev_shield.into()).get_f32();
                let stop_frame = StopModule::get_hit_stop_real_frame(fighter.module_accessor) as f32;
                let fade_frame = fighter.get_int(*FIGHTER_STATUS_GUARD_ON_WORK_INT_SCALE_FADE_START_FRAME) as f32;
                let frame = ((stop_frame - 1.0) / fade_frame).clamp(0.0, 1.0);
                interpolation::Lerp::lerp(&frame, &1.0, &(prev_scale / scale))
            } else {
                1.0
            };
            EffectModule::set_scale(fighter.module_accessor, handle as u32, &(Vector3f {x: size * 0.1, y: size * 0.1, z: size * 0.1}));
        }
        scale
    };
    ModelModule::set_joint_scale( fighter.module_accessor, Hash40::new("throw"), &(Vector3f {x: scale, y: scale, z: scale}));

    guard_on::exec::sub_ftStatusUniqProcessGuardOn_execStop_Inner(fighter, FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_DELAY_MUL.into());
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_execStop)]
unsafe fn ftStatusUniqProcessGuardDamage_execStop(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuardDamage_execStop_common(fighter);
    return 0.into();
}

#[skyline::hook(replace = L2CFighterCommon_FighterStatusUniqProcessGuardDamage_leave_stop)]
unsafe fn FighterStatusUniqProcessGuardDamage_leave_stop(
    fighter: &mut L2CFighterCommon,
    arg1: L2CValue,
    arg2: L2CValue
) -> L2CValue {
    effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sys_shield_damage2"), false, false);
    effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sys_shield_damage3"), false, false);

    if !fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        return 0.into();
    }
    
    if fighter.get_param_int("param_motion", "just_shield_motion") == 0 {
        let rate = MotionModule::rate(fighter.module_accessor);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("guard_off"), 0.0, rate, false, 0.0, false, false);
    }

    if fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD_SLOW_WHOLE) 
    && SlowModule::whole_frame(fighter.module_accessor) > 0 {
        SlowModule::clear_whole(fighter.module_accessor);
        fighter.off_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD_SLOW_WHOLE);
    }
    return 0.into();
}

pub fn install() {
    skyline::install_hooks!(
        sub_ftStatusUniqProcessGuardDamage_execStatus_common,
        sub_ftStatusUniqProcessGuardDamage_execStop_common,
        ftStatusUniqProcessGuardDamage_execStatus,
        ftStatusUniqProcessGuardDamage_execStop,
        FighterStatusUniqProcessGuardDamage_leave_stop
    );
}
