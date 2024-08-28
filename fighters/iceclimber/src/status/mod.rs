use super::*;
use globals::*;
// status script import

// FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP //

pub unsafe extern "C" fn special_hi_jump_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn throw_nana(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_near_cliff = GroundModule::is_near_cliff(fighter.boma(), 30.0, 30.0);
    let pos = *PostureModule::pos(fighter.module_accessor);
    let is_under_platform = GroundModule::ray_check(
        fighter.module_accessor, 
        &Vector2f{ x: pos.x, y: pos.y + 38.0}, 
        &Vector2f{ x: 0.0, y: -26.0},
        true
    ) == 1;
    let motion = if is_near_cliff {
        // TODO: this check assumes that the direction of ledge is always outwards,
        // and that the mathematical origin is contained within the stage.
        // It will fail if grabbing from a platform that's past ledge, 
        // or if grabbing on a stage that has been shifted far horizontally in lvd.
        if PostureModule::lr(fighter.boma()) == GroundModule::get_center_pos(fighter.boma()).signum() {
            Hash40::new("throw_f")
        } else {
            Hash40::new("throw_b")
        }
    } else if is_under_platform {
        Hash40::new("throw_hi")
    } else {
        let selected = app::sv_math::rand(hash40("fighter"), 100);
        match selected {
            0..=49 => Hash40::new("throw_f"),
            _ => Hash40::new("throw_lw")
        }
    };

    // change into the selected motion
    MotionModule::change_motion(fighter.boma(), motion, 0.0, 1.0, false, 0.0, false, false);

    // shift into the L2CFighterCommon's throw impl (instead of nana's default, modified impl)
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

pub fn install_popo(agent: &mut Agent) {
    agent.status(Exit, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_exit);
}

pub fn install_nana(agent: &mut Agent) {
    agent.status(Exit, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_exit);
    agent.status(Main, *FIGHTER_STATUS_KIND_THROW, throw_nana);
    agent.status(Pre, *FIGHTER_STATUS_KIND_CATCH_WAIT, nana_catch_wait_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, nana_catch_wait_main);
    agent.status(Pre, *FIGHTER_POPO_STATUS_KIND_THROW_NANA, popo_status_kind_throw_nana_pre);
    agent.status(Main, *FIGHTER_POPO_STATUS_KIND_THROW_NANA, popo_status_kind_throw_nana_main);
}
