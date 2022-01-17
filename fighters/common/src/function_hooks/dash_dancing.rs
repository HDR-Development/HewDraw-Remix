use super::*;
use globals::*;

macro_rules! interrupt {
    () => { return L2CValue::I32(1); };
    ($fighter:ident, $status:expr, $repeat:expr) => {{ $fighter.change_status($status.into(), $repeat.into()); interrupt!(); }}
}

//===================================================================
//== TURN DASH SUB STATUS SCRIPT HOOK
//===================================================================

//Turn dash (runs once at the beginning of the status)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_TurnDash_Sub)]
pub unsafe fn status_turndash_sub_hook(fighter: &mut L2CFighterCommon) {
    let ret = original!()(fighter);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
	let mut f_agent = fighter.agent;
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);

	let mut initial_speed = VarModule::get_float(fighter.object(), vars::common::CURR_DASH_SPEED);

	if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::prev_status_kind(boma, 0)) {
		// println!("not after dash/turndash");
		initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
	}

	// println!("turndash total speed: {}", initial_speed);

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, initial_speed);
	app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
	
	VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, initial_speed);
	VarModule::set_int(fighter.object(), vars::common::TURN_DASH_FRAME, 0);
	VarModule::set_float(fighter.object(), vars::common::MOONWALK_SPEED, 0.0);

	ret
}

/***#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_TurnDash_Main)]
pub unsafe fn status_turndash_main_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!()(fighter);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
	let mut f_agent = fighter.agent;
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let dash_speed = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
	let run_accel_mul = WorkModule::get_param_float(boma, hash40("run_accel_mul"), 0);
	let run_accel_add = WorkModule::get_param_float(boma, hash40("run_accel_add"), 0);
	let prev_speed = VarModule::get_float(fighter.object(), vars::common::CURR_DASH_SPEED);
	let stick_x = fighter.global_table[hdr_modules::consts::globals::STICK_X].get_f32();
	let mut applied_speed = (dash_speed * PostureModule::lr(boma)) + (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;
	let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
	let td_frame = VarModule::get_int(fighter.object(), vars::common::TURN_DASH_FRAME);
    VarModule::set_int(fighter.object(), vars::common::TURN_DASH_FRAME, td_frame + 1);
	//println!("Current dash speed: {}", dash_speed);

	if VarModule::get_int(fighter.object(), vars::common::TURN_DASH_FRAME) == 2 && stick_x != 0.0 {
		println!("Changing current turndash speed");
		let applied_speed_clamped = clamp(applied_speed, -run_speed_max, run_speed_max);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, applied_speed_clamped);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, applied_speed_clamped);
	}
	if VarModule::get_int(fighter.object(), vars::common::TURN_DASH_FRAME) == 3 && stick_x != 0.0 {
		applied_speed = (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;
		let applied_speed_clamped = clamp(applied_speed, -run_speed_max, run_speed_max);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, applied_speed_clamped);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, applied_speed_clamped);
	}
	if VarModule::get_int(fighter.object(), vars::common::TURN_DASH_FRAME) == 4 {
		applied_speed = (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;
		let applied_speed_clamped = clamp(applied_speed, -run_speed_max, run_speed_max);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed_clamped);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
	}
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	let speed_control = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	println!("speed_control: {}", speed_control);
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
	let speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	println!("speed_motion: {}", speed_motion);
	println!("total speed: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL));
	//println!("Current turn dash velocity: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
	//println!("Turn Dash status script main");

	ret
}***/

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_TurnDash)]
pub unsafe fn status_turndash_end_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!()(fighter);
	let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
	let mut f_agent = fighter.agent;
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
	let mut initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
	let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);

	if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
		// println!("initial end turndash total speed: {}", initial_speed);

		fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
        app::sv_kinetic_energy::unable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, (initial_speed + (ground_brake * PostureModule::lr(boma))) * 0.25);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
	}
	else if is_moonwalk[get_player_number(boma)] {
		// println!("moonwalk off");
		is_moonwalk[get_player_number(boma)] = false;

		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
		let speed_stop = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
		let added_speed = speed_stop - (PostureModule::lr(fighter.module_accessor) * -2.0 * ground_brake);
		let added_speed_clamped = clamp(added_speed, -run_speed_max, run_speed_max);

		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, added_speed_clamped);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
	}

	VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, initial_speed);

	// println!("end turndash total speed: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL));

	ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Dash)]
pub unsafe fn status_dash_pre_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!()(fighter);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
	let mut f_agent = fighter.agent;
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
	let mut initial_speed = VarModule::get_float(fighter.object(), vars::common::CURR_DASH_SPEED);

	if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::prev_status_kind(boma, 0)) {
		// println!("not after dash/turndash");
		initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
	}

	// println!("dash total speed: {}", initial_speed);
	
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, initial_speed);
	app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

	VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, initial_speed);
	VarModule::set_int(fighter.object(), vars::common::TURN_DASH_FRAME, 0);
	VarModule::set_float(fighter.object(), vars::common::MOONWALK_SPEED, 0.0);

	ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Dash_Main_common)]
pub unsafe fn status_dash_main_common_hook(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
	let ret = original!()(fighter, arg);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
	let mut f_agent = fighter.agent;
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let run_accel_mul = WorkModule::get_param_float(boma, hash40("run_accel_mul"), 0);
    let run_accel_add = WorkModule::get_param_float(boma, hash40("run_accel_add"), 0);
    let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
    let dash_speed: f32 = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
	let mut pivot_boost: smash::phx::Vector3f = smash::phx::Vector3f {x: dash_speed * 0.75, y: 0.0, z: 0.0};
    let stick_x = fighter.global_table[hdr_modules::consts::globals::STICK_X].get_f32();
	let td_frame = VarModule::get_int(fighter.object(), vars::common::TURN_DASH_FRAME);
    VarModule::set_int(fighter.object(), vars::common::TURN_DASH_FRAME, td_frame + 1);
    // println!("frame: {}", VarModule::get_int(fighter.object(), vars::common::TURN_DASH_FRAME));

	if VarModule::get_int(fighter.object(), vars::common::TURN_DASH_FRAME) > 15 && WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_stick_x")) <= fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) && fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) < WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dash_stick_x")) && fighter.global_table[STICK_Y].get_f32() < -0.65 {
		//println!("sticky walk!");
		VarModule::on_flag(fighter.object(), vars::common::IS_STICKY_WALK);
		VarModule::on_flag(fighter.object(), vars::common::ENABLE_BOOST_RUN);
		interrupt!(fighter, FIGHTER_STATUS_KIND_RUN, true);
	}

    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_DASH {
        let prev_speed = VarModule::get_float(fighter.object(), vars::common::CURR_DASH_SPEED);
        let applied_speed = (dash_speed * PostureModule::lr(boma)) + (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;

        if VarModule::get_int(fighter.object(), vars::common::TURN_DASH_FRAME) == 1 && stick_x != 0.0 {
            // println!("Changing current dash speed: {}", applied_speed);
            let applied_speed_clamped = clamp(applied_speed, -run_speed_max, run_speed_max);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed_clamped);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let speed_control = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        // println!("speed_control: {}", speed_control);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
        let speed_stop = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        // println!("speed_stop: {}", speed_stop);
        // println!("total speed: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL));
    }
	// all characters call this common dash func during their turndashes
	else if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_TURN_DASH {
		// println!("dash within turndash");
		let prev_speed = VarModule::get_float(fighter.object(), vars::common::CURR_DASH_SPEED);
		let mut applied_speed = (dash_speed * PostureModule::lr(boma)) + (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;

		// perfect pivots
		if VarModule::get_int(fighter.object(), vars::common::TURN_DASH_FRAME) <= 2 && stick_x == 0.0 {
			if [*FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH].contains(&StatusModule::prev_status_kind(boma, 0)) && ![*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_TURN].contains(&StatusModule::prev_status_kind(boma, 1)) {
				PostureModule::reverse_lr(boma);
				KineticModule::clear_speed_all(boma);
				KineticModule::add_speed(boma, &pivot_boost);
				interrupt!(fighter, FIGHTER_STATUS_KIND_TURN, true);
			}
			else {
				PostureModule::reverse_lr(boma);
				interrupt!(fighter, FIGHTER_STATUS_KIND_TURN, true);
			}
		}
		if VarModule::get_int(fighter.object(), vars::common::TURN_DASH_FRAME) == 2 && stick_x != 0.0 {
			// println!("Changing current turndash speed");
			let applied_speed_clamped = clamp(applied_speed, -run_speed_max, run_speed_max);
			fighter.clear_lua_stack();
			lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, applied_speed_clamped);
			app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
			VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, applied_speed_clamped);
		}
		if VarModule::get_int(fighter.object(), vars::common::TURN_DASH_FRAME) == 3 && stick_x != 0.0 {
			applied_speed = (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;
			let applied_speed_clamped = clamp(applied_speed, -run_speed_max, run_speed_max);
			fighter.clear_lua_stack();
			lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, applied_speed_clamped);
			app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
			VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, applied_speed_clamped);
		}
		if VarModule::get_int(fighter.object(), vars::common::TURN_DASH_FRAME) == 4 {
			applied_speed = (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;
			let applied_speed_clamped = clamp(applied_speed, -run_speed_max, run_speed_max);
			fighter.clear_lua_stack();
			lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed_clamped);
			app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		}
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		let speed_control = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
		// println!("speed_control: {}", speed_control);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
		let speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
		// println!("speed_motion: {}", speed_motion);
		// println!("total speed: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL));
		//println!("Current turn dash velocity: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
		//println!("Turn Dash status script main");
	}

	ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_Dash)]
pub unsafe fn status_dash_end_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!()(fighter);
	let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
	let mut f_agent = fighter.agent;
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
	let mut initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
	let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);

	if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
		// println!("initial end dash total speed: {}", initial_speed);

		let mut applied_speed = (initial_speed + (-2.0 * ground_brake * PostureModule::lr(boma))) * 0.25;

		if FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true) {
			applied_speed = (initial_speed + (-2.0 * ground_brake * PostureModule::lr(boma)));
		}

		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

		initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
	}
	else if [*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_WALK].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
		// println!("run next");
		let applied_speed = (initial_speed + (-2.0 * ground_brake * PostureModule::lr(boma)));

		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

		initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
	}
	else if is_moonwalk[get_player_number(boma)] {
		// println!("moonwalk off");
		is_moonwalk[get_player_number(boma)] = false;

		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
		let speed_stop = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
		let added_speed = speed_stop - (PostureModule::lr(fighter.module_accessor) * -2.0 * ground_brake);
		let added_speed_clamped = clamp(added_speed, -run_speed_max, run_speed_max);

		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, added_speed_clamped);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
	}
	if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_RUN {
		VarModule::off_flag(fighter.object(), vars::common::ENABLE_BOOST_RUN);
	}
	
	VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, initial_speed);

	// println!("end dash total speed: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL));

	ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Walk)]
pub unsafe fn status_walk_pre_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
	let ret = original!()(fighter);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
	let mut f_agent = fighter.agent;
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);

	let mut initial_speed = VarModule::get_float(fighter.object(), vars::common::CURR_DASH_SPEED);

	if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::prev_status_kind(boma, 0)) {
		// println!("not after dash/turndash");
		initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
	}

	// println!("walk initial speed: {}", initial_speed);

	VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, initial_speed);

	ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Walk_Main_common)]
pub unsafe fn status_walk_main_common_hook(fighter: &mut L2CFighterCommon, arg: L2CValue, arg2: L2CValue, arg3: L2CValue, arg4: L2CValue) -> L2CValue {
	let ret = original!()(fighter, arg, arg2, arg3, arg4);
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let walk_accel_mul = WorkModule::get_param_float(boma, hash40("walk_accel_mul"), 0);
    let walk_accel_add = WorkModule::get_param_float(boma, hash40("walk_accel_add"), 0);
    let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
    let dash_speed: f32 = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let walk_speed_max = WorkModule::get_param_float(boma, hash40("walk_speed_max"), 0);
	let stick_x = fighter.global_table[hdr_modules::consts::globals::STICK_X].get_f32();
	let prev_speed = VarModule::get_float(fighter.object(), vars::common::CURR_DASH_SPEED);
	let mut lr_modifier = 1.0;

	if [hash40("walk_slow_b"), hash40("walk_middle_b"), hash40("walk_fast_b")].contains(&MotionModule::motion_kind(boma)) {
		lr_modifier = -1.0;
	}

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
	let mut speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);

	if prev_speed * PostureModule::lr(boma) * lr_modifier < 0.0 {
		let applied_speed = (stick_x.signum() * ((walk_accel_mul + (walk_accel_add * stick_x.abs())))) + prev_speed;
		// println!("negative speed");
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed - speed_motion);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, applied_speed);
	}
	else if KineticModule::is_enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
	}

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	let speed_control = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	// println!("walk speed_control: {}", speed_control);
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
	speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	// println!("run speed_motion: {}", speed_motion);
	// println!("walk total speed: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL));

	ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Run)]
pub unsafe fn status_run_pre_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!()(fighter);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
	let mut f_agent = fighter.agent;
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);

	let mut initial_speed = VarModule::get_float(fighter.object(), vars::common::CURR_DASH_SPEED);

	if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::prev_status_kind(boma, 0)) {
		// println!("not after dash/turndash");
		initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
	}

	// println!("run initial speed: {}", initial_speed);

	VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, initial_speed);

	ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Run_Main)]
pub unsafe fn status_run_main_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
	let ret = original!()(fighter);
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let run_accel_mul = WorkModule::get_param_float(boma, hash40("run_accel_mul"), 0);
    let run_accel_add = WorkModule::get_param_float(boma, hash40("run_accel_add"), 0);
    let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
    let dash_speed: f32 = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
	let stick_x = fighter.global_table[hdr_modules::consts::globals::STICK_X].get_f32();
	let prev_speed = VarModule::get_float(fighter.object(), vars::common::CURR_DASH_SPEED);

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
	let mut speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);

	if prev_speed * PostureModule::lr(boma) < 0.0 {
		let applied_speed = (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;
		// println!("negative speed");
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed - speed_motion);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, applied_speed);
	}
	else if KineticModule::is_enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
	}

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	let speed_control = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	// println!("run speed_control: {}", speed_control);
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
	speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	// println!("run speed_motion: {}", speed_motion);
	// println!("run total speed: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL));

	ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_TurnRun)]
pub unsafe fn status_turnrun_pre_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
	let ret = original!()(fighter);
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let mut initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
	
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, initial_speed);
	app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

	initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
	VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, initial_speed);

	// println!("pre turnrun total speed: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL));

	ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_TurnRun_Main)]
pub unsafe fn status_turnrun_main_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
	let ret = original!()(fighter);
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let run_accel_mul = WorkModule::get_param_float(boma, hash40("run_accel_mul"), 0);
    let run_accel_add = WorkModule::get_param_float(boma, hash40("run_accel_add"), 0);
    let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
	let stick_x = fighter.global_table[hdr_modules::consts::globals::STICK_X].get_f32();
	let prev_speed = VarModule::get_float(fighter.object(), vars::common::CURR_DASH_SPEED);

	if StatusModule::is_changing(boma) {
		let applied_speed = (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, applied_speed);
	}

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	let mut speed_control = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);

	if VarModule::is_flag(fighter.object(), vars::common::ENABLE_BOOST_RUN) && speed_control.abs() + (run_accel_mul + (run_accel_add * stick_x.abs())) > run_speed_max {
		// println!("bypass max run speed");
		let added_speed = stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())));

		fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
        app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP, added_speed);
        app::sv_kinetic_energy::add_speed(fighter.lua_state_agent);
	}

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	speed_control = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	// println!("turnrun speed_control: {}", speed_control);
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
	let speed_stop = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	// println!("turnrun speed_stop: {}", speed_stop);
	// println!("turnrun total speed: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL));

	ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_TurnRun)]
pub unsafe fn status_turnrun_end_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
	let ret = original!()(fighter);
	let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let mut initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);

	if KineticModule::is_enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP) {
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
	}

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, initial_speed);
	app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

	initial_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
	VarModule::set_float(fighter.object(), vars::common::CURR_DASH_SPEED, initial_speed);

	// println!("end turnrun total speed: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL));

	VarModule::off_flag(fighter.object(), vars::common::ENABLE_BOOST_RUN);

	ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_RunBrake)]
pub unsafe fn status_runbrake_end_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
	let ret = original!()(fighter);

	if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_TURN_RUN {
		VarModule::off_flag(fighter.object(), vars::common::ENABLE_BOOST_RUN);
	}

	ret
}

extern "C" {
    #[link_name = "\u{1}_ZN7lua2cpp14L2CFighterBase13change_statusEN3lib8L2CValueES2_"]
    fn change_status(fighter: &mut L2CFighterCommon, status_kind: L2CValue, arg3: L2CValue);
}

#[skyline::hook(replace=change_status)]
unsafe fn change_status_hook(fighter: &mut L2CFighterCommon, status: L2CValue, arg3: L2CValue) {
	let mut status_kind = status.get_i32();
	// println!("change status: {}", status_kind);
	original!()(fighter, status, arg3)
}