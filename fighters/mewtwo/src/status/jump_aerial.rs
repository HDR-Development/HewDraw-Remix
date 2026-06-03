use super::*;

// FIGHTER_STATUS_KIND_JUMP_AERIAL

unsafe extern "C" fn jump_aerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(consume_resource as *const () as _));
    fighter.status_JumpAerial();
    0.into()
}

unsafe extern "C" fn consume_resource(fighter: &mut L2CFighterCommon) -> L2CValue {
    let buffer = ControlModule::get_command_life_count_max(fighter.module_accessor) as i32;
    if fighter.global_table[CURRENT_FRAME].get_i32() == buffer {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::DISABLE_FLOAT);
    }
    0.into()
}

unsafe extern "C" fn jump_aerial_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != statuses::mewtwo::FLOAT {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::DISABLE_FLOAT);
    }
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_JUMP_AERIAL)(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, jump_aerial_main);
    agent.status(End, *FIGHTER_STATUS_KIND_JUMP_AERIAL, jump_aerial_end);
}