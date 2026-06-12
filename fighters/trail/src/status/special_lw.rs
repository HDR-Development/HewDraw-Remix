use super::*;

const FIGHTER_TRAIL_STATUS_SPECIAL_AIR_LW_FLAG_FALLING: i32 = 0x21000014;

const FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_IS_JUMPING: i32 = 0x21000016;
const FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_IS_CONTROL_ENABLED: i32 = 0x21000018;

const JUMP_MUL: f32 = 1.0;
const JUMP_ANGLE: f32 = 0.0;
const JUMP_STICK_BASE: f32 = 0.75;
const JUMP_STICK_MUL: f32 = 0.75;
const JUMP_CONTROL_ACCEL_MUL: f32 =  0.25;
const JUMP_CONTROL_MAX_MUL: f32 = 1.0;
const JUMP_FALL_GRAVITY_MUL: f32 =  0.5;
const JUMP_FALL_STABLE_MUL: f32 = 1.0;

const AIR_FALL_SPEED_X: f32 = 0.0;
const AIR_FALL_SPEED_Y: f32 = -3.5;
//const AIR_LANDING_LAG: f32 = 27.0;

pub unsafe extern "C" fn speciallw_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        return speciallw_air_main(fighter);
    }
    return speciallw_ground_main(fighter);
}

pub unsafe extern "C" fn speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN as u32), //important b-reverse part
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW) as u32,
        0
    );
    0.into()
}
/*
GROUND
*/
pub unsafe extern "C" fn speciallw_ground_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_IS_JUMPING);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(
        set_speed_mul_2nd,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        1.0,
        JUMP_MUL
    );
    sv_kinetic_energy!(
        set_angle,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        (-JUMP_ANGLE).to_radians()
    );

    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);

	fighter.sub_shift_status_main(L2CValue::Ptr( speciallw_ground_main_loop as *const () as _))
}

unsafe extern "C" fn speciallw_handle_angle(fighter: &mut L2CFighterCommon){
    let stick_min = 0.25;
    let mut stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut is_reverse = stick_x.signum() != lr;
    if stick_x.abs() <= 0.25 {
        stick_x = 0.0;
        is_reverse = false;
    }
    let mut angle = JUMP_STICK_MUL*stick_x;
    angle += JUMP_STICK_BASE*lr;
    
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        angle
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0
    );
}

unsafe extern "C" fn speciallw_start_control(fighter: &mut L2CFighterCommon) {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_IS_CONTROL_ENABLED);
    StatusModule::set_keep_situation_air(fighter.module_accessor, false);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.01
    );
    sv_kinetic_energy!(
        set_angle,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.0
    );
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
        speed_x,
        0.0,
        0.0,
        0.0,
        0.0
    );
    
    //I hate that I gotta do all this...
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        air_accel_x_mul * JUMP_CONTROL_ACCEL_MUL
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        air_accel_x_add * JUMP_CONTROL_ACCEL_MUL
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * JUMP_CONTROL_MAX_MUL,
        0.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * JUMP_CONTROL_MAX_MUL,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
}

unsafe extern "C" fn speciallw_start_gravity(fighter: &mut L2CFighterCommon) {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let fall_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -fall_speed_y*JUMP_FALL_GRAVITY_MUL
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_speed_y_stable*JUMP_FALL_STABLE_MUL
    );
}

unsafe extern "C" fn speciallw_ground_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_y: f32 = MotionModule::trans_move_speed(fighter.module_accessor).y();
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let sit = StatusModule::situation_kind(fighter.module_accessor);

    //Check for jump
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_IS_JUMPING) {
        if motion_y > 0.0 {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_IS_JUMPING);
            let lr = PostureModule::lr(fighter.module_accessor);
            /* 
            sv_kinetic_energy!(
                set_angle,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_MOTION,
                (-30.0f32).to_radians()*lr
            );
            */

            let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
            WorkModule::set_int(fighter.module_accessor, jump_count_max, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);

            fighter.set_situation(SITUATION_KIND_AIR.into());
            StatusModule::set_keep_situation_air(fighter.module_accessor, true);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        return 0.into();
    }
    //Jumping loop...

    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_IS_CONTROL_ENABLED) {
        //speciallw_handle_angle(fighter);
        if VarModule::is_flag(fighter.battle_object,vars::trail::status::SPECIAL_LW_ENABLE_CONTROL) {
            speciallw_start_control(fighter);
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::trail::status::SPECIAL_LW_ENABLE_GRAVITY) {
        if VarModule::is_flag(fighter.battle_object,vars::trail::status::SPECIAL_LW_ENABLE_CONTROL) {
            VarModule::off_flag(fighter.battle_object, vars::trail::status::SPECIAL_LW_ENABLE_GRAVITY);
            speciallw_start_gravity(fighter);
        }
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor,
            false,
            *FIGHTER_TREADED_KIND_NO_REAC,
            false,
            false,
            false,
            (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            0,
            (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW) as u32,
            0
        );
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            // if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
            //     KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(1.0, 0.8, 1.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            // }
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        //Maybe add landing lag here?
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_LANDING.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

/*
AIR
*/
pub unsafe extern "C" fn speciallw_air_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    }

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::clear_speed_all(fighter.module_accessor);
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_FREE,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
	fighter.sub_shift_status_main(L2CValue::Ptr( speciallw_air_main_loop as *const () as _))
}

unsafe extern "C" fn speciallw_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if VarModule::is_flag(fighter.battle_object,vars::trail::status::SPECIAL_LW_AIR_FALL) {
        if !KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

        let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::FighterKineticEnergyGravity;
        smash::app::lua_bind::FighterKineticEnergyGravity::set_speed(gravity_energy, AIR_FALL_SPEED_Y);
        smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(gravity_energy, 0.0);
        
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_TRAIL_STATUS_SPECIAL_AIR_LW_FLAG_FALLING);
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) 
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_TRAIL_STATUS_SPECIAL_AIR_LW_FLAG_FALLING) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) || 
    (WorkModule::is_flag(fighter.module_accessor, FIGHTER_TRAIL_STATUS_SPECIAL_AIR_LW_FLAG_FALLING) 
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND)
    {
        //Maybe add landing lag here? Or maybe go into a different status?
        fighter.change_status_by_situation(statuses::trail::SPECIAL_LW_LANDING.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn speciallw_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
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
        0,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}
pub unsafe extern "C" fn speciallw_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
    
    //calculate landing frame?
    //let motion = hash40();
    //let landing_rate = fighter.sub_get_landing_motion_rate(motion.into(),AIR_LANDING_LAG.into()).get_f32();

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_rebound"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(speciallw_landing_main_loop as *const () as _))
}
pub unsafe extern "C" fn speciallw_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) &&
    StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    0.into()
}
pub unsafe extern "C" fn speciallw_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
	agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, speciallw_main);
    
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, speciallw_pre);
    agent.status(Pre, statuses::trail::SPECIAL_LW_LANDING, speciallw_landing_pre);
    agent.status(Main, statuses::trail::SPECIAL_LW_LANDING, speciallw_landing_main);
    agent.status(End, statuses::trail::SPECIAL_LW_LANDING, speciallw_landing_end);
}