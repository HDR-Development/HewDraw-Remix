use super::*;

unsafe extern "C" fn attack_squat_2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let func = smashline::api::get_target_function("lua2cpp_demon.nrs", 0x2c450).unwrap();
    fighter.status_AttackLw3_common_param(
        true.into(),
        L2CValue::Ptr(func as *const () as _),
        (
            0x180 | // *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_LW3_W
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ).into()
    );

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("attack_squat_2"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, 0x18 /*FIGHTER_LOG_ATTACK_KIND_ATTACK_LW3_W*/);
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, false);

    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT_CLIFF_STOP);

    fighter.sub_shift_status_main(L2CValue::Ptr(attack_squat_2_main_loop as *const () as _))
}

unsafe extern "C" fn attack_squat_2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if demon_attack_loop_common(fighter, FIGHTER_STATUS_KIND_SQUAT_WAIT.into()).get_bool() {
        return 0.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_2.into(), true.into());
        return 1.into();
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_2, attack_squat_2_main);
}