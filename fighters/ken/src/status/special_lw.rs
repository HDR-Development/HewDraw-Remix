use super::*;
use globals::*;
use smashline::*;

pub fn install() {
    install_status_scripts!(
        init_special_lw,
    );
}

// FIGHTER_STATUS_KIND_SPECIAL_LW //

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_special_lw(fighter: &mut L2CFighterCommon) -> L2CValue {
    // once-per-airtime (refreshes on hit)
    VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_LW);
    original!(fighter)
}