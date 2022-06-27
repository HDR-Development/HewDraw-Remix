use utils::{
    *,
    ext::*,
    consts::*,
	consts::globals::*
};
use crate::misc::calc_melee_momentum;
use smash_script::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::*;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::app::{self, lua_bind::*, sv_kinetic_energy, sv_animcmd};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;


//===================================================================
//== MOMENTUM TRANSFER HELPER
//== Performs some extra calculations to help with momentum handling
//===================================================================
pub unsafe fn momentum_transfer_helper(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, curr_frame: f32) {

	if (fighter_kind == *FIGHTER_KIND_PFUSHIGISOU && status_kind == *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_LW_OUT)
		|| (fighter_kind == *FIGHTER_KIND_PLIZARDON && status_kind == *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_LW_OUT)
		|| (fighter_kind == *FIGHTER_KIND_PZENIGAME && status_kind == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_LW_OUT)
		|| (fighter_kind == *FIGHTER_KIND_EFLAME && status_kind == *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_OUT)
		|| (fighter_kind == *FIGHTER_KIND_ELIGHT && status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_OUT) 
		|| status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
		let ratio = (WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0) / WorkModule::get_param_float(boma, hash40("run_speed_max"), 0));
		VarModule::set_float(fighter.battle_object, vars::common::instance::JUMP_SPEED_RATIO, ratio);
	}

	if [*FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status_kind) {
        VarModule::set_float(boma.object(), vars::common::instance::RAR_LENIENCY, (0.8*(MotionModule::end_frame(boma) - MotionModule::frame(boma)*2.0 + 6.0)/MotionModule::end_frame(boma)).clamp(0.1, 0.8)); // You have a limited amount of time to get full RAR momentum from turn brake or run brake, with a 3F leniency
    }

    if situation_kind == *SITUATION_KIND_GROUND {
        VarModule::set_float(boma.object(), vars::common::instance::GROUND_VEL, KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
    }

	if fighter_kind == *FIGHTER_KIND_PICKEL && [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT].contains(&status_kind) && VarModule::get_int(boma.object(), vars::common::instance::JUMP_SQUAT_FRAME) < WorkModule::get_param_int(boma, hash40("jump_squat_frame"), 0) {
		VarModule::set_float(boma.object(), vars::common::instance::JUMPSQUAT_VELOCITY, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
	}

	if fighter_kind == *FIGHTER_KIND_TANTAN && [*FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT].contains(&status_kind) && VarModule::get_int(boma.object(), vars::common::instance::JUMP_SQUAT_FRAME) < WorkModule::get_param_int(boma, hash40("jump_squat_frame"), 0) {
		VarModule::set_float(boma.object(), vars::common::instance::JUMPSQUAT_VELOCITY, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
	}

	if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_JUMP {
		if !VarModule::is_flag(boma.object(), vars::common::instance::JUMP_NEXT) {
			VarModule::on_flag(boma.object(), vars::common::instance::JUMP_NEXT);
			/* Moves that should bypass the momentum logic (in terms of the jump status script) */
			const MOMENTUM_EXCEPTION_MOVES: [smash::lib::LuaConst ; 1] = [
				FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP
			];

			if !MOMENTUM_EXCEPTION_MOVES.iter().any(|x| *x == fighter.global_table[FIGHTER_KIND] ) {
				let mut new_speed = calc_melee_momentum(fighter, false, false, false);
				fighter.clear_lua_stack();
				lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, new_speed);
				app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
				fighter.clear_lua_stack();
				//println!("Post-jump horizontal velocity: {}", new_speed);
				VarModule::set_float(fighter.battle_object, vars::common::instance::CURRENT_MOMENTUM, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN)); // Set the current momentum to what was just calculated
			}
		}
	}
	else {
		VarModule::off_flag(boma.object(), vars::common::instance::JUMP_NEXT);
	}
}

unsafe fn additional_momentum_transfer_moves(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, curr_frame: f32) {

    /*      ADDITIONAL MOVES THAT SHOULD CONSERVE MOMENTUM       */

    if situation_kind == *SITUATION_KIND_AIR && curr_frame <= 1.0 {

        //characters whose neutral special should conserve momentum
        let should_conserve_special_momentum =
        ( [*FIGHTER_KIND_MARIO, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_GANON]
          .contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N )
        || ( fighter_kind == *FIGHTER_KIND_DIDDY && [*FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_SHOOT].contains(&status_kind) )
        || ( fighter_kind == *FIGHTER_KIND_KIRBY && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK].contains(&status_kind) );

		if should_conserve_special_momentum && (KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN)).abs() > 0.0 {
			if *FIGHTER_STATUS_KIND_JUMP == StatusModule::prev_status_kind(boma, 0) {
				let new_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURRENT_MOMENTUM_SPECIALS);
				fighter.clear_lua_stack();
				lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, new_speed);
				app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
				fighter.clear_lua_stack();
			}
        }

    }
}


pub unsafe fn run(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    let curr_frame = MotionModule::frame(boma);
    momentum_transfer_helper(fighter, lua_state, l2c_agent, boma, status_kind, situation_kind, fighter_kind, curr_frame);
    additional_momentum_transfer_moves(fighter, lua_state, l2c_agent, boma, status_kind, situation_kind, fighter_kind, curr_frame);
}
