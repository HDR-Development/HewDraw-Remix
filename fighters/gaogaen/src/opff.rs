use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn cross_chop_cancel_dj_reset(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN {
        if hdr::get_jump_count(boma) == hdr::get_jump_count_max(boma) {
            WorkModule::set_int(boma, hdr::get_jump_count_max(boma) - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            VarModule::on_flag(boma, common::UP_SPECIAL_CANCEL);
        }
    }
}

// Incineroar Fthrow Movement
unsafe fn fthrow_movement(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_THROW {
        if motion_kind == hash40("throw_f") {
            if situation_kind == *SITUATION_KIND_GROUND {
                if stick_x != 0.0 {
                    let motion_vec = moveset_utils::x_motion_vec(1.0, stick_x);
                    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
                }
            }
        }
    }
}

unsafe fn rotate_bust(boma: &mut BattleObjectModuleAccessor, max_angle: f32, min_angle: f32, strength: f32) {
    let mut angle = min_angle.abs();
    if strength > 0.0 {
        angle = max_angle
    }
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: ((angle * -1.0 * strength) - 2.5)};

    // this has to be called every frame, or you snap back to the normal joint angle
    ModelModule::set_joint_rotate(boma, Hash40::new("waist"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
}

// boma: its a boma 
// lean_frame: frame to interpolate to the intended angle amount until
// return_frame: frame to start interpolating back to regular angle
// max_angle: maximum angle you can lean upwards, in degrees
// min_angle: minimum angle that we should be able to rotate downwards, in degrees
unsafe fn catch_lean(boma: &mut BattleObjectModuleAccessor, lean_frame: f32, return_frame: f32, max_angle: f32, min_angle: f32) {
    let stick_y = ControlModule::get_stick_y(boma);
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let grab_y = VarModule::get_float(boma, common::ANGLE_GRAB_STICK_Y);
    if frame >= 0.0 && frame < lean_frame {
        // linear interpolate to stick position,
        // while getting stick position still
        VarModule::set_float(boma, common::ANGLE_GRAB_STICK_Y, stick_y);
        rotate_bust(boma, max_angle, min_angle, stick_y * ((frame as f32) / 7.0));
    } else if frame >= lean_frame && frame < return_frame {
        // rotate at selected angle for each frame
        rotate_bust(boma, max_angle, min_angle, grab_y);
    } else {
        // linear interpolate back to normal
        rotate_bust(boma, max_angle, min_angle, grab_y * (1.0 - ((frame - return_frame) / (end_frame - return_frame))));
    }
}

unsafe fn angled_grab(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    
    if status_kind == *FIGHTER_STATUS_KIND_CATCH {
        catch_lean(boma, 7.0, 30.0, 50.0, 30.0);
    } else if status_kind == *FIGHTER_STATUS_KIND_CATCH_TURN {
        catch_lean(boma, 12.0, 30.0, 30.0, 15.0);
    } else if status_kind == *FIGHTER_STATUS_KIND_CATCH_DASH {
        catch_lean(boma, 11.0, 30.0, 30.0, 15.0);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    cross_chop_cancel_dj_reset(boma, id, status_kind, cat[0]);
    fthrow_movement(boma, status_kind, situation_kind, motion_kind, stick_x);
    angled_grab(boma, status_kind); 
}

#[utils::opff(FIGHTER_KIND_GAOGAEN )]
pub fn gaogaen_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		gaogaen_frame(fighter)
    }
}

pub unsafe fn gaogaen_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}