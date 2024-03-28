use super::*;
use globals::*;
 
// FIGHTER_STATUS_KIND_GUARD

// keep shield article visible while shielding
pub unsafe extern "C" fn guard_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !ArticleModule::is_exist(fighter.boma(), *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF){
        ArticleModule::generate_article(fighter.boma(), *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, false, -1);
        ArticleModule::set_rate(fighter.boma(), *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, 0.0);
    }

    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_GUARD)(fighter)
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

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD, guard_main);
    agent.status(Pre, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guarddamage_pre);
    agent.status(End, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guarddamage_end);
}