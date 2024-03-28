// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe extern "C" fn tico_frame(weapon: &mut L2CFighterBase) {
    unsafe {
		let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		let rosetta = utils::util::get_battle_object_from_id(owner_id);
		let rosetta_boma = &mut *(*rosetta).module_accessor;
		if StatusModule::is_changing(weapon.module_accessor) || weapon.is_status(*WEAPON_ROSETTA_TICO_STATUS_KIND_REBIRTH) {
			VarModule::off_flag(rosetta, vars::rosetta::instance::IS_TICO_UNAVAILABLE);
		}
		if weapon.is_status_one_of(&[
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_STANDBY,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_NONE,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_AIR,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FALL,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FLY,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FLY_REFLECT_U]) {
			VarModule::on_flag(rosetta, vars::rosetta::instance::IS_TICO_UNAVAILABLE);
		}
		else if rosetta_boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) {
			if rosetta_boma.motion_frame() == 1.0 {
				VarModule::off_flag(rosetta, vars::rosetta::instance::IS_TICO_UNAVAILABLE);
			}
			if rosetta_boma.motion_frame() == 25.0 {
				VarModule::set_int(rosetta, vars::rosetta::instance::TICO_X, PostureModule::pos_x(weapon.module_accessor) as i32);
				VarModule::set_int(rosetta, vars::rosetta::instance::TICO_Y, PostureModule::pos_y(weapon.module_accessor) as i32);
			}
		}
		if VarModule::get_int(rosetta, vars::rosetta::status::LUMA_STATE) > 0 {
			if VarModule::get_int(rosetta, vars::rosetta::status::LUMA_STATE) == 1 {	// set startup effects
				EFFECT(weapon, Hash40::new("rosetta_escape"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
			}
			if VarModule::get_int(rosetta, vars::rosetta::status::LUMA_STATE) == 2 {	// prepare for teleport
				HitModule::set_whole(weapon.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				VisibilityModule::set_whole(weapon.module_accessor, false);
				JostleModule::set_status(weapon.module_accessor, false);
			}
			if VarModule::get_int(rosetta, vars::rosetta::status::LUMA_STATE) == 3 {	// perform swap
				EFFECT(weapon, Hash40::new("rosetta_escape_end"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				let new_x = VarModule::get_int(rosetta, vars::rosetta::instance::ROSA_X) as f32;
				let new_y = VarModule::get_int(rosetta, vars::rosetta::instance::ROSA_Y) as f32;
				let pos = Vector3f { x: new_x, y: new_y, z: 0.0 };
				PostureModule::set_pos(weapon.module_accessor, &pos);
				PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
			}
			if VarModule::get_int(rosetta, vars::rosetta::status::LUMA_STATE) == 4 {	// revert to normal state
				JostleModule::set_status(weapon.module_accessor, true);	
				VisibilityModule::set_whole(weapon.module_accessor, true);
				VarModule::set_int(rosetta, vars::rosetta::instance::COOLDOWN, 300); //300 Frame (5 second) cooldown
				VarModule::set_int(rosetta, vars::rosetta::status::LUMA_STATE, 0);
				HitModule::set_whole(weapon.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
			}
		}
	}
}

pub fn install(agent: &mut Agent) {
	agent.on_line(Main, tico_frame);
}
