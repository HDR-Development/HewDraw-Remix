use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        pre_special_hi_escape
    );
}

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE //

#[status_script(agent = "koopajr", status = FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_special_hi_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR);
    return 1.into()
}