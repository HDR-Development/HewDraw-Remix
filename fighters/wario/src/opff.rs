// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn bite_early_throw_turnaround(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE {
        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
            boma.change_status_req(*FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE_END, false);
        }
    }
    if status_kind == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE_END {
        if frame < 7.0 {
            if facing * stick_x < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

// Wario Bthrow Movement
unsafe fn bthrow_movement(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_THROW {
        if motion_kind == hash40("throw_b") {
            if situation_kind == *SITUATION_KIND_GROUND {
                let currentFrame = MotionModule::frame(boma);
                let maxFrame = 46.0;
                if stick_x != 0.0 
                && currentFrame < maxFrame {
                    let motion_vec = x_motion_vec(1.0, stick_x);
                    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
                }
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    bite_early_throw_turnaround(boma, status_kind, stick_x, facing, frame);
    bthrow_movement(boma, status_kind, situation_kind, motion_kind, stick_x);
}

#[utils::macros::opff(FIGHTER_KIND_WARIO )]
pub fn wario_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		wario_frame(fighter)
    }
}

pub unsafe fn wario_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}