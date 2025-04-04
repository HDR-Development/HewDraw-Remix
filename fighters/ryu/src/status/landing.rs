use super::*;

pub unsafe extern "C" fn landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
        if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_RYU_STATUS_KIND_FINAL_HIT {
            fighter.sub_landing_uniq_process_init_main(L2CValue::new_int(0xd6c194559));
            return false.into();
        }
        if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            if StatusModule::prev_status_kind(fighter.module_accessor, 1) == *FIGHTER_RYU_STATUS_KIND_FINAL_HIT {
                fighter.sub_landing_uniq_process_init_main(L2CValue::new_int(0xd6c194559));
                return false.into();
            }
        }
    }
    fighter.sub_landing_uniq_process_init();
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING, landing_init);
    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING_LIGHT, landing_init);
    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, landing_init);
}
