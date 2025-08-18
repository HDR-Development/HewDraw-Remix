use super::{*, uniq_float_start::*};
use globals::*;

#[no_mangle]
unsafe fn peach_float_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_FALL_CONTROL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_ATTACK_AIR_CONTROL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_FALL_TIME);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_TRANS_ID_ATTACK_AIR_TIME);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_TRANS_ID_SPECIAL_LW_ITEM_THROW);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);

    WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_ENABLE_UNIQ);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("fuwafuwa"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    let shield_stiff_mul_attack_air = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_stiff_mul_attack_air"));
    AttackModule::set_shield_stiff_mul(fighter.module_accessor, shield_stiff_mul_attack_air);
    peach_float_check_aerial(fighter);

    fighter.sub_shift_status_main(L2CValue::Ptr(peach_float_main_loop_common as *const () as _))
}

// FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT

unsafe extern "C" fn uniq_float_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    peach_float_main_common(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, uniq_float_main);
}