use super::*;

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON)  as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    
    0.into()
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    sv_kinetic_energy!(clear_speed_ex, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.set_int(1, *FIGHTER_PEACH_STATUS_SPECIAL_S_WORK_INT_ENABLE_UNIQ);
    if !StopModule::is_stop(fighter.module_accessor) {
        special_s_check_flick(fighter);
    }
    VarModule::off_flag(fighter.battle_object, vars::daisy::instance::SPECIAL_S_GROUND_START);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        VarModule::on_flag(fighter.battle_object, vars::daisy::instance::SPECIAL_S_GROUND_START);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(special_s_check_flick as *const () as _));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) { Hash40::new("special_s_start") } else { Hash40::new("special_air_s_start") };
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);

    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP.into(), false.into());
    }

    return 0.into();
}

unsafe extern "C" fn special_s_check_flick(fighter: &mut L2CFighterCommon) -> L2CValue {
    let flick_x = ControlModule::get_flick_no_reset_x(fighter.module_accessor);
    if flick_x <= fighter.get_param_int("common", "special_smash_flick_x") {
        fighter.on_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_FLAG_FLICK_START);
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(0x20cbc92683));
        fighter.push_lua_stack(&mut L2CValue::I32(1));
        fighter.push_lua_stack(&mut L2CValue::I32(*FIGHTER_LOG_DATA_INT_HAJIKI_NUM));
        app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        fighter.set_int(0, *FIGHTER_PEACH_STATUS_SPECIAL_S_WORK_INT_ENABLE_UNIQ);
    }

    return 0.into();
}

unsafe extern "C" fn special_s_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_S_JUMP,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_s_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::daisy::instance::DISABLE_SPECIAL_S);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_jump"), 0.0, 1.0, false, 0.0, false, false);
    fighter.main_shift(special_s_jump_main_loop)
}

unsafe extern "C" fn special_s_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) || fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT);
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_s_away_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END)(fighter);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(0.75, 1.0, 1.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ID_GROUND);
    }
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(Pre, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_pre);
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_main);
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END, special_s_away_end_main);
}