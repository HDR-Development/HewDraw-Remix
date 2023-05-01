use super::*;
use globals::*;


#[status_script(agent = "falco", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.0
    );
    ret
}

pub fn install() {
    install_status_scripts!(
        special_hi_main
    );
}