// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn actionable_teleport_air(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 && situation_kind == *SITUATION_KIND_AIR && frame > 8.0 {
        if boma.get_num_used_jumps() < boma.get_jump_count_max() {
            VarModule::on_flag(boma.object(), vars::common::UP_SPECIAL_CANCEL);
            CancelModule::enable_cancel(boma);
        }
    }
}

// Put Mewtwo into fall after teleport if he has his double jump
unsafe fn fall_after_teleport(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 {
        if boma.get_num_used_jumps() < boma.get_jump_count_max() {
            VarModule::on_flag(boma.object(), vars::common::UP_SPECIAL_CANCEL);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            if MotionModule::is_end(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
            }
        }
    }
}

pub unsafe fn mewtwo_teleport_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, id: usize) {
    if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2 {
        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
            ControlModule::clear_command(boma, true);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, false);
        }
    }
    // Wall Ride momentum fixes
    let mut wall_ride = Vector3f{x: 1.0, y: 1.0, z: 1.0};
    let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
    let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
    let warp_speed = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("wrap_speed_add")) + WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("wrap_speed_multi"));

    if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2 {
        if touch_right || touch_left || VarModule::is_flag(boma.object(), vars::common::IS_TELEPORT_WALL_RIDE) {
            VarModule::on_flag(boma.object(), vars::common::IS_TELEPORT_WALL_RIDE);
            if (touch_right && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0) || (touch_left && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0) {
                let rise_speed = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if rise_speed > 0.0 {
                    wall_ride = Vector3f{x: 0.0, y: (warp_speed / rise_speed), z: 1.0};
                }
                else {
                    wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                }
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }
    else if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 {
        if touch_right || touch_left {
            if (touch_right && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0) || (touch_left && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0) {
                wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }
    else {
        VarModule::off_flag(boma.object(), vars::common::IS_TELEPORT_WALL_RIDE);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    actionable_teleport_air(boma, id, status_kind, situation_kind, frame);
    fall_after_teleport(boma, id, status_kind);
    nspecial_cancels(boma, status_kind, situation_kind);
    mewtwo_teleport_cancel(boma, status_kind, id);
}
#[utils::macros::opff(FIGHTER_KIND_MEWTWO )]
pub fn mewtwo_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		mewtwo_frame(fighter)
    }
}

pub unsafe fn mewtwo_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}