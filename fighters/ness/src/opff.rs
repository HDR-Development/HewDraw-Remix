// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 // Magnet Jump Cancel and Turnaround
unsafe fn psi_magnet_jump_cancel_turnaround(fighter: &mut L2CFighterCommon) {
    if fighter.is_status (*FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD) {
        let facing = PostureModule::lr(fighter.module_accessor);
        let stick_x = fighter.stick_x();
        if stick_x * facing < 0.0 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
    }
    if ((fighter.is_status (*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.status_frame() > 5)  // Allows for jump cancel on frame 7 in game
    || fighter.is_status_one_of(&[
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END]))
    && !fighter.is_in_hitlag()
        {
            fighter.check_jump_cancel(false);
        }
}   


// Ness PK Fire Fast Fall
unsafe fn pk_fire_ff(boma: &mut BattleObjectModuleAccessor, stick_y: f32) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) {
        if boma.is_situation(*SITUATION_KIND_AIR) {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Ness PK Thunder cancel
unsafe fn pk_thunder_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD {
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if  !VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT) {
                VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT);
            }
            if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME) {
                VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL); // Disallow more up specials
            }
            StatusModule::change_status_request_from_script(boma, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, true);
        }
    }

    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END
        && situation_kind == *SITUATION_KIND_AIR {
        if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT) &&  !VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME) {
            VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }


    /*
    if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT) {
        println!("Up Special Interrupt flag active")
    }

    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END{
        println!("..... PKT1 COOLDOWN .....");
    }

    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END
        && (MotionModule::frame(boma) >= (MotionModule::end_frame(boma)-3.0))
        && situation_kind == *SITUATION_KIND_AIR {
        println!("PKT ending animation is over");
        if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT) &&  !VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME) {
            println!("PKT special airtime interrupt flag set");
            VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
    */
}

// PK Thunder wall ride momentum fix
unsafe fn pk_thunder_wall_ride(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    let wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
    let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
    let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
    let touch_high = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_UP_SIDE as u32);
    let touch_low =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_DOWN_SIDE as u32);
    let touch_side =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32);

    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK{
        if touch_left || touch_right || touch_high || touch_low || touch_side {
            KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
    }

}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    psi_magnet_jump_cancel_turnaround(fighter);
    pk_thunder_cancel(boma, id, status_kind, situation_kind);
    pk_thunder_wall_ride(boma, id, status_kind, situation_kind);
    pk_fire_ff(boma, stick_y);
}

#[utils::macros::opff(FIGHTER_KIND_NESS )]
pub fn ness_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		ness_frame(fighter)
    }
}

pub unsafe fn ness_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame_callback]
pub fn pkthunder_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.kind() != WEAPON_KIND_NESS_PK_THUNDER {
            return
        }
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    }
}