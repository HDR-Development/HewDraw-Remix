use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        init_specials,
        special_lw_main
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
            let reset_speed_2f = smash::phx::Vector2f { x: aerial_x_speed, y: aerial_y_speed };
            let reset_speed_gravity_2f = smash::phx::Vector2f { x: 0.0, y: 0.0 };
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
    if VarModule::is_flag(fighter.battle_object, vars::lucina::status::SPECIAL_LW_SPECIAL_CHECK)
    && fighter.global_table[globals::PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
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
        VarModule::set_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION, mot);
        VarModule::set_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION_AIR, mot_air);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        special_lw_main_motion_helper(fighter);
        let stick_x = fighter.global_table[globals::STICK_X].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        if stick_x < -0.33 {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
        VarModule::off_flag(fighter.battle_object,vars::lucina::status::SPECIAL_LW_SPECIAL_CHECK);
    }

}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    special_lw_check_follow_up(fighter);

    if !StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        special_lw_main_motion_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.boma().is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if fighter.boma().is_situation(*SITUATION_KIND_AIR) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

