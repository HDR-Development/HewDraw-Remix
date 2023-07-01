use super::*;
use globals::*;
use smashline::*;

pub fn install() {
  install_status_scripts!(
      pre_dash
  );
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
	let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
	let mut initial_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED);

	if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
		// println!("not after dash/turndash");
		initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	}

	// println!("dash total speed: {}", initial_speed);
	
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, initial_speed);
	app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

	VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, initial_speed);
	VarModule::set_float(fighter.battle_object, vars::common::instance::MOONWALK_SPEED, 0.0);

    original!(fighter)
}