use super::*;

unsafe extern "C" fn szerosuit_rebirth_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install() {
    Agent::new("snake")
        .status(End, *FIGHTER_STATUS_KIND_REBIRTH, szerosuit_rebirth_end)
        .install();
}
