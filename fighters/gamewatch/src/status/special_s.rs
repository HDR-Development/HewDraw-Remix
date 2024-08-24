use super::*;

unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_PREV_KIND);
    let kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND);
    // println!("prev prev judge kind: {}", prev_kind);
    // println!("prev judge kind: {}", kind);

    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);

    // Generates RNG and makes sure it isn't the same as the previous two Judge values
    // RNG values are 0 indexed, so 0 > 1, 1 > 2, etc
    let mut rng = sv_math::rand(hash40("fighter"), 9);
    while rng == prev_kind || rng == kind {
        rng = sv_math::rand(hash40("fighter"), 9);
    }

    // Enables the TABEMONO flag if RNG is equal to 6 (7, but it's 0 indexed)
    WorkModule::set_flag(fighter.module_accessor, rng == 6, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT_TABEMONO);
    WorkModule::set_int(fighter.module_accessor, kind, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_PREV_KIND);
    WorkModule::set_int(fighter.module_accessor, rng, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND);
    WorkModule::set_int(fighter.module_accessor, rng, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);

    VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("panel") as i64, hash40(&format!("no_{}", rng + 1)) as i64);

    // Makes sure we aren't showing the actual number on the first frame.
    let mut rng_dummy = sv_math::rand(hash40("fighter"), 9);
    VisibilityModule::set_int64(fighter.module_accessor, hash40("panel") as i64, hash40(&format!("no_{}", rng_dummy + 1)) as i64);

    WorkModule::set_int64(
        fighter.module_accessor,
        hash40(&format!("special_s_{}", rng + 1)) as i64,
        *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
    );
    WorkModule::set_int64(
        fighter.module_accessor,
        hash40(&format!("special_air_s_{}", rng + 1)) as i64,
        *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
    );
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
}
