// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn spr_auto_end(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL) {
        if fighter.status_frame() >= 30 {
            fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL_JUMP.into(), false.into());
        }
    }
}

// Angle tail based on stick y position and frame
unsafe fn rotate_bone(boma: &mut BattleObjectModuleAccessor, max_angle: f32, min_angle: f32, strength: f32) {
    let mut angle = min_angle.abs();
    if strength > 0.0 {
        angle = max_angle
    }
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: -((angle * -1.0 * strength) - 2.5)};
    let fighter = utils::util::get_fighter_common_from_accessor(boma);
    fighter.set_joint_rotate("tail1", rotation);
}

// boma: its a boma 
// lean_frame: frame to interpolate to the intended angle amount until
// return_frame: frame to start interpolating back to regular angle
// max_angle: maximum angle you can lean upwards, in degrees
// min_angle: minimum angle that we should be able to rotate downwards, in degrees
unsafe fn tail_lean(boma: &mut BattleObjectModuleAccessor, lean_frame: f32, return_frame: f32, max_angle: f32, min_angle: f32) {
    let stick_y = ControlModule::get_stick_y(boma);
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let tail_y = VarModule::get_float(boma.object(), vars::ridley::status::SPECIAL_LW_STICK_Y);
    if frame >= 0.0 && frame < lean_frame {
        // linear interpolate to stick position,
        // while getting stick position still
        VarModule::set_float(boma.object(), vars::ridley::status::SPECIAL_LW_STICK_Y, stick_y);
        rotate_bone(boma, max_angle, min_angle, stick_y * ((frame as f32) / 30.0));
    } else if frame >= lean_frame && frame < return_frame {
        // rotate at selected angle for each frame
        rotate_bone(boma, max_angle, min_angle, tail_y);
    } else {
        // linear interpolate back to normal
        rotate_bone(boma, max_angle, min_angle, tail_y * (1.0 - ((frame - return_frame) / (end_frame - return_frame))));
    }
}

// Handles angling of tail
unsafe fn angled_skewer(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.is_situation(*SITUATION_KIND_GROUND) {
        tail_lean(fighter.boma(), 31.0, 41.0, 25.0, -15.0);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_CHARGE,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_FAILURE,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FAILURE,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_WALL,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL_JUMP,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_CEIL,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_WALL
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon) {
    spr_auto_end(fighter);
    angled_skewer(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn ridley_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		ridley_frame(fighter)
    }
}

pub unsafe fn ridley_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, ridley_frame_wrapper);
}