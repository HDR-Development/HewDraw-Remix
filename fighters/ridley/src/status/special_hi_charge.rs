use super::*;

unsafe extern "C" fn special_hi_charge_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_charge_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let decide_stick_x = VarModule::get_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_HOVER_DECIDE_STICK_X);
    let decide_stick_y = VarModule::get_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_HOVER_DECIDE_STICK_Y);
    let mut rad = 0.0_f32.to_radians();
    let charge_speed_hi = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_speed_hi"));
    let lr = PostureModule::lr(fighter.module_accessor);

    let stick_added = decide_stick_x.abs() + decide_stick_y.abs();

    if stick_added >= 0.5 {
        let atan = (decide_stick_x * lr).atan2(decide_stick_y);
        rad = atan;
    }

    let rush_speed_x = charge_speed_hi * rad.sin() * lr;
    let rush_speed_y = charge_speed_hi * rad.cos();

    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, rush_speed_x, rush_speed_y);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    let charge_degree_lw = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_degree_lw"));
    let rot_angle = if rad.to_degrees().abs() <= charge_degree_lw + 90.0 {
        rad.to_degrees()
    } else {
        rad.to_degrees() - 180.0 * rad.signum()
    };

    fighter.set_joint_rotate("rot", Vector3f{x: rot_angle, y: 0.0, z: 0.0});

    VarModule::set_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_CHARGE_DIR, rad);

    0.into()
}

unsafe extern "C" fn special_hi_charge_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_CHARGE_DECCEL);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_COUNT);

    let charge_degree_lw = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_degree_lw"));
    let dir = VarModule::get_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_CHARGE_DIR);
    let degrees = dir.to_degrees();

    let motion = if degrees.abs() <= charge_degree_lw + 90.0 {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_STATUS);
        "special_air_hi_charge_hi"
    } else {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_STATUS);
        "special_air_hi_charge_lw"
    };

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new(motion),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_frame_hi")) as f32;
    let rate = MotionModule::end_frame(fighter.module_accessor) / charge_frame;
    MotionModule::set_rate(fighter.module_accessor, rate);
    
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
    fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());

    fighter.main_shift(special_hi_charge_hi_main_loop)
}

unsafe extern "C" fn special_hi_charge_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 0.into();
    }

    if !fighter.global_table[IS_STOPPING].get_bool() {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            let normal_x = GroundModule::get_touch_normal_x(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
            let normal_y = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
            let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let angle = sv_math::vec2_angle(normal_x, normal_y, speed_x, speed_y);
            let charge_landing_angle = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_degree_lw"));
            let landing_angle_threshold = (charge_landing_angle + 90.0).to_radians();
            if angle > landing_angle_threshold {
                fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
                return 0.into();
            }
        }
    }

    0.into()
}

unsafe extern "C" fn special_hi_charge_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_COUNT);

    let dir = VarModule::get_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_CHARGE_DIR);
    let rot_angle = if !fighter.is_motion(Hash40::new("special_air_hi_charge_lw")) {
        dir.to_degrees()
    } else {
        dir.to_degrees() - 180.0 * dir.signum()
    };
    fighter.set_joint_rotate("rot", Vector3f{x: rot_angle, y: 0.0, z: 0.0});

    VarModule::set_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_CHARGE_DIR, dir);

    let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_COUNT);

    let passable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("passable_frame"));
    let passable = frame <= passable_frame;
    GroundModule::set_passable_check(fighter.module_accessor, passable);

    let deccel_start_frame_hi = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("deccel_start_frame_hi"));

    if frame <= deccel_start_frame_hi {
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        return 0.into();
    }

    let charge_deccel_hi = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_deccel_hi"));
    let mut deccel_x = (dir.sin() * charge_deccel_hi).abs();
    let mut deccel_y = (dir.cos() * charge_deccel_hi).abs();

    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, deccel_x, deccel_y);

    0.into()
}

unsafe extern "C" fn special_hi_charge_hi_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dir = VarModule::get_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_CHARGE_DIR);
    let rot_angle = if !fighter.is_motion(Hash40::new("special_air_hi_charge_lw")) {
        dir.to_degrees()
    } else {
        dir.to_degrees() - 180.0 * dir.signum()
    };
    fighter.set_joint_rotate("rot", Vector3f{x: rot_angle, y: 0.0, z: 0.0});

    0.into()
}

unsafe extern "C" fn special_hi_charge_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END {
        fighter.set_joint_rotate("rot", Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }

    0.into()
}

unsafe extern "C" fn special_hi_charge_hi_fix_pos_slow(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}


pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_pre);
    agent.status(Init, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_init);
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_main);
    agent.status(Exec, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_exec);
    agent.status(ExecStop, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_exec_stop);
    agent.status(End, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_end);
    agent.status(FixPosSlow, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, special_hi_charge_hi_fix_pos_slow);
}