use super::*;
use globals::*;


// FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH

#[status_script(agent = "edge", status = FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);

    // Grounded Blade Rush shorten mechanic
    let dir_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_DIR_X);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    && dir_x != 0.0 {
        let rush_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_speed"));
        let stopEnergy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergyNormal;
        let vec2 = Vector2f{x: rush_speed * dir_x, y: 0.0};
        app::lua_bind::KineticEnergyNormal::set_speed(stopEnergy, &vec2);
    }

    ret
}

pub fn install() {
    install_status_scripts!(
        special_hi_rush_main
    );
}