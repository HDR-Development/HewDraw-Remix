// status imports
use super::*;
use globals::*;

pub fn wm_param_to_int(boma: *mut app::BattleObjectModuleAccessor, param_cat: u64, param: u64, what: i32) {
    unsafe {
        WorkModule::set_int(
            boma,
            WorkModule::get_param_int(boma, param_cat, param),
            what
        );
    }
}

#[common_status_script(status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon15status_pre_DashEv")]
unsafe fn status_pre_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
	let mut initial_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED);

	if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
		//println!("not after dash/turn");
		initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	}

	//println!("pre dash total speed: {}", initial_speed);
	
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, initial_speed);
	app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, initial_speed);

    call_original!(fighter)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon17status_DashCommonEv")]
unsafe fn status_DashCommon(fighter: &mut L2CFighterCommon) {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    
    // added to hdr, not present in original
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW);


    wm_param_to_int(fighter.module_accessor, hash40("common"), hash40("turn_dash_frame"), *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FRAME);
    wm_param_to_int(fighter.module_accessor, hash40("common"), hash40("retry_turn_dash_frame"), *FIGHTER_STATUS_DASH_WORK_INT_RETRY_TURN_DASH_FRAME);
    wm_param_to_int(fighter.module_accessor, hash40("common"), hash40("dash_enable_attack_frame"), *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME);
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_RUN_BRAKE {
        let escape_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("run_brake_attack_escape_frame"));
        WorkModule::set_int(fighter.module_accessor, escape_frame - fighter.global_table[0x25].get_i32(), *FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME);
        if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME) {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT);
        }
    }
	VarModule::set_float(fighter.battle_object, vars::common::instance::MOONWALK_SPEED, 0.0);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::ENABLE_BOOST_RUN);
    VarModule::off_flag(fighter.battle_object, vars::common::status::IS_MOONWALK);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
    VarModule::off_flag(fighter.battle_object, vars::common::status::DISABLE_BACKDASH);
}


#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon15status_Dash_SubEv")]
unsafe fn status_Dash_sub(fighter: &mut L2CFighterCommon) {
    let value: f32 = if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_LANDING || fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_LANDING_LIGHT {
        6.0
    } else {
        0.0
    };

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("dash"), 0.0, 1.0, false, value, false, false);
    status_DashCommon(fighter);
}

macro_rules! interrupt {
    () => { return L2CValue::I32(1); };
    ($fighter:ident, $status:expr, $repeat:expr) => {{ $fighter.change_status($status.into(), $repeat.into()); interrupt!(); }}
}

macro_rules! interrupt_if {
    ($e:expr) => {{
        if $e { interrupt!() }
    }};
    ($fighter:ident, $status:expr, $repeat:expr, $e:expr) => {{
        if $e {
            interrupt!($fighter, $status, $repeat);
        }
    }}
}

macro_rules! ok {
    () => { return L2CValue::I32(0); };
}

macro_rules! ok_if {
    ($e:expr) => {{
        if $e { ok!() }
    }}
}

#[common_status_script(status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon11status_DashEv")]
unsafe fn status_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_Dash_sub(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_dash_main as *const () as _))
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon16status_Dash_MainEv")]
unsafe extern "C" fn status_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_dash_main_common(fighter, 0.into());
    0.into()
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon23status_Dash_Main_commonEN3lib8L2CValueE")]
unsafe extern "C" fn status_dash_main_common(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    let run_accel_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("run_accel_add"), 0);
    let run_accel_add = WorkModule::get_param_float(fighter.module_accessor, hash40("run_accel_mul"), 0);
    let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);

    let dash_speed: f32 = WorkModule::get_param_float(fighter.module_accessor, hash40("dash_speed"), 0);
    let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);
    let dash_stick_x: f32 = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dash_stick_x"));
    let stick_x = fighter.global_table[STICK_X].get_f32();

    if fighter.global_table[DASH_CALLBACK].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[DASH_CALLBACK].get_ptr());
        interrupt_if!(callable(fighter).get_bool());
    }

    interrupt_if!(fighter, FIGHTER_STATUS_KIND_FALL, false, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);

    interrupt_if!(CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_wait_ground_check_common(false.into()).get_bool());

    interrupt_if!(fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool());

    let can_s4 = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) < WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x206138766c)
    } else {
        true
    };

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && can_s4 {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        app::sv_module_access::item(fighter.lua_state_agent);
        let should_throw = if fighter.pop_lua_stack(1).get_bool() {
            true
        } else {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
            app::sv_module_access::item(fighter.lua_state_agent);
            if fighter.pop_lua_stack(1).get_bool() {
                ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0
            } else {
                false
            }
        };
        interrupt_if!(fighter, FIGHTER_STATUS_KIND_ITEM_THROW, false, should_throw);
    }

    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ITEM_THROW_DASH,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH) &&
        {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
            app::sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } &&
        fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    );

    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ITEM_THROW,
        false,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE) &&
        ItemModule::is_have_item(fighter.module_accessor, 0) &&
        fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 &&
        {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
            app::sv_module_access::item(fighter.lua_state_agent);
            !fighter.pop_lua_stack(1).get_bool()
        }
    );

    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ITEM_THROW_DASH,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH) &&
        ItemModule::is_have_item(fighter.module_accessor, 0) &&
        fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 &&
        {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
            app::sv_module_access::item(fighter.lua_state_agent);
            !fighter.pop_lua_stack(1).get_bool()
        }
    );

    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_CATCH_TURN,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) &&
        fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x")) &&
        fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0  &&
        !ItemModule::is_have_item(fighter.module_accessor, 0)
    );

    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_CATCH_DASH,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) &&
        fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 &&
        !ItemModule::is_have_item(fighter.module_accessor, 0)
    );

    // present in the original, removed in hdr
    /*
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ESCAPE_B,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) &&
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B) &&
        fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B != 0
    );

    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ESCAPE_F,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) &&
        ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) &&
        !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_ESCAPE)
    );
    */

    interrupt_if!(fighter.sub_transition_group_check_ground_guard().get_bool());
    interrupt_if!(fighter.sub_transition_group_check_special_command().get_bool());
    interrupt_if!(fighter.sub_transition_group_check_ground_special().get_bool());
    interrupt_if!(fighter.sub_transition_specialflag_hoist().get_bool());
    
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ITEM_SWING_S4_START,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4) &&
        {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
            app::sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } &&
        fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0 &&
        !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4)
    );

    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ITEM_SWING_S4_START,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4) &&
        {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
            app::sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } &&
        fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0 &&
        !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4)
    );

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4) 
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        app::sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    }
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        if ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0 {
            interrupt!(fighter, FIGHTER_STATUS_KIND_ITEM_THROW, false);
        } else {
            interrupt!(fighter, FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT, true);
        }
    }

    interrupt_if!(
        fighter.global_table[0x57].get_bool() &&
        std::mem::transmute::<*const skyline::libc::c_void, extern "C" fn(&mut L2CFighterCommon) -> L2CValue>(
            fighter.global_table[0x57].get_ptr()
        )(fighter).get_bool()
    );

    // dash startup -> fsmash leniency window
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ATTACK_S4_START,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START)
            && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0  
            && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4)
    );

    // dash startup -> ftilt leniency window for tilt attack button, just like fsmash
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3)
       && fighter.is_cat_flag(CatHdr::TiltAttack)
       && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        KineticModule::mul_speed(fighter.boma(), &Vector3f::new(0.0, 0.0, 0.0), *FIGHTER_KINETIC_ENERGY_ID_STOP);
        interrupt!(fighter, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
    }
    
    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ITEM_SWING_DASH,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH) &&
        {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
            app::sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } &&
        fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    );

    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_ATTACK_DASH,
        true,
        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH) &&
        fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    );

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

    interrupt_if!(fighter.sub_transition_group_check_ground_attack().get_bool());

    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME) > 0
    && (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    || app::FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, false)) {
        if fighter.global_table[CMD_CAT1].get_i32() & (
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
        ) != 0 {
            if {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY);
                app::sv_module_access::item(fighter.lua_state_agent);
                fighter.pop_lua_stack(1).get_bool()
            }
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH)
            && !ItemModule::is_have_item(fighter.module_accessor, 0) {
                interrupt!(fighter, FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, true);
            } else if ItemModule::get_pickable_item_size(fighter.module_accessor) == ITEM_SIZE_LIGHT
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH) &&
            {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM);
                app::sv_module_access::item(fighter.lua_state_agent);
                fighter.pop_lua_stack(1).get_bool()
            } {
                interrupt!(fighter, FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, true);
            }
        }
    }

    let is_dash_input: bool = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0;  // we register a dash input by 1. Using game's command cat dash check, or 2. Checking if cstick has been input and is > 0.6 (max cstick x value is 0.625)
    let is_backdash_input: bool = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0;

    if fighter.global_table[CURRENT_FRAME].get_i32() > 15  // if after frame 15 of dash (meant to be after dash-to-run transition frame but haven't found a way to pull that for each character) 
    && WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_stick_x")) <= fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor)  // AND stick_x is >= run threshold
    && !is_dash_input {  // AND you haven't input another dash
        //println!("sticky walk");
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_STICKY_WALK);
		VarModule::on_flag(fighter.battle_object, vars::common::instance::ENABLE_BOOST_RUN);
        interrupt!(fighter, FIGHTER_STATUS_KIND_RUN, true);
    }

    if fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) >= 0.0 {  // if stick is no longer backwards
        if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_MOONWALK) && KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP) {
            //println!("moonwalk off");
            if !is_dash_input {
                //println!("no dash input");
                // if anything but another dash input, we apply the last frame's moonwalk speed to the next status
                VarModule::off_flag(fighter.battle_object, vars::common::status::IS_MOONWALK);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
                let speed_stop = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
                let added_speed = speed_stop - (PostureModule::lr(fighter.module_accessor) * -2.0 * ground_brake);
                let added_speed_clamped = added_speed.clamp(-run_speed_max, run_speed_max);

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
        }
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH)  // if backdash transition term is enabled (it is by default)
    && is_backdash_input {  // AND is a backdash input
        //println!("transition to backdash");
        if fighter.global_table[CURRENT_FRAME].get_i32() <= 2 {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
        }
        else {
            VarModule::off_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
        }
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
        interrupt!(fighter, FIGHTER_STATUS_KIND_TURN, true);
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH)
    && WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("re_dash_frame")) as f32 <= MotionModule::frame(fighter.module_accessor)  // if current frame is after redash frame
    && is_dash_input {  // AND is a dash input
        // println!("transition to dash");
        interrupt!(fighter, FIGHTER_STATUS_KIND_DASH, true);
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN) {
        if WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_stick_x")) <= fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_RUN) {
                interrupt!(fighter, FIGHTER_STATUS_KIND_WALK, true);
            } else {
                interrupt!(fighter, FIGHTER_STATUS_KIND_RUN, true);
            }
        }
    }

    interrupt_if!(fighter.sub_transition_group_check_ground_jump().get_bool());

    interrupt_if!(
        arg.get_bool() &&
        std::mem::transmute::<*const skyline::libc::c_void, extern "C" fn(&mut L2CFighterCommon) -> L2CValue>(
            arg.get_ptr()
        )(fighter).get_bool()
    );

    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_WALK_BRAKE,
        false,
        GroundModule::get_down_friction(fighter.module_accessor) < 1.0 &&
        FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true)
    );

    interrupt_if!(
        fighter,
        FIGHTER_STATUS_KIND_WAIT,
        false,
        MotionModule::is_end(fighter.module_accessor)
    );

    interrupt_if!(fighter.sub_ground_check_stop_wall().get_bool());

    // f3 perfect pivots
    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_TURN 
    && StatusModule::prev_status_kind(fighter.module_accessor, 1) == *FIGHTER_STATUS_KIND_DASH  // if you are in a backdash
    && fighter.global_table[CURRENT_FRAME].get_i32() == 1  // AND you are on f2 of current dash
    && stick_x.abs() < dash_stick_x {  // AND stick_x < dash stick threshold
        // trigger late perfect pivot
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_LATE_PIVOT);
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        interrupt!(fighter, FIGHTER_STATUS_KIND_TURN, true);
    }

    // moonwalk stuff
    if !is_dash_input
    && fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) < 0.0
    && !VarModule::is_flag(fighter.battle_object, vars::common::status::IS_MOONWALK) {
        //println!("moonwalk start");
        VarModule::on_flag(fighter.battle_object, vars::common::status::IS_MOONWALK);
    }

    if !is_dash_input  // if not a dash input
    && fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) < 0.0  // AND stick is backwards
    && VarModule::is_flag(fighter.battle_object, vars::common::status::IS_MOONWALK) {  // AND moonwalk has been triggered
        // apply moonwalk speed
        let mut prev_speed = 0.0;
        if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            prev_speed = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        }
        else {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
            prev_speed = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        }
        let added_speed = fighter.global_table[STICK_X].get_f32().signum() * ((run_accel_mul + (run_accel_add * fighter.global_table[STICK_X].get_f32().abs())));
        let moonwalk_speed = prev_speed + added_speed;
        let moonwalk_speed_clamped = moonwalk_speed.clamp(-run_speed_max, run_speed_max);

        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        app::sv_kinetic_energy::unable(fighter.lua_state_agent);

        //println!("moonwalkin: {}", moonwalk_speed);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
        app::sv_kinetic_energy::enable(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP, moonwalk_speed_clamped);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
    //fighter.clear_lua_stack();
    //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    //let speed_control = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    //println!("speed_control: {}", speed_control);
    //fighter.clear_lua_stack();
    //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
    //let speed_stop = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    //println!("speed_stop: {}", speed_stop);
    //println!("total speed: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));

    ok!()
}

#[common_status_script(status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon15status_end_DashEv")]
unsafe fn status_end_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
	let mut initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);

    if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
        app::sv_kinetic_energy::unable(fighter.lua_state_agent);
    }
	else if [*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_WALK].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
		//println!("run next");
        let applied_speed_clamped = initial_speed.clamp(-run_speed_max, run_speed_max);

		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
		app::sv_kinetic_energy::unable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::enable(fighter.lua_state_agent);
		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed_clamped);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

		initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	}
	else if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_MOONWALK) {
		//println!("moonwalk off");
		VarModule::off_flag(fighter.battle_object, vars::common::status::IS_MOONWALK);

		fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
		let speed_stop = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
		let added_speed = speed_stop + (ground_brake * PostureModule::lr(fighter.module_accessor));
		let added_speed_clamped = added_speed.clamp(-run_speed_max, run_speed_max);

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
		VarModule::off_flag(fighter.battle_object, vars::common::instance::ENABLE_BOOST_RUN);
	}
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_TURN {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
    }
	
	VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, initial_speed);

	//println!("end dash total speed: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));

    call_original!(fighter)
}


// FIGHTER_STATUS_KIND_TURN_DASH


#[common_status_script(status = FIGHTER_STATUS_KIND_TURN_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon19status_pre_TurnDashEv")]
unsafe fn status_pre_turndash(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("pre turndash");
    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN);
    return 1.into()
}


pub fn install() {
    install_hooks!(
        status_DashCommon,
        status_Dash_sub,
        status_dash_main,
        status_dash_main_common
    );

    install_status_scripts!(
        status_pre_dash,
        status_pre_turndash,
        status_dash,
        status_end_dash
    );
}