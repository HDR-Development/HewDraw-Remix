use super::*;
use globals::*;

unsafe extern "C" fn metaknight_attack100_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_Attack100_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_attack100_main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_attack100_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if 0 < jump_attack_frame {
        if !StopModule::is_stop(fighter.module_accessor)
        && fighter.sub_check_button_jump().get_bool() {
            let log = fighter.status_attack();
            let info = log[0x10f40d7b92u64].get_i64();
            let mot = MotionModule::motion_kind(fighter.module_accessor);
            MotionAnimcmdModule::call_script_single(
                fighter.module_accessor,
                *FIGHTER_ANIMCMD_EXPRESSION,
                Hash40::new_raw(mot),
                -1
            );
            WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 0.into();
        }
    }
    if 1 == jump_attack_frame {
        if !fighter.global_table[IS_STOPPING].get_bool()
        && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
            let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_STEP);
    if step == *FIGHTER_STATUS_ATTACK_100_STEP_START {
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.attack_100_start_uniq_chk(false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_attack_100_start_uniq_chk as *const () as _));
        let motion = MotionModule::motion_kind(fighter.module_accessor);
        if motion == hash40("attack_100_start")
        && !MotionModule::is_end(fighter.module_accessor) {
            return 0.into();
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_INPUT);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("attack_100"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_100_STEP_LOOP, *FIGHTER_STATUS_ATTACK_WORK_INT_100_STEP);
    }
    else if step == *FIGHTER_STATUS_ATTACK_100_STEP_LOOP {
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.sub_attack_100_uniq_check(false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_attack_100_uniq_check as *const () as _));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE) {
            return 0.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        //if MotionModule::is_end(fighter.module_accessor) {
        
        //}
    }
    0.into()
}

pub fn install() {
    smashline::Agent::new("metaknight")
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_100, metaknight_attack100_main)
        .install();
}