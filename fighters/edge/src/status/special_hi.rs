use super::*;

pub unsafe extern "C" fn special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH)(fighter);

    // Grounded Blade Rush shorten mechanic
    let dir_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_DIR_X);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    && dir_x != 0.0 {
    //&& (dir_x != 0.0 || VarModule::is_flag(fighter.battle_object, vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX)){
        let rush_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_speed"));
        let stopEnergy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergyNormal;
        let vec2 = Vector2f{x: rush_speed * dir_x, y: 0.0};
        // let mut movement_mul = 1.0;
        // if VarModule::is_flag(fighter.battle_object, vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX) {
        //     movement_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "blade_dash.fakeout_dash_movement_mul");;
        // }
        // let vec2 = Vector2f{x: rush_speed * dir_x * movement_mul, y: 0.0};
        app::lua_bind::KineticEnergyNormal::set_speed(stopEnergy, &vec2);
    }

    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_main);
}