use super::*;

pub unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_TRAMPOLINE_JUMP) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_HI_JUMP_NUM);
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            //println!("started in air");
            VarModule::on_flag(fighter.battle_object, vars::pacman::instance::DISABLE_SPECIAL_HI);
            VarModule::off_flag(fighter.battle_object, vars::pacman::instance::SPECIAL_HI_GROUND_START);
        }
        else {
            //println!("started on ground");
            VarModule::on_flag(fighter.battle_object, vars::pacman::instance::SPECIAL_HI_GROUND_START);
        }
    }
    if fighter.is_prev_situation(*SITUATION_KIND_GROUND) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_HI_WORK_FLAG_AIR);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_HI_WORK_FLAG_AIR);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
    }

    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);

    fighter.main_shift(special_hi_main_loop)
}

pub unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

pub unsafe extern "C" fn special_hi_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_loop"), 0.0, 1.0, false, 0.0, false, false);
    let mut start_height_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_height_y")) * 10.0;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_HI_WORK_FLAG_AIR) {
        let air_start_height_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_start_height_y_mul"));
        start_height_y *= air_start_height_y_mul;
    }
    let mut height_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_HEIGHT_MUL);
    if height_mul != 0.0 {
        let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_HI_JUMP_NUM);
        let mul = if jump_count == 1 { 0.9 } else { 0.89 };
        start_height_y *= height_mul * mul;
    }
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_HEIGHT_MUL);
    let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("accel_y"));
    let jump_speed_y = app::KineticUtility::get_jump_speed_y(start_height_y, accel_y);

    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, jump_speed_y);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);

    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let param_air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_accel_x_mul"));
    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul * param_air_accel_x_mul);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add * param_air_accel_x_mul);

    let air_max_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_max_speed_x_mul"));
    sv_kinetic_energy!(mul_x_speed_max, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_max_speed_x_mul);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    let frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("frame"));
    WorkModule::set_int(fighter.module_accessor, frame, *FIGHTER_PACMAN_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

    let mut attack_mul = 1.0;
    let mut attack_size_mul = 1.0;
    let jump_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_HI_JUMP_NUM);
    if jump_num > 0 {
        // don't disable up special from multiple bounces off a grounded trampoline
        if !VarModule::is_flag(fighter.battle_object, vars::pacman::instance::SPECIAL_HI_GROUND_START) {
            //println!("used air bounce");
            VarModule::on_flag(fighter.battle_object, vars::pacman::instance::DISABLE_SPECIAL_HI);
        }
        // else {
        //     println!("bounced from grounded trampoline, carry on");
        // }
        let mut ivar4 = 0;
        while ivar4 < jump_num {
            let param_attack_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("attack_mul"));
            attack_mul *= param_attack_mul;
            let param_attack_size_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("attack_size_mul"));
            attack_size_mul *= param_attack_size_mul;
            ivar4 += 1;
        }
    }
    AttackModule::set_power_mul(fighter.module_accessor, attack_mul);
    AttackModule::set_attack_scale(fighter.module_accessor, attack_size_mul, true);

    if !StopModule::is_stop(fighter.module_accessor) { special_hi_loop_substatus(fighter, false.into()); }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_hi_loop_substatus as *const () as _));
    fighter.main_shift(special_hi_loop_main_loop)
}

pub unsafe extern "C" fn special_hi_loop_substatus(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    let max_angle = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("max_angle")) * fighter.stick_x();
    let facing = PostureModule::lr(fighter.module_accessor);
    let rot_angle = max_angle * facing;
    ModelModule::set_joint_rotate(
        fighter.module_accessor,
        Hash40::new("rot"),
        &Vector3f::new(rot_angle, 0.0, 0.0),
        MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},
        MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8}
    );
    if param_2.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    }

    return 0.into();
}

pub unsafe extern "C" fn special_hi_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_HI_WORK_INT_FRAME) <= 0 {
        fighter.change_status(FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 1.into();
    }
    special_hi_check_aerial(fighter);

    return 0.into();
}

pub unsafe extern "C" fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_rot(fighter.module_accessor, &Vector3f::zero(), 0);
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) { Hash40::new("special_hi_end") } else { Hash40::new("special_air_hi_end") };
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);

    fighter.main_shift(special_hi_end_main_loop)
}

pub unsafe extern "C" fn special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_end"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi_end"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    special_hi_check_aerial(fighter);
    if MotionModule::is_end(fighter.module_accessor) {
        let new_status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL } else { FIGHTER_STATUS_KIND_FALL_SPECIAL };
        fighter.change_status(new_status.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

pub unsafe extern "C" fn special_hi_check_aerial(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, vars::pacman::status::SPECIAL_HI_AERIAL) {
        if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
        }
        if fighter.sub_transition_group_check_air_attack().get_bool() {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);
            VarModule::on_flag(fighter.battle_object, vars::pacman::instance::SPECIAL_HI_AERIAL_USED);
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(Main, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP, special_hi_loop_main);
    agent.status(Main, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_END, special_hi_end_main);
}