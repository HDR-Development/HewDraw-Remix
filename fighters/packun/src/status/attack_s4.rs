use super::*;
use globals::*;

#[status_script(agent = "packun", status = FIGHTER_STATUS_KIND_ATTACK_S4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_s4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS4Start();
    if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s_2"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackS4Start_Main as *const () as _))
}

#[status_script(agent = "packun", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_attack_s4(fighter, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackS4_Main as *const () as _))
}

unsafe extern "C" fn sub_attack_s4(fighter: &mut L2CFighterCommon, param_1: bool) {
    let hash = if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 { hash40("attack_s4_s_2") } else { hash40("attack_s4_s") };
    WorkModule::set_int64(fighter.module_accessor, hash as i64, *FIGHTER_STATUS_ATTACK_WORK_INT_MOTION_KIND);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
}

#[status_script(agent = "packun", status = FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_s4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_smash_hold_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_smash_hold_uniq as *const () as _));
    let hold_frame = WorkModule::get_param_float(fighter.module_accessor, 0x1351539e6d, 0);
    let s4_hold_frame = WorkModule::get_param_int(fighter.module_accessor, 0x14d34d14d0, 0);
    let ratio = (s4_hold_frame as f32 / hold_frame);
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_BASE_FRAME);
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME);
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_TOTAL_FRAME);
    let keep_frame = WorkModule::get_param_int(fighter.module_accessor, 0x192bdc7824, 0);
    WorkModule::set_int(fighter.module_accessor, keep_frame, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME);
    if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_hold_2"), 0.0, s4_hold_frame as f32 / ratio, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_hold"), 0.0, s4_hold_frame as f32 / ratio, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackS4Hold_main as *const () as _))
}

pub fn install() {
    install_status_scripts!(
        attack_s4_start_main,
        attack_s4_main,
        attack_s4_hold_main,
    );
}