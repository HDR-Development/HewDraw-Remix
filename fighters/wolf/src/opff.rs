// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn airdodge_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            if frame > 17.0 {
                if boma.is_cat_flag(Cat1::AirEscape) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        }
    }
}

// Wolf Shine Jump Cancels
unsafe fn shine_jump_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if [*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        if boma.is_input_jump() {
            if situation_kind == *SITUATION_KIND_AIR {
                if boma.get_num_used_jumps() < boma.get_jump_count_max() {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            } else if situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
        }
    }
}

// Wolf Flash Shortens
unsafe fn flash_shortens(boma: &mut BattleObjectModuleAccessor, id: usize, motion_kind: u64, frame: f32) {
    /*
    if motion_kind == hash40("special_s") || motion_kind == hash40("special_air_s") {
        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) && !WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END) {
            let motion_vec = Vector3f{x: 0.1, y: 1.0, z: 1.0};
            WorkModule::on_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END);
            KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
    }
    */
    if motion_kind == hash40("special_s") || motion_kind == hash40("special_air_s") {
        if frame <= 1.0 {
            VarModule::off_flag(boma.object(), vars::common::ILLUSION_SHORTEN);
            VarModule::off_flag(boma.object(), vars::common::ILLUSION_SHORTENED);
        }
        if VarModule::is_flag(boma.object(), vars::common::ILLUSION_SHORTEN) &&  !VarModule::is_flag(boma.object(), vars::common::ILLUSION_SHORTENED) {
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            VarModule::on_flag(boma.object(), vars::common::ILLUSION_SHORTENED);
        }

        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) &&  !VarModule::is_flag(boma.object(), vars::common::ILLUSION_SHORTENED) {
            VarModule::on_flag(boma.object(), vars::common::ILLUSION_SHORTEN);
            WorkModule::on_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END);
        }
    }
}

// Side Special Cancels
unsafe fn side_special_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            if frame > 20.0 {
                CancelModule::enable_cancel(boma);
            }
            if frame >= MotionModule::end_frame(boma) - 3.0{
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    airdodge_cancel(boma, status_kind, situation_kind, cat[0], frame);
    shine_jump_cancel(boma, status_kind, situation_kind, cat[0]);
    flash_shortens(boma, id, motion_kind, frame);

    // Magic Series
    side_special_cancels(boma, status_kind, frame);

    // Frame Data
    //frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if frame <= 13.0 {
            MotionModule::set_rate(boma, 1.4);
        }
        if frame > 13.0 {
            MotionModule::set_rate(boma, 1.0);
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_WOLF )]
pub fn wolf_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		wolf_frame(fighter)
    }
}

pub unsafe fn wolf_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}