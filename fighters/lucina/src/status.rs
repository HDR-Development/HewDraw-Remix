use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        init_specials,
        special_lw_main,
        lucina_specials_main,
        lucina_specials2_main,
        lucina_specials3_main,
        lucina_specials3_exec_stop,
    );
}

pub fn set_gravity_delay_resume_frame(energy: *mut FighterKineticEnergyGravity, frames: i32) {
    unsafe {
      *(energy as *mut i32).add(0x50 / 4) = frames;
      *(energy as *mut bool).add(0x5C) = false;
    }
  }

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_specials(fighter: &mut L2CFighterCommon, arg: u64) -> L2CValue {
    let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    let customize_special_hi_no = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_HI_NO);
    let start_spd_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("start_spd_x_mul"));
    let air_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_spd_y"));
    let mut stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut KineticEnergy;
    let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut KineticEnergy;
    let mut motion_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut KineticEnergy;

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
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2].contains(&current_status_kind) {
        if current_situation_kind == *SITUATION_KIND_GROUND {
            let reset_speed_2f = smash::phx::Vector2f { x: 0.0, y: 0.0 };
            let reset_speed_3f = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            smash::app::lua_bind::KineticEnergy::reset_energy(motion_energy, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS_IGNORE_NORMAL, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            smash::app::lua_bind::KineticEnergy::enable(motion_energy);
            smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        else if current_situation_kind == *SITUATION_KIND_AIR {
            if !VarModule::is_flag(fighter.battle_object, vars::common::instance::SPECIAL_STALL_USED) {
                VarModule::on_flag(fighter.battle_object, vars::common::instance::SPECIAL_STALL_USED);
                aerial_y_speed = air_spd_y;
            }
            else{
                aerial_y_speed = 0.0;
            }
            let reset_speed_2f = smash::phx::Vector2f { x: aerial_x_speed, y: 0.0 };
            let reset_speed_gravity_2f = smash::phx::Vector2f { x: 0.0, y: aerial_y_speed };
            let reset_speed_3f = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            smash::app::lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            smash::app::lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &reset_speed_gravity_2f, &reset_speed_3f, fighter.module_accessor);
            smash::app::lua_bind::KineticEnergy::enable(stop_energy);
            smash::app::lua_bind::KineticEnergy::enable(gravity_energy);
            smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
    if [*FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4].contains(&current_status_kind) {
        if current_situation_kind == *SITUATION_KIND_GROUND {
            let reset_speed_2f = smash::phx::Vector2f { x: 0.0, y: 0.0 };
            let reset_speed_3f = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            smash::app::lua_bind::KineticEnergy::reset_energy(motion_energy, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS_IGNORE_NORMAL, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            smash::app::lua_bind::KineticEnergy::enable(motion_energy);
            smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else if current_situation_kind == *SITUATION_KIND_AIR {
            let reset_speed_2f = smash::phx::Vector2f { x: 0.0, y: 0.0 };
            let reset_speed_3f = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            smash::app::lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            smash::app::lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            smash::app::lua_bind::KineticEnergy::enable(stop_energy);
            smash::app::lua_bind::KineticEnergy::enable(gravity_energy);
        }
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    0.into()
}


#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    lucina_specials_reset_helper(fighter);
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::set_int( fighter.module_accessor, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS);
    if !StopModule::is_stop(fighter.module_accessor) {
        lucina_specials_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(lucina_specials_substatus as *const () as _));
    WorkModule::set_int64(fighter.module_accessor,
        hash40("special_s1") as i64,
        *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
    );
    WorkModule::set_int64(
        fighter.module_accessor,
        hash40("special_air_s1") as i64,
        *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specials_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            lucina_specials_mot_helper(fighter);
        }
    }
    else {
        lucina_specials_mot_helper(fighter);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    lucina_specials_status_change_helper(fighter);
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    lucina_specials_reset_helper(fighter);
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3,
        *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        lucina_specials_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(lucina_specials_substatus as *const () as _));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_LW) {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_s2_lw") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
        );
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_air_s2_lw") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
        );
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_HI) {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_s2_hi") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
        );
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_air_s2_hi") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
        );
    }
    else {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_s2_lw") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
        );
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_air_s2_lw") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
        );
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_HI);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_LW);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specials_main_loop as *const () as _))
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_LW) {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_s3_lw") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
        );
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_air_s3_lw") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
        );
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_HI) {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_s3_hi") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
        );
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_air_s3_hi") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
        );
    }
    else {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_s3_s") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
        );
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_air_s3_s") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
        );
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_HI);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_LW);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specials3_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_specials3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            lucina_specials_mot_helper(fighter);
        }
    }
    else {
        lucina_specials_mot_helper(fighter);
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

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STOP)]
unsafe fn lucina_specials3_exec_stop(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_specials_substatus(fighter: &mut L2CFighterCommon, _param_1: L2CValue) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_FAILURE) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_SUCCESS) {
            return 0.into();
        }
        if !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            return 0.into();
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_FAILURE);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_SUCCESS);
            let enable_hi_lw = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("enable_input_hi_lw"));
            if enable_hi_lw == 0 {
                return 0.into();
            }
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
            if stick_y > -squat_stick_y {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_HI);
            }
            else if stick_y < squat_stick_y {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_LW);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn lucina_specials_reset_helper(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_SUCCESS);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_FAILURE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
}

unsafe extern "C" fn lucina_specials_mot_helper(fighter: &mut L2CFighterCommon) {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    let correct;
    let mot;
    if sit != *SITUATION_KIND_GROUND {
        correct = *GROUND_CORRECT_KIND_AIR;
        mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
    }
    else {
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT) {
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
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
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

unsafe extern "C" fn lucina_specials_status_change_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_SUCCESS) {
        if !MotionModule::is_end(fighter.module_accessor) {
            return 0.into();
        }
        let sit = fighter.global_table[SITUATION_KIND].get_i32();
        if sit != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 0.into();
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE) {
            if !MotionModule::is_end(fighter.module_accessor) {
                return 0.into();
            }
            let sit = fighter.global_table[SITUATION_KIND].get_i32();
            if sit != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            return 0.into();
        }
        let status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS);
        fighter.change_status(status.into(), false.into());
    }
    1.into()
}


#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    VarModule::set_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION, hash40("special_lw"));
    VarModule::set_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION_AIR, hash40("special_air_lw"));
    special_lw_main_motion_helper(fighter);
    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_motion_helper(fighter: &mut L2CFighterCommon) {
    let situation = fighter.global_table[globals::SITUATION_KIND].get_i32();
    if situation != *SITUATION_KIND_GROUND {
        let mot = VarModule::get_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION_AIR);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0);
        }
    } 
    else {
        let mot = VarModule::get_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0);
        }
    }

}

unsafe extern "C" fn special_lw_check_follow_up(fighter: &mut L2CFighterCommon) {
    let stick_y = fighter.global_table[globals::STICK_Y].get_f32();
    let mot;
    let mot_air;
    if stick_y >= 0.5 {
        mot = hash40("special_s4_hi");
        mot_air = hash40("special_air_s4_hi");
    }
    else if stick_y <= -0.5 {
        mot = hash40("special_s4_lw");
        mot_air = hash40("special_air_s4_lw")
    }
    else {
        mot = hash40("special_s4_s");
        mot_air = hash40("special_air_s4_s")
    }
    let stick_x = fighter.global_table[globals::STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    if stick_x * lr < -0.33 {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    VarModule::set_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION, mot);
    VarModule::set_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION_AIR, mot_air);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    special_lw_main_motion_helper(fighter);
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor) {
        if VarModule::is_flag(fighter.battle_object, vars::lucina::status::SPECIAL_LW_SPECIAL_CHECK)
        && fighter.global_table[globals::PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
            special_lw_check_follow_up(fighter);
            VarModule::off_flag(fighter.battle_object,vars::lucina::status::SPECIAL_LW_SPECIAL_CHECK);
        }
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_lw_main_motion_helper(fighter);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

