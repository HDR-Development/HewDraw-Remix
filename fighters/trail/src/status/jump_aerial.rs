use super::*;

// FIGHTER_STATUS_KIND_JUMP_AERIAL

// preserve momentum if double jumping out of sonic blade
unsafe extern "C" fn jump_aerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x_speed = VarModule::get_float(fighter.battle_object, vars::trail::instance::JUMP_CANCEL_MOMENTUM_HANDLER);

    let original = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_JUMP_AERIAL);

    if fighter.is_prev_status(*FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: x_speed.abs() / 3.0, y: 0.0, z: 0.0});
    }

    original(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, jump_aerial_main);
}
