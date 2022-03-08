use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::app::*;
use smash::lib::lua_const::*;
use smash::hash40;
use utils::ext::*;
use utils::consts::*;
use utils::*;

use globals::*;

pub fn install() {
    smashline::install_agent_resets!(fighter_reset);
}

#[smashline::fighter_reset]
pub fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let ratio = (WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x_max"), 0) / WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0));
        VarModule::set_float(fighter.battle_object, vars::common::JUMP_SPEED_RATIO, ratio);
        if fighter.kind() == *FIGHTER_KIND_KEN || fighter.kind() == *FIGHTER_KIND_RYU || fighter.kind() == *FIGHTER_KIND_DOLLY {
            MeterModule::reset(fighter.battle_object);
        }
    }

}

pub unsafe fn calc_melee_momentum(fighter: &mut L2CFighterCommon, aerial_attack: bool, attack_cancel: bool, walking: bool) -> f32 {
  let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
  let jump_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x"), 0);
  let jump_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x_mul"), 0);
  let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
  let dash_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("dash_speed"), 0);
  let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);
  let walk_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_speed_max"), 0);
  let js_frames = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_squat_frame"), 0);
  let traction = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
  let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
  let ratio = VarModule::get_float(fighter.battle_object, vars::common::JUMP_SPEED_RATIO);
  //println!("run_speed_max: {}", run_speed_max);
  //println!("jump_speed_ratio: {}", ratio); 

  let jump_speed_x_max = run_speed_max * ratio;

  let is_speed_backward = (KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN)) * PostureModule::lr(fighter.module_accessor) < 0.0;

  let mut x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);

  if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
      //println!("Jumpsquat momentum...");
      x_vel = VarModule::get_float(fighter.battle_object, vars::common::JUMPSQUAT_VELOCITY);
      //println!("x_vel: {}", x_vel);
  }

  if fighter_kind == *FIGHTER_KIND_PICKEL && [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
      x_vel = VarModule::get_float(fighter.battle_object, vars::common::JUMPSQUAT_VELOCITY);
  }

  if fighter_kind == *FIGHTER_KIND_TANTAN && [*FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
      x_vel = VarModule::get_float(fighter.battle_object, vars::common::JUMPSQUAT_VELOCITY);
  }

  //println!("jumpsquat velocity: {}", x_vel);
  //println!("current velocity: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));

  let mut calcJumpSpeed = (x_vel * jump_speed_x_mul) + (jump_speed_x * stick_x);  // Calculate jump momentum based on the momentum you had on the last frame of jumpsquat

  // Helper momentum for attack cancel aerials
  if attack_cancel {
      //println!("Attack Cancel! Calculated jump speed so far: {}", calcJumpSpeed);
      // If Attack Cancel RAR
      if    [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 2)) {
          //println!("Initial Dash Attack Cancel");
          /*
          if dash_speed > run_speed_max{
              calcJumpSpeed = ((jump_speed_x * x_vel/dash_speed) + (jump_speed_x_mul * x_vel));
          }
          else{
              calcJumpSpeed = ((jump_speed_x * x_vel/run_speed_max) + (jump_speed_x_mul * x_vel));
          }
          */

      }
      /*
      if walking{
          calcJumpSpeed = ((jump_speed_x * x_vel/walk_speed_max) + (jump_speed_x_mul * x_vel));
      }
      */
  }

  //println!("calcJumpSpeed: {}", calcJumpSpeed);

  let jumpSpeedClamped = calcJumpSpeed.clamp(-jump_speed_x_max, jump_speed_x_max);  //melee jump speed calculation... courtesey of Brawltendo

  //println!("jumpSpeedClamped: {}", jumpSpeedClamped);

  jumpSpeedClamped
}