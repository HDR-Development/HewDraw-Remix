use super::*;


unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

// unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
//     special_hi_check_exit(fighter);
//     return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
// }

// unsafe extern "C" fn special_hi_turn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
//     special_hi_check_exit(fighter);
//     return smashline::original_status(End, fighter, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN)(fighter);
// }

unsafe extern "C" fn special_hi_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, vars::gaogaen::instance::SPECIAL_HI_ENABLE_FREEFALL) {
        if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
            // if air jump has been used, restore it and enable exit check logic
            WorkModule::set_int(fighter.module_accessor, fighter.get_jump_count_max() - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            VarModule::on_flag(fighter.battle_object, vars::gaogaen::instance::SPECIAL_HI_ENABLE_FREEFALL);
        }
    }

    return smashline::original_status(Main, fighter, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL)(fighter);
}

unsafe extern "C" fn special_hi_fall_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_hi_check_exit(fighter);
    return smashline::original_status(End, fighter, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL)(fighter);
}

unsafe extern "C" fn special_hi_loop_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_hi_check_exit(fighter);
    return smashline::original_status(End, fighter, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_LOOP)(fighter);
}

unsafe fn special_hi_check_exit(fighter: &mut L2CFighterCommon) {
    if !(&[
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN,
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL,
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_LOOP,
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_END,
    ]).contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
        if VarModule::is_flag(fighter.battle_object, vars::gaogaen::instance::SPECIAL_HI_ENABLE_FREEFALL) {
            // if the jump has been restored from the dive, take it away again
            WorkModule::set_int(fighter.module_accessor, fighter.get_jump_count_max(), *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        VarModule::on_flag(fighter.battle_object, vars::gaogaen::instance::SPECIAL_HI_ENABLE_FREEFALL);
    }
}

unsafe extern "C" fn special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if VarModule::is_flag(fighter.battle_object, vars::gaogaen::instance::SPECIAL_HI_ENABLE_FREEFALL) {
    //     KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    //     KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
    //     StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL);
    //     return 1.into();
    // }

    // prevent future jump restoration this airtime
    VarModule::on_flag(fighter.battle_object, vars::gaogaen::instance::SPECIAL_HI_ENABLE_FREEFALL);

    return smashline::original_status(Pre, fighter, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_END)(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);

    //agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end);
    //agent.status(End, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN, special_hi_turn_end);
    
    agent.status(Main, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL, special_hi_fall_main);
    agent.status(End, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL, special_hi_fall_end);

    agent.status(End, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_LOOP, special_hi_loop_end);

    agent.status(Pre, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_END, special_hi_end_pre);
}