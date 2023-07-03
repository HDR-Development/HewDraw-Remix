use super::*;
use globals::*;

 
pub fn install() {
    install_status_scripts!(
        special_n_charge_end,
    );
}

#[status_script(agent = "bayonetta", status = FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_charge_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_GUARD_ON {
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    original!(fighter)
}