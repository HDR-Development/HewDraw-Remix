use super::*;

// FIGHTER_STATUS_KIND_REBIRTH

pub unsafe extern "C" fn rebirth_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, -1);
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_AIR_FOLLOW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_REBIRTH)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_end);
}
