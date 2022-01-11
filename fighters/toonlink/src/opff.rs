use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn heros_bow_ff(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 {
                if KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                    WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
    }
}

// Toon Link Bomb pull B-Reverse
unsafe fn bomb_pull_b_reverse(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
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

// Lengthen sword
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
	let long_sword_scale = Vector3f{x: 1.2, y: 1.0, z: 1.0};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword2"), &long_sword_scale);
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    heros_bow_ff(boma, status_kind, situation_kind, cat[1], stick_y);
    bomb_pull_b_reverse(boma, id, status_kind, stick_x, facing, frame);
	sword_length(boma);
    links::moveset(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);

    // Frame Data
    frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
        if motion_kind == hash40("attack_11") {
            if frame < 5.0 {
                MotionModule::set_rate(boma, 1.667);
            }
            if frame >= 5.0 {
                MotionModule::set_rate(boma, 1.0);
            }
        }
    }
}

#[utils::opff(FIGHTER_KIND_TOONLINK )]
pub fn toonlink_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		toonlink_frame(fighter)
    }
}

pub unsafe fn toonlink_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}