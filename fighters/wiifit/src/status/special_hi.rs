use super::*;


// FIGHTER_STATUS_KIND_SPECIAL_HI

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

// FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_JUMP

unsafe extern "C" fn special_hi_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_jump"), 0.0, 1.0, false, 0.0, false, false);
    fighter.set_float(0.0, *FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_RESTART_BACKUP_SPEED_Y);
    fighter.set_float(0.0, *FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_RESTART_FRAME);
    fighter.on_flag(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_FLAG_JUMP_RESTART);
    fighter.on_flag(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_FLAG_JUMP_RESTART_NOW);
    fighter.set_float(1.0, *FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_MOTION_RATE_NOW);
    fighter.set_float(0.0, *FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_SE_PITCH);

    fighter.shift(L2CValue::Ptr(special_hi_jump_main_loop as *const () as _));
    return 0.into();
}

unsafe extern "C" fn special_hi_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    let hulahoop_jump_frame = fighter.get_param_int("param_special_hi", "hulahoop_jump_frame");
    if fighter.status_frame() >= hulahoop_jump_frame {
        fighter.change_status(FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 1.into();
    }
    let mut jump_flag = true;
    if fighter.is_flag(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_FLAG_JUMP_RESTART) {
        let restart_frame = fighter.get_float(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_RESTART_FRAME);
        let prohibition_frame = fighter.get_param_int("param_special_hi", "hulahoop_jump_restart_prohibition_frame");
        if fighter.status_frame() as f32 - restart_frame <= prohibition_frame as f32 {
            jump_flag = false;
        }
    }
    if jump_flag {
        let restart_start_frame = fighter.get_param_int("param_special_hi", "hulahoop_jump_restart_start_frame");
        if fighter.status_frame() >= restart_start_frame {
            let restart_end_frame = fighter.get_param_int("param_special_hi", "hulahoop_jump_restart_end_frame");
            if fighter.status_frame() <= restart_end_frame {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    let add_speed_y = fighter.get_param_float("param_special_hi", "hulahoop_jump_restart_add_speed_y");
                    let mut speed_y = sum_speed_y + add_speed_y;
                    let max_speed_y = fighter.get_param_float("param_special_hi", "hulahoop_jump_max_speed_y");
                    if speed_y > max_speed_y {
                        speed_y = max_speed_y;
                    }
                    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
                    fighter.set_float(speed_y, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_JUMP_SPEED_Y);
                    //println!("setting speed_y to {}", speed_y);
                    let motion_rate_now = fighter.get_float(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_MOTION_RATE_NOW);
                    let add_motion_rate = fighter.get_param_float("param_special_hi", "hulahoop_jump_restart_add_motion_rate");
                    let motion_rate = motion_rate_now + (add_motion_rate / 100.0);
                    MotionModule::set_rate(fighter.module_accessor, motion_rate);
                    fighter.set_float(motion_rate, *FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_MOTION_RATE_NOW);
                    let se_pitch = fighter.get_float(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_SE_PITCH);
                    SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_wiifit_special_h01"), 0);
                    SoundModule::play_status_se(fighter.module_accessor, Hash40::new("se_wiifit_special_h01"), false, false, false);
                    SoundModule::set_se_pitch_status(fighter.module_accessor, se_pitch);
                    fighter.set_float(se_pitch + 300.0, *FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_SE_PITCH);
                    fighter.on_flag(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_FLAG_JUMP_RESTART);
                    fighter.on_flag(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_FLAG_JUMP_RESTART_NOW);
                    let frame = fighter.status_frame();
                    fighter.set_float(frame as f32, *FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_RESTART_FRAME);
                }
            }
        }
    }
    if fighter.is_flag(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_FLAG_JUMP_RESTART_NOW) {
        let restart_frame = fighter.get_float(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_RESTART_FRAME);
        let some_frame = fighter.status_frame() - restart_frame as i32;
        let add_motion_rate_frame = fighter.get_param_int("param_special_hi", "hulahoop_jump_restart_add_motion_rate_frame");
        if add_motion_rate_frame < some_frame {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            fighter.set_float(1.0, *FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_MOTION_RATE_NOW);
            fighter.off_flag(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_FLAG_JUMP_RESTART_NOW);
            fighter.set_float(0.0, *FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_SE_PITCH);
        }
    }
    else {
        let jump_speed_y = fighter.get_float(*FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_JUMP_SPEED_Y);
        let jump_gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x1d57d7b043);
        let jump_speed_y_mod = (jump_speed_y - jump_gravity).max(0.6);
        let jump_y_speed = fighter.get_param_float("param_special_hi", "hulahoop_jump_y_speed");
        let init_speed_x = fighter.get_float(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_INIT_SPEED_X);
        let max_speed_y_rate = fighter.get_param_float("param_special_hi", "hulahoop_jump_stick_max_speed_y_rate");
        let max_speed_x = fighter.get_param_float("param_special_hi", "hulahoop_jump_stick_max_speed_x");
        let fuckshit = 1.0 - (1.0 - max_speed_y_rate) * (init_speed_x / max_speed_x).abs();
        let mut l120 = jump_y_speed * fuckshit;
        if jump_speed_y_mod < l120 {
            l120 = jump_speed_y_mod;
        }
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, l120);
        fighter.set_float(l120, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_JUMP_SPEED_Y);
    }
    let init_speed_x = fighter.get_float(*FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_INIT_SPEED_X);
    let mut l90 = init_speed_x;
    let stick_x = fighter.stick_x();
    if stick_x != 0.0 {
        let x_speed_rate = fighter.get_param_float("param_special_hi", "hulahoop_jump_stick_x_speed_rate");
        let some_x_speed = stick_x * x_speed_rate;
        l90 = init_speed_x + stick_x;
        let max_speed_x = fighter.get_param_float("param_special_hi", "hulahoop_jump_stick_max_speed_x");
        if max_speed_x < l90.abs() {
            if stick_x > 0.0 {
                l90 = max_speed_x;
            }
            else {
                l90 = -max_speed_x;
            }
        }
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, l90, 0.0);
        fighter.set_float(l90, *FIGHTER_WIIFIT_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_INIT_SPEED_X);
    }
    let facing = fighter.lr();
    let x_inclination_rate = fighter.get_param_float("param_special_hi", "hulahoop_jump_stick_x_inclination_rate");
    let lb0 = x_inclination_rate * l90 * facing;
    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f::new(0.0, 0.0, lb0), MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_NONE as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    fighter.set_float(lb0, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_JUMP_ROTATION);

    return 0.into();
}

// FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_END

unsafe extern "C" fn special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);

    agent.status(Pre, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_pre);
    agent.status(Main, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_main);

    agent.status(Pre, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_END, special_hi_end_pre);
}