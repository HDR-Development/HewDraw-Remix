use super::*;

unsafe extern "C" fn demon_attack_step_2s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("attack_step_2s"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_14);
    ControlModule::reset_special_command(fighter.module_accessor, true);
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, false);
    let rage_system = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM);
    WorkModule::set_flag(fighter.module_accessor, rage_system, *FIGHTER_DEMON_STATUS_ATTACK_STEP_FLAG_RAGE_SYSTEM);

    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_step_2s_main_loop as *const () as _))
}

unsafe extern "C" fn demon_attack_step_2s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if VarModule::is_flag(fighter.battle_object, vars::demon::status::ATTACK_STEP2S_ENABLE_CANCEL) {
        if fighter.is_cat_flag(Cat4::Command623NB) {
            fighter.change_status(statuses::demon::CANCEL_STEP.into(), false.into());
            return 0.into();
        }
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STEP_FLAG_CANCEL);
            return 0.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S, demon_attack_step_2s_main);
}