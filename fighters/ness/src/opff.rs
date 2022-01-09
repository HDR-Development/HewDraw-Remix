use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn psi_magnet_jump_cancel_turnaround(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    if [*FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD].contains(&status_kind) {
        if moveset_utils::jump_checker_buffer(boma, cat1) {
            if (status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD && frame > 3.0)
                || (status_kind != *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD) {
                if situation_kind == *SITUATION_KIND_AIR {
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                } else if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }

        if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD {
            if stick_x * facing < 0.0 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

// Ness PK Thunder cancel
unsafe fn pk_thunder_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD {
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if !up_special_interrupt[id] {
                up_special_interrupt[id] = true;
            }
            if up_special_interrupt_airtime[id] {
                VarModule::on_flag(boma, common::UP_SPECIAL_CANCEL); // Disallow more up specials
            }
            StatusModule::change_status_request_from_script(boma, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, true);
        }
    }

    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END
        && situation_kind == *SITUATION_KIND_AIR {
        if up_special_interrupt[id] && !up_special_interrupt_airtime[id] {
            up_special_interrupt_airtime[id] = true;
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }


    /*
    if up_special_interrupt[id] {
        println!("Up Special Interrupt flag active")
    }

    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END{
        println!("..... PKT1 COOLDOWN .....");
    }

    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END
        && (MotionModule::frame(boma) >= (MotionModule::end_frame(boma)-3.0))
        && situation_kind == *SITUATION_KIND_AIR {
        println!("PKT ending animation is over");
        if up_special_interrupt[id] && !up_special_interrupt_airtime[id] {
            println!("PKT special airtime interrupt flag set");
            up_special_interrupt_airtime[id] = true;
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

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    psi_magnet_jump_cancel_turnaround(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    pk_thunder_cancel(boma, id, status_kind, situation_kind);
    pk_thunder_wall_ride(boma, id, status_kind, situation_kind);
    pk::moveset(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

#[utils::opff(FIGHTER_KIND_NESS )]
pub fn ness_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		ness_frame(fighter)
    }
}

pub unsafe fn ness_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}