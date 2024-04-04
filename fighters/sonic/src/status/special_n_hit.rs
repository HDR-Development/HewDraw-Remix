use super::*;

// FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT

// Randomized Homing Attack Poses

pub unsafe extern "C" fn special_n_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    let ret = smashline::original_status(Main, fighter, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT)(fighter);
    
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

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, special_n_hit_main);
}
