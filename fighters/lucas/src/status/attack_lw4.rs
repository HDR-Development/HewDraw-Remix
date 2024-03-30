use super::*;

// FIGHTER_STATUS_KIND_ATTACK_LW4

pub unsafe extern "C" fn attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw4_common();
    fighter.main_shift(status_attacklw4_main_param)
}

unsafe extern "C" fn status_attacklw4_main_param(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo = ComboModule::count(fighter.module_accessor) as i32;
        let lw4_combo_max = 2;
        if combo < lw4_combo_max && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)  {
            ComboModule::set(fighter.module_accessor, 2);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw4_2"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, attack_lw4_main);
}