use super::*;

// FIGHTER_STATUS_KIND_REBIRTH

unsafe extern "C" fn rebirth_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX) {
        ArticleModule::shoot(
            fighter.module_accessor,
            *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
            false
        );
    }
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_end);
}