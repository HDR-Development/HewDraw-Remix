// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

//Launch Star Cancel
unsafe fn launch_star_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_JUMP {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_END, false);
        }
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

//Rosalina Teleport
unsafe fn teleport(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
	if StatusModule::is_changing(boma) {
        return;
    }
	if !smash::app::sv_information::is_ready_go(){
		VarModule::set_int(fighter.battle_object, vars::rosetta::instance::COOLDOWN, 0);
		VarModule::off_flag(boma.object(), vars::rosetta::instance::IS_TICO_UNAVAILABLE);
		VarModule::off_flag(fighter.battle_object, vars::rosetta::status::IS_INVALID_TELEPORT);
	}
	//Teleport!
	if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && !VarModule::is_flag(fighter.battle_object, vars::rosetta::instance::IS_TICO_UNAVAILABLE) && VarModule::get_int(fighter.battle_object, vars::rosetta::instance::COOLDOWN) == 0 {
		let frame = MotionModule::frame(boma);
		let rosa_x = VarModule::get_int(fighter.battle_object, vars::rosetta::instance::ROSA_X) as f32;
		let rosa_y = VarModule::get_int(fighter.battle_object, vars::rosetta::instance::ROSA_Y) as f32;
		let tico_x = VarModule::get_int(fighter.battle_object, vars::rosetta::instance::TICO_X) as f32;
		let tico_y = VarModule::get_int(fighter.battle_object, vars::rosetta::instance::TICO_Y) as f32;
		VarModule::set_int(fighter.battle_object, vars::rosetta::instance::TICO_RAYCAST, (GroundModule::ray_check(boma, &smash::phx::Vector2f{ x: rosa_x, y: rosa_y}, &Vector2f{ x: tico_x, y: tico_y}, false)) as i32);
		VarModule::set_int(fighter.battle_object, vars::rosetta::instance::TICO_X_DIST, (rosa_x-tico_x) as i32);
		VarModule::set_int(fighter.battle_object, vars::rosetta::instance::TICO_Y_DIST, (rosa_y-tico_y) as i32);
		if frame == 13.0 {
			macros::EFFECT(fighter, Hash40::new("rosetta_escape"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			VarModule::set_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE, 1);
		}
		if !VarModule::is_flag(fighter.battle_object, vars::rosetta::status::IS_INVALID_TELEPORT) {
			if frame > 17.0 && frame < 20.0 {
				HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				VisibilityModule::set_whole(boma, false);
				JostleModule::set_status(boma, false);	
				let new_x = tico_x;
				let new_y = tico_y;
				let pos = smash::phx::Vector3f { x: new_x, y: new_y, z: 0.0 };
				PostureModule::set_pos(boma, &pos);
				PostureModule::init_pos(boma, &pos, true, true);
				VarModule::set_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE, 2);
			}
			if frame == 26.0 {
				macros::EFFECT(fighter, Hash40::new("rosetta_escape_end"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				VarModule::set_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE, 3);
			}
			if frame > 26.0{
				VisibilityModule::set_whole(boma, true);
				JostleModule::set_status(boma, true);	
				VarModule::set_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE, 4);
				HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
			}
			if frame > 38.0 {
				CancelModule::enable_cancel(boma);
			}
		}
	}
	else if VarModule::get_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE) > 0 {
		// prevent the successful teleport logic if Luma is put into hitstun or killed during startup
		VarModule::set_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE, 0);
		VarModule::on_flag(fighter.battle_object, vars::rosetta::status::IS_INVALID_TELEPORT);
	}
	else {
		VarModule::set_int(fighter.battle_object, vars::rosetta::instance::ROSA_X, PostureModule::pos_x(boma) as i32);
		VarModule::set_int(fighter.battle_object, vars::rosetta::instance::ROSA_Y, PostureModule::pos_y(boma) as i32);
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
			HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
			JostleModule::set_status(boma, true);	
			VisibilityModule::set_whole(boma, true);
			if frame > 38.0 {
				CancelModule::enable_cancel(boma);
			}
		}
	}
	if VarModule::get_int(fighter.battle_object, vars::rosetta::instance::COOLDOWN) > 0 {
		VarModule::dec_int(fighter.battle_object, vars::rosetta::instance::COOLDOWN);
	}
	if VarModule::get_int(fighter.battle_object, vars::rosetta::instance::COOLDOWN) == 1 {
		gimmick_flash(boma);
	}
	if status_kind == *FIGHTER_STATUS_KIND_DEAD {
		VarModule::off_flag(fighter.battle_object, vars::rosetta::instance::IS_TICO_UNAVAILABLE);
	}
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    launch_star_cancel(boma, status_kind);
	teleport(fighter, boma, status_kind);
}

#[utils::macros::opff(FIGHTER_KIND_ROSETTA )]
pub fn rosetta_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		rosetta_frame(fighter)
    }
}

pub unsafe fn rosetta_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
#[weapon_frame( agent = WEAPON_KIND_ROSETTA_TICO )]
fn tico_frame(weapon: &mut L2CFighterBase) {
    unsafe {
		if StatusModule::is_changing(boma) {
			VarModule::off_flag(fighter.battle_object, vars::rosetta::instance::IS_TICO_IN_HITSTUN);
		}
		let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		let rosetta = utils::util::get_battle_object_from_id(owner_id);
		if weapon.is_status_one_of(&[
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_NONE,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_AIR,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FALL,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FLY,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
			*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FLY_REFLECT_U]) {
			VarModule::on_flag(rosetta, vars::rosetta::instance::IS_TICO_UNAVAILABLE);
		}
		if VarModule::get_int(rosetta, vars::rosetta::status::LUMA_STATE) > 0 {
			if VarModule::get_int(rosetta, vars::rosetta::status::LUMA_STATE) == 1 {
				macros::EFFECT(weapon, Hash40::new("rosetta_escape"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
			}
			if VarModule::get_int(rosetta, vars::rosetta::status::LUMA_STATE) == 2 {
				let new_x = VarModule::get_int(rosetta, vars::rosetta::instance::ROSA_X) as f32;
				let new_y = VarModule::get_int(rosetta, vars::rosetta::instance::ROSA_Y) as f32;
				HitModule::set_whole(weapon.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				VisibilityModule::set_whole(weapon.module_accessor, false);
				JostleModule::set_status(weapon.module_accessor, false);	
				let pos = smash::phx::Vector3f { x: new_x, y: new_y, z: 0.0 };
				PostureModule::set_pos(weapon.module_accessor, &pos);
				PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
			}
			if VarModule::get_int(rosetta, vars::rosetta::status::LUMA_STATE) == 3 {
				macros::EFFECT(weapon, Hash40::new("rosetta_escape_end"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			}
			if VarModule::get_int(rosetta, vars::rosetta::status::LUMA_STATE) == 4 {
				JostleModule::set_status(weapon.module_accessor, true);	
				VisibilityModule::set_whole(weapon.module_accessor, true);
				VarModule::set_int(rosetta, vars::rosetta::instance::COOLDOWN, 300); //300 Frame (5 second) cooldown
				VarModule::set_int(rosetta, vars::rosetta::status::LUMA_STATE, 0);
				HitModule::set_whole(weapon.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
			}
		} else {
			VarModule::set_int(rosetta, vars::rosetta::instance::TICO_X, PostureModule::pos_x(weapon.module_accessor) as i32);
			VarModule::set_int(rosetta, vars::rosetta::instance::TICO_Y, PostureModule::pos_y(weapon.module_accessor) as i32);
		}
	}
}