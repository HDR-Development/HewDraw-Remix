use super::*;

// statuses::pickel::SPECIAL_RUN

pub unsafe extern "C" fn special_run_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_WARP as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    
    return false.into();
}

const FIGHTER_TEAM_2ND_PICKEL_TROLLEY: i32 = 0x1f;

pub unsafe extern "C" fn special_run_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_run"), 0.0, 1.0, false, 0.0, false, false);

    let team_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) + FIGHTER_TEAM_2ND_PICKEL_TROLLEY;
    TeamModule::set_team_second(fighter.module_accessor, team_id);
    TeamModule::set_hit_team_second(fighter.module_accessor, team_id);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_run_main_loop as *const () as _))
}

pub unsafe extern "C" fn special_run_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue{
    fighter.change_status(FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_RIDE.into(), false.into());

    return false.into();
}

pub unsafe extern "C" fn special_run_end(fighter: &mut L2CFighterCommon) -> L2CValue{
    KineticModule::clear_speed_all(fighter.module_accessor);
    
    return false.into();
}

// FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_FAILED

pub unsafe extern "C" fn special_s_failed_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_DASH | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_DASH as u32,
        0
    );
    
    return false.into();
}

pub unsafe extern "C" fn special_s_failed_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    special_s_failed_situation_helper(fighter);
    fighter.sub_change_motion_by_situation(
        Hash40::new("special_s_failed").into(), 
        Hash40::new("special_s_failed").into(), 
        false.into()
    );
    // fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack_dash_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_failed_main_loop as *const () as _))
}

pub unsafe extern "C" fn special_s_failed_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue{
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_s_failed_situation_helper(fighter);
            return false.into();
        }
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return true.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into();
    }

    return false.into();
}

pub unsafe extern "C" fn special_s_failed_situation_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_turn_smoke"), false, false);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_landing_smoke"), false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::pickel::SPECIAL_RUN, special_run_pre);
    agent.status(Main, statuses::pickel::SPECIAL_RUN, special_run_main);
    agent.status(End, statuses::pickel::SPECIAL_RUN, special_run_end);

    agent.status(Pre, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_FAILED, special_s_failed_pre);
    agent.status(Main, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_FAILED, special_s_failed_main);
}