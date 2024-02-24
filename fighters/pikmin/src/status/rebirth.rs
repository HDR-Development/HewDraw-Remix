use super::*;
use globals::*;

pub unsafe extern "C" fn rebirth_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, -1);
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_REBIRTH)(fighter)
}

pub fn install() {
    smashline::Agent::new("pikmin")
        .status(End, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_end)
        .install();
}
