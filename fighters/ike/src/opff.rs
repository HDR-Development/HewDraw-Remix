use ::common::opff_import::*;
use super::*;
use globals::*;
use ::common::opff::*;
 
unsafe fn aether_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, stick_x: f32, facing: f32) {
    if situation_kind != *SITUATION_KIND_AIR {
        return;
    }

    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2].contains(&status_kind) {
        if stick_x != 0.0 {
            let motion_vec = moveset_utils::x_motion_vec(0.3, stick_x);
            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }
}

// Ike Quick Draw B-Reverse
unsafe fn quickdraw_b_reverse(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD].contains(&status_kind) {
        if frame < 5.0 {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                if frame > 1.0 && frame < 5.0 &&  !VarModule::is_flag(boma.object(), vars::common::B_REVERSED) {
                    let b_reverse = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &b_reverse, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    VarModule::on_flag(boma.object(), vars::common::B_REVERSED);
                }
            }
        }
    }
}

// Ike Quick Draw Jump, Wall Jump, and Attack Cancels
unsafe fn jump_attack_cancels(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32) {
    if status_kind != *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH {
        return;
    }

    // Wall Jump
    if situation_kind == *SITUATION_KIND_AIR {
        if  !VarModule::is_flag(boma.object(), vars::common::SPECIAL_WALL_JUMP) {
            let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
            let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
            if touch_left || touch_right {
                if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH | *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) {
                    VarModule::on_flag(boma.object(), vars::common::SPECIAL_WALL_JUMP);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                }
            }
        }
    }

    // Jump and Attack cancels
    let pad_flag = ControlModule::get_pad_flag(boma);

    if moveset_utils::jump_checker_buffer(boma, cat1) {
        if situation_kind == *SITUATION_KIND_GROUND {
            if facing * stick_x < 0.0 {
                PostureModule::reverse_lr(boma);
            }
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        }
    } else if compare_mask(pad_flag, *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) || compare_mask(pad_flag, *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK, true);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    aether_drift(boma, status_kind, situation_kind, stick_x, facing);
    quickdraw_b_reverse(boma, id, status_kind, stick_x, facing, frame);
    jump_attack_cancels(boma, id, status_kind, situation_kind, cat[0], stick_x, facing);
}

#[utils::opff(FIGHTER_KIND_IKE )]
pub fn ike_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		ike_frame(fighter)
    }
}

pub unsafe fn ike_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}