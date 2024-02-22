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

#[skyline::hook(replace = L2CFighterCommon_status_DashCommon)]
unsafe fn status_DashCommon(fighter: &mut L2CFighterCommon) {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
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
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
}

#[skyline::hook(replace = L2CFighterCommon_status_Dash_Sub)]
unsafe fn status_Dash_sub(fighter: &mut L2CFighterCommon) {
    let value: f32 = if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_LANDING || fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_LANDING_LIGHT {
        6.0
    } else {
        0.0
    };

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("dash"), 0.0, 1.0, false, value, false, false);
    fighter.status_DashCommon();
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

#[skyline::hook(replace = L2CFighterCommon_status_Dash)]
unsafe fn status_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Dash_Sub();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Dash_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_Dash_Main_common)]
unsafe extern "C" fn status_dash_main_common(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
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
    
    if fighter.sub_transition_group_check_special_command().get_bool()
    || fighter.sub_transition_group_check_ground_special().get_bool()
    || fighter.sub_transition_specialflag_hoist().get_bool()
    {
        VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
        return L2CValue::I32(1);
    }

    // dash startup -> ftilt leniency window for tilt attack button, just like fsmash
    if fighter.is_button_trigger(Buttons::TiltAttack)
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4)
    {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S3.into(), true.into());
        VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
        return L2CValue::I32(1);
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4)
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
        app::sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    }
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4)
    {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_S4_START.into(), true.into());
        VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
        return L2CValue::I32(1);
    }

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
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT.into(), true.into());
        }
        VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
        return L2CValue::I32(1);
    }

    interrupt_if!(
        fighter.global_table[0x57].get_bool() &&
        std::mem::transmute::<*const skyline::libc::c_void, extern "C" fn(&mut L2CFighterCommon) -> L2CValue>(
            fighter.global_table[0x57].get_ptr()
        )(fighter).get_bool()
    );

    // dash startup -> fsmash leniency window
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START)
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0  
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4)
    {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_START.into(), true.into());
        VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
        return L2CValue::I32(1);
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0
    {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
        return L2CValue::I32(1);
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

    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("pass_flick_y"));
    if GroundModule::is_passable_ground(fighter.module_accessor)
    && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y
    && fighter.global_table[STICK_Y].get_f32() < pass_stick_y
    {
        // Param-based plat drop lockout window for Ryu, Ken, Terry, and Kazuya
        if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_RYU || fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_KEN || fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_DOLLY || fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_DEMON {
            let dash_pass_disable_frame = ParamModule::get_int(fighter.object(), ParamType::Agent, "dash_pass_disable_frame");
            if fighter.global_table[CURRENT_FRAME].get_i32() >= dash_pass_disable_frame {
                interrupt!(fighter, *FIGHTER_STATUS_KIND_PASS, true);
            }
        }
        // Normal behavior for all other fighters
        else{
            interrupt!(fighter, *FIGHTER_STATUS_KIND_PASS, true);
        }
    }

    if fighter.sub_transition_group_check_ground_attack().get_bool() {
        VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
        return L2CValue::I32(1);
    }

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
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
                VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
                return L2CValue::I32(1);
            } else if ItemModule::get_pickable_item_size(fighter.module_accessor) == ITEM_SIZE_LIGHT
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH) &&
            {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM);
                app::sv_module_access::item(fighter.lua_state_agent);
                fighter.pop_lua_stack(1).get_bool()
            } {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
                VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
                return L2CValue::I32(1);
            }
        }
    }

    let is_dash_input: bool = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0;
    let is_backdash_input: bool = fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) <= ParamModule::get_float(fighter.object(), ParamType::Common, "dashback_stick_x")
                                && fighter.global_table[FLICK_X].get_i32() <= WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dash_flick_x"));

    if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_AFTER_DASH_TO_RUN_FRAME)  // if after dash-to-run transition frame
    && WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_stick_x")) <= fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor)  // AND stick_x is >= run threshold
    && !is_dash_input {  // AND you haven't input another dash
        interrupt!(fighter, FIGHTER_STATUS_KIND_RUN, true);
    }
    if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_DASH_TO_RUN_FRAME) {
        VarModule::off_flag(fighter.battle_object, vars::common::status::IS_DASH_TO_RUN_FRAME);
        VarModule::on_flag(fighter.battle_object, vars::common::status::IS_AFTER_DASH_TO_RUN_FRAME);
    }

    interrupt_if!(fighter.sub_transition_group_check_ground_jump().get_bool());
    
    // Disables dashbacks when stick falls below threshold
    // For ease of moonwalking
    let moonwalk_disable_dashback_stick_y = ParamModule::get_float(fighter.battle_object, ParamType::Common, "moonwalk_disable_dashback_stick_y");
    if fighter.global_table[STICK_Y].get_f32() <= moonwalk_disable_dashback_stick_y
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH)  // if backdash transition term is enabled (it is by default)
    && is_backdash_input {  // AND is a backdash input
        //println!("transition to backdash");
        let perfect_pivot_window = ParamModule::get_int(fighter.object(), ParamType::Common, "dash_perfect_pivot_window");
        if fighter.global_table[CURRENT_FRAME].get_i32() <= perfect_pivot_window {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
        }
        else {
            VarModule::off_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
        }
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
        VarModule::on_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL);
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
    if fighter.global_table[CURRENT_FRAME].get_i32() == 1  // if you are on f2 of current dash
    && StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_TURN 
    && StatusModule::prev_status_kind(fighter.module_accessor, 1) == *FIGHTER_STATUS_KIND_DASH  // AND you are in a backdash
    && stick_x.abs() < dash_stick_x {  // AND stick_x < dash stick threshold
        // trigger late pivot
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_LATE_PIVOT);
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        interrupt!(fighter, FIGHTER_STATUS_KIND_TURN, true);
    }

    ok!()
}

#[skyline::hook(replace = L2CFighterCommon_sub_dash_uniq_process_main_internal)]
unsafe fn sub_dash_uniq_process_main_internal(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if !WorkModule::is_enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH) {
        let stick_x = fighter.left_stick_x();
        let walk_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x206138766c);
        let lr = PostureModule::lr(fighter.module_accessor);
        let is_backdash = if param_1.get_bool() { -1.0 } else { 1.0 };
        if stick_x * lr * is_backdash < walk_threshold {
            WorkModule::enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
            WorkModule::enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
            WorkModule::enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
    }
    let dash_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_COUNT);
    WorkModule::set_int(fighter.module_accessor, dash_count + 1, *FIGHTER_STATUS_DASH_WORK_INT_COUNT);
    let dash_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_COUNT);
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_TURN_DASH
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH_ON) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FROM_DASH_COUNT);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4)
    && FighterControlModuleImpl::get_param_dash_s4_frame(fighter.module_accessor) as i32 <= dash_count {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_ESCAPE) {
        let dash_escape_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dash_escape_frame"));
        if dash_escape_frame <= dash_count as f32 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_ESCAPE);
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_SLIP) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_SLIP);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FRAME);
    }
    let turn_dash_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FRAME);
    if 0 <= turn_dash_frame {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FRAME);
        if turn_dash_frame - 1 < 0 {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
        }
    }
    let retry_dash_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_RETRY_TURN_DASH_FRAME);
    if 0 <= retry_dash_frame {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_RETRY_TURN_DASH_FRAME);
        if retry_dash_frame - 1 < 0 {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
        }
    }
    let attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME);
    if 0 <= attack_frame {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME);
        if attack_frame - 1 < 0 {
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY);
        }
    }
    let invalid_attack_escape_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME);
    if 0 < invalid_attack_escape_frame {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME);
        if invalid_attack_escape_frame - 1 == 0 {
            let transitions = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT
            ];
            for val in transitions.iter() {
                WorkModule::enable_transition_term(fighter.module_accessor, *val);
            }
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Dash)]
unsafe fn status_end_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
	let mut initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);

	if [*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_WALK].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
        let applied_speed_clamped = initial_speed.clamp(-run_speed_max, run_speed_max);

		initial_speed = applied_speed_clamped;
	}
    if StatusModule::status_kind_next(fighter.module_accessor) == *FIGHTER_STATUS_KIND_GUARD_ON {
        fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);
    }
    if StatusModule::status_kind_next(fighter.module_accessor) == *FIGHTER_STATUS_KIND_TURN {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_LATE_PIVOT) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT) {
                VarModule::off_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
                let dash_speed: f32 = WorkModule::get_param_float(fighter.module_accessor, hash40("dash_speed"), 0);
                let speed_mul = ParamModule::get_float(fighter.object(), ParamType::Common, "perfect_pivot_speed_mul");
                let pivot_boost = dash_speed * speed_mul * PostureModule::lr(fighter.module_accessor) * -1.0;
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, pivot_boost);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            }
        }
    }
    else {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
    }

    if VarModule::is_flag(fighter.battle_object, vars::common::status::APPLY_DASH_END_SPEED_MUL) {
        let applied_speed_clamped = initial_speed.clamp(-run_speed_max, run_speed_max);
        let dash_end_speed_mul = ParamModule::get_float(fighter.battle_object, ParamType::Common, "dash_end_speed_mul");

        fighter.clear_lua_stack();
		lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed_clamped * dash_end_speed_mul);
		app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
    VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, initial_speed);

    if StatusModule::status_kind_next(fighter.module_accessor) == *FIGHTER_STATUS_KIND_RUN {
        let mut hip_translate = Vector3f::zero();
        MotionModule::joint_local_tra(fighter.module_accessor, Hash40::new("hip"), false, &mut hip_translate);
        VarModule::set_float(fighter.battle_object, vars::common::instance::DASH_HIP_OFFSET_X, hip_translate.z);
    }

    call_original!(fighter)
}

// FIGHTER_STATUS_KIND_TURN_DASH

#[skyline::hook(replace = L2CFighterCommon_status_pre_TurnDash)]
unsafe fn status_pre_turndash(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("pre turndash");
    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN);
    return 1.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_DashCommon,
            status_Dash_sub,
            status_dash,
            sub_dash_uniq_process_main_internal,
            status_end_dash,
            status_dash_main_common,
            status_pre_turndash
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}