use super::*;

// FIGHTER_STATUS_KIND_ATTACK_100

unsafe extern "C" fn attack_100_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_100_count_loop = fighter.get_param_int("param_special_hi", "special_hi_hundred_attack_point");
    let attack_100_count = fighter.get_int(*FIGHTER_REFLET_STATUS_ATTACK_INT_COUNT_FOR_ATTACK_100);
    if attack_100_count > 0 {
        fighter.dec_int(*FIGHTER_REFLET_STATUS_ATTACK_INT_COUNT_FOR_ATTACK_100);
    } else if fighter.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE) {
        if fighter.is_button_off(Buttons::AttackAll) || fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) == 0 {
            fighter.dec_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
            if fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
            fighter.off_flag(*FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE);
        } else {
            fighter.set_int(attack_100_count_loop, *FIGHTER_REFLET_STATUS_ATTACK_INT_COUNT_FOR_ATTACK_100);
            fighter.dec_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
        }
    }
    0.into()
}

unsafe extern "C" fn attack_100_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Attack100()//discard queue handled during status (not end) to play nice with discard mech
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_100, attack_100_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_100, attack_100_end);
}