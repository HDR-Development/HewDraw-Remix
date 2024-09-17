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
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_NONE, lr * 1.9, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        } else {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, lr * 1.5, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        }
    }
    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, special_s_command_init);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, special_s_loop_init);
}
