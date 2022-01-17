use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        dash,
    );
}

// FIGHTER_STATUS_KIND_DASH //

#[status_script(agent = "popo", status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    ics_dash(fighter)
}