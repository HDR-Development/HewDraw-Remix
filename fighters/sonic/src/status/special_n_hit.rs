use super::*;
use globals::*;
use smashline::*;

pub fn install() {
  install_status_scripts!(
    special_n_hit_main
  );
}

// Randomized Homing Attack Poses
#[status_script(agent = "sonic", status = FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    let ret = original!(fighter);
    
    let hit_pose: [&str; 8] = [
        "special_n_hit1",
        "special_n_hit2",
        "special_n_hit3",
        "special_n_hit4",
        "special_n_hit5",
        "special_n_hit6",
        "special_n_hit7",
        "special_n_hit8"
    ];

    let pose_number = VarModule::get_int(fighter.battle_object, vars::sonic::instance::SPECIAL_N_POSE);

    if pose_number != 0 {
        let pose_hash = Hash40::new(hit_pose[(pose_number - 1) as usize]);
        MotionModule::change_motion(fighter.module_accessor, pose_hash, 0.0, 1.0, false, 0.0, false, false);
    }

    if pose_number != 8 {
      VarModule::inc_int(fighter.battle_object, vars::sonic::instance::SPECIAL_N_POSE);
    } else {
      VarModule::set_int(fighter.battle_object, vars::sonic::instance::SPECIAL_N_POSE, 0);
    }

    ret
}