// status imports
use super::*;
use globals::*;

macro_rules! interrupt {
    () => { return L2CValue::I32(1); };
    ($fighter:ident, $status:expr, $repeat:expr) => {{ $fighter.change_status($status.into(), $repeat.into()); interrupt!(); }}
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_Walk)]
unsafe fn status_pre_walk(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);

	let mut initial_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED);

	if ![*FIGHTER_STATUS_KIND_DASH].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
		//println!("not after dash");
		initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	}

	//println!("walk initial speed: {}", initial_speed);

	VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, initial_speed);

    call_original!(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_status_Walk_Common)]
unsafe fn status_walk_common(fighter: &mut L2CFighterCommon) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WALK_FLAG_SLIP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP);

    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
}

#[skyline::hook(replace = L2CFighterCommon_sub_walk_uniq_check)]
unsafe extern "C" fn sub_walk_uniq_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WALK_FLAG_SLIP) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP);
    }
    else {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WALK_FLAG_SLIP);
    }
    return 0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_Walk)]
unsafe fn status_walk(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_walk_common(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_walk_uniq_check(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(sub_walk_uniq_check as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(status_walk_main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_Walk_Main)]
unsafe extern "C" fn status_walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let arg1 = *FIGHTER_STATUS_KIND_WALK_BRAKE;
    let arg2 = fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor);
    let arg3 = true;
    let arg4 = 0;
    status_walk_main_common(fighter, arg1.into(), arg2.into(), arg3.into(), arg4.into());
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_Walk_Main_common)]
unsafe extern "C" fn status_walk_main_common(fighter: &mut L2CFighterCommon, arg1: L2CValue, arg2: L2CValue, arg3: L2CValue, arg4: L2CValue) -> L2CValue {
    let walk_accel_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_accel_mul"), 0);
    let walk_accel_add = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_accel_add"), 0);
    let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
    let walk_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_speed_max"), 0);
	let stick_x = fighter.global_table[STICK_X].get_f32();
	let prev_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED);
	let mut lr_modifier = 1.0;

	if [hash40("walk_slow_b"), hash40("walk_middle_b"), hash40("walk_fast_b")].contains(&MotionModule::motion_kind(fighter.module_accessor)) { // for auto-turn characters
		lr_modifier = -1.0;
	}

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
	let mut speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);

	if prev_speed * PostureModule::lr(fighter.module_accessor) * lr_modifier < 0.0 {
		let applied_speed = (stick_x.signum() * ((walk_accel_mul + (walk_accel_add * stick_x.abs())))) + prev_speed;
		//println!("negative speed");
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed - speed_motion);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, applied_speed);
	}
	else if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
	}

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	let speed_control = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	//println!("walk speed_control: {}", speed_control);
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
	speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
	//println!("run speed_motion: {}", speed_motion);
	//println!("walk total speed: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));

    call_original!(fighter, arg1, arg2, arg3, arg4)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_walk_common,
            sub_walk_uniq_check,
            status_walk_main,
            status_walk_main_common,
            status_pre_walk,
            status_walk
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);

    Agent::new("fighter")
        .status(Pre, *FIGHTER_STATUS_KIND_WALK, status_pre_walk)
        .status(Main, *FIGHTER_STATUS_KIND_WALK, status_walk)
        .install();
}