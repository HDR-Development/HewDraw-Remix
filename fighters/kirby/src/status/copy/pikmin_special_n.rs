use super::*;

// FIGHTER_KIRBY_STATUS_KIND_PIKMIN_SPECIAL_N

// In vanilla, when Kirby uses it in the air it's not considered to have failed
// since it was not possible to fail when grounded.
// Now that the grounded version has a "failure" variant,
// the aerial version is now handled in the new "failure" status.

pub unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    
    // Fails to pluck another Pikmin if there are 2 in play or in the air.
    if ArticleModule::get_active_num(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN) >= 3
    || fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        // println!("there are 3 or more or is in the air so welp");
        fighter.change_status(statuses::kirby::PIKMIN_SPECIAL_N_FAILURE.into(), true.into());
    }

    StatusModule::init_settings(fighter.module_accessor,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into()
}


pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_KIRBY_STATUS_KIND_PIKMIN_SPECIAL_N, special_n_pre);
}