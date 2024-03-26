use super::*;
use globals::*;
// status script import

pub unsafe extern "C" fn attack_hi4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_GROUND as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT,
        (*FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_NO_REACTION),
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_HI4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_4 as u32,
        0,
    );
    0.into()
}

pub unsafe extern "C" fn attack_hi4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackHi4_common(L2CValue::Hash40s("attack_hi4"));
    // fighter.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    // let restart_frame = fighter.get_float(*FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    // MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("attack_hi4"), restart_frame, 1.0, 0.0);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
    attack_hi_set_kinetic(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_hi4_main_loop as *const () as _))
}

unsafe extern "C" fn attack_hi4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        attack_hi_set_kinetic(fighter);
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) 
        && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
        if fighter.is_situation(*SITUATION_KIND_AIR) 
        && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    if MotionModule::trans_move_speed(fighter.module_accessor).y < 0.0 
    && fighter.sub_transition_group_check_air_landing().get_bool() {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn attack_hi_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        // KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        // KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
}
pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_HI4, attack_hi4_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI4, attack_hi4_main);
}