use super::*;
use globals::*;

use vars::wolf::status::*;
use consts::statuses::wolf::*;


unsafe extern "C" fn special_s_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    0.into()
}


unsafe extern "C" fn special_s_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int(-1, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    fighter.set_int(0, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_COUNT);

    fighter.set_int_from_param(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_FRAME, "param_special_s", "revert_angle_frame");
    fighter.set_float_from_param(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MAX_RUSH_DEGREE, "param_special_s", "max_rush_degree");
    fighter.set_float_from_param(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MIN_STICK, "param_special_s", "min_stick_y");

    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor, "special_s_start".to_hash(), 0.0, 1.0, false, 0.0, false, false);
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor, "special_air_s_start".to_hash(), 0.0, 1.0, false, 0.0, false, false);
    }

    let speed_mul = fighter.get_param_float("param_special_s", "illusion_rush_speed_mul");
    let speed_mul_power_up = fighter.get_param_float("param_special_s", "illusion_rush_speed_mul_power_up");
    fighter.set_float(speed_mul * speed_mul_power_up, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s_start_sub_status as *const () as _));
    fighter.main_shift(special_s_start_main_loop)
}

unsafe extern "C" fn special_s_start_sub_status(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if arg.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }
    0.into()
}

unsafe extern "C" fn special_s_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 0.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor) && (MotionModule::is_end(fighter.module_accessor) || StatusModule::is_situation_changed(fighter.module_accessor)) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_to_custom_status(SPECIAL_S_RUSH, false, false);
            return 0.into();
        } else if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.is_situation(*SITUATION_KIND_AIR) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            } else {
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
            }
            fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_start"), L2CValue::Hash40s("special_air_s_start"), true.into());
        }
    }

    let max_deg = fighter.get_float(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MAX_RUSH_DEGREE);
    let min_stick = fighter.get_float(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MIN_STICK);
    let stick_y = fighter.stick_y();

    let angle = stick_y.signum() * max_deg * f32::max(stick_y.abs() - min_stick, 0.0) / (1.0 - min_stick);
    fighter.set_float(angle, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_RUSH_DEGREE);

    0.into()
}


unsafe extern "C" fn special_s_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}


unsafe extern "C" fn special_s_start_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int_from_param(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME, "param_special_s", "illusion_stop_y_frame");
    let total_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let x_mul = fighter.get_param_float("param_special_s", "illusion_start_x_mul");
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 6, total_x * x_mul, 0.0, 0.0, 0.0, 0.0);
        let illusion_accel_x = fighter.get_param_float("param_special_s", "illusion_accel_x");
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_accel_x, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if fighter.get_int(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME) != 0 {
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    } else {
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, total_x * x_mul, 0.0);
    }
    
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    0.into()
}


unsafe extern "C" fn special_s_start_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_situation(*SITUATION_KIND_AIR) {
        return 0.into();
    }

    if fighter.get_int(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME) == 0 && fighter.get_param_int("param_special_s", "illusion_stop_y_frame") != 0 {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let accel = fighter.get_param_float("param_special_s", "illusion_accel_y");
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel);
    }

    0.into()
}

unsafe extern "C" fn special_s_rush_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_s_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        MotionModule::change_motion(fighter.module_accessor, "special_air_s".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_S as u32);
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS.into());
    } else {
        MotionModule::change_motion(fighter.module_accessor, "special_s".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        let speed_mul = fighter.get_float(*FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
    }
    fighter.main_shift(special_s_rush_main_loop)
}

unsafe extern "C" fn special_s_rush_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 0.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_to_custom_status(SPECIAL_S_END, false, false);
            return 0.into();
        } else if StatusModule::is_situation_changed(fighter.module_accessor) {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), true.into());

            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                let speed_mul = fighter.get_float(*FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
                sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
            } else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_S as u32);
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS.into());
            }
        }

        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_to_custom_status(SPECIAL_S_END, false, false);
        }
    }

    let angle = fighter.get_float(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_RUSH_DEGREE);

    PostureModule::set_rot(
        fighter.module_accessor,
        &Vector3f::new(-angle, 0.0, 0.0),
        0
    );

    0.into()
}

unsafe extern "C" fn special_s_rush_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_rot(fighter.module_accessor, &Vector3f::zero(), 0);
    0.into()
}

unsafe extern "C" fn special_s_rush_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    0.into()
}

unsafe extern "C" fn special_s_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK // This bit allows us to keep the weak hitbox spawned in ACMD so that it can interpolate behind wolf and not leave a blindspot in the move
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        // KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(fighter.module_accessor, "special_air_s_end".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
    } else {
        // KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, "special_s_end".to_hash(), 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s_end_sub_status as *const () as _));
    fighter.main_shift(special_s_end_main_loop)
}

unsafe extern "C" fn special_s_end_sub_status(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if arg.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_COUNT);
    }
    0.into()
}

unsafe extern "C" fn special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && VarModule::get_int(fighter.object(), vars::common::instance::LAST_ATTACK_HITBOX_ID) == 0 {
        VarModule::on_flag(fighter.object(), SPECIAL_S_RESERVE_FALL);
    }

    if fighter.motion_frame() >= 20.0 && VarModule::is_flag(fighter.object(), SPECIAL_S_RESERVE_FALL) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 0.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor) {
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            } else if VarModule::is_flag(fighter.object(), SPECIAL_S_RESERVE_FALL) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            }
            return 0.into();
        } else if StatusModule::is_situation_changed(fighter.module_accessor) {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_end"), L2CValue::Hash40s("special_air_s_end"), true.into());
            fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                fighter.set_float(0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 0.into();
            } else {
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
            }
        }
    }

    let revert_count = fighter.get_int(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_COUNT);
    let revert_frame = fighter.get_int(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_FRAME);
    let mut angle = 0.0;
    if revert_count < revert_frame && 0 < revert_frame {
        let degree = fighter.get_float(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_RUSH_DEGREE);
        angle = degree - (degree * revert_count as f32 / revert_frame as f32);
    }
    PostureModule::set_rot(fighter.module_accessor, &Vector3f::new(-angle, 0.0, 0.0), 0);
    0.into()
}

unsafe extern "C" fn special_s_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_rot(fighter.module_accessor, &Vector3f::zero(), 0);
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.on_flag(*FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_ILLUSION_LANDING);
    } else {
        fighter.off_flag(*FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_ILLUSION_LANDING);
        fighter.off_flag(*FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING);
    }
    0.into()
}

unsafe extern "C" fn special_s_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int_from_param(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME, "param_special_s", "illusion_end_air_stop_y_frame");
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        let brake = fighter.get_param_float("param_special_s", "illusion_end_brake_x");
        let end_speed = fighter.get_param_float("param_special_s", "illusion_end_speed_x");
        let lr = PostureModule::lr(fighter.module_accessor);

        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, brake, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, end_speed * lr, 0.0);
    } else {
        let brake = fighter.get_param_float("param_special_s", "illusion_end_air_brake_x");
        let end_speed = fighter.get_param_float("param_special_s", "illusion_end_air_speed_x");
        let lr = PostureModule::lr(fighter.module_accessor);

        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, brake, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, end_speed * lr, 0.0);
    }

    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 10.0, 10.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    let speed = fighter.get_param_float("param_special_s", "illusion_end_c3_speed_y");
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 10.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    0.into()
}

unsafe extern "C" fn special_s_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_situation(*SITUATION_KIND_AIR) {
        return 0.into();
    }

    let brake = fighter.get_param_float("param_special_s", "illusion_end_air_brake_x");
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    if brake != sv_kinetic_energy::get_brake_x(fighter.lua_state_agent) {
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            brake,
            0.0
        );
    }

    if fighter.get_int(*FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME) == 0 {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let accel = fighter.get_param_float("param_special_s", "illusion_end_air_accel_y");
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    0.into()
}



extern "C" fn wolf_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() != *FIGHTER_KIND_WOLF {
            return;
        }

        let status_rush = CustomStatusModule::get_agent_status_kind(fighter.object(), SPECIAL_S_RUSH);
        let instruction = 0x7100001Fu32 | ((status_rush as u32 & 0xFFF) << 10);
        skyline::patching::Patch::in_text(0x12c29a0).data(instruction);
    }
}

pub fn install() {
    smashline::Agent::new("wolf")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_start_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_start_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_start_end)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_start_init)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_start_exec)
        .status(Pre, SPECIAL_S_RUSH, special_s_rush_pre)
        .status(Main, SPECIAL_S_RUSH, special_s_rush_main)
        .status(End, SPECIAL_S_RUSH, special_s_rush_end)
        .status(Init, SPECIAL_S_RUSH, special_s_rush_init)
        .status(Pre, SPECIAL_S_END, special_s_end_pre)
        .status(Main, SPECIAL_S_END, special_s_end_main)
        .status(End, SPECIAL_S_END, special_s_end_end)
        .status(Init, SPECIAL_S_END, special_s_end_init)
        .status(Init, SPECIAL_S_END, special_s_end_exec)
        .on_init(wolf_init)
        .install();
}
