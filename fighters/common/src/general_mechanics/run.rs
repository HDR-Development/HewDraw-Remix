// status imports
use super::*;
use globals::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon14status_pre_RunEv")]
unsafe fn status_pre_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);

	let mut initial_speed = VarModule::get_float(fighter.battle_object, vars::common::CURR_DASH_SPEED);

	if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
		//println!("not after dash/turn");
		initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	}

	//println!("run initial speed: {}", initial_speed);

	VarModule::set_float(fighter.battle_object, vars::common::CURR_DASH_SPEED, initial_speed);

    call_original!(fighter)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon14status_Run_SubEv")]
unsafe fn status_run_sub(fighter: &mut L2CFighterCommon) {
    let value: f32 = if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_DASH || fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_TURN {
        WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_RUN_WORK_FLOAT_START_FRAME)
    } else {
        0.0
    };
    
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("run"), value, 1.0, false, 0.0, false, false);

    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_GEKIKARA_RUN_BRAKE);
    }

    // new to hdr
    if !VarModule::is_flag(fighter.battle_object, vars::common::IS_STICKY_WALK) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW);
}

#[common_status_script(status = FIGHTER_STATUS_KIND_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon10status_RunEv")]
unsafe fn status_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_run_sub(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_run_main as *const () as _))
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon15status_Run_MainEv")]
unsafe extern "C" fn status_run_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let run_accel_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("run_accel_add"), 0);
    let run_accel_add = WorkModule::get_param_float(fighter.module_accessor, hash40("run_accel_mul"), 0);
    let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
    let dash_speed: f32 = WorkModule::get_param_float(fighter.module_accessor, hash40("dash_speed"), 0);
    let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);
	let stick_x = fighter.global_table[hdr_modules::consts::globals::STICK_X].get_f32();
	let prev_speed = VarModule::get_float(fighter.battle_object, vars::common::CURR_DASH_SPEED);

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
	let mut speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);

	if prev_speed * PostureModule::lr(fighter.module_accessor) < 0.0 {
		let added_speed = (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;
		let applied_speed = added_speed - speed_motion;
		let applied_speed_clamped = clamp(applied_speed, -run_speed_max, run_speed_max);

		//println!("negative speed");
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed_clamped);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		VarModule::set_float(fighter.battle_object, vars::common::CURR_DASH_SPEED, added_speed);
	}
	else if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
	}

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	let speed_control = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	//println!("run speed_control: {}", speed_control);
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
	speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	//println!("run speed_motion: {}", speed_motion);
	//println!("run total speed: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));

    call_original!(fighter)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon14status_end_RunEv")]
unsafe fn status_end_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ![*FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
        VarModule::off_flag(fighter.battle_object, vars::common::ENABLE_BOOST_RUN);
    }
	if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_RUN_BRAKE {
		VarModule::off_flag(fighter.battle_object, vars::common::IS_STICKY_WALK);
	}
    0.into()
}

#[common_status_script(status = FIGHTER_STATUS_KIND_TURN_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon18status_pre_TurnRunEv")]
unsafe fn status_pre_turnrun(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, initial_speed);
	app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

	initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	VarModule::set_float(fighter.battle_object, vars::common::CURR_DASH_SPEED, initial_speed);

	//println!("pre turnrun total speed: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));

	call_original!(fighter)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_TURN_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon14status_TurnRunEv")]
unsafe fn status_turnrun(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_TurnRun_Sub(L2CValue::Hash40(Hash40::new_raw(0x8afd9b84f)), L2CValue::Bool(true));
    fighter.sub_shift_status_main(L2CValue::Ptr(status_turnrun_main as *const () as _))
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon19status_TurnRun_MainEv")]
unsafe extern "C" fn status_turnrun_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let run_accel_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("run_accel_add"), 0);
    let run_accel_add = WorkModule::get_param_float(fighter.module_accessor, hash40("run_accel_mul"), 0);
    let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);
	let stick_x = fighter.global_table[hdr_modules::consts::globals::STICK_X].get_f32();
	let prev_speed = VarModule::get_float(fighter.battle_object, vars::common::CURR_DASH_SPEED);

	if StatusModule::is_changing(fighter.module_accessor) {
		let applied_speed = (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;
		let applied_speed_clamped = clamp(applied_speed, -run_speed_max, run_speed_max);

		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		VarModule::set_float(fighter.battle_object, vars::common::CURR_DASH_SPEED, applied_speed);
	}

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	let mut speed_control = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);

	if VarModule::is_flag(fighter.battle_object, vars::common::ENABLE_BOOST_RUN) && speed_control.abs() + (run_accel_mul + (run_accel_add * stick_x.abs())) > run_speed_max {
		//println!("bypass max run speed");
		let added_speed = stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())));

		fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, added_speed);
        app::sv_kinetic_energy::add_speed(fighter.lua_state_agent);
	}

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	speed_control = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	//println!("turnrun speed_control: {}", speed_control);
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
	let speed_stop = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	//println!("turnrun speed_stop: {}", speed_stop);
	//println!("turnrun total speed: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));

	call_original!(fighter)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_TURN_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon18status_end_TurnRunEv")]
unsafe fn status_end_turnrun(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);

	if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) {
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
	}

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, initial_speed);
	app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

	initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	VarModule::set_float(fighter.battle_object, vars::common::CURR_DASH_SPEED, initial_speed);

	//println!("end turnrun total speed: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));

	VarModule::off_flag(fighter.battle_object, vars::common::ENABLE_BOOST_RUN);

	call_original!(fighter)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_RUN_BRAKE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon19status_end_RunBrakeEv")]
unsafe fn status_end_runbrake(fighter: &mut L2CFighterCommon) -> L2CValue {
	//println!("end runbrake");
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_TURN_RUN {
		VarModule::off_flag(fighter.battle_object, vars::common::ENABLE_BOOST_RUN);
	}
	VarModule::off_flag(fighter.battle_object, vars::common::IS_STICKY_WALK);

	call_original!(fighter)
}

pub fn install() {
    install_hooks!(
        status_run_sub,
        status_run_main,
        status_turnrun_main
    );

    install_status_scripts!(
        status_pre_run,
        status_run,
        status_end_run,
        status_pre_turnrun,
		status_turnrun,
        status_end_turnrun,
        status_end_runbrake
    );
}