use super::*;

unsafe extern "C" fn special_n_dir_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR)(fighter);

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_status_func as *const () as _));  

    ret
}

unsafe extern "C" fn special_n_turn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN)(fighter);

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_status_func as *const () as _));  

    ret
}

unsafe extern "C" fn sub_status_func(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {  
    if !param_1.get_bool() {  
        fighter.inc_int(*FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_CHARGE);  
    }  
    return false.into();
}  

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR, special_n_dir_main);
    agent.status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN, special_n_turn_main);
}