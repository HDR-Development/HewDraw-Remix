use super::*;
use globals::*;
// status script import

/// heavy item carry

unsafe extern "C" fn super_lift_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.boma(), Hash40::new("super_lift_landing"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(super_lift_landing_main_loop as *const () as _))
}

unsafe extern "C" fn super_lift_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if you are in the air, somehow during landing status, transition into fall
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_FALL.into(), false.into());
        return 1.into();
    }
 
    // if its been long enough, transition into wait
    // HDR only: reduce the landing lag
    if fighter.motion_frame() > MotionModule::end_frame(fighter.boma()) / 4.0 {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_WAIT.into(), false.into());
            return 1.into();
        }
    }

    // no action was taken
    return 0.into();
}

/// subroutine for initiating platdrop in super lift
pub unsafe fn super_lift_platdrop(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_cat_flag(Cat2::FallJump)
        && fighter.stick_y() < -0.66 
        && GroundModule::is_passable_ground(fighter.boma()) {
        // transition into platdrop
        fighter.change_status_req(*FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_PASS, false);
    }
}

/// subroutine for beginning a fastfall in super lift
pub unsafe fn super_lift_fastfall(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    fighter.sub_air_check_dive();
}

/// callback run during SUPER_LIFT_WAIT
pub unsafe fn super_lift_wait(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
    super_lift_platdrop(fighter);
}

/// callback run during SUPER_LIFT_WALK
pub unsafe fn super_lift_walk(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
    super_lift_platdrop(fighter);
}

/// callback run during SUPER_LIFT_TURN
pub unsafe fn super_lift_turn(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
    super_lift_platdrop(fighter);
    fighter.set_rate(1.5);
    if fighter.status_frame() >= 5 && fighter.stick_x().abs() > 0.33 {
        fighter.change_status_req(*FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_WALK, false);
    }

    // allow transition into item toss during turnaround
    if fighter.is_cat_flag(Cat1::AttackS4 | Cat1::AttackS3) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY, false);
    }

    // enable actionability for all of the turn status
    //if fighter.status_frame() > 0 {
    //    WorkModule::on_flag(fighter.boma(), *FIGHTER_STATUS_TURN_FLAG_TURN);
    //}
}

/// callback run during SUPER_LIFT_FALL
pub unsafe fn super_lift_fall(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
    super_lift_fastfall(fighter);
}

/// callback run during SUPER_LIFT_JUMP
pub unsafe fn super_lift_jump(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
    super_lift_fastfall(fighter);
}

/// callback run during SUPER_LIFT_PASS
pub unsafe fn super_lift_pass(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
    if fighter.status_frame() > 5 {
        super_lift_fastfall(fighter);
    }
}

/// callback run during SUPER_LIFT_LANDING
pub unsafe fn super_lift_landing(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
}

/// callback run during SUPER_LIFT_JUMP_SQUAT
pub unsafe fn super_lift_jump_squat(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
}

/// callback run during SUPER_LIFT_JUMP_SQUAT_B
pub unsafe fn super_lift_jump_squat_b(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
}

/// opff specifically against the super lift mains

pub unsafe extern "C" fn donkey_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    // call the super lift subroutines
    let status = fighter.status();
    if status == *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_WAIT { super_lift_wait(fighter) } 
    if status == *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_WALK { super_lift_walk(fighter) } 
    if status == *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_TURN { super_lift_turn(fighter) } 
    if status == *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_FALL { super_lift_fall(fighter) } 
    if status == *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_JUMP { super_lift_jump(fighter) } 
    if status == *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_PASS { super_lift_pass(fighter) } 
    if status == *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_LANDING { super_lift_landing(fighter) } 
    if status == *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_JUMP_SQUAT { super_lift_jump_squat(fighter) } 
    if status == *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_JUMP_SQUAT_B { super_lift_jump_squat_b(fighter) } 
}

pub fn install() {
    smashline::Agent::new("donkey")
        .status(
            Main,
            *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_LANDING,
            super_lift_landing_main,
        )
        .on_line(Main, donkey_frame_wrapper)
        .install();
}
