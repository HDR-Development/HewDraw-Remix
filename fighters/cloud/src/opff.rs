use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn dspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like down-b canceling
    if status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_INT_CANCEL_STATUS);
            }
        }
    }
}

// Cloud Limit Charge start and release B-Reverse
unsafe fn limit_charge_start_b_rev(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        if frame < 5.0 && situation_kind == *SITUATION_KIND_AIR {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                if frame > 1.0 && frame < 5.0 &&  !VarModule::is_flag(get_battle_object_from_accessor(boma), vars::common::B_REVERSED) {
                    let b_reverse = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &b_reverse, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    VarModule::on_flag(get_battle_object_from_accessor(boma), vars::common::B_REVERSED);
                }
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    dspecial_cancels(boma, status_kind, situation_kind);
    limit_charge_start_b_rev(boma, id, status_kind, situation_kind, stick_x, facing, frame);
}
#[utils::opff(FIGHTER_KIND_CLOUD )]
pub fn cloud_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		cloud_frame(fighter)
    }
}

pub unsafe fn cloud_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

