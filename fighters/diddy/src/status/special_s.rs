use super::*;
use globals::*;


// FIGHTER_STATUS_KIND_SPECIAL_S //

#[status_script(agent = "diddy", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::plizardon::instance::DISABLE_SPECIAL_S);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        init_special_s
    );
}