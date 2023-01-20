// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn bowser_bomb_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G].contains(&status_kind) {
        if frame > 19.0 && frame < 30.0 {
            if situation_kind == *SITUATION_KIND_AIR {
                boma.check_jump_cancel(false);
            }
        }
    }
}

// Ground Bowser Bomb jump drift
unsafe fn ground_bowser_bomb_jump_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G].contains(&status_kind) {
        if frame > 13.0 && frame < 30.0 {
            if stick_x != 0.0 {
                let motion_vec = x_motion_vec(1.25, stick_x);
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
// unsafe fn noknok_reset(boma: &mut BattleObjectModuleAccessor) {
//     if VarModule::is_flag(boma.object(), vars::koopa::instance::NOKNOK_SHELL) {
//         if boma.is_status_one_of(
//     &[*FIGHTER_STATUS_KIND_DEAD,
//             *FIGHTER_STATUS_KIND_REBIRTH,
//             *FIGHTER_STATUS_KIND_WIN,
//             *FIGHTER_STATUS_KIND_LOSE,
//             *FIGHTER_STATUS_KIND_ENTRY]) 
//         {
//             VarModule::off_flag(boma.object(), vars::koopa::instance::NOKNOK_SHELL);
//         }
//     }
// }

// TRAINING MODE
// NokNok shell flag reset via taunt
// unsafe fn noknok_training(boma: &mut BattleObjectModuleAccessor) {
//     if is_training_mode() {
//         if boma.is_status(*FIGHTER_STATUS_KIND_APPEAL) {
//             VarModule::off_flag(boma.object(), vars::koopa::instance::NOKNOK_SHELL);
//         }
//     }
// }

unsafe fn up_special_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL && StatusModule::prev_status_kind(boma, 1) == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    bowser_bomb_jc(boma, status_kind, situation_kind, cat[0], frame);
    ground_bowser_bomb_jump_drift(boma, status_kind, stick_x, frame);
    flame_cancel(boma, status_kind, situation_kind, frame);
    // noknok_reset(boma);
    //up_special_land_cancel(boma, status_kind);
    // noknok_training(boma);
}


#[utils::macros::opff(FIGHTER_KIND_KOOPA )]
pub fn koopa_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		koopa_frame(fighter)
    }
}

pub unsafe fn koopa_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}