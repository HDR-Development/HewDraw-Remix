use super::*;
use globals::*;
use smashline::*;

pub fn install() {
    smashline::Agent::new("ken")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, pre_special_n)
        .status(Pre, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND, pre_special_n_command)
        .install();
}

// FIGHTER_STATUS_KIND_SPECIAL_N //

pub unsafe extern "C" fn pre_special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND);
        return 1.into();
    }
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
}

// FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND //

pub unsafe extern "C" fn pre_special_n_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND);
        return 1.into();
    }
    smashline::original_status(Init, fighter, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND)(fighter)
}

