use super::*;
use globals::*;


// FIGHTER_STATUS_KIND_SPECIAL_HI

#[status_script(agent = "fox", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
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

// FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END

#[status_script(agent = "fox", status = FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_rush_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    ret
}

pub fn install() {
    install_status_scripts!(
        special_hi_main,
        special_hi_rush_end_main
    );
}