use super::*;

// FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND

pub unsafe extern "C" fn special_s_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }
    smashline::original_status(Init, fighter, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND)(fighter)
}

// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
}

// FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP

pub unsafe extern "C" fn special_s_loop_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Init, fighter, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP)(fighter);
    if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::EX_SPECIAL_USED) {
        let lr = PostureModule::lr(fighter.module_accessor);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_NONE, lr * 1.8, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        } else {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, lr * 1.5, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        }
    }
    return ret;
}

pub unsafe extern "C" fn special_s_loop_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    // reduce speed on hitting shield
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        let strength = fighter.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
        let speed_x = if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::EX_SPECIAL_USED) {
                1.8
            } else if strength == *FIGHTER_RYU_STRENGTH_S {
                fighter.get_param_float("param_special_s", "speed_x_s")
            } else if strength == *FIGHTER_RYU_STRENGTH_M {
                fighter.get_param_float("param_special_s", "speed_x_m")
            } else {
                fighter.get_param_float("param_special_s", "speed_x_w")
            }
        } else {
            if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::EX_SPECIAL_USED) {
                1.5
            } else if strength == *FIGHTER_RYU_STRENGTH_S {
                fighter.get_param_float("param_special_s", "air_speed_x_s")
            } else if strength == *FIGHTER_RYU_STRENGTH_M {
                fighter.get_param_float("param_special_s", "air_speed_x_m")
            } else {
                fighter.get_param_float("param_special_s", "air_speed_x_w")
            }
        };
        let lr = PostureModule::lr(fighter.module_accessor);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, lr * speed_x * 0.6, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    }
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, special_s_command_init);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, special_s_loop_init);
    agent.status(Exec, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, special_s_loop_exec);
}
