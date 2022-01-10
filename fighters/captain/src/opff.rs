use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn air_falcon_kick_jump_reset(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            let jump_count_max = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == jump_count_max {
                WorkModule::set_int(boma, jump_count_max - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
        }
    }
}



unsafe fn falcon_punch_b_reverse(boma: &mut BattleObjectModuleAccessor, id: usize, motion_kind: u64, stick_x: f32, facing: f32, frame: f32) {
    if motion_kind == hash40("special_air_n") {
        if frame < 5.0 {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                if frame > 1.0 && frame < 5.0 && !b_reversed[id] {
                    let b_reverse = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &b_reverse, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    b_reversed[id] = true;
                }
            }
        }
    }
}

unsafe fn falcon_kick_b_reverse(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if frame > 1.0 && frame < 5.0 {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                if !b_reversed[id] {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                    b_reversed[id] = true;
                }
            }
        }
    }
}

unsafe fn repeated_falcon_punch_turnaround(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN {
        if frame > 21.0 && frame < 40.0 {
            if stick_x * facing < 0.0 && stick_x.abs() > 0.1 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN, true);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    air_falcon_kick_jump_reset(boma, status_kind, situation_kind);
    falcon_punch_b_reverse(boma, id, motion_kind, stick_x, facing, frame);
    falcon_kick_b_reverse(boma, id, status_kind, stick_x, facing, frame);
    repeated_falcon_punch_turnaround(boma, status_kind, stick_x, facing, frame);
}
#[utils::opff(FIGHTER_KIND_CAPTAIN )]
pub fn captain_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		captain_frame(fighter)
    }
}

pub unsafe fn captain_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}