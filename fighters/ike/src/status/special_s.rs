use super::*;
use globals::*;

// FIGHTER_STATUS_KIND_SPECIAL_S //

#[status_script(agent = "ike", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        // Quick Draw once-per-airtime (refreshes on hit)
        VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
    }
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        init_special_s
    );
}