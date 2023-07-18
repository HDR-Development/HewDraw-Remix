use super::*;
use globals::*;


// FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END

#[status_script(agent = "sheik", status = FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let fall_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_max_x_mul");
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * fall_x_mul,
            0.0
        );
    }          
    ret
}

pub fn install() {
    install_status_scripts!(
        special_hi_end_main
    );
}