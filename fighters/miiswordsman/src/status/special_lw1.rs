use super::*;

//FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT

unsafe extern "C" fn special_lw1_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if StatusModule::situation_kind(fighter.module_accessor) == WorkModule::get_int(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV) {
            return L2CValue::I32(0)
        }
        let speed = KineticModule::get_sum_speed3f(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {
            WorkModule::set_int(fighter.module_accessor,*SITUATION_KIND_GROUND,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV);
        }
        else {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw1_hit") || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw1_hit") {
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_STOP,ENERGY_STOP_RESET_TYPE_AIR,speed.x,0.0,0.0,0.0,0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                let brake_x = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_start_air_acl_x"));
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_STOP,brake_x,0.0);
                app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_STOP);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_GRAVITY,ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,speed.y,0.0,0.0,0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                let accel_y = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_attack_acl_y"));
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_GRAVITY,accel_y);
                app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);
                let stable_y = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_attack_max_y"));
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_GRAVITY,stable_y);
                app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);
                KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
                WorkModule::set_int(fighter.module_accessor,*SITUATION_KIND_AIR,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV);
            }
            else{
                //fighter.clear_lua_stack();
                //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR,speed.x,0.0,0.0,0.0,0.0);
                //app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                //let brake_x = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_start_air_acl_x"));
                //fighter.clear_lua_stack();
                //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP,brake_x,0.0);
                //app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);
                //fighter.clear_lua_stack();
                //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
                //app::sv_kinetic_energy::enable(fighter.lua_state_agent);
                //fighter.clear_lua_stack();
                //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,speed.y,0.0,0.0,0.0);
                //app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                //let accel_y = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_attack_acl_y"));
                //fighter.clear_lua_stack();
                //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY,accel_y);
                //app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);
                let stable_y = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_attack_max_y"));
                //fighter.clear_lua_stack();
                //lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_GRAVITY,stable_y);
                //app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
                //lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                //app::sv_kinetic_energy::enable(fighter.lua_state_agent);
                KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                fighter.clear_lua_stack();
                lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_MOTION);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);
                KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_MOTION);
                WorkModule::set_int(fighter.module_accessor,*SITUATION_KIND_AIR,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV);
            }
        }
    }
    return L2CValue::I32(0)
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT, special_lw1_hit_main);
}