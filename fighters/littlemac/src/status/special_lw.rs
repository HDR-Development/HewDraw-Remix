use super::*;
use globals::*;

#[status_script(agent = "littlemac", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if fighter.is_situation(*SITUATION_KIND_GROUND) {
    //     sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    // }
    // else {
    //     sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    // }
    fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START.into(), false.into());
    return 1.into()
}

pub fn install() {
    install_status_scripts!(
        special_lw_main
    );
}