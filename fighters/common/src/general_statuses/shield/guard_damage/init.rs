// status imports
use super::*;
use globals::*;

use super::super::fighter_status_guard;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_initStatus_Inner)]
unsafe fn sub_ftStatusUniqProcessGuardDamage_initStatus_Inner(fighter: &mut L2CFighterCommon) {
    let shield_power = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_POWER,
    );
    let setoff_mul = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_SETOFF_MUL,
    );
    let param_setoff_mul = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("common"),
        hash40("shield_setoff_mul"),
    );

    // let analog = InputModule::get_analog_for_guard(fighter.battle_object);
    // let param_setoff_mul = if analog > 0.0 && analog < 1.0 {
    //     ((1.0 - analog) * 0.65 + param_setoff_mul / 1.5) * 1.5
    // } else {
    //     param_setoff_mul
    // };

    let mut shield_power = shield_power * setoff_mul * param_setoff_mul;
    let object_id = WorkModule::get_int(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_OBJECT_ID,
    );

    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD,
    ) {
        shield_power *= WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("common"),
            hash40("just_shield_setoff_mul"),
        );
    }
    shield_power += WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("common"),
        hash40("shield_setoff_add"),
    );
    let max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x16a1c7df3f);
    if max < shield_power {
        shield_power = max;
    }

    /*    if object_id != 0x50000000 {
            capture!(fighter, MA_MSC_CMD_CAPTURE_SET_IGNORE_OBJECT_ID, object_id);
            let mut cancel_frame = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
                shield_power
            } else {
                shield_power + WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_cancel_frame")) as f32
            };
            cancel_frame *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_ignore_capture_rate"));
            WorkModule::set_int(fighter.module_accessor, cancel_frame as i32, *FIGHTER_INSTANCE_WORK_ID_INT_GUARD_INVALID_CAPTURE_FRAME);
        }
    */
    // This block of code restricts an attacker from grabbing an opponent during their shield stun and shield release frames unless the opponent was hit by another item instead.
    // By removing this, an opponent can be grabbed during their blockstun.

    WorkModule::set_int(
        fighter.module_accessor,
        shield_power as i32,
        *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME,
    );
    if !WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD,
    ) {
        let catch_frame = WorkModule::get_param_int(
            fighter.module_accessor,
            hash40("common"),
            hash40("shield_setoff_catch_frame"),
        );
        if 0 < catch_frame {
            WorkModule::set_int(
                fighter.module_accessor,
                catch_frame,
                *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME,
            );
        }

        let motion_rate = app::sv_fighter_util::get_guard_damage_motion_rate(
            fighter.lua_state_agent,
            Hash40::new("guard_damage"),
        );
        let weight = MotionModule::weight(fighter.module_accessor);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("guard_damage"),
            0.0,
            motion_rate,
            false,
            0.0,
            false,
            false,
        );
        if !WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION,
        ) {
            MotionModule::add_motion_2nd(
                fighter.module_accessor,
                Hash40::new("guard"),
                0.0,
                1.0,
                false,
                1.0,
            );
            MotionModule::set_rate_2nd(fighter.module_accessor, 0.0);
            MotionModule::set_weight(fighter.module_accessor, weight, true);
            let prev_x = WorkModule::get_float(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_X,
            );
            let prev_y = WorkModule::get_float(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_Y,
            );
            fighter_status_guard::set_guard_blend_motion_angle(
                fighter,
                prev_x.into(),
                prev_y.into(),
            );
        }
    } else {
        let mut cancel_frame = FighterMotionModuleImpl::get_cancel_frame(
            fighter.module_accessor,
            Hash40::new_raw(0xfefe225e5),
            true,
        );
        if cancel_frame == 0.0 {
            cancel_frame = MotionModule::end_frame_from_hash(
                fighter.module_accessor,
                Hash40::new_raw(0xfefe225e5),
            );
        }

        let adjusted = cancel_frame / shield_power.floor();
        if WorkModule::get_param_int(
            fighter.module_accessor,
            hash40("param_motion"),
            hash40("just_shield_motion"),
        ) != 0
        {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(0xfefe225e5),
                0.0,
                adjusted.max(1.0),
                false,
                0.0,
                false,
                false,
            );
        } else {
            let end_frame = MotionModule::end_frame_from_hash(
                fighter.module_accessor,
                Hash40::new_raw(0xfefe225e5),
            );
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(0xfefe225e5),
                end_frame,
                adjusted.max(1.0),
                false,
                0.0,
                false,
                false,
            );
        }
        MotionAnimcmdModule::call_script_single(
            fighter.module_accessor,
            *FIGHTER_ANIMCMD_EXPRESSION,
            Hash40::new_raw(0x1a29f56bfb),
            -1,
        );
    }

    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD,
    ) {
        WorkModule::inc_int(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_SHEILD_COUNT,
        );
        if fighter_status_guard::is_continue_just_shield_count(fighter).get_bool() {
            ShieldModule::set_status(
                fighter.module_accessor,
                *FIGHTER_SHIELD_KIND_GUARD,
                app::ShieldStatus(*SHIELD_STATUS_NORMAL),
                0,
            );
            ShieldModule::set_shield_type(
                fighter.module_accessor,
                app::ShieldType(*SHIELD_TYPE_JUST_SHIELD),
                *FIGHTER_SHIELD_KIND_GUARD,
                0,
            );
            ReflectorModule::set_status(
                fighter.module_accessor,
                0,
                app::ShieldStatus(*SHIELD_STATUS_NORMAL),
                *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD,
            );
            GroundModule::correct(
                fighter.module_accessor,
                GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP),
            );
        } else {
            CancelModule::enable_cancel(fighter.module_accessor);
            WorkModule::on_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_DISABLE_HIT_STOP_DELAY_STICK,
            );
        }
    } else {
        ShieldModule::set_status(
            fighter.module_accessor,
            *FIGHTER_SHIELD_KIND_GUARD,
            app::ShieldStatus(*SHIELD_STATUS_NORMAL),
            0,
        );
        ControlModule::clear_command(fighter.module_accessor, false);
    }

    let setoff_speed_mul = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("common"),
        hash40("shield_setoff_speed_mul"),
    );

    // let analog = InputModule::get_analog_for_guard(fighter.battle_object);
    // let setoff_speed_mul = if analog > 0.0 && analog < 1.0 {
    //     0.195 * (1.0 - analog) + setoff_speed_mul
    // } else {
    //     setoff_speed_mul
    // };

    let mut setoff_speed = shield_power * setoff_speed_mul;

    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD,
    ) {
        setoff_speed *= WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("common"),
            hash40("just_shield_speed_rate"),
        );
    }

    setoff_speed = setoff_speed.min(WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("common"),
        hash40("shield_setoff_speed_max"),
    ));
    let shield_lr = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR,
    );
    let directed_speed = setoff_speed * -shield_lr;
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_DAMAGE,
        ENERGY_STOP_RESET_TYPE_GUARD_DAMAGE,
        directed_speed,
        0.0,
        0.0,
        0.0,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    let stop_frame = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_HIT_STOP_FRAME,
    );
    let stop_frame = stop_frame
        * WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x2434ca61df);
    WorkModule::set_int(
        fighter.module_accessor,
        stop_frame as i32,
        *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_PREV_SHIELD_SCALE_FRAME,
    );
    let hit_stop_mul =
        WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_initStatus)]
unsafe fn ftStatusUniqProcessGuardDamage_initStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuardDamage_initStatus_Inner(fighter);
    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD,
    ) {
        fighter_status_guard::set_just_shield_scale(fighter);
    } else {
        let shield_scale = if 0 < WorkModule::get_int(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_PREV_SHIELD_SCALE_FRAME,
        ) {
            let prev_shield = WorkModule::get_float(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_PREV_SHIELD,
            );
            fighter_status_guard::calc_shield_scale(fighter, prev_shield.into()).get_f32()
        } else {
            let shield = WorkModule::get_float(
                fighter.module_accessor,
                *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD,
            );
            fighter_status_guard::calc_shield_scale(fighter, shield.into()).get_f32()
        };
        ModelModule::set_joint_scale(
            fighter.module_accessor,
            Hash40::new("throw"),
            &Vector3f {
                x: shield_scale,
                y: shield_scale,
                z: shield_scale,
            },
        );
    }
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hooks!(
        sub_ftStatusUniqProcessGuardDamage_initStatus_Inner,
        ftStatusUniqProcessGuardDamage_initStatus
    );
}
