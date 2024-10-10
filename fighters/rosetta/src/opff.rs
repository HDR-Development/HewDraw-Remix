// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

//Launch Star Cancel
unsafe fn launch_star_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_JUMP
	&& MotionModule::frame(boma) > 2.0 {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_END, false);
        }
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

use vars::common::instance::GIMMICK_TIMER;
use vars::rosetta::instance::*;
use vars::rosetta::status::*;

// down special teleport
unsafe fn teleport(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
	// handle the cooldown timer
	let cooldown_frame = VarModule::get_int(boma.object(), GIMMICK_TIMER);
	if cooldown_frame > 0 { VarModule::dec_int(boma.object(), GIMMICK_TIMER); }
	if cooldown_frame == 1 {
		gimmick_flash(boma);
	}

	// set the conditions for a successful teleport
	let can_teleport = !VarModule::is_flag(boma.object(), SPECIAL_LW_TICO_UNAVAILABLE) && cooldown_frame == 0;
	let warp_effect = VarModule::get_int(boma.object(), SPECIAL_LW_WARP_EFFECT_HANDLE);

	// makes rosalina's wand glow if teleport is available
	if can_teleport
	&& !EffectModule::is_exist_effect(boma, warp_effect as u32) {
		let eff_offset = &Vector3f::new(0.0, 8.0, 0.0);
		let handle = EffectModule::req_follow(boma, Hash40::new("sys_status_all_up"), Hash40::new("havel"), eff_offset, &Vector3f::zero(), 0.28, false, 0, 0, 0, 0, 0, false, false) as u32;
		EffectModule::set_rate(boma, handle, 0.5);
		VarModule::set_int(boma.object(), SPECIAL_LW_WARP_EFFECT_HANDLE, handle as i32);
	} else {
		if EffectModule::is_exist_effect(boma, warp_effect as u32) 
		& !can_teleport {
			EffectModule::kill(boma, warp_effect as u32, false, false);
		}
	}
	
	if !fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) { return; }

	if !can_teleport {
		// prevent the successful teleport logic if Luma is put into hitstun or killed during startup
		if (13.0..17.0).contains(&frame) {
			VarModule::on_flag(boma.object(), SPECIAL_LW_INVALID_WARP);
		}
	}

	// transition rosalina to special fall after a successful aerial teleport
	if frame > 38.0 
	&& !VarModule::is_flag(boma.object(), SPECIAL_LW_INVALID_WARP) 
	&& !VarModule::is_flag(boma.object(), SPECIAL_LW_WARP_GROUND_START) {
		//println!("successful aerial teleport. entering special fall");
		StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
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
	teleport(fighter, boma, frame);
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
