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
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let fall_x_mul = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("param_special_hi"),
            hash40("fire_fall_x_mul")
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * fall_x_mul,
            0.0
        );
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_HI as u32); 
    }          
    ret
}

// FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND

#[status_script(agent = "fox", status = FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_hi_bound_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);

    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_frame"), 0);
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    
    ret
}

pub fn install() {
    install_status_scripts!(
        special_hi_main,
        special_hi_rush_end_main,
        special_hi_bound_end
    );
}