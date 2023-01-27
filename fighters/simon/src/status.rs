use super::*;
use globals::*;

pub fn install() {
    install_status_scripts!(
        attack_air_main,
        attack_hi3_main,
        attack_hi4_main,
        attack_lw4_main
    );
}

#[status_script(agent = "simon", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 0.into();
    }
    original!(fighter)
}

#[status_script(agent = "simon", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_hi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 0.into();
    }
    original!(fighter)
}

#[status_script(agent = "simon", status = FIGHTER_STATUS_KIND_ATTACK_HI4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_hi4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 0.into();
    }
    original!(fighter)
}

#[status_script(agent = "simon", status = FIGHTER_STATUS_KIND_ATTACK_LW4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 0.into();
    }
    original!(fighter)
}