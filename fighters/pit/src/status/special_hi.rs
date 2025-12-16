use super::*;


// FIGHTER_STATUS_KIND_SPECIAL_HI

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE,
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("pit_fly_miracle_start"), true, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("pit_ikaros_wing_flare"), true, true);
    }

    return 0.into();
}

// FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH

unsafe extern "C" fn special_hi_rush_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Init, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH)(fighter);

    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let mut rad = 0.0_f32.to_radians();
    let rush_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_speed"));
    let lr = PostureModule::lr(fighter.module_accessor);

    let stick_added = stick_x.abs() + stick_y.abs();

    if stick_added >= 0.5 {
        let atan = (stick_x * lr).atan2(stick_y);
        rad = atan;
    }

    let rush_speed_x = rush_speed * rad.sin() * lr;
    let rush_speed_y = rush_speed * rad.cos();

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, rush_speed_x, rush_speed_y);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    fighter.set_joint_rotate("rot", Vector3f{x: rad.to_degrees(), y: 0.0, z: 0.0});

    WorkModule::set_float(fighter.module_accessor, rad, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SDIR);
    WorkModule::set_float(fighter.module_accessor, rad, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SDIR_INIT);

    ret
}

unsafe extern "C" fn special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH)(fighter);

    let rush_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.rush_frame") as f32;
    let rate = MotionModule::end_frame(fighter.module_accessor) / rush_frame;
    MotionModule::set_rate(fighter.module_accessor, rate);

    ret
}

unsafe extern "C" fn special_hi_rush_handle_landing(fighter: &mut L2CFighterCommon) -> bool {
    // Check for landing
    let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_INT_RUSH_FRAME);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    if fighter.is_situation(*SITUATION_KIND_GROUND)
    && (speed_y <= 0.5 || frame > 6) {
        let normal_x = GroundModule::get_touch_normal_x(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let touch_normal_angle = sv_math::vec2_angle(normal_x, normal_y, speed_x, speed_y);
        let rush_land_angle: f32 = 20.0;
        let adjusted_angle_rad: f32 = (rush_land_angle + 90.0).to_radians();

        // If angle is too steep, then land. Otherwise, continue forward
        if adjusted_angle_rad < touch_normal_angle {
            fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END.into(), false.into());
            return true;
        }
        else {
            let speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let lr = PostureModule::lr(fighter.module_accessor);

            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SDIR);

            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed.abs() * lr, -0.02);
        }
    }

    false
}

unsafe extern "C" fn special_hi_rush_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH)(fighter);

    let dir = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SDIR_INIT);

    fighter.set_joint_rotate("rot", Vector3f{x: dir.to_degrees(), y: 0.0, z: 0.0});

    WorkModule::set_float(fighter.module_accessor, dir, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SDIR);

    let rush_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_speed"));

    let mut rush_speed_x = rush_speed * dir.sin() * PostureModule::lr(fighter.module_accessor);
    let mut rush_speed_y = rush_speed * dir.cos();

    let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_INT_RUSH_FRAME);
    let rush_brake_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_brake_frame"));

    if frame > rush_brake_frame {
        let rush_brake = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.rush_brake");
        let deccel_x = rush_brake * dir.sin() * PostureModule::lr(fighter.module_accessor) * (frame as f32 - 1.0);
        let deccel_y = rush_brake * dir.cos() * (frame as f32 - 1.0);

        rush_speed_x = if deccel_x.abs() > rush_speed_x.abs() {
            0.0
        }
        else {
            rush_speed_x - deccel_x
        };

        rush_speed_y = if deccel_y.abs() > rush_speed_y.abs() {
            0.0
        }
        else {
            rush_speed_y - deccel_y
        };
    }

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, rush_speed_x, rush_speed_y);

    if special_hi_rush_handle_landing(fighter) {
        return 1.into();
    }

    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);

    ret
}

unsafe extern "C" fn special_hi_rush_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dir = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SDIR_INIT);

    fighter.set_joint_rotate("rot", Vector3f{x: dir.to_degrees(), y: 0.0, z: 0.0});

    0.into()
}

unsafe extern "C" fn special_hi_rush_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(End, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH)(fighter);

    if fighter.global_table[STATUS_KIND] != FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END {
        fighter.set_joint_rotate("rot", Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }

    ret
}

// FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END

pub unsafe extern "C" fn special_hi_rush_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIT_SPECIAL_HI_RUSH_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIT_SPECIAL_HI_RUSH_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIT_SPECIAL_HI_RUSH_END_FLOAT,
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
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    
    return 0.into();
}

pub unsafe extern "C" fn special_hi_rush_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_end"), 0.0, 1.0, false, 0.0, false, false);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let x_max_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * x_max_mul, 0.0);
    fighter.select_cliff_hangdata_from_name("special_hi");
    
    fighter.main_shift(special_hi_rush_end_main_loop)
}

unsafe extern "C" fn special_hi_rush_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
    }
    return 0.into();
}

unsafe extern "C" fn special_hi_rush_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut dir = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SDIR).to_degrees();
    if dir > 0.0 {
        dir -= 4.0;

        fighter.set_joint_rotate("rot", Vector3f{x: dir, y: 0.0, z: 0.0});

        WorkModule::set_float(fighter.module_accessor, dir.to_radians(), *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SDIR);
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_rush_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(End, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END)(fighter);

    fighter.set_joint_rotate("rot", Vector3f{x: 0.0, y: 0.0, z: 0.0});

    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end);

    agent.status(Init, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_init);
    agent.status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_main);
    agent.status(Exec, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_exec);
    agent.status(ExecStop, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_exec_stop);
    agent.status(End, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_end);

    agent.status(Pre, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_rush_end_pre);
    agent.status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_rush_end_main);
    agent.status(Exec, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_rush_end_exec);
    agent.status(ExecStop, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_rush_exec_stop);
    agent.status(End, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_rush_end_end);
}