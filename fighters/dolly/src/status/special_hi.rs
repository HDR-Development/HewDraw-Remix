use super::*;

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_command_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI_COMMAND) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_command = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_DOLLY_STRENGTH_S, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);

    WorkModule::set_customize_no(fighter.module_accessor, 1, 2);

    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::set_int(fighter.module_accessor, situation, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);

    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_speed_x_mul"));
    let start_speed_x = sum_speed_x * start_speed_x_mul;

    if situation != *SITUATION_KIND_GROUND {
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let start_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_speed_y_mul"));
        let start_speed_y = sum_speed_y * start_speed_y_mul;

        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);

        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            0.0,
            0.0
        );

        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            start_speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            start_speed_y,
            0.0,
            0.0,
            0.0
        );
        let start_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_accel_y"));
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -start_accel_y
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);

        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            0.0,
            0.0
        );

        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_GROUND,
            start_speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }

    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

    let log = if is_command {
        let command_power_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("command_power_mul"));
        AttackModule::set_power_mul_status(fighter.module_accessor, command_power_mul);
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_06
    }
    else {
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_05
    };
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, log - 1);

    let mot = if is_command {
        if situation == *SITUATION_KIND_GROUND {
            Hash40::new("special_hi_command")
        }
        else {
            Hash40::new("special_air_hi_command")
        }
    }
    else {
        if situation == *SITUATION_KIND_GROUND {
            Hash40::new("special_hi")
        }
        else {
            Hash40::new("special_air_hi")
        }
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    ItemModule::set_change_status_event(fighter.module_accessor, false);

    if !StopModule::is_stop(fighter.module_accessor) {
        special_hi_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_hi_substatus as *const () as _));

    special_hi_set_kinetic(fighter, true.into());

    fighter.select_cliff_hangdata_from_name("special_hi");

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND);
    special_hi_main(fighter)
}

unsafe extern "C" fn special_hi_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR);
            let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
            if lr_stick_x < fighter.global_table[STICK_X].get_f32().abs() {
                PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }
        }
    }
    else {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_BUTTON_ON_TIMER);
    }
    0.into()
}

unsafe extern "C" fn special_hi_set_kinetic(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if situation != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if param_1.get_bool() {
            return;
        }

        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);

        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        );

        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

        let start_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_accel_y"));
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -start_accel_y
        );
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if param_1.get_bool() {
            return;
        }

        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);

        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP.into(), false.into());
        return 0.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_hi_set_kinetic(fighter, false.into());
    }

    0.into()
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP

pub unsafe extern "C" fn special_hi_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.select_cliff_hangdata_from_name("special_hi");
    0.into()
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_FALL

pub unsafe extern "C" fn special_hi_fall_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.select_cliff_hangdata_from_name("special_hi");
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, special_hi_command_pre);

    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, special_hi_command_main);

    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_init);
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_FALL, special_hi_fall_init);
}