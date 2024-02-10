use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        init_specials,
        special_s_back
    );
}

pub fn set_gravity_delay_resume_frame(energy: *mut FighterKineticEnergyGravity, frames: i32) {
    unsafe {
      *(energy as *mut i32).add(0x50 / 4) = frames;
      *(energy as *mut bool).add(0x5C) = false;
    }
  }

#[status_script(agent = "roy", status = FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_back(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::roy::instance::REVERSE_SIDEB) {
        let lrc = PostureModule::lr(fighter.module_accessor);
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);

        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s4_back"), 0.0, 1.0, 0.0, false, false);
            VarModule::off_flag(fighter.battle_object, vars::roy::instance::REVERSE_SIDEB);

            if lrc == 1.0 && stick_x < 0.0 {
                PostureModule::set_lr(fighter.module_accessor, -1.0);
            } else if lrc == -1.0 && stick_x > 0.0 {
                PostureModule::set_lr(fighter.module_accessor, 1.0);
            }

            return 0.into();
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s4_back"), 0.0, 1.0, 0.0, false, false);
            VarModule::off_flag(fighter.battle_object, vars::roy::instance::REVERSE_SIDEB);

            if lrc == 1.0 && stick_x < 0.0 {
                PostureModule::set_lr(fighter.module_accessor, -1.0);
            } else if lrc == -1.0 && stick_x > 0.0 {
                PostureModule::set_lr(fighter.module_accessor, 1.0);
            }

            return 0.into();
        }
    }

    original!(fighter)
}

#[status_script(agent = "roy", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
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
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2].contains(&current_status_kind) {
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
    if [*FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4].contains(&current_status_kind) {
        if current_situation_kind == *SITUATION_KIND_GROUND {
            let reset_speed_2f = smash::phx::Vector2f { x: 0.0, y: 0.0 };
            let reset_speed_3f = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            smash::app::lua_bind::KineticEnergy::reset_energy(motion_energy, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS_IGNORE_NORMAL, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            smash::app::lua_bind::KineticEnergy::enable(motion_energy);
            smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

            if VarModule::is_flag(fighter.battle_object, vars::roy::instance::REVERSE_SIDEB) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s4_back"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        else if current_situation_kind == *SITUATION_KIND_AIR {
            let reset_speed_2f = smash::phx::Vector2f { x: 0.0, y: 0.0 };
            let reset_speed_3f = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            smash::app::lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            smash::app::lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
            smash::app::lua_bind::KineticEnergy::enable(stop_energy);
            smash::app::lua_bind::KineticEnergy::enable(gravity_energy);

            if VarModule::is_flag(fighter.battle_object, vars::roy::instance::REVERSE_SIDEB) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s4_back"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    0.into()
}