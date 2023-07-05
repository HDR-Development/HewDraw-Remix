use super::*;
use globals::*;
utils::import!(common::djc::attack_air_main_status);
// status script import
 
pub fn install() {
    install_status_scripts!(
        attack_air
    );
}

// FIGHTER_STATUS_KIND_ATTACK_AIR //

#[status_script(agent = "mewtwo", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::djc::attack_air_main_status(fighter)
}