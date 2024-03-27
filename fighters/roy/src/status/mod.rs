use super::*;
use globals::*;
// status script import

pub fn set_gravity_delay_resume_frame(energy: *mut app::FighterKineticEnergyGravity, frames: i32) {
    unsafe {
      *(energy as *mut i32).add(0x50 / 4) = frames;
      *(energy as *mut bool).add(0x5C) = false;
    }
  }

pub unsafe extern "C" fn init_specials(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    let customize_special_hi_no = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_HI_NO);
    let start_spd_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("start_spd_x_mul"));
    let air_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_spd_y"));
    let mut stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
    let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::KineticEnergy;
    let mut motion_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut app::KineticEnergy;

    let mut aerial_y_speed = 0.0;
    let mut aerial_x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * start_spd_x_mul;

    // [v] Disable motion energy
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    let sum_speed_main = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    // Check for the side B status you're currently in
    let current_status_kind = StatusModule::status_kind(fighter.module_accessor);
    let current_situation_kind = StatusModule::situation_kind(fighter.module_accessor);

    // alStack192 = gravity energy
    // alStack176 = stop energy
    // alStack208 = motion energy
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2].contains(&current_status_kind) {
        if current_situation_kind == *SITUATION_KIND_GROUND {
            let reset_speed_2f = Vector2f { x: 0.0, y: 0.0 };
            let reset_speed_3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            lua_bind::KineticEnergy::reset_energy(motion_energy, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS_IGNORE_NORMAL, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            lua_bind::KineticEnergy::enable(motion_energy);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        else if current_situation_kind == *SITUATION_KIND_AIR {
            if !VarModule::is_flag(fighter.battle_object, vars::common::instance::SPECIAL_STALL_USED) {
                VarModule::on_flag(fighter.battle_object, vars::common::instance::SPECIAL_STALL_USED);
                aerial_y_speed = air_spd_y;
            }
            else{
                aerial_y_speed = 0.0;
            }
            let reset_speed_2f = Vector2f { x: aerial_x_speed, y: 0.0 };
            let reset_speed_gravity_2f = Vector2f { x: 0.0, y: aerial_y_speed };
            let reset_speed_3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &reset_speed_gravity_2f, &reset_speed_3f, fighter.module_accessor);
            lua_bind::KineticEnergy::enable(stop_energy);
            lua_bind::KineticEnergy::enable(gravity_energy);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
    if [*FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4].contains(&current_status_kind) {
        if current_situation_kind == *SITUATION_KIND_GROUND {
            let reset_speed_2f = Vector2f { x: 0.0, y: 0.0 };
            let reset_speed_3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            lua_bind::KineticEnergy::reset_energy(motion_energy, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS_IGNORE_NORMAL, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            lua_bind::KineticEnergy::enable(motion_energy);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else if current_situation_kind == *SITUATION_KIND_AIR {
            let reset_speed_2f = Vector2f { x: 0.0, y: 0.0 };
            let reset_speed_3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            lua_bind::KineticEnergy::enable(stop_energy);
            lua_bind::KineticEnergy::enable(gravity_energy);
        }
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    0.into()
}

pub unsafe extern "C" fn special_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    let mot = if VarModule::is_flag(fighter.battle_object, vars::roy::status::SIDE_B_REVERSE) {
        (hash40("special_s4_back") as i64, hash40("special_air_s4_back") as i64)
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_LW) {
        (hash40("special_s4_lw") as i64, hash40("special_air_s4_lw") as i64)
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_HI) {
        (hash40("special_s4_hi") as i64, hash40("special_air_s4_hi") as i64)
    }
    else {
        (hash40("special_s4_s") as i64, hash40("special_air_s4_s") as i64)
    };
    WorkModule::set_int64(fighter.module_accessor, mot.0, *FIGHTER_ROY_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, mot.1, *FIGHTER_ROY_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
    VarModule::off_flag(fighter.battle_object, vars::roy::status::SIDE_B_REVERSE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_HI);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_LW);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s4_main_loop as *const () as _))
}

unsafe extern "C" fn special_s4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_s_mot_helper(fighter);
        }
    }
    else {
        special_s_mot_helper(fighter);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let sit = fighter.global_table[SITUATION_KIND].get_i32();
        if sit != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn special_s_mot_helper(fighter: &mut L2CFighterCommon) {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    let correct;
    let mot;
    if sit != *SITUATION_KIND_GROUND {
        correct = *GROUND_CORRECT_KIND_AIR;
        mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
    }
    else {
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, init_specials);
    agent.status(Main, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, special_s4_main);
}
