use super::*;
use globals::*;
 

pub fn install() {
    install_status_scripts!(
        special_n_hold_wait,
        special_n_hold_dash_f,
        special_n_hold_dash_b,
        special_n_hold_walk_f,
        special_n_hold_walk_b,
        special_n_hold_landing,
        special_n_hold_jump_squat,
        special_n_hold_walk_brake_f,
        special_n_hold_walk_brake_b
    );
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_hold_wait(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_WAIT {
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    original!(fighter)
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_DASH_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_hold_dash_f(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_WAIT {
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    original!(fighter)
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_DASH_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_hold_dash_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_WAIT {
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    original!(fighter)
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_hold_walk_f(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_WAIT {
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    original!(fighter)
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_hold_walk_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_WAIT {
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    original!(fighter)
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_hold_landing(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_WAIT {
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    original!(fighter)
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_hold_jump_squat(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_WAIT {
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    original!(fighter)
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_BRAKE_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_hold_walk_brake_f(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_WAIT {
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    original!(fighter)
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_BRAKE_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_hold_walk_brake_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_WAIT {
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    original!(fighter)
}

