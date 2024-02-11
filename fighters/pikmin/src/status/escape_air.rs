use super::*;
use globals::*;


pub fn install() {
    install_status_scripts!(
        escape_air_end
    );
}

#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn escape_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::pikmin::instance::SPECIAL_HI_CANCEL_ESCAPE_AIR);
    fighter.status_end_EscapeAir()
}