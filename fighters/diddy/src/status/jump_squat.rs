use super::*;

// FIGHTER_STATUS_KIND_JUMP_SQUAT

unsafe extern "C" fn jump_squat_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_JumpSquat();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_JUMP_SQUAT, jump_squat_end);
}