use super::*;

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_MAX_HOLD_COUNT);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        special_n_hold_set_physics_2(fighter);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("pacman_special_n_hold"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        special_n_hold_set_physics_1(fighter);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("pacman_special_air_n_hold"), 0.0, 1.0, false, 0.0, false, false);
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x240e24407a));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    
    if !StopModule::is_stop(fighter.module_accessor) { special_n_hold_substatus(fighter, false.into()); }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_n_hold_substatus as *const () as _));
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.main_shift(special_n_hold_main_loop)
}

pub unsafe extern "C" fn special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            special_n_hold_set_physics_2(fighter);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("pacman_special_n_hold"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            special_n_hold_set_physics_1(fighter);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("pacman_special_air_n_hold"), -1.0, 1.0, 0.0, false, false);
        }
    }

    let mut charge_up_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("charge_up_frame"));
    let unk = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), 0x1b9db69840);
    let charge_rank = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
    if unk <= charge_rank {
        charge_up_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("changed_charge_up_frame"));
    }
    let charge_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_CHARGE_FRAME);
    if charge_up_frame <= charge_frame {
        if charge_rank < *PACMAN_SPECIAL_N_RANK_MAX {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
            if *PACMAN_SPECIAL_N_RANK_MAX <= charge_rank {
                WorkModule::set_int(fighter.module_accessor, *PACMAN_SPECIAL_N_RANK_MAX, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_HAVE_ITEM);
            }
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_CHARGE_FRAME);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x240e24407a));
        }
    }
    if fighter.is_button_trigger(Buttons::Special) {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_PACMAN_SPECIAL_N_SHOOT.into(), false.into());
        return 1.into();
    }
    if !fighter.is_pad_flag(PadFlag::SpecialTrigger) {
        if !fighter.is_pad_flag(PadFlag::AttackTrigger) {
            if *PACMAN_SPECIAL_N_RANK_MAX <= charge_rank {
                let max_hold_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_MAX_HOLD_COUNT);
                if max_hold_count == 1 {
                    effect!(fighter, MA_MSC_EFFECT_REQUEST_FOLLOW, Hash40::new("pacman_fruit_max"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true, EFFECT_SUB_ATTRIBUTE_NONE, 0, 0);
                    app::FighterUtil::flash_eye_info(fighter.module_accessor);
                }
                let max_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
                if max_charge_frame <= max_hold_count {
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x22baf2b632));
                    fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_PACMAN_SPECIAL_N_CANCEL.into(), false.into());
                    return 1.into();
                }
            }
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                if fighter.is_pad_flag(PadFlag::GuardTrigger) {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_NEXT_STATUS);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW);
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x22baf2b632));
                    fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_PACMAN_SPECIAL_N_CANCEL.into(), false.into());
                    return 1.into();
                }
                if fighter.sub_check_jump_in_charging().get_bool() {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_NEXT_STATUS);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW);
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x22baf2b632));
                    fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_PACMAN_SPECIAL_N_CANCEL.into(), true.into());
                    return 1.into();
                }
                // roll transitions removed
            }
            else {
                if fighter.is_pad_flag(PadFlag::GuardTrigger) {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_NEXT_STATUS);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW);
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x22baf2b632));
                    fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_PACMAN_SPECIAL_N_CANCEL.into(), false.into());
                    return 1.into();
                }
                if fighter.sub_check_jump_in_charging().get_bool() {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_NEXT_STATUS);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW);
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x22baf2b632));
                    fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_PACMAN_SPECIAL_N_JUMP_CANCEL.into(), true.into());
                    return 1.into();
                }
                // airdodge transition removed
            }
        }
    }

    return 0.into();
}

//FUN_710002d300
pub unsafe extern "C" fn special_n_hold_set_physics_1(fighter: &mut L2CFighterCommon) {
    let mut sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("speed_y_max"));
    if speed_y_max < sum_speed_y {
        sum_speed_y = speed_y_max;
    }
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_accel_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_accel_y_mul"));
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let air_max_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_max_speed_y_mul"));

    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, sum_speed_y);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -(air_accel_y * air_accel_y_mul));
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable * air_max_speed_y_mul);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    special_n_hold_set_physics_2(fighter);
}

//FUN_710002df20
pub unsafe extern "C" fn special_n_hold_set_physics_2(fighter: &mut L2CFighterCommon) {
    let mut sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x * 0.7, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x * 0.7, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
}

//FUN_710002ef60
pub unsafe extern "C" fn special_n_hold_substatus(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if param_2.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_CHARGE_FRAME);
        let charge_rank = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
        if *PACMAN_SPECIAL_N_RANK_MAX <= charge_rank {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_MAX_HOLD_COUNT);
        }
    }

    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_KIRBY_STATUS_KIND_PACMAN_SPECIAL_N, special_n_pre);
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_PACMAN_SPECIAL_N_HOLD, special_n_hold_main);
}