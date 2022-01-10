use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn egg_roll_jc_waveland(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_TURN].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat1::JumpButton) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
            }
        }
    }

    if status_kind == *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END {
        if moveset_utils::jump_checker_buffer(boma, cat1) {
            if situation_kind == *SITUATION_KIND_AIR {
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            } else if situation_kind == *SITUATION_KIND_GROUND {
                if stick_x * facing < 0.0 {
                    PostureModule::reverse_lr(boma);
                }
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
        }
    }
}


// Flutter Kick
unsafe fn flutter_kick(boma: &mut BattleObjectModuleAccessor, id: usize, situation_kind: i32, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("attack_air_lw") {
        let motion_vec = Vector3f{x: 0.0, y: 1.275, z: 0.0};
        if !aerial_command_risen[id] {
            if frame <= 44.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
                    || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)
                    /*|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)*/ {
                    aerial_command_rising[id] = true;
                    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);

                    if aerial_command_rising[id] && !aerial_command_risen[id] {
                        // Reset momentum on the rise initialization
                        if !aerial_command_momentum_reset[id]{
                            // Reset momentum
                            KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                            KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                            KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                            aerial_command_momentum_reset[id] = true;
                        }
                    }

                }
            }
        }
    }
    if aerial_command_rising[id] && (motion_kind != hash40("attack_air_lw") || (motion_kind == hash40("attack_air_lw") && frame > 44.0)) {
        aerial_command_risen[id] = true;
        aerial_command_rising[id] = false;
        aerial_command_momentum_reset[id] = false;
    }
    if situation_kind == *SITUATION_KIND_GROUND && aerial_command_risen[id] {
        aerial_command_risen[id] = false;
        aerial_command_momentum_reset[id] = false;
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    egg_roll_jc_waveland(boma, status_kind, situation_kind, cat[0], stick_x, facing);
    flutter_kick(boma, id, situation_kind, motion_kind, frame);
}

#[utils::opff(FIGHTER_KIND_YOSHI )]
pub fn yoshi_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		yoshi_frame(fighter)
    }
}

pub unsafe fn yoshi_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}