use super::*;
use globals::*;
// status script import

// FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP //

pub unsafe extern "C" fn special_hi_jump_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_hi_fail_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

pub fn install_popo(agent: &mut Agent) {
    agent.status(Exit, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_exit);
}

/// NANA

unsafe extern "C" fn select_throw_motion(fighter: &mut L2CFighterCommon) -> Hash40 {
    let (mag, rad ) = fighter.stick_polar();
    
    // if not holding the stick, it's random
    if mag.abs() == 0.0 {
        let rand = app::sv_math::rand(hash40("fighter"), 4);
        return match rand {
            0 => Hash40::new("throw_f"),
            1 => Hash40::new("throw_b"),
            2 => Hash40::new("throw_hi"),
            _ => Hash40::new("throw_lw")
        }
    }

    // else, use the stick position
    let lr = PostureModule::lr(fighter.boma());
    return match rad {
        r if r.sin() > r.cos().abs() => Hash40::new("throw_hi"),
        r if r.sin() < -r.cos().abs() => Hash40::new("throw_lw"),
        r if r.cos() * lr > 0.0 => Hash40::new("throw_f"),
        _ => Hash40::new("throw_b"),
    }
}

pub unsafe extern "C" fn throw_nana(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = select_throw_motion(fighter);
    MotionModule::change_motion(fighter.boma(), motion, 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Throw_Main as *const () as _))
}

unsafe extern "C" fn nana_catch_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_THROW);
    return true.into();
}

unsafe extern "C" fn nana_catch_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(nana_catch_wait_main_loop as *const () as _))
}

unsafe extern "C" fn nana_catch_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::set_frame(fighter.module_accessor, MotionModule::end_frame(fighter.module_accessor), true);
    fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
    return true.into();
}

unsafe extern "C" fn nana_catch_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_THROW);
    return true.into();
}

unsafe extern "C" fn nana_catch_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(nana_catch_wait_main_loop as *const () as _))
}

unsafe extern "C" fn nana_catch_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::set_frame(fighter.module_accessor, MotionModule::end_frame(fighter.module_accessor), true);
    fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
    return true.into();
}

unsafe extern "C" fn popo_status_kind_throw_nana_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::set_frame(fighter.module_accessor, MotionModule::end_frame(fighter.module_accessor), true);
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT);
    return true.into();
}

unsafe extern "C" fn popo_status_kind_throw_nana_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(popo_status_kind_throw_nana_main_loop as *const () as _))
}

unsafe extern "C" fn popo_status_kind_throw_nana_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::set_frame(fighter.module_accessor, MotionModule::end_frame(fighter.module_accessor), true);
    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    return true.into();
}

pub fn install_nana(agent: &mut Agent) {
    agent.status(Exit, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_exit);
    agent.status(Pre, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL, special_hi_fail_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_THROW, throw_nana);
    agent.status(Pre, *FIGHTER_STATUS_KIND_CATCH_WAIT, nana_catch_wait_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, nana_catch_wait_main);
    agent.status(Pre, *FIGHTER_STATUS_KIND_CATCH_ATTACK, nana_catch_attack_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, nana_catch_attack_main);
    agent.status(Pre, *FIGHTER_POPO_STATUS_KIND_THROW_NANA, popo_status_kind_throw_nana_pre);
    agent.status(Main, *FIGHTER_POPO_STATUS_KIND_THROW_NANA, popo_status_kind_throw_nana_main);
}