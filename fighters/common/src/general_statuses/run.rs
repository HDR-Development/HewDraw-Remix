// status imports
use super::*;
use globals::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon14status_pre_RunEv")]
unsafe fn status_pre_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);

	let mut initial_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED);

	if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
		//println!("not after dash/turn");
		initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	}

	//println!("run initial speed: {}", initial_speed);

	VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, initial_speed);

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

    let mut hip_translate = Vector3f::zero();
    MotionModule::joint_local_tra(fighter.module_accessor, Hash40::new("hip"), false, &mut hip_translate);
    VarModule::set_float(fighter.battle_object, vars::common::instance::RUN_HIP_OFFSET_X, hip_translate.z);
    let dash_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::DASH_HIP_OFFSET_X);
    hip_translate.z += dash_hip_offset_x - hip_translate.z;
    ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("hip"), &Vector3f{ x: hip_translate.x, y: hip_translate.y, z: hip_translate.z }, false, false);

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
    let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);
	let stick_x = fighter.global_table[STICK_X].get_f32();
	let prev_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED);

	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
	let mut speed_motion = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);

	if prev_speed * PostureModule::lr(fighter.module_accessor) < 0.0 {
		let added_speed = (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;
		let applied_speed = added_speed - speed_motion;
		let applied_speed_clamped = applied_speed.clamp(-run_speed_max, run_speed_max);

		//println!("negative speed");
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed_clamped);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
		VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, added_speed);
	}
	else if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
	}

    let ret = call_original!(fighter);

    if ret.get_i32() != 0 {
        if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U)
            && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0)
        || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW)
            && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0)
        || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S)
            && (fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0
            || fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0))
        && {
            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be));
            app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            interrupt!(fighter, *FIGHTER_STATUS_KIND_APPEAL, false);
        }
    }

    ret
}

#[common_status_script(status = FIGHTER_STATUS_KIND_RUN_BRAKE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon15status_RunBrakeEv")]
unsafe fn status_runbrake(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_RunBrake();

    let dash_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::DASH_HIP_OFFSET_X);
    let run_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::RUN_HIP_OFFSET_X);
    let mut hip_translate = Vector3f::zero();
    MotionModule::joint_local_tra(fighter.module_accessor, Hash40::new("hip"), false, &mut hip_translate);
    hip_translate.z += dash_hip_offset_x - run_hip_offset_x;
    ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("hip"), &Vector3f{ x: hip_translate.x, y: hip_translate.y, z: hip_translate.z }, false, false);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(status_runbrake_main as *const () as _))
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon20status_RunBrake_MainEv")]
unsafe fn status_runbrake_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0)
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0)
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S)
        && (fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0
        || fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0))
    && {
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be));
        app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } {
        interrupt!(fighter, *FIGHTER_STATUS_KIND_APPEAL, false);
    }
    if fighter.is_parry_input() {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_GUARD_OFF, true);
        VarModule::on_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
        return true.into()
    }

	call_original!(fighter)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_TURN_RUN_BRAKE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon19status_TurnRunBrakeEv")]
unsafe fn status_turnrunbrake(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_TurnRunBrake_Sub();
    let dash_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::DASH_HIP_OFFSET_X);
    let run_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::RUN_HIP_OFFSET_X);
    let mut hip_translate = Vector3f::zero();
    MotionModule::joint_local_tra(fighter.module_accessor, Hash40::new("hip"), false, &mut hip_translate);
    hip_translate.z += (dash_hip_offset_x - run_hip_offset_x) * 0.5;
    ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("hip"), &Vector3f{ x: hip_translate.x, y: hip_translate.y, z: hip_translate.z }, false, false);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(status_turnrunbrake_main as *const () as _))
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon24status_TurnRunBrake_MainEv")]
unsafe fn status_turnrunbrake_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_parry_input() {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_GUARD_OFF, true);
        VarModule::on_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
        return true.into()
    }

	call_original!(fighter)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_TURN_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon14status_TurnRunEv")]
unsafe fn status_turnrun(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_TurnRun_Sub(L2CValue::Hash40s("turn_run"), L2CValue::Bool(true));

    if fighter.is_prev_status(*FIGHTER_STATUS_KIND_RUN_BRAKE) {
        let dash_hip_offset_x = VarModule::get_float(fighter.object(), vars::common::instance::DASH_HIP_OFFSET_X);
        let run_hip_offset_x = VarModule::get_float(fighter.object(), vars::common::instance::RUN_HIP_OFFSET_X);
        let mut hip_translate = Vector3f::zero();
        MotionModule::joint_local_tra(fighter.module_accessor, Hash40::new("hip"), false, &mut hip_translate);
        hip_translate.z += dash_hip_offset_x - run_hip_offset_x;
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("hip"), &Vector3f{ x: hip_translate.x, y: hip_translate.y, z: hip_translate.z }, false, false);
    }
    
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_TurnRun_Main as *const () as _))
}

pub fn install() {
    install_hooks!(
        status_run_sub,
        status_run_main,
        status_runbrake_main,
        status_turnrunbrake_main,
    );

    install_status_scripts!(
        status_pre_run,
        status_run,
        status_runbrake,
        status_turnrunbrake,
        status_turnrun,
    );
}