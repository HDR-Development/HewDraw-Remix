use super::*;

// reimplements most of steve's guard statuses so that he cannot spawn the table during them

// FIGHTER_STATUS_KIND_GUARD_ON

unsafe extern "C" fn guardon_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_GuardOn()
}

pub unsafe extern "C" fn guardon_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.boma(), *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION);
    fighter.sub_status_guard_on_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(guardon_main_loop as *const () as _))
}

pub unsafe extern "C" fn guardon_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOn_Main();
    0.into()
}

unsafe extern "C" fn guardon_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_GuardOn()
}
 
// FIGHTER_STATUS_KIND_GUARD

unsafe extern "C" fn guard_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Guard()
}

pub unsafe extern "C" fn guard_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // keep shield article visible while shielding
    if !ArticleModule::is_exist(fighter.boma(), *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF){
        ArticleModule::generate_article(fighter.boma(), *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, false, -1);
        ArticleModule::set_rate(fighter.boma(), *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, 0.0);
    }
    WorkModule::off_flag(fighter.boma(), *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION);

    fighter.sub_status_guard_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(guard_main_loop as *const () as _))
}

pub unsafe extern "C" fn guard_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Guard_Main();
    0.into()
}

unsafe extern "C" fn guard_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Guard()
}

// FIGHTER_STATUS_KIND_GUARD_DAMAGE

// prevent steve's shield from being locked in place after it is damaged (vanilla bug) 
unsafe extern "C" fn guarddamage_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.boma(), *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION);

    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_GUARD_DAMAGE)(fighter)
}

unsafe extern "C" fn guarddamage_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_GuardDamage()
}

// FIGHTER_STATUS_KIND_GUARD_OFF 

unsafe extern "C" fn guardoff_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    // removes steve's shield after an attempted parry
    if fighter.motion_frame() < 2.0 
    && ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }

    fighter.sub_ftStatusUniqProcessGuardOff_execStatus()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_GUARD_ON, guardon_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD_ON, guardon_main);
    agent.status(End, *FIGHTER_STATUS_KIND_GUARD_ON, guardon_end);

    agent.status(Pre, *FIGHTER_STATUS_KIND_GUARD, guard_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD, guard_main);
    agent.status(End, *FIGHTER_STATUS_KIND_GUARD, guard_end);

    agent.status(Pre, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guarddamage_pre);
    agent.status(End, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guarddamage_end);

    agent.status(Exec, *FIGHTER_STATUS_KIND_GUARD_OFF, guardoff_exec);
}