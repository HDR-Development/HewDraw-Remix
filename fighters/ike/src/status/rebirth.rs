use super::*;

unsafe extern "C" fn ike_rebirth_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_REBIRTH, ike_rebirth_end);
}
