use super::*;

// swapping the cycle order of thundaga and blizzaga

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[0x10].get_i32();
    WorkModule::set_int(fighter.module_accessor, prev_status, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_STATUS_KIND_ATTACK_PREV);
    let mut magic_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND);
    let fire = *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_FIRE;
    let blizzard = *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_BLIZZARD;
    let thunder = *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_THUNDER;
    if magic_kind == blizzard { // change to fire
        magic_kind = fire;
    } else if magic_kind == fire { // change to blizzard
        magic_kind = blizzard;
    }
    if ![fire, blizzard, thunder].contains(&magic_kind) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N1);
    } else if magic_kind == fire {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N1);
    } else if magic_kind == blizzard { 
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N2);
    } else if magic_kind == thunder {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N3); 
    }

    return 1.into();
}

unsafe extern "C" fn special_n2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
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
        0,
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0,
    );
    return 0.into();
}

unsafe extern "C" fn special_n2_main_loop_function(fighter: &mut L2CFighterCommon, flag_shooted: i32, work_id_n2_hop: i32) {
    if WorkModule::is_flag(fighter.module_accessor, flag_shooted) {
        WorkModule::off_flag(fighter.module_accessor, flag_shooted);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        if situation_kind == *SITUATION_KIND_AIR
        && WorkModule::is_flag(fighter.module_accessor, work_id_n2_hop) {
            WorkModule::on_flag(fighter.module_accessor, work_id_n2_hop);
            let x_param_float = WorkModule::get_param_float(fighter.module_accessor, smash::hash40("param_special_n"), smash::hash40("hop_add_speed_x"));
            let posture_module_lr = app::lua_bind::PostureModule::lr(fighter.module_accessor);
            let inertia = x_param_float * posture_module_lr;
            let y_param_float = WorkModule::get_param_float(fighter.module_accessor, smash::hash40("param_special_n"), smash::hash40("hop_add_speed_y"));
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, inertia, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            app::sv_kinetic_energy::enable(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, y_param_float);
        }
    }
    return;
}

unsafe extern "C" fn special_n2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !(CancelModule::is_enable_cancel(fighter.module_accessor)
        && fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool()
        || fighter.sub_air_check_fall_common().get_bool()) {
        if !(app::lua_bind::MotionModule::is_end(fighter.module_accessor)) {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n2"), L2CValue::Hash40s("special_air_n2"), true.into());
            fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_n"));
            fighter.sub_set_ground_correct_by_situation(true.into());
            special_n2_main_loop_function(fighter, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_SHOOTED, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N2_HOP);
            return 0.into();
        }
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }

    return 0.into()
}

unsafe extern "C" fn special_n2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n2"), L2CValue::Hash40s("special_air_n2"), false.into());
    let initial_speed_y = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N2_HOP) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        Some(app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent))
    }
    else {
        None
    };
    // Added code
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    if situation_kind == *SITUATION_KIND_AIR {
        // Glide a small amount in the air unless there's too much positive y energy (to avoid flying to the top blastzone)
        let mut aerial_y_speed = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let mut aerial_x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * 0.001;
        // let mut x_string = aerial_x_speed.to_string();
        // let mut y_string = aerial_y_speed.to_string();
        // println!("Pre X: {}" , x_string);
        // println!("Pre Y: {}" , y_string);
        let mut reset_speed_2f = Vector2f { x: aerial_x_speed, y: 0.0 };
        let mut reset_speed_gravity_2f = Vector2f { x: 0.0, y: 0.0 };
        let mut reset_speed_3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
        let mut stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
        let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::KineticEnergy;
        lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
        lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &reset_speed_gravity_2f, &reset_speed_3f, fighter.module_accessor);
        lua_bind::KineticEnergy::enable(stop_energy);
        lua_bind::KineticEnergy::enable(gravity_energy);
        // Don't allow drift during the move and set accelleration to slow descent
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        lua_bind::FighterKineticEnergyGravity::set_accel(fighter.get_gravity_energy(), -0.02);
        lua_bind::FighterKineticEnergyGravity::set_gravity_coefficient(fighter.get_gravity_energy(), 0.7);
        // Keep accelleration
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, aerial_x_speed);
        // Bounds here were made based off testing, may need tweaking if not getting float effect
        // when expected or vice versa
        if aerial_y_speed >= 0.9 && (aerial_x_speed <= 0.0011 && aerial_x_speed >= -0.0011)  {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
            fighter.sub_set_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_n"));
        }
    }  
    // End of added code 
    if let Some(speed) = initial_speed_y {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed);
        Some(app::sv_kinetic_energy::set_speed(fighter.lua_state_agent));
    }
    fighter.sub_set_ground_correct_by_situation(true.into());
    return fighter.main_shift(special_n2_main_loop);
}

pub fn install() {
    smashline::Agent::new("trail")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre)
        .status(Pre, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N2, special_n2_pre)
        .status(Main, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N2, special_n2_main)
        .install();
}
