use super::*;

// FIGHTER_STATUS_KIND_TURN_DASH

pub unsafe extern "C" fn pre_turndash(fighter: &mut L2CFighterCommon) -> L2CValue {
    app::FighterSpecializer_Demon::update_opponent_lr_1on1(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN_DASH);
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr != 0.0 {
        if PostureModule::lr(fighter.module_accessor) == lr {
            if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_STATUS_KIND_TURN {
                StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_DASH_BACK);
                return L2CValue::I32(1);
            }
        }
    }
    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN);
    return 1.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_TURN_DASH, pre_turndash);
}