// status imports
use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_transition_group_check_air_tread_jump,
            status_treadjump,
            sub_tread_jump_uniq_check
        );
    }
}

unsafe fn is_button_tread_jump(fighter: &mut L2CFighterCommon) -> bool {
    // input check
    if !fighter.is_cat_flag(CatHdr::TreadJump) {
        return false;
    }

    // lockout
    let previous_lockout_frame = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME);
    let footstool_lockout_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "footstool_lockout_frame");
    fighter.set_int(footstool_lockout_frame, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME);
    if dbg!(previous_lockout_frame) != 0 {
        return false;
    }

    // transition term
    if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_BUTTON) {
        return false;
    }

    // check speed limit
    let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
    let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
    if tread_jump_speed_limit > tread_speed_y {
        return false;
    }

    // magic bullshit
    fighter.clear_lua_stack();
    lua_args!(fighter, 0x21bfbd3f83u64);
    smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    return fighter.pop_lua_stack(1).get_bool();
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_transition_group_check_air_tread_jump)]
pub unsafe fn sub_transition_group_check_air_tread_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cont = if fighter.global_table[0x30].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x30].get_ptr());
        callable(fighter).get_bool()
    } else {
        false
    };
    if cont {
        return true.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        return false.into();
    }

    if is_button_tread_jump(fighter) {
        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
        return true.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_NO_TRIGGER) {
        fighter.clear_lua_stack();
        lua_args!(fighter, 0x21bfbd3f83u64, true);
        smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), false.into());
            return true.into();
        }
    }
    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_status_TreadJump)]
unsafe fn status_treadjump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
        fighter.on_flag(*FIGHTER_STATUS_TREAD_FLAG_BUTTON);
        ControlModule::reset_trigger(fighter.module_accessor);
    } else {
        ControlModule::reset_flick_y(fighter.module_accessor);
    }
    fighter.inc_int(*FIGHTER_INSTANCE_WORK_ID_INT_TREAD_JUMP_COUNT);

    let tread_jump_disable_frame = fighter.get_param_int("common", "tread_jump_disable_frame");
    fighter.set_int(tread_jump_disable_frame, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME);
    fighter.set_int(*FIGHTER_STATUS_JUMP_FROM_TREAD, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_JUMP_FROM);
    fighter.sub_tread_jump_unique_process_init_inner();

    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_tread_jump_uniq_check();
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_tread_jump_uniq_check as *const () as _));

    let mut tread_attack_frame = fighter.get_param_int("common", "tread_attack_frame");
    if MotionModule::is_flag_start_1_frame(fighter.module_accessor) {
        tread_attack_frame -= 1;
    }
    fighter.set_float(tread_attack_frame as f32, *FIGHTER_STATUS_TREAD_WORK_FLOAT_ATTACK_FRAME);

    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_TreadJump_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_sub_tread_jump_uniq_check)]
unsafe fn sub_tread_jump_uniq_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TREAD_FLAG_NO_REACTION) {
        let jump_mini = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TREAD_FLAG_BUTTON) {
            // If any valid footstool button is held, do not turn on the short hop flag
            ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
        } else {
            let jump_neutral_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y"));
            fighter.global_table[STICK_Y].get_f32() < jump_neutral_y
        };

        if jump_mini {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    return false.into();
}
