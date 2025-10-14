use super::*;

unsafe extern "C" fn special_hi_finish2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_hi_finish2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] change motion and enable energy so that we can drift
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_end2"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    
    // [v] reset the control energy after we've enabled it to ensure that we don't have any left over drift impacting us
    let lr = fighter.lr();
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
        -0.5 * lr,
        0.0,
        0.0,
        0.0,
        0.0
    );

    // [v] set the stable speed to 0.0 (gets changed when FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL is true)
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        0.0,
        0.0
    );

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        2.0
    );

    // let accel = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    // sv_kinetic_energy!(
    //     set_accel,
    //     fighter,
    //     FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
    //     -accel * 0.6
    // );

    fighter.main_shift(special_hi_finish2_main_loop)
}

unsafe extern "C" fn special_hi_finish2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] check if we have grabbed a ledge
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    // [hdr]
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    // [v] check if we have passed the landing frame and are on the ground, if so transition to special fall
    //      which will transition to the appropriate landing lag
    if fighter.global_table[globals::CURRENT_FRAME].get_i32() > fighter.get_param_int("param_special_hi", "can_landing_frame")
        && fighter.is_situation(*SITUATION_KIND_GROUND)
    {
        let interrupt_frame = fighter.get_param_int("param_special_hi", "interrupt_landing_fix_frame");
        fighter.set_float(interrupt_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 0.into();
    }

    // [v] set our kinetic energy infos only once
    if fighter.is_flag(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL) {
        fighter.off_flag(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);

        let accel_mul = fighter.get_param_float("param_special_hi", "finish_accel_x_mul");
        let accel_max = fighter.get_param_float("param_special_hi", "finish_speed_x_max");

        sv_kinetic_energy!(
            set_accel_x_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            accel_mul
        );

        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            accel_max
        );
    }

    // [v] check if you are fastfalling and if so zero out your motion's y movement and enable gravity
    //      to allow fast fall kinetic to take over
    if fighter.is_flag(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_CHECK_DIVE) {
        fighter.sub_air_check_dive();
    }

    if !MotionModule::is_end(fighter.module_accessor) {
        return 0.into();
    }

    // [v] when the animation is over, set transition into special fall
    let fall_special_accel = fighter.get_param_float("param_special_hi", "fall_special_accel_x_mul");
    fighter.set_float(fall_special_accel, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_ACCEL_MUL);

    let fall_special_speed_max = fighter.get_param_float("param_special_hi", "fall_special_speed_x_max_mul");
    fighter.set_float(fall_special_speed_max, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);

    // [v] the landing fix frame is different depending on whether you used spreadbullet or not
    let landing_fix_frame;
    if fighter.is_flag(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_SPREADBULLET) {
        landing_fix_frame = fighter.get_param_int("param_special_hi", "attack2_landing_fix_frame");
    } else {
        landing_fix_frame = fighter.get_param_int("param_special_hi", "attack1_landing_fix_frame");
    }

    fighter.set_float(landing_fix_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::elight::SPECIAL_HI_FINISH2, special_hi_finish2_pre);
    agent.status(Main, statuses::elight::SPECIAL_HI_FINISH2, special_hi_finish2_main);
}