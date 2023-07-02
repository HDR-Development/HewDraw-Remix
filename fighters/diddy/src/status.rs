use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        end_jump_squat
    );
}

// FIGHTER_STATUS_KIND_JUMP_SQUAT

#[status_script(agent = "diddy", status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn end_jump_squat(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_JumpSquat();
    0.into()
}