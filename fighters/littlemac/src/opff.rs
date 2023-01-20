// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn normal_side_special(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if WorkModule::is_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S) {
        if [*FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) {
            WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
        }
    }
}

// Straight Lunge charge jump cancel
unsafe fn straight_lunge_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, cat2: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START].contains(&status_kind) {
        if frame > 25.0 {
            if !boma.is_in_hitlag() {
                boma.check_jump_cancel(false);
            }
            /*
            if boma.is_cat_flag(Cat2::CommonGuard) {
                for trans_group in [*FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE].iter() {
                    WorkModule::unable_transition_term_group(boma, *trans_group);
                }
                if situation_kind == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                }
            }
            */
        }
    }
}

// Tech Roll distance help
unsafe fn tech_roll_help(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, facing: f32, frame: f32) {
    if frame < 5.0 {
        let mut motion_vec = Vector3f{x: 1.75, y: 0.0, z: 0.0};
        if motion_kind == hash40("passive_stand_f") {
            motion_vec.x *= facing;
        } else if motion_kind == hash40("passive_stand_b") {
            motion_vec.x *= -facing;
        } else {
            return; // Break out if not passive_stand_x
        }
        KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat1::AirEscape) {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    normal_side_special(boma, status_kind);
    straight_lunge_cancels(boma, status_kind, situation_kind, cat[0], cat[1], frame);
    tech_roll_help(boma, motion_kind, facing, frame);
    nspecial_cancels(boma, status_kind, situation_kind, cat[0]);
}

#[utils::macros::opff(FIGHTER_KIND_LITTLEMAC )]
pub fn littlemac_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		littlemac_frame(fighter)
    }
}

pub unsafe fn littlemac_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}