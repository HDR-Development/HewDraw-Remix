use super::*;
use globals::*;
utils::import!(popo::{ics_dash, ics_cheer});
// status script import
 
pub fn install() {
    install_status_scripts!(
        dash,
        cheer
    );
}

// FIGHTER_STATUS_KIND_DASH //

#[status_script(agent = "nana", status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    popo::ics_dash(fighter)
}

// FIGHTER_POPO_STATUS_KIND_CHEER_NANA //

#[status_script(agent = "nana", status = FIGHTER_POPO_STATUS_KIND_CHEER_NANA, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn cheer(fighter: &mut L2CFighterCommon) -> L2CValue {
    popo::ics_cheer(fighter)
}