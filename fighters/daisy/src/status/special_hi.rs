use super::*;

unsafe extern "C" fn special_hi_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exit);
}