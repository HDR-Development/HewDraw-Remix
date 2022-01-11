use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn psi_magnet_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    if [*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        if frame > 4.0 {
            if moveset_utils::jump_checker_buffer(boma, cat1) {
                if situation_kind == *SITUATION_KIND_AIR {
                    if hdr::get_jump_count(boma) < hdr::get_jump_count_max(boma) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                } else if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

// Lucas PK Thunder cancel
unsafe fn pk_thunder_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
   if status_kind == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD {
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if  !VarModule::is_flag(get_battle_object_from_accessor(boma), vars::common::UP_SPECIAL_INTERRUPT) {
                VarModule::on_flag(get_battle_object_from_accessor(boma), vars::common::UP_SPECIAL_INTERRUPT);
            }
            if VarModule::is_flag(get_battle_object_from_accessor(boma), vars::common::UP_SPECIAL_INTERRUPT_AIRTIME) {
                VarModule::on_flag(boma.object(), common::UP_SPECIAL_CANCEL); // Disallow more up specials
            }
            StatusModule::change_status_request_from_script(boma, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, true);
        }
    }

    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END
        && situation_kind == *SITUATION_KIND_AIR {
        if VarModule::is_flag(get_battle_object_from_accessor(boma), vars::common::UP_SPECIAL_INTERRUPT) &&  !VarModule::is_flag(get_battle_object_from_accessor(boma), vars::common::UP_SPECIAL_INTERRUPT_AIRTIME) {
            VarModule::on_flag(get_battle_object_from_accessor(boma), vars::common::UP_SPECIAL_INTERRUPT_AIRTIME);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

// Lucas DJC and momentum tracker
unsafe fn djc_momentum_helper(id: usize, status_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
        VarModule::set_float(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_FRAME, value_here)  frame;
    }
    /*
    if VarModule::get_float(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_FRAME) == 1.0 {
        VarModule::set_float(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_TIMER, 1.0);
    }
    if VarModule::get_float(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_TIMER) > 0.0 && (status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR) {
        VarModule::add_float(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_TIMER, 1.0);
    }
    if VarModule::is_flag(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_STOP) && VarModule::get_float(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_TIMER) == 0.0 {
        VarModule::off_flag(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_STOP);
        println!("DJ stop flag reset");
    }
    if VarModule::is_flag(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_STOP) {
        VarModule::set_float(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_TIMER, 0.0);
        println!("Ended!");
    }
    if VarModule::get_float(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_TIMER) >= 15.0 {
        VarModule::on_flag(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_STOP);
        println!("Ending DJC motion blending");
    }
    //println!("Lucas DJ timer: Frame {}", VarModule::get_float(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_TIMER));
    */
    if status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR {
        VarModule::off_flag(get_battle_object_from_accessor(boma), vars::common::DOUBLE_JUMP_CANCELED);
    }
}

// PK Thunder wall ride momentum fix
unsafe fn pk_thunder_wall_ride(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    let wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
    let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
    let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
    let touch_high = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_UP_SIDE as u32);
    let touch_low =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_DOWN_SIDE as u32);
    let touch_side =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32);

    if status_kind == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK{
        if touch_left || touch_right || touch_high || touch_low || touch_side {
            KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
    }

}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    psi_magnet_jc(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    pk_thunder_cancel(boma, id, status_kind, situation_kind);
    pk_thunder_wall_ride(boma, id, status_kind, situation_kind);
    djc_momentum_helper(id, status_kind, frame);
    pk::moveset(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

#[utils::opff(FIGHTER_KIND_LUCAS )]
pub fn lucas_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		lucas_frame(fighter)
    }
}

pub unsafe fn lucas_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}