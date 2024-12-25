use super::*;
use smash::cpp::root::app::KineticEnergy;

unsafe extern "C" fn special_hi2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK as i32
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
        (*FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_2_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let mut length = sv_math::vec2_length(stick_x, stick_y).min(1.0);

    let wrap_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("wrap_stick"));
    let mut follow_stick = false;
    if length <= wrap_stick {
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
            follow_stick = true;
        }
        else {
            if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
                if GroundModule::is_passable_ground(fighter.module_accessor) {
                    follow_stick = true;
                }
                else {
                    let normal_x = GroundModule::get_touch_normal_x(fighter.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32);
                    let normal_y = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32);
                    let angle = sv_math::vec2_angle(normal_x, normal_y, stick_x, stick_y);
                    if angle < 90.0_f32.to_radians() {
                        follow_stick = true;
                    }
                }
            }
        }
    }
    else {
        follow_stick = true;
    }

    if 0.00001 < stick_x {
        PostureModule::set_lr(fighter.module_accessor, 1.0);
    }
    else if stick_x < -0.00001 {
        PostureModule::set_lr(fighter.module_accessor, -1.0);
    }
    PostureModule::update_rot_y_lr(fighter.module_accessor);

    let lr = PostureModule::lr(fighter.module_accessor);

    let wrap_speed_multi = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("wrap_speed_multi"));
    let wrap_speed_add = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("wrap_speed_add"));

    let mut speed_x;
    let mut speed_y = 0.0;
    if !follow_stick {
        let atan = stick_y.atan2(stick_x * lr);
        let length_mul = wrap_speed_multi * length;

        let speed = length_mul + wrap_speed_add;
        let cos = atan.cos();
        speed_x = speed * cos;
        speed_x *= lr;
    }
    else {
        let angle = if length < wrap_stick {
            length = 1.0;
            90.0_f32.to_radians()
        }
        else {
            stick_y.atan2(stick_x * lr)
        };

        let length_mul = wrap_speed_multi * length;

        let speed = length_mul + wrap_speed_add;
        let cos = angle.cos();
        speed_x = speed * cos;
        speed_x *= lr;

        let sin = angle.sin();
        speed_y = speed * sin;

        if angle != 0.0 && angle.to_degrees() != 180.0 { //previously would force airborne on horizontal angles..
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::set_attach_ground(fighter.module_accessor, false);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        } 
    }

    KineticModule::unable_energy_all(fighter.module_accessor);

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_FREE,
        speed_x,
        speed_y,
        0.0,
        0.0,
        0.0
    );

    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );

    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );

    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        -1.0,
        -1.0
    );

    sv_kinetic_energy!(
        enable,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );

    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    GroundModule::clear_cliff_point(fighter.module_accessor);

    0.into()
}

unsafe extern "C" fn special_hi2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn special_hi3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        false,
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, "special_hi".to_hash(), 0.0, 1.0, false, 0.0, false, false);
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_int(*SITUATION_KIND_AIR, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, "special_air_hi".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        let landing_lag = fighter.get_param_float("param_special_hi", "landing_frame");
        WorkModule::set_float(fighter.module_accessor, landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        //special fall speed
        let x_max = fighter.get_param_float("param_special_hi", "fall_x_mull_value");
        WorkModule::set_float(fighter.module_accessor, x_max, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    }
    
    fighter.main_shift(special_hi3_main_loop)
}

unsafe extern "C" fn special_hi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let control = ControlModule::get_attack_air_kind(fighter.module_accessor);
    if control == *FIGHTER_COMMAND_ATTACK_AIR_KIND_NONE {
        FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
    } //?
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
          return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            if !VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_TELEPORT_CANCEL) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    //momentum and fastfall
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_1) { //drift enable frame
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            fighter.off_flag(*FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_1); 
        } else {
            if fighter.status_frame() < 7 { //should run 8 times frames like pm?
                let stop_energy: *mut KineticEnergy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                let speed = Vector2f{x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x, speed.y *0.9);
            } else if fighter.status_frame() == 7 {
                let stop_energy: *mut KineticEnergy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                let speed = Vector2f{x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x, 0.0);
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed.y);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, special_hi2_pre);
    agent.status(Init, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, special_hi_2_init);
    agent.status(Exec, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, special_hi2_exec);
    
    agent.status(Pre, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, special_hi3_pre);
    agent.status(Main, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, special_hi3_main);
}