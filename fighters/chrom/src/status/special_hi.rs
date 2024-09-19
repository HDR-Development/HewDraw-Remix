use super::*;

pub unsafe extern "C" fn special_hi_common_init(fighter: &mut L2CFighterCommon, status_kind: i32) {
    GroundModule::select_cliff_hangdata(fighter.module_accessor,*FIGHTER_CLIFF_HANG_DATA_DEFAULT as u32); //hmmm
    let mut param_accel_x_mul = 0.0;//hash40("air_accel_x_mul")
    let mut param_speed_x_max_mul = 0.0;//hash40("air_accel_x_add")
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        param_accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.rise_accel_x_mul");
        param_speed_x_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.rise_speed_x_max_mul");
    }
    else if fighter.is_status(statuses::chrom::SPECIAL_HI_FLIP) {
        param_accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.flip_accel_x_mul");
        param_speed_x_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.flip_speed_x_max_mul");
    }
    else {
        param_accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.dive_accel_x_mul");
        param_speed_x_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.dive_speed_x_max_mul");
    }
    let accel_x_mul = param_accel_x_mul;
    let speed_x_max_mul = param_speed_x_max_mul;
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    
    if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
        let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
            let lr = PostureModule::lr(fighter.module_accessor);
            let rise_speed_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.rise_speed_x_mul");
            speed_x = ControlModule::get_stick_x(fighter.module_accessor) * lr * rise_speed_x_mul;
        }
        else if fighter.is_status(statuses::chrom::SPECIAL_HI_DIVE) /*&& USE_MOTION_ANGLE*/ {
            let lr = PostureModule::lr(fighter.module_accessor);
            let control_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_WORK_FLOAT_STICK_CONTROL_ANGLE);
            speed_x = air_speed_x_stable.abs() * control_x * 0.75 * lr;
            speed_x = if (lr > 0.0) { speed_x.max(0.0) } else { speed_x.min(0.0) };
        }

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            speed_x,
            0.0, 0.0, 0.0, 0.0
        );
    }
    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul * accel_x_mul);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add * accel_x_mul);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * speed_x_max_mul, 0.0);
    WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MUL_FALL_X_ACCEL);
    WorkModule::set_float(fighter.module_accessor, speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_FALL_SPECIAL]) {
        return;
    }

    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let max_y_param = if (status_kind == statuses::chrom::SPECIAL_HI_DIVE) { hash40("air_speed_y_stable") } else { hash40("dive_speed_y") };
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, max_y_param, 0);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let motion_y: f32 = MotionModule::trans_move_speed(fighter.module_accessor).y;
    if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let flip_speed_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.flip_speed_y_mul");
        let dive_speed_y_start = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.dive_speed_y_start");
        let new_speed_y = if fighter.is_status(statuses::chrom::SPECIAL_HI_FLIP) { speed_y * flip_speed_y_mul } else { dive_speed_y_start };
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0, new_speed_y, 0.0, 0.0, 0.0
        );
    }
    let mut param_accel_y_mul = 0.0;
    let mut param_speed_y_max_mul = 0.0;
    if fighter.is_status(statuses::chrom::SPECIAL_HI_FLIP) {
        param_accel_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.flip_accel_y_mul");
        param_speed_y_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.flip_speed_y_max_mul");
    }
    else {
        param_accel_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.dive_accel_y_mul");
        param_speed_y_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.dive_speed_y_max_mul");
    }
    let accel_y_mul = param_accel_y_mul;
    let speed_y_max_mul = param_speed_y_max_mul;
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y * accel_y_mul);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable * speed_y_max_mul);
}

pub unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,//*FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    0.into()
}

pub unsafe extern "C" fn special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_ROY_INSTANCE_WORK_ID_INT_SPECIAL_HI_CLIFF_NUM);
    WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE_SET_ANGLE);  // USE_MOTION_ANGLE
    0.into()
}

pub unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_1"), L2CValue::Hash40s("special_air_hi_1"), false.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE.into(), FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE.into());
    fighter.sub_set_ground_correct_by_situation(false.into());
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    let jump_speed_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.jump_speed_mul");
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, jump_speed_mul);
    let landing_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.flip_landing_frame");
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

pub unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.set_situation(SITUATION_KIND_NONE.into());
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
        let stick_x = fighter.stick_x() * PostureModule::lr(fighter.module_accessor);
        let turn_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("status_start_turn_stick_x"));
        if (stick_x <= turn_stick && stick_x.abs() >= 0.2) {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_ROY_STATUS_SPECIAL_HI_WORK_FLOAT_STICK_CONTROL_ANGLE);
            PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE_CHROM) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_1"), L2CValue::Hash40s("special_air_hi_1"), true.into());
            fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_MOTION.into(), FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE.into());
            fighter.sub_set_ground_correct_by_situation(true.into());
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }        
        if MotionModule::is_end(fighter.module_accessor) {
            let new_status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL } else { FIGHTER_STATUS_KIND_FALL_SPECIAL };
            let accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_accel_x_mul");
            let speed_x_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_speed_x_max_mul");
            WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MUL_FALL_X_ACCEL);
            WorkModule::set_float(fighter.module_accessor, speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
            fighter.change_status(new_status.into(), false.into());
            return 1.into();
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(statuses::chrom::SPECIAL_HI_DIVE.into(), false.into());
            return 1.into();
        }
        //check for flip out
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_FREE_FALL_CHK) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_FREE_FALL_CHK);
            fighter.change_status(statuses::chrom::SPECIAL_HI_FLIP.into(), false.into());
            return 1.into();
        }
    }
    
    return 0.into();
}

pub unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE_CHROM) {
        let move_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut smash::app::KineticEnergy;
        let move_speed_y = lua_bind::KineticEnergy::get_speed_y(move_energy);
        let motion_y: f32 = MotionModule::trans_move_speed(fighter.module_accessor).y;

        //If rising via motion, or triggered via acmd...
        if motion_y > 0.0 || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE_CHROM);
            StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

            //set angle if param is true
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE_SET_ANGLE) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE_SET_ANGLE);
                let lr = PostureModule::lr(fighter.module_accessor);
                let stick_x = fighter.stick_x();
                WorkModule::set_float(fighter.module_accessor, stick_x * lr, *FIGHTER_ROY_STATUS_SPECIAL_HI_WORK_FLOAT_STICK_CONTROL_ANGLE);
                let rise_angle_base = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.rise_angle_base");
                let rise_angle_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.rise_angle_mul");
                let rise_angle_min = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.rise_angle_min");
                let rise_angle_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.rise_angle_max");
                let angle = (rise_angle_base + (rise_angle_mul * stick_x)) * -1.0;
                let angle = angle.clamp(rise_angle_min, rise_angle_max).to_radians();
                sv_kinetic_energy!(set_angle, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, angle);
            }
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
        }
    }
    //enable control flag
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTROL) && !true { // USE_MOTION_ANGLE
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTROL);
        special_hi_common_init(fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI);
    }

    0.into()
}

pub unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_EFFECT_AFTER_IMAGE_OFF, 0);
    sv_module_access::effect(fighter.lua_state_agent);
    0.into()
}

pub unsafe extern "C" fn special_hi_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !(&[
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3,
        statuses::chrom::SPECIAL_HI_FLIP,
        statuses::chrom::SPECIAL_HI_DIVE
    ]).contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
        EffectModule::remove_all_after_image(fighter.module_accessor, 0, 0);
    }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ROY_INSTANCE_WORK_ID_INT_SPECIAL_HI_CLIFF_NUM);
    0.into()
}

pub unsafe extern "C" fn special_hi_2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    0.into()
}

pub unsafe extern "C" fn special_hi_2_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_hi_common_init(fighter,statuses::chrom::SPECIAL_HI_FLIP);
    0.into()
}

pub unsafe extern "C" fn special_hi_2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_2"), 0.0, 1.0, false, 0.0, false, false);
    let landing_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.flip_landing_frame");
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_2_main_loop as *const () as _))
}

pub unsafe extern "C" fn special_hi_2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::chrom::status::SPECIAL_HI_DIVE_ENABLE)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_3_start"), 0.0, 1.0, false, 0.0, false, false);
        VarModule::off_flag(fighter.battle_object, vars::chrom::status::SPECIAL_HI_DIVE_ENABLE);
        VarModule::on_flag(fighter.battle_object, vars::chrom::status::SPECIAL_HI_DIVE_START);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if VarModule::is_flag(fighter.battle_object, vars::chrom::status::SPECIAL_HI_DIVE_START) {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            fighter.change_status(statuses::chrom::SPECIAL_HI_DIVE.into(), false.into());
            return 1.into()
        }
        else {
            let new_status = if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL} else {FIGHTER_STATUS_KIND_FALL_SPECIAL};
            let accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_accel_x_mul");
            let speed_x_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_speed_x_max_mul");
            WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MUL_FALL_X_ACCEL);
            WorkModule::set_float(fighter.module_accessor, speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
            fighter.change_status(new_status.into(), false.into());
            return 1.into()
        }
    }

    0.into()
}
pub unsafe extern "C" fn special_hi_2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor,  *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if speed_y < 0.0 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}
pub unsafe extern "C" fn special_hi_3_attack(fighter: &mut L2CFighterCommon, param2: &L2CValue, param3: &L2CValue) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn special_hi_3_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT);  
    special_hi_common_init(fighter, statuses::chrom::SPECIAL_HI_DIVE);
    0.into()
}

pub unsafe extern "C" fn special_hi_3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_3"), 0.0, 1.0, false, 0.0, false, false);
    let landing_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.dive_landing_frame");
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_3_main_loop as *const () as _))
}

pub unsafe extern "C" fn special_hi_3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    let is_spinning = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT);
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            // Don't transition to Hi4 if Chrom used his spin final hit
            let dive_hi4_max_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.dive_hi4_max_frame");
            let new_status = if is_spinning || MotionModule::frame(fighter.module_accessor) < dive_hi4_max_frame 
            { FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4 } else { FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL };
            fighter.change_status(new_status.into(), false.into());
            return 1.into();
        }
    }

    if is_spinning {
        let frame = fighter.status_frame() as f32;
        let motion_frame = MotionModule::frame(fighter.module_accessor);
        let end_frame = MotionModule::end_frame(fighter.module_accessor);
        let manual_transition = !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && frame >= ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.dive_transition_frame_min")
        && motion_frame >= end_frame - 1.0;
        let auto_transition = frame >= ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.dive_transition_frame_force")
        && motion_frame >= end_frame - 2.0;

        if manual_transition || auto_transition {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT); 
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_2"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            let new_status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL } else { FIGHTER_STATUS_KIND_FALL_SPECIAL };
            let accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_accel_x_mul");
            let speed_x_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_speed_x_max_mul");
            WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MUL_FALL_X_ACCEL);
            WorkModule::set_float(fighter.module_accessor, speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
            fighter.change_status(new_status.into(), false.into());
            return 1.into();
        }
    }

    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exit);

    agent.status(Pre, statuses::chrom::SPECIAL_HI_FLIP, special_hi_2_pre);
    agent.status(Init, statuses::chrom::SPECIAL_HI_FLIP, special_hi_2_init);
    agent.status(Main, statuses::chrom::SPECIAL_HI_FLIP, special_hi_2_main);
    agent.status(Exec, statuses::chrom::SPECIAL_HI_FLIP, special_hi_2_exec);
    agent.status(Exit, statuses::chrom::SPECIAL_HI_FLIP, special_hi_exit);

    agent.status(Pre, statuses::chrom::SPECIAL_HI_DIVE, special_hi_2_pre);
    agent.status(Init, statuses::chrom::SPECIAL_HI_DIVE, special_hi_3_init);
    agent.status(Main, statuses::chrom::SPECIAL_HI_DIVE, special_hi_3_main);
    agent.status(CheckAttack, statuses::chrom::SPECIAL_HI_DIVE, special_hi_3_attack);
    agent.status(Exit, statuses::chrom::SPECIAL_HI_DIVE, special_hi_exit);
}