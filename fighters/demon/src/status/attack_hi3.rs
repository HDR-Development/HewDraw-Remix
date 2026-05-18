use super::*;

unsafe extern "C" fn demon_attack_hi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_CHECK_STEP);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_INC_STEP);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_WORK_INT_COMBO);

    fighter.clear_lua_stack();
    let motion = sv_fighter_util::get_attack_hi3_motion(fighter.lua_state_agent);
    fighter.status_AttackHi3_Common(motion.into(), Hash40::new("attack_hi3").into());

    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack3_uniq_check(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack3_uniq_check as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_hi3_main_loop as *const () as _))
}

unsafe extern "C" fn demon_attack_hi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackHi3_Main();

    if StatusModule::is_changing(fighter.module_accessor) {
        return 0.into();
    }

    let combo = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_WORK_INT_COMBO);
    if combo + 1 < 2
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_INC_STEP);
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_CHECK_STEP)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_INC_STEP) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_INC_STEP);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_CHECK_STEP);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_WORK_INT_COMBO);
        let rate = MotionModule::rate(fighter.module_accessor);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("attack_hi32"),
            0.0,
            rate,
            false,
            0.0,
            false,
            false
        );
        fighter.clear_lua_stack();
        sv_kinetic_energy::set_motion_energy_update_flag(fighter.lua_state_agent);
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI3, demon_attack_hi3_main);
}