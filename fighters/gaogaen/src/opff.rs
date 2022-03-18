// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn cross_chop_cancel_dj_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN) {
        if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
            WorkModule::set_int(fighter.module_accessor, fighter.get_jump_count_max() - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        if fighter.is_button_on(Buttons::Guard) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, true); 
            VarModule::on_flag(fighter.object(), vars::common::UP_SPECIAL_CANCEL);
        }
    }
}

// Incineroar Fthrow Movement
unsafe fn fthrow_movement(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_THROW) 
     && fighter.is_motion(smash::phx::Hash40::new("throw_f"))
     && fighter.is_situation(*SITUATION_KIND_GROUND) 
     && fighter.stick_x() != 0.0 {

        let motion_vec = x_motion_vec(1.0, fighter.stick_x());
        KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        
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
    let grab_y = VarModule::get_float(boma.object(), vars::gaogaen::ANGLE_GRAB_STICK_Y);
    if frame >= 0.0 && frame < lean_frame {
        // linear interpolate to stick position,
        // while getting stick position still
        VarModule::set_float(boma.object(), vars::gaogaen::ANGLE_GRAB_STICK_Y, stick_y);
        rotate_bust(boma, max_angle, min_angle, stick_y * ((frame as f32) / 7.0));
    } else if frame >= lean_frame && frame < return_frame {
        // rotate at selected angle for each frame
        rotate_bust(boma, max_angle, min_angle, grab_y);
    } else {
        // linear interpolate back to normal
        rotate_bust(boma, max_angle, min_angle, grab_y * (1.0 - ((frame - return_frame) / (end_frame - return_frame))));
    }
}

unsafe fn angled_grab(fighter: &mut L2CFighterCommon) {

    if fighter.is_status(*FIGHTER_STATUS_KIND_CATCH) {
        catch_lean(fighter.boma(), 7.0, 30.0, 50.0, 30.0);
    } else if fighter.is_status(*FIGHTER_STATUS_KIND_CATCH_TURN) {
        catch_lean(fighter.boma(), 12.0, 30.0, 30.0, 15.0);
    } else if  fighter.is_status(*FIGHTER_STATUS_KIND_CATCH_DASH) {
        catch_lean(fighter.boma(), 11.0, 30.0, 30.0, 15.0);
    }
}

#[utils::macros::opff(FIGHTER_KIND_GAOGAEN )]
pub fn gaogaen_opff(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		cross_chop_cancel_dj_reset(fighter);
        fthrow_movement(fighter);
        angled_grab(fighter); 
    }
}
