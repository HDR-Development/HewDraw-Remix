use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn bowser_bomb_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G].contains(&status_kind) {
        if frame > 19.0 && frame < 30.0 {
            if moveset_utils::jump_checker_buffer(boma, cat1) {
                if situation_kind == *SITUATION_KIND_AIR {
                    let jump_count = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                    let jump_count_max = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
                    if jump_count < jump_count_max {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                }
            }
        }
    }
}

// Ground Bowser Bomb jump drift
unsafe fn ground_bowser_bomb_jump_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G].contains(&status_kind) {
        if frame > 13.0 && frame < 30.0 {
            if stick_x != 0.0 {
                let motion_vec = moveset_utils::x_motion_vec(1.25, stick_x);
                KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
            }
        }
    }
}

// Bowser Flame Cancel
unsafe fn flame_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if frame < 22.0 {
            if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                MotionModule::set_frame(boma, 22.0, true);
            }
        }
    }
}

// NokNok shell flag reset
unsafe fn noknok_reset(id: usize, status_kind: i32) {
    if noknok_shell[id] {
        if [*FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_WIN,
            *FIGHTER_STATUS_KIND_LOSE,
            *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
                noknok_shell[id] = false;
        }
    }
}

// TRAINING MODE
// NokNok shell flag reset via taunt
unsafe fn noknok_training(id: usize, status_kind: i32) {
    if hdr::is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            noknok_shell[id] = false;
        }
    }
}

unsafe fn up_special_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL && StatusModule::prev_status_kind(boma, 1) == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    bowser_bomb_jc(boma, status_kind, situation_kind, cat[0], frame);
    ground_bowser_bomb_jump_drift(boma, status_kind, stick_x, frame);
    flame_cancel(boma, status_kind, situation_kind, frame);
    noknok_reset(id, status_kind);
    //up_special_land_cancel(boma, status_kind);
    noknok_training(id, status_kind);
}


#[utils::opff(FIGHTER_KIND_KOOPA )]
pub fn koopa_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		koopa_frame(fighter)
    }
}

pub unsafe fn koopa_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}