use super::*;

// FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END

unsafe extern "C" fn special_hi_air_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StopModule::is_stop(fighter.module_accessor) {
        special_hi_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_hi_substatus as *const () as _));
    smashline::original_status(Main, fighter, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END)(fighter)
}

unsafe extern "C" fn special_hi_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && param_1.get_bool() {
        fighter.sub_air_check_dive();
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END, special_hi_air_end_main);
}
