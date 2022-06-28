// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn laser_fastfall_landcancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        } else if situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Fox Shine Jump Cancels
unsafe fn shine_jump_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 {
        GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    if [*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        if boma.is_input_jump() && !boma.is_in_hitlag() {
            if situation_kind == *SITUATION_KIND_AIR {
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            } else if situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
        }
    }
}

// Fox Illusion Shortens
unsafe fn illusion_shorten_(boma: &mut BattleObjectModuleAccessor, id: usize, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("special_s") || motion_kind == hash40("special_air_s") {
        if frame <= 1.0 {
            VarModule::off_flag(boma.object(), vars::fox::status::ILLUSION_SHORTEN);
            VarModule::off_flag(boma.object(), vars::fox::status::ILLUSION_SHORTENED);
        }
        if VarModule::is_flag(boma.object(), vars::fox::status::ILLUSION_SHORTEN) &&  !VarModule::is_flag(boma.object(), vars::fox::status::ILLUSION_SHORTENED) {
            let motion_vec = Vector3f{x: 0.25, y: 1.0, z: 1.0}; // Unused
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            VarModule::on_flag(boma.object(), vars::fox::status::ILLUSION_SHORTENED);
        }

        /*
        if WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END) {
            let motion_vec = Vector3f{x: 0.25, y: 1.0, z: 1.0};
            KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_MOTION); // Nope
            KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); // Nope
            KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
            KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
            KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
            KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_GROUND_MOVEMENT);
        }
        */

        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) &&  !VarModule::is_flag(boma.object(), vars::fox::status::ILLUSION_SHORTENED) {
            VarModule::on_flag(boma.object(), vars::fox::status::ILLUSION_SHORTEN);
            WorkModule::on_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END);
        }
    }
}

// Utaunt cancel into Fire Fox
unsafe fn utaunt_cancel_fire_fox(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("appeal_hi_r") || motion_kind == hash40("appeal_hi_l") {
        if frame > 40.0 && frame < 43.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, false);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    laser_fastfall_landcancel(boma, status_kind, situation_kind, cat[1], stick_y);
    shine_jump_cancel(boma, status_kind, situation_kind, cat[0]);
    illusion_shorten_(boma, id, motion_kind, frame);
    utaunt_cancel_fire_fox(boma, motion_kind, frame);

    // Frame Data
    frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("throw_hi") {
        if frame >= 9.0 {
            MotionModule::set_rate(boma, 1.8);
        }
    }
}
#[utils::macros::opff(FIGHTER_KIND_FOX )]
pub fn fox_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		fox_frame(fighter)
    }
}

pub unsafe fn fox_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
