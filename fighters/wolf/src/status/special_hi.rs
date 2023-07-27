use super::*;
use globals::*;


// FIGHTER_STATUS_KIND_SPECIAL_HI

#[status_script(agent = "wolf", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
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

// FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_BOUND

#[status_script(agent = "wolf", status = FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_BOUND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_hi_bound_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);

    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_frame"), 0);
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    
    ret
}

pub fn install() {
    install_status_scripts!(
        special_hi_main,
        special_hi_bound_end
    );
}