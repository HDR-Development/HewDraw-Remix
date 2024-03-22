use super::*;
use globals::*;

unsafe extern "C" fn special_hi_open_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        agent.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        agent.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_hi_open_main(agent: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(agent.battle_object, vars::gamewatch::instance::UP_SPECIAL_PARACHUTE);
    ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_PARACHUTE, false, -1);
    ArticleModule::change_motion(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, Hash40::new("special_hi_open"), false, -1.0);
    MotionModule::change_motion(agent.module_accessor, Hash40::new("special_hi_open"), 0.0, 1.0, false, 0.0, false, false);
    agent.sub_shift_status_main(L2CValue::Ptr(special_hi_open_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_open_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        let status = if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            { FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL } else { FIGHTER_STATUS_KIND_LANDING };
        agent.change_status(status.into(), true.into());
        return 1.into()
    }
    agent.sub_air_check_dive();
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool()
        || agent.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(agent.module_accessor) || agent.status_frame() > 45 {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    return 0.into()
}

unsafe extern "C" fn special_hi_open_exit(agent: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_PARACHUTE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::gamewatch::SPECIAL_HI_OPEN, special_hi_open_pre);
    agent.status(Main, statuses::gamewatch::SPECIAL_HI_OPEN, special_hi_open_main);
    agent.status(End, statuses::gamewatch::SPECIAL_HI_OPEN, special_hi_open_exit);
}
