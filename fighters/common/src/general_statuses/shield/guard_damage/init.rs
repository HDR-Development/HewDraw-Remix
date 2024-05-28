// status imports
use super::*;
use globals::*;

use super::super::fighter_status_guard;

unsafe fn lucario_guard_damage_init(fighter: &mut L2CFighterCommon) {
    if fighter.kind() != *FIGHTER_KIND_LUCARIO {
        return;
    }
    if fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        let mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "aura.parry_meter_gain_mul");
        MeterModule::add(fighter.object(),mul * fighter.get_float(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_POWER));
        VarModule::set_int(fighter.battle_object,vars::lucario::instance::METER_PAUSE_REGEN_FRAME,0);
    } else {
        let mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "aura.shield_damage_meter_drain_mul");
        MeterModule::drain_direct(fighter.object(),mul * fighter.get_float(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_POWER));
        let frames = (90).max(VarModule::get_int(fighter.object(), vars::lucario::instance::METER_PAUSE_REGEN_FRAME));
        VarModule::set_int(fighter.object(),vars::lucario::instance::METER_PAUSE_REGEN_FRAME,frames);
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_initStatus_Inner)]
unsafe fn sub_ftStatusUniqProcessGuardDamage_initStatus_Inner(fighter: &mut L2CFighterCommon) {
    lucario_guard_damage_init(fighter);
    let boma = fighter.module_accessor;
    let shield_power = fighter.get_float(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_POWER);
    let setoff_mul = fighter.get_float(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_SETOFF_MUL);
    let param_setoff_mul = fighter.get_param_float("common", "shield_setoff_mul");

    // let analog = InputModule::get_analog_for_guard(fighter.battle_object);
    // let param_setoff_mul = if analog > 0.0 && analog < 1.0 {
    //     ((1.0 - analog) * 0.65 + param_setoff_mul / 1.5) * 1.5
    // } else {
    //     param_setoff_mul
    // };

    let mut shield_power = shield_power * setoff_mul * param_setoff_mul;
    if fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        shield_power *= fighter.get_param_float( "common", "just_shield_setoff_mul");
    }
    shield_power += fighter.get_param_float("common", "shield_setoff_add");
    let max = fighter.get_param_float("common", "shield_stiff_frame_max");
    if max < shield_power {
        shield_power = max;
    }
    fighter.set_int(shield_power as i32, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME);

    if fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        if fighter.get_param_int("param_motion", "just_shield_motion") != 0 {
            MotionModule::change_motion(boma, Hash40::new("just_shield_off"), 0.0, 1.0, false, 0.0, false, false);
        } else {
            let end_frame = MotionModule::end_frame_from_hash(boma, Hash40::new("just_shield_off"));
            MotionModule::change_motion(boma, Hash40::new("just_shield_off"), end_frame, 1.0, false, 0.0, false, false);
        }

        MotionAnimcmdModule::call_script_single(boma, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(0x1a29f56bfb), -1);
        EffectModule::kill_kind(boma, Hash40::new("sys_genesis_end"), true, true);
        SoundModule::stop_se(boma, Hash40::new("se_item_backshield_guard01"), 0);

        WorkModule::inc_int(boma, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_SHEILD_COUNT);
        if fighter_status_guard::is_continue_just_shield_count(fighter).get_bool() {
            ShieldModule::set_status(boma, *FIGHTER_SHIELD_KIND_GUARD, app::ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
            ShieldModule::set_shield_type(boma, app::ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
            ReflectorModule::set_status(boma, 0, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        } else {
            CancelModule::enable_cancel(boma);
            fighter.on_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_DISABLE_HIT_STOP_DELAY_STICK);
        }
    } else {
        let catch_frame = fighter.get_param_int("common", "shield_setoff_catch_frame");
        if catch_frame > 0 {
            fighter.set_int(catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME);
        }

        let motion_rate = app::sv_fighter_util::get_guard_damage_motion_rate(fighter.lua_state_agent, Hash40::new("guard_damage"));
        let weight = MotionModule::weight(boma);
        MotionModule::change_motion(boma, Hash40::new("guard_damage"), 0.0, motion_rate, false, 0.0, false, false);
        if !fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
            MotionModule::add_motion_2nd(boma, Hash40::new("guard"), 0.0, 1.0, false, 1.0);
            MotionModule::set_rate_2nd(boma, 0.0);
            MotionModule::set_weight(boma, weight, true);
            let prev_x = fighter.get_float(*FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_X);
            let prev_y = fighter.get_float( *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_Y);
            fighter_status_guard::set_guard_blend_motion_angle(fighter, prev_x.into(), prev_y.into());
        }
        
        ShieldModule::set_status(boma, *FIGHTER_SHIELD_KIND_GUARD, app::ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
        ControlModule::clear_command(boma, false);
    } 

    let setoff_speed_mul = fighter.get_param_float("common","shield_setoff_speed_mul");

    // let analog = InputModule::get_analog_for_guard(fighter.battle_object);
    // let setoff_speed_mul = if analog > 0.0 && analog < 1.0 {
    //     0.195 * (1.0 - analog) + setoff_speed_mul
    // } else {
    //     setoff_speed_mul
    // };

    let mut setoff_speed = shield_power * setoff_speed_mul;

    if fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        setoff_speed *= fighter.get_param_float( "common", "just_shield_speed_rate");
    }

    setoff_speed = setoff_speed.min(fighter.get_param_float("common", "shield_setoff_speed_max"));
    let shield_lr = fighter.get_float(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR);
    let directed_speed = setoff_speed * -shield_lr;
    if !fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE, ENERGY_STOP_RESET_TYPE_GUARD_DAMAGE, directed_speed, 0.0, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    }
    let stop_frame = fighter.get_float(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_HIT_STOP_FRAME);
    let stop_frame = stop_frame * WorkModule::get_param_float(boma, hash40("common"), 0x2434ca61df);
    fighter.set_int(stop_frame as i32, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_PREV_SHIELD_SCALE_FRAME);
    let hit_stop_mul = if fighter.is_flag(*FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) { 1.0 } else { WorkModule::get_param_float(boma, hash40("common"), 0x20d241cd64) };
    ShieldModule::set_hit_stop_mul(boma, hit_stop_mul);
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_initStatus)]
unsafe fn ftStatusUniqProcessGuardDamage_initStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuardDamage_initStatus_Inner(fighter);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        fighter_status_guard::set_just_shield_scale(fighter);
        return 0.into();
    } 
    let shield_scale = if fighter.get_int(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_PREV_SHIELD_SCALE_FRAME) > 0 {
        let prev_shield = fighter.get_float(*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_PREV_SHIELD);
        fighter_status_guard::calc_shield_scale(fighter, prev_shield.into()).get_f32()
    } else {
        let shield = fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        fighter_status_guard::calc_shield_scale(fighter, shield.into()).get_f32()
    };
    ModelModule::set_joint_scale( fighter.module_accessor, Hash40::new("throw"), &(Vector3f {x: shield_scale, y: shield_scale, z: shield_scale}));
    return 0.into();
}

pub fn install() {
    skyline::install_hooks!(
        sub_ftStatusUniqProcessGuardDamage_initStatus_Inner,
        ftStatusUniqProcessGuardDamage_initStatus
    );
}
