use super::*;
use smash::app::BattleObjectModuleAccessor;
// Up Special Reverse
unsafe fn up_special_reverse(boma: &mut BattleObjectModuleAccessor, fighter_kind: i32, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }

    // Marth/Lucina frame 6
    // Roy frame 10
    let mut target_frame = 6.0;
    if fighter_kind == *FIGHTER_KIND_ROY {
        target_frame = 10.0;
    }

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if frame == target_frame {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

// Lengthen swords
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
    let long_sword_scale = Vector3f{x: 1.015, y: 1.115, z: 1.045};
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &long_sword_scale);
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
}

#[no_mangle]
pub unsafe extern "Rust" fn fe_common(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        fe_moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub unsafe fn fe_moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    let fighter_kind = boma.kind();
    up_special_reverse(boma, fighter_kind, status_kind, stick_x, facing, frame);
    sword_length(boma);
}
