use super::*;

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_FALL,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

// FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END

unsafe extern "C" fn special_lw_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION);
    if start_situation == *SITUATION_KIND_AIR
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        return 0.into();
    }
    smashline::original_status(Init, fighter, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END)(fighter)
}

unsafe extern "C" fn special_lw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END)(fighter);

    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION) == *SITUATION_KIND_AIR {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(0.8, 1.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        else {
            // Allows you to slide when landing late into Falcon Kick's animation
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION);
        }
    }

    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);

    agent.status(Init, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, special_lw_end_init);
    agent.status(Main, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, special_lw_end_main);
}