// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn egg_roll_jc_waveland(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_TURN].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat1::AirEscape) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
            }
        }
    }

    if status_kind == *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END {
        if boma.is_input_jump() {
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
/*unsafe fn flutter_kick(boma: &mut BattleObjectModuleAccessor, id: usize, situation_kind: i32, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("attack_air_lw") {
        let motion_vec = Vector3f{x: 0.0, y: 1.275, z: 0.0};
        if  !VarModule::is_flag(boma.object(), vars::common::AERIAL_COMMAND_RISEN) {
            if frame <= 44.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
                    || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)
                    /*|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)*/ {
                    VarModule::on_flag(boma.object(), vars::common::AERIAL_COMMAND_RISING);
                    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);

                    if VarModule::is_flag(boma.object(), vars::common::AERIAL_COMMAND_RISING) &&  !VarModule::is_flag(boma.object(), vars::common::AERIAL_COMMAND_RISEN) {
                        // Reset momentum on the rise initialization
                        if  !VarModule::is_flag(boma.object(), vars::common::AERIAL_COMMAND_MOMENTUM_RESET){
                            // Reset momentum
                            KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                            KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                            KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                            VarModule::on_flag(boma.object(), vars::common::AERIAL_COMMAND_MOMENTUM_RESET);
                        }
                    }

                }
            }
        }
    }
    if VarModule::is_flag(boma.object(), vars::common::AERIAL_COMMAND_RISING) && (motion_kind != hash40("attack_air_lw") || (motion_kind == hash40("attack_air_lw") && frame > 44.0)) {
        VarModule::on_flag(boma.object(), vars::common::AERIAL_COMMAND_RISEN);
        VarModule::off_flag(boma.object(), vars::common::AERIAL_COMMAND_RISING);
        VarModule::off_flag(boma.object(), vars::common::AERIAL_COMMAND_MOMENTUM_RESET);
    }
    if situation_kind == *SITUATION_KIND_GROUND && VarModule::is_flag(boma.object(), vars::common::AERIAL_COMMAND_RISEN) {
        VarModule::off_flag(boma.object(), vars::common::AERIAL_COMMAND_RISEN);
        VarModule::off_flag(boma.object(), vars::common::AERIAL_COMMAND_MOMENTUM_RESET);
    }
}*/

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    egg_roll_jc_waveland(boma, status_kind, situation_kind, cat[0], stick_x, facing);
    //flutter_kick(boma, id, situation_kind, motion_kind, frame);
}

#[utils::macros::opff(FIGHTER_KIND_YOSHI )]
pub fn yoshi_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		yoshi_frame(fighter)
    }
}

pub unsafe fn yoshi_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}