use core::f32;

use super::*;
use smash::cpp::root::app::KineticEnergy;

unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    // dj leniency window
    let teleport_jump_aerial_refresh_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "teleport_jump_aerial_refresh_frame");
    let jump_max = fighter.get_jump_count_max();
    if fighter.get_num_used_jumps() == jump_max && fighter.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) && fighter.global_table[PREV_STATUS_FRAME].get_i32() <= teleport_jump_aerial_refresh_frame {
        fighter.set_int(jump_max - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("sys_jump_aerial"), true, true);
        smash::app::sv_animcmd::EFFECT_OFF_KIND(fighter.lua_state_agent);
    }
    // touching ground at any point counts as a grounded start
    if !VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_GROUNDED_TELEPORT) && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        VarModule::on_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_GROUNDED_TELEPORT);
    }
    0.into()
}

unsafe extern "C" fn special_hi2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        false, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK as i32
    );
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor,
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
    0.into()
}

unsafe extern "C" fn angler(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.left_stick_x();
    let stick_y = fighter.left_stick_y();
    let mut length = sv_math::vec2_length(stick_x, stick_y);
    let wrap_stick = fighter.get_param_float("param_special_hi", "wrap_stick");

    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    let lr = fighter.lr();
    let mut angle = if length >= wrap_stick {
        stick_y.atan2(stick_x * lr)
    } else {
        90.0_f32.to_radians()
    };
    let test_angle = if angle < f32::consts::PI {angle} else {angle - f32::consts::PI};
    let mut detach = false;
    if test_angle < f32::consts::PI && test_angle > 0.0 {
        detach = true;
    }
    let wrap_speed_multi = fighter.get_param_float("param_special_hi", "wrap_speed_multi");
    let wrap_speed_add = fighter.get_param_float("param_special_hi", "wrap_speed_add");
    let mut speed_x = 0.0;
    let mut speed_y = wrap_speed_multi + wrap_speed_add;
    // if angled w/ stick
    if length > wrap_stick {
        let length_mul = wrap_speed_multi * length;
        let speed = length_mul + wrap_speed_add;
        let cos = angle.cos();
        speed_x = speed * cos;
        speed_x *= lr;

        let sin = angle.sin();
        speed_y = speed * sin;
    }
    // If teleport angle is upwards or you are already in air
    // force airborne state
    if detach || fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::set_attach_ground(fighter.module_accessor, false);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    KineticModule::unable_energy_all(fighter.module_accessor);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_FREE, speed_x, speed_y, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, -1.0);
    sv_kinetic_energy!(enable, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    GroundModule::clear_cliff_point(fighter.module_accessor);
    0.into()
}

unsafe extern "C" fn special_hi2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    VisibilityModule::set_whole(fighter.module_accessor, false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
    fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
    fighter.set_int(0, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    angler(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi2_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    let frame = fighter.get_int(*FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    let move_time = fighter.get_param_int("param_special_hi", "move_time");
    if frame >= move_time {
        fighter.change_status(FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3.into(), true.into())
    }
    if StatusModule::is_changing(fighter.module_accessor) || StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
        // slide-offs keep jump
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        && GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) { // prevent weird interactions with sloped ledge?
            VarModule::on_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_GROUNDED_TELEPORT);
        }
    }
    // substatus
    if !StatusModule::is_changing(fighter.module_accessor)
    & !StopModule::is_stop(fighter.module_accessor) {
        fighter.inc_int(*FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        let move_xlu = fighter.get_param_int("param_special_hi", "move_xlu"); // time ignoring platforms
        let cliff_check_frame = fighter.get_param_int("param_special_hi", "move_cliff_check");
        if frame == move_xlu {
            GroundModule::set_passable_check(fighter.module_accessor, false);
        }
        if frame == cliff_check_frame {
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
        }
        if frame == 1 {
            fighter.on_flag(*FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND);
        }
    }
    // floor ride
    special_hi_2_check_ground(fighter);
    0.into()
}

// Copies nasty vanilla math
// with adjusted logic to control wall-ride/floor-ride behavior
unsafe extern "C" fn special_hi_2_check_ground(fighter: &mut L2CFighterCommon) {
    let init_speed_x = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_X);
    let init_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_Y);
    let floor_speed_x = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_FLOOR_SPEED_X);
    if floor_speed_x.abs() > 0.0 && init_speed_y < 0.0 && init_speed_x.abs() > 0.0 
    && (GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) && !GroundModule::is_passable_ground(fighter.module_accessor)) {
        // Travel speed for diagonally-down floor-rides
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, floor_speed_x, 0.0, 0.0);
    } else {
        // Travel speed for all other scenarios
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, init_speed_x, init_speed_y, 0.0);
    }
    // If on a platform,
    // skip floor-ride speed redirection
    if GroundModule::is_passable_check(fighter.module_accessor) && GroundModule::is_passable_ground(fighter.module_accessor) {
        return;
    }
    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
    let speed = Vector2f {x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};
    // If not a diagonally-down teleport,
    // or if already grounded,
    // skip floor-ride speed redirection
    if !GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    || (speed.x.abs() < 0.001 || speed.y > -0.001)
    || fighter.global_table[PREV_SITUATION_KIND] != SITUATION_KIND_AIR {
        return;
    }
    // Compute a new ground-aligned velocity vector
    // for intended floor-ride speed redirection
    // 
    // Only intended to run on the first frame you land during the travel
    let mut length = sv_math::vec3_length(speed.x, speed.y, 0.0);
    if length > 0.0 {
        let touch_x = GroundModule::get_touch_normal_x(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let touch_y = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);

        let touch = fighter.Vector3__create(touch_x.into(), touch_y.into(), 0.0_f32.into());
        let something = fighter.Vector3__create(0.0_f32.into(), 0.0_f32.into(), 1.0_f32.into());
        let mut cross = fighter.Vector3__cross(touch.clone(), something);

        let math = 1.0 / length;
        let speed_mul = Vector3f {
            x: speed.x * math,
            y: speed.y * math,
            z: 0.0,
        };
        let mut final_dot = sv_math::vec3_dot(cross["x"].get_f32(), cross["y"].get_f32(), cross["z"].get_f32(), speed_mul.x, speed_mul.y, speed_mul.z);
        if -0.00001 <= final_dot && final_dot <= 0.00001 {
            final_dot = sv_math::vec3_dot(cross["x"].get_f32(), cross["y"].get_f32(), cross["z"].get_f32(), fighter.lr(), 0.0, 0.0);
        }

        if final_dot < 0.0 {
            let x = cross["x"].get_f32();
            let y = cross["y"].get_f32();
            let z = cross["z"].get_f32();
            cross["x"].assign(&L2CValue::F32(x * -1.0));
            cross["y"].assign(&L2CValue::F32(y * -1.0));
            cross["z"].assign(&L2CValue::F32(z * -1.0));
        }
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, cross["x"].get_f32() * length, cross["y"].get_f32() * length, cross["z"].get_f32() * length);
        // set new speed to be reapplied each frame
        VarModule::set_float(fighter.battle_object, vars::common::status::TELEPORT_FLOOR_SPEED_X, cross["x"].get_f32() * length);
    }
}

unsafe extern "C" fn special_hi2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_hi2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        // enable cancel
        VarModule::on_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_ENABLE_FREEFALL);
        if fighter.get_num_used_jumps() < fighter.get_jump_count_max() {
            VarModule::off_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_ENABLE_FREEFALL);
        }
        // dj resource handling
        if !VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_GROUNDED_TELEPORT) {
            let jump_max = fighter.get_jump_count_max();
            fighter.set_int(jump_max, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::DISABLE_FLOAT);
        }
        VarModule::off_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_GROUNDED_TELEPORT);
    }
    // reverse re-appearance
    if StatusModule::status_kind_next(fighter.module_accessor) == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 {
        //if !VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_ENABLE_FREEFALL) {
        if fighter.left_stick_x() * fighter.lr()
        <= fighter.get_param_float("common", "turn_stick_x") {
            // prevents turning around with buffered aerials
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
        //}
    } else {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
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
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, "special_hi".to_hash(), 0.0, 1.0, false, 0.0, false, false);
    } else {
        fighter.set_int(*SITUATION_KIND_AIR, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        if !VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_ENABLE_FREEFALL) {
            MotionModule::change_motion(fighter.module_accessor, "special_air_hi_cancel".to_hash(), 0.0, 1.0, false, 0.0, false, false);
            VarModule::on_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_ENABLE_FREEFALL);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_LAG);
        } else {
            MotionModule::change_motion(fighter.module_accessor, "special_air_hi".to_hash(), 0.0, 1.0, false, 0.0, false, false);
            let landing_lag = fighter.get_param_float("param_special_hi", "landing_frame");
            fighter.set_float(landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            // special fall speed
            let x_max = fighter.get_param_float("param_special_hi", "fall_x_mull_value");
            fighter.set_float(x_max, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
        }
    }
    fighter.main_shift(special_hi3_main_loop)
}

unsafe extern "C" fn special_hi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let control = ControlModule::get_attack_air_kind(fighter.module_accessor);
    if control == *FIGHTER_COMMAND_ATTACK_AIR_KIND_NONE {
        FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
    } // ?
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into())
        } else {
            // special fall based on resource
            if fighter.is_motion(Hash40::new("special_air_hi_cancel")) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into())
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into())
            }
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if fighter.is_motion(Hash40::new("special_air_hi_cancel")) {
                let lag = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.empty_landing");
                fighter.set_float(lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            }
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into())
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into())
        }
    }
    // momentum and fastfall
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        reappearance_decel_drift(fighter);
    }
    return 0.into();
}

unsafe extern "C" fn reappearance_decel_drift(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    // drift / fastfall enable flag
    if fighter.is_flag(*FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_1) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        fighter.off_flag(*FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_1);
    }
    if fighter.status_frame() < 7 {
        let stop_energy: *mut KineticEnergy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
        let speed = Vector2f {
            x: lua_bind::KineticEnergy::get_speed_x(stop_energy),
            y: lua_bind::KineticEnergy::get_speed_y(stop_energy),
        };
        let hi3_brake_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.hi3_brake_y_mul");
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x, speed.y * hi3_brake_y_mul);
    } else if fighter.status_frame() == 7 {
        let stop_energy: *mut KineticEnergy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
        let speed = Vector2f {
            x: lua_bind::KineticEnergy::get_speed_x(stop_energy),
            y: lua_bind::KineticEnergy::get_speed_y(stop_energy),
        };
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed.y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    return 0.into();
}

unsafe extern "C" fn special_hi3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
        if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            fighter.set_float(1.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
            fighter.set_float(0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
    }
    VisibilityModule::set_whole(fighter.module_accessor, true);
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exec);

    agent.status(Pre, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, special_hi2_pre);
    agent.status(Main, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, special_hi2_main);
    agent.status(Exec, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, special_hi2_exec);
    agent.status(End, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, special_hi2_end);

    agent.status(Pre, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, special_hi3_pre);
    agent.status(Main, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, special_hi3_main);
    agent.status(End, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, special_hi3_end);
}
