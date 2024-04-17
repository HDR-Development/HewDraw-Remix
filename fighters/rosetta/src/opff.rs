// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

//Launch Star Cancel
unsafe fn launch_star_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_JUMP
	&& !StatusModule::is_changing(boma)
	&& MotionModule::frame(boma) > 2.0 {
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
		VarModule::set_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE, 0);
		VarModule::off_flag(boma.object(), vars::rosetta::instance::IS_TICO_UNAVAILABLE);
		VarModule::off_flag(fighter.battle_object, vars::rosetta::status::IS_INVALID_TELEPORT);
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
	//Teleport!
	if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
		if !VarModule::is_flag(fighter.battle_object, vars::rosetta::instance::IS_TICO_UNAVAILABLE) && VarModule::get_int(fighter.battle_object, vars::rosetta::instance::COOLDOWN) == 0 {
			let frame = MotionModule::frame(boma);
			if frame == 13.0 {	// set teleport startup effects
				EFFECT(fighter, Hash40::new("rosetta_escape"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				VarModule::set_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE, 1);
			}
			if !VarModule::is_flag(fighter.battle_object, vars::rosetta::status::IS_INVALID_TELEPORT) {
				if frame == 17.0 {	// disappear in preparation for the teleport
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					VisibilityModule::set_whole(boma, false);
					JostleModule::set_status(boma, false);
					VarModule::set_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE, 2);
					if fighter.is_situation(*SITUATION_KIND_GROUND) {
						VarModule::on_flag(fighter.battle_object, vars::rosetta::status::GROUNDED_TELEPORT);
					}
				}
				if frame == 26.0 {	// perform the actual swap
					EFFECT(fighter, Hash40::new("rosetta_escape_end"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
					VarModule::set_int(fighter.battle_object, vars::rosetta::instance::ROSA_X, PostureModule::pos_x(boma) as i32);
					VarModule::set_int(fighter.battle_object, vars::rosetta::instance::ROSA_Y, PostureModule::pos_y(boma) as i32);
					VarModule::set_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE, 3);
					let tico_x = VarModule::get_int(fighter.battle_object, vars::rosetta::instance::TICO_X) as f32;
					let tico_y = VarModule::get_int(fighter.battle_object, vars::rosetta::instance::TICO_Y) as f32;
					let new_x = tico_x;
					let new_y = tico_y;
					let pos = Vector3f { x: new_x, y: new_y, z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
				}
				if frame == 27.0 {	// revert to normal state
					VisibilityModule::set_whole(boma, true);
					JostleModule::set_status(boma, true);	
					VarModule::set_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE, 4);
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				}
				if frame > 38.0 {
					if VarModule::is_flag(fighter.battle_object, vars::rosetta::status::GROUNDED_TELEPORT) {
						CancelModule::enable_cancel(boma);
					}
					else {
						StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
					}
				}
			}
		}
		else if MotionModule::frame(boma) > 13.0 && MotionModule::frame(boma) <= 17.0 && VarModule::get_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE) > 0 {
			// prevent the successful teleport logic if Luma is put into hitstun or killed during startup
			VarModule::set_int(fighter.battle_object, vars::rosetta::status::LUMA_STATE, 0);
			VarModule::on_flag(fighter.battle_object, vars::rosetta::status::IS_INVALID_TELEPORT);
		}
		else {
			HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
			JostleModule::set_status(boma, true);	
			VisibilityModule::set_whole(boma, true);
			if MotionModule::frame(boma) > 38.0 {
				VarModule::off_flag(fighter.battle_object, vars::rosetta::instance::IS_TICO_UNAVAILABLE);
				if VarModule::is_flag(fighter.battle_object, vars::rosetta::status::GROUNDED_TELEPORT) {
					CancelModule::enable_cancel(boma);
				}
				else {
					StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
				}
			}
		}
	}
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
		*FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_N_SHOOT,
		*FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_N_CHARGE,
		*FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_N_RETURN,
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    launch_star_cancel(boma, status_kind);
	teleport(fighter, boma, status_kind);
	fastfall_specials(fighter);
}

pub extern "C" fn rosetta_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
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

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, rosetta_frame_wrapper);
}
