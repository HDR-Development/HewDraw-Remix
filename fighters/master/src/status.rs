use super::*;
use globals::*;

pub fn install() {
    install_status_scripts!(
        special_n_max_shoot
    );
}

#[status_script(agent = "master", status = FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_MAX_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_n_max_shoot(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 0.into();
    }
    original!(fighter)
}