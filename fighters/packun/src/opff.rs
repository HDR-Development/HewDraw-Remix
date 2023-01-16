// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn piranhacopter_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, false);
        }
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

/*unsafe fn stance_head(boma: &mut BattleObjectModuleAccessor) {
    // Enable meshes for stances
    // HeadA is the normal head
	// HeadB is the poison head
	// HeadS is the spike head
    // extra effects needed for fire
    if VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 0 {
        ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("heads"), false);
    }
    else if VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 1  {
        ModelModule::set_mesh_visibility(boma, Hash40::new("headb"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("heads"), false);
    }
    else if VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 2  {
        ModelModule::set_mesh_visibility(boma, Hash40::new("heads"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), false);
    }
}*/

unsafe fn sspecial_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE) == *FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_NONE, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE);
                //ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

/*unsafe fn check_apply_speeds(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    // handle speed application once
    if VarModule::is_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS) {
        if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 0 {
            apply_status_speed_mul(fighter, 1.0);
        } else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
            apply_status_speed_mul(fighter, 0.7);
        } else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
            apply_status_speed_mul(fighter, 0.5);
        }
        VarModule::off_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS);
    }
}

unsafe fn apply_status_speed_mul(fighter: &mut smash::lua2cpp::L2CFighterCommon, mul: f32) {
    // set the X motion speed multiplier (where movement is baked into an anim)
    lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter.get_motion_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_mul( fighter.get_controller_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_add( fighter.get_controller_energy(), mul);

    // set the X speed max multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_speed_max(fighter.get_controller_energy(), mul);
}*/


pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    piranhacopter_cancel(boma, status_kind, cat[0]);
	//spike_head_mesh_test(boma);
    sspecial_cancel(boma, status_kind, situation_kind);
    //stance_head(boma);
    //check_apply_speeds(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_PACKUN )]
pub fn packun_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		packun_frame(fighter);
        adaptive_roots::run(fighter);
    }
}

pub unsafe fn packun_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
