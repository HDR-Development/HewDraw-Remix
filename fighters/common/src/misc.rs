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
    //skyline::install_hooks!(
    //    set_hit_team_hook,
    //);
}

/*#[skyline::hook(replace=TeamModule::set_hit_team)]
unsafe fn set_hit_team_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    if (boma.kind() == *ITEM_KIND_BARREL) {
        return;
    }
}*/

#[smashline::fighter_reset]
pub fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let ratio = (WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x_max"), 0) / WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0));
        VarModule::set_float(fighter.battle_object, vars::common::instance::JUMP_SPEED_RATIO, ratio);
        if fighter.kind() == *FIGHTER_KIND_KEN || fighter.kind() == *FIGHTER_KIND_RYU || fighter.kind() == *FIGHTER_KIND_DOLLY {
            MeterModule::reset(fighter.battle_object);
        }
    }

}

pub unsafe fn calc_melee_momentum(fighter: &mut L2CFighterCommon, aerial_attack: bool, attack_cancel: bool, walking: bool) -> f32 {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let jump_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x"), 0);
    let jump_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x_mul"), 0);
    let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);
    let ratio = VarModule::get_float(fighter.battle_object, vars::common::instance::JUMP_SPEED_RATIO);
    //println!("run_speed_max: {}", run_speed_max);
    //println!("jump_speed_ratio: {}", ratio); 

    // get the multiplier for any special mechanics that require additional jump speed max (meta quick, etc)
    let mut jump_speed_max_mul = VarModule::get_float(fighter.object(), vars::common::instance::JUMP_SPEED_MAX_MUL);
    match jump_speed_max_mul {
        // if its not between 0.1 and 3.0, it is likely not a real value and we should ignore it
        0.1..=3.0 => {},
        _ => { jump_speed_max_mul = 1.0 }
    }

    let jump_speed_x_max = run_speed_max * ratio * jump_speed_max_mul;

    let mut x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);

    if fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_SQUAT) {
        //println!("Jumpsquat momentum...");
        x_vel = VarModule::get_float(fighter.battle_object, vars::common::instance::JUMPSQUAT_VELOCITY);
        //println!("x_vel: {}", x_vel);
    }

    if fighter_kind == *FIGHTER_KIND_PICKEL && fighter.is_status_one_of(&[*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT]) {
        x_vel = VarModule::get_float(fighter.battle_object, vars::common::instance::JUMPSQUAT_VELOCITY);
    }

    if fighter_kind == *FIGHTER_KIND_TANTAN && fighter.is_status_one_of(&[*FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT]) {
        x_vel = VarModule::get_float(fighter.battle_object, vars::common::instance::JUMPSQUAT_VELOCITY);
    }

    //println!("current velocity: {}", x_vel);

    let mut calcJumpSpeed = (x_vel * jump_speed_x_mul) + (jump_speed_x * fighter.left_stick_x());  // Calculate jump momentum based on the momentum you had on the last frame of jumpsquat

    //println!("calcJumpSpeed: {}", calcJumpSpeed);

    let jumpSpeedClamped = calcJumpSpeed.clamp(-jump_speed_x_max, jump_speed_x_max);  //melee jump speed calculation... courtesey of Brawltendo

    //println!("jumpSpeedClamped: {}", jumpSpeedClamped);

    jumpSpeedClamped
}