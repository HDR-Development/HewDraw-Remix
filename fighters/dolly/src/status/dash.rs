use super::*;

// FIGHTER_STATUS_KIND_DASH

pub unsafe extern "C" fn dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DASH)(fighter);
    if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_DASH_CANCEL) {
        MeterModule::drain(fighter.battle_object, 1);
    }
    return ret;
}

// FIGHTER_STATUS_KIND_TURN_DASH

pub unsafe extern "C" fn turn_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1,
    );
    if lr != 0.0 {
        if PostureModule::lr(fighter.module_accessor) == lr {
            if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_STATUS_KIND_TURN {
                StatusModule::set_status_kind_interrupt(
                    fighter.module_accessor,
                    *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK,
                );
                return L2CValue::I32(1);
            }
        }
    }
    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN);
    return 1.into();
}

// FIGHTER_DOLLY_STATUS_KIND_DASH_BACK

pub unsafe extern "C" fn dash_back_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = fgc_dashback_main(fighter);
    if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_DASH_CANCEL) {
        MeterModule::drain(fighter.battle_object, 1);
    }
    return ret;
}

pub unsafe extern "C" fn dash_back_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::shoto_status::fgc_end_dashback(fighter);
    smashline::original_status(End, fighter, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_DASH, dash_main);
    agent.status(Pre, *FIGHTER_STATUS_KIND_TURN_DASH, turn_dash_pre);
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, dash_back_main);
    agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, dash_back_end);
}