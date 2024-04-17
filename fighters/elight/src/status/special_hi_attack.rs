use super::*;

// FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1

unsafe extern "C" fn special_hi_attack1_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_RESET,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    0.into()
}

// FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2

unsafe extern "C" fn special_hi_attack2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_RESET,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_hi_attack1_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] special_hi_attack1 and special_hi_attack2 both use the same inner block they just start
    //      with different motions
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi1"), 0.0, 1.0, false, 0.0, false, false);
    fighter.main_shift(special_hi_attack_main_loop)
}

unsafe extern "C" fn special_hi_attack2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] special_hi_attack1 and special_hi_attack2 both use the same inner block they just start
    //      with different motions
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi2"), 0.0, 1.0, false, 0.0, false, false);
    fighter.main_shift(special_hi_attack_main_loop)
}

unsafe extern "C" fn special_hi_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] check if you have grabbed a ledge
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    // [v] no extra logic, just transition immediatley into the finish script
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_FINISH.into(), false.into());
    }

    0.into()
}

unsafe extern "C" fn special_hi_attack1_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] empty status
    0.into()
}

unsafe extern "C" fn special_hi_attack2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] empty status
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1, special_hi_attack1_pre);
    agent.status(Pre, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2, special_hi_attack2_pre);
    agent.status(Main, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1, special_hi_attack1_main);
    agent.status(Main, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2, special_hi_attack2_main);
    agent.status(End, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1, special_hi_attack1_end);
    agent.status(End, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2, special_hi_attack2_end);
}